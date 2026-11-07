// Videos and sounds played in the viewer.

use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

use tiny_skia::Pixmap;

use super::walled::{self, Ask, Playing, Probe, RATE};

const CHUNK: usize = RATE as usize / 50;
const AHEAD: usize = RATE as usize;
const SPEAKER_LAG: f64 = 0.06;
const PIPE: i32 = 8192;
const FPS_MOST: f64 = 30.0;
const FRAME_MOST: (u32, u32) = (1920, 1080);
const SETTLE: Duration = Duration::from_millis(250);

static VOLUME: AtomicU32 = AtomicU32::new(0x3f80_0000);
static MUTED: AtomicBool = AtomicBool::new(false);

type Repaint = Box<dyn Fn() + Send>;
static REPAINT: Mutex<Option<Repaint>> = Mutex::new(None);

pub fn set_repaint(call: impl Fn() + Send + 'static) {
    if let Ok(mut r) = REPAINT.lock() {
        *r = Some(Box::new(call));
    }
}

fn repaint() {
    if let Ok(r) = REPAINT.lock() {
        if let Some(call) = r.as_ref() {
            call();
        }
    }
}

pub fn volume() -> f32 {
    f32::from_bits(VOLUME.load(Ordering::Relaxed))
}

pub fn set_volume(v: f32) {
    VOLUME.store(v.clamp(0.0, 1.0).to_bits(), Ordering::Relaxed);
    MUTED.store(false, Ordering::Relaxed);
}

pub fn muted() -> bool {
    MUTED.load(Ordering::Relaxed)
}

pub fn set_muted(on: bool) {
    MUTED.store(on, Ordering::Relaxed);
}

fn gain() -> f32 {
    if muted() {
        0.0
    } else {
        volume().powi(3)
    }
}

fn speaker_here() -> bool {
    if cfg!(test) {
        return false;
    }
    static P: OnceLock<bool> = OnceLock::new();
    *P.get_or_init(|| std::env::var_os("PATH").is_some_and(|p| std::env::split_paths(&p).any(|d| d.join("pw-play").is_file())))
}

fn speaker(video: bool) -> Option<Child> {
    if !speaker_here() {
        return None;
    }
    let rate = RATE.to_string();
    Command::new("pw-play")
        .args(["--raw", "--format", "s16", "--rate", &rate, "--channels", "2", "--latency", "30ms",
               "--media-role", if video { "Movie" } else { "Music" }, "-"])
        .stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null())
        .spawn().ok()
}

pub fn frame_size(size: (u32, u32), room: (f32, f32)) -> (u32, u32) {
    let (w, h) = (size.0.max(1) as f32, size.1.max(1) as f32);
    let k = (room.0 / w).min(room.1 / h).min(FRAME_MOST.0 as f32 / w).min(FRAME_MOST.1 as f32 / h).min(1.0);
    let even = |x: f32| ((x.round() as u32) & !1).max(2);
    (even(w * k), even(h * k))
}

fn fps_for(own: Option<f64>) -> f64 {
    own.filter(|f| f.is_finite() && *f >= 1.0).unwrap_or(FPS_MOST).min(FPS_MOST)
}

#[derive(Default)]
struct Ring {
    first: u64,
    data: VecDeque<[i16; 2]>,
    done: bool,
}

impl Ring {
    fn take(&mut self, at: u64, n: usize) -> Vec<[i16; 2]> {
        let stale = at.saturating_sub(self.first).min(self.data.len() as u64) as usize;
        self.data.drain(..stale);
        self.first += stale as u64;
        if self.first > at {
            return vec![[0, 0]; ((self.first - at) as usize).min(n)];
        }
        let n = n.min(self.data.len());
        self.first += n as u64;
        self.data.drain(..n).collect()
    }
}

fn loudness(frames: &[[i16; 2]], gain: f32) -> Vec<u8> {
    let mut out = Vec::with_capacity(frames.len() * 4);
    for f in frames {
        for s in f {
            let v = if gain >= 1.0 { *s } else { (*s as f32 * gain).round() as i16 };
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

struct Shared {
    stop: AtomicBool,
    began: Instant,
    sound: bool,
    zero: OnceLock<Instant>,
    heard: AtomicU64,
    heard_at: AtomicU64,
    ring: Mutex<Ring>,
    moved: Condvar,
    frames_read: AtomicU64,
    samples_read: AtomicU64,
    video_done: AtomicBool,
    wake: Arc<AtomicBool>,
    sound_done: AtomicBool,
}

impl Shared {
    fn new(sound: bool, wake: Arc<AtomicBool>) -> Shared {
        Shared {
            stop: AtomicBool::new(false), began: Instant::now(), sound, zero: OnceLock::new(),
            heard: AtomicU64::new(0), heard_at: AtomicU64::new(0), ring: Mutex::new(Ring::default()), moved: Condvar::new(),
            frames_read: AtomicU64::new(0), samples_read: AtomicU64::new(0),
            video_done: AtomicBool::new(false), sound_done: AtomicBool::new(false), wake,
        }
    }

    fn stopped(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    fn clock(&self) -> f64 {
        if self.sound {
            let heard = self.heard.load(Ordering::Relaxed) as f64 / RATE as f64;
            let at = Duration::from_micros(self.heard_at.load(Ordering::Relaxed));
            let since = self.began.elapsed().saturating_sub(at).as_secs_f64().min(0.05);
            if self.heard.load(Ordering::Relaxed) == 0 { 0.0 } else { heard + since }
        } else {
            self.zero.get().map_or(0.0, |z| z.elapsed().as_secs_f64())
        }
    }

    fn hear(&self, frames: u64) {
        self.heard.store(frames, Ordering::Relaxed);
        self.heard_at.store(self.began.elapsed().as_micros() as u64, Ordering::Relaxed);
    }
}

struct Run {
    from: f64,
    one: bool,
    video: bool,
    playing: Playing,
    speaker: Option<Child>,
    shared: Arc<Shared>,
}

impl Run {
    fn finished(&self) -> bool {
        let s = &self.shared;
        (!self.video || s.video_done.load(Ordering::Relaxed)) && (self.one || !s.sound || s.sound_done.load(Ordering::Relaxed))
    }
}

impl Drop for Run {
    fn drop(&mut self) {
        self.shared.stop.store(true, Ordering::Relaxed);
        self.shared.moved.notify_all();
        if let Some(mut child) = self.speaker.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

pub struct Player {
    file: PathBuf,
    pub info: Probe,
    frames: Option<(u32, u32, f64)>,
    run: Option<Run>,
    at: f64,
    pub frame: Arc<Mutex<Option<Pixmap>>>,
    waiting: Option<(f64, bool)>,
    started: Option<Instant>,
    pub failed: bool,
    pub wake: Arc<AtomicBool>,
    ended: bool,
}

impl Drop for Player {
    fn drop(&mut self) {
        self.run = None;
    }
}

impl Player {
    pub fn new(file: &Path, info: Probe, room: (f32, f32)) -> Player {
        let frames = info.video.map(|(w, h, fps)| {
            let (fw, fh) = frame_size((w, h), room);
            (fw, fh, fps_for(fps))
        });
        Player { file: file.to_path_buf(), info, frames, run: None, at: 0.0, frame: Default::default(), waiting: None, started: None, wake: Default::default(),
                 failed: false, ended: false }
    }

    pub fn video(&self) -> Option<(u32, u32)> {
        self.frames.map(|(w, h, _)| (w, h))
    }

    pub fn length(&self) -> Option<f64> {
        self.info.length
    }

    pub fn playing(&self) -> bool {
        self.waiting.map_or(self.run.as_ref().is_some_and(|r| !r.one), |w| w.1)
    }

    pub fn position(&self) -> f64 {
        let at = match (&self.run, self.waiting) {
            (Some(r), None) if !r.one => r.from + r.shared.clock(),
            _ => self.at,
        };
        self.length().map_or(at, |l| at.min(l))
    }

    #[cfg(test)]
    pub fn boxed_id(&self) -> Option<u32> {
        self.run.as_ref().map(|r| r.playing.id())
    }

    pub fn play(&mut self) {
        if self.failed {
            return;
        }
        if self.ended || self.length().is_some_and(|l| self.at >= l - 0.05) {
            self.at = 0.0;
        }
        self.ended = false;
        self.want(self.at, true);
    }

    pub fn pause(&mut self) {
        self.at = self.position();
        if let Some((to, _)) = self.waiting {
            self.waiting = Some((to, false));
        }
        if self.run.as_ref().is_some_and(|r| !r.one) {
            self.run = None;
        }
    }

    pub fn toggle(&mut self) {
        if self.playing() {
            self.pause();
        } else {
            self.play();
        }
    }

    pub fn seek(&mut self, to: f64) {
        let to = self.length().map_or(to, |l| to.min(l)).max(0.0);
        let play = self.playing();
        self.at = to;
        self.ended = false;
        self.want(to, play);
    }

    fn want(&mut self, to: f64, play: bool) {
        self.run = None;
        if self.started.is_some_and(|s| s.elapsed() < SETTLE) {
            self.waiting = Some((to, play));
            self.wake.store(true, Ordering::Relaxed);
            return;
        }
        self.waiting = None;
        self.start(to, !play);
    }

    pub fn tick(&mut self) {
        if let Some((to, play)) = self.waiting {
            self.wake.store(true, Ordering::Relaxed);
            if !self.started.is_some_and(|s| s.elapsed() < SETTLE) {
                self.waiting = None;
                self.start(to, !play);
            }
            return;
        }
        let length = self.length();
        let Some(run) = &mut self.run else { return };
        if run.speaker.as_mut().is_some_and(|c| !matches!(c.try_wait(), Ok(None))) {
            run.speaker = None;
        }
        let read = run.shared.frames_read.load(Ordering::Relaxed) + run.shared.samples_read.load(Ordering::Relaxed);
        let ended = run.playing.ended();
        if read == 0 && ended == Some(false) {
            self.failed = true;
            self.run = None;
            return;
        }
        if run.finished() {
            if read == 0 && ended.is_none() {
                self.wake.store(true, Ordering::Relaxed);
                return;
            }
            if !run.one {
                self.at = length.unwrap_or(run.from + run.shared.clock());
                self.ended = true;
            }
            self.run = None;
        }
    }

    fn start(&mut self, from: f64, one: bool) {
        if one && self.frames.is_none() {
            return;
        }
        self.started = Some(Instant::now());
        let speaker = if one || !self.info.sound { None } else { speaker(self.frames.is_some()) };
        let ask = Ask { from, frames: self.frames, sound: self.info.sound && !one, one };
        let Some(mut playing) = walled::play(&self.file, &ask, self.length()) else {
            self.failed = true;
            return;
        };
        let shared = Arc::new(Shared::new(ask.sound, self.wake.clone()));
        if let (Some(out), Some((w, h, fps))) = (playing.frames.take(), self.frames) {
            let room = (w as usize * h as usize * 4).min(1 << 20) as libc::c_int;
            unsafe { libc::fcntl(out.as_raw_fd(), libc::F_SETPIPE_SZ, room) };
            let (s, slot) = (shared.clone(), self.frame.clone());
            let _ = std::thread::Builder::new().name("folders-frames".into())
                .spawn(move || show_frames(out, (w, h), fps, one, &s, &slot));
        }
        let mut speaker = speaker;
        if let Some(sound) = playing.sound.take() {
            let s = shared.clone();
            let _ = std::thread::Builder::new().name("folders-sound-in".into()).spawn(move || read_sound(sound, &s));
            let (s, input) = (shared.clone(), speaker.as_mut().and_then(|c| c.stdin.take()));
            let _ = std::thread::Builder::new().name("folders-sound-out".into()).spawn(move || send_sound(input, &s));
        }
        self.run = Some(Run { from, one, video: self.frames.is_some(), playing, speaker, shared });
    }
}

fn show_frames(mut out: impl Read, (w, h): (u32, u32), fps: f64, one: bool, s: &Shared, slot: &Mutex<Option<Pixmap>>) {
    let len = w as usize * h as usize * 4;
    let mut buf = vec![0u8; len];
    let mut n: u64 = 0;
    let gap = 1.0 / fps;
    let mut last = Instant::now();
    while !s.stopped() && out.read_exact(&mut buf).is_ok() {
        s.frames_read.fetch_add(1, Ordering::Relaxed);
        let t = n as f64 * gap;
        n += 1;
        let first = s.zero.set(Instant::now()).is_ok();
        if !one && !first {
            if t + gap < s.clock() && last.elapsed() < Duration::from_millis(250) {
                continue;
            }
            while s.clock() < t {
                if s.stopped() {
                    return;
                }
                std::thread::sleep(Duration::from_secs_f64((t - s.clock()).clamp(0.001, 0.01)));
            }
        }
        if s.stopped() {
            return;
        }
        let Some(size) = tiny_skia::IntSize::from_wh(w, h) else { break };
        let Some(next) = Pixmap::from_vec(std::mem::take(&mut buf), size) else { break };
        if let Ok(mut shown) = slot.lock() {
            buf = shown.replace(next).map(Pixmap::take).filter(|b| b.len() == len).unwrap_or_default();
        }
        if buf.len() != len {
            buf = vec![0u8; len];
        }
        repaint();
        last = Instant::now();
        if one {
            break;
        }
    }
    s.video_done.store(true, Ordering::Relaxed);
    s.wake.store(true, Ordering::Relaxed);
}

fn read_sound(mut sound: File, s: &Shared) {
    let mut buf = vec![0u8; 16 * 1024];
    let mut part: Vec<u8> = Vec::new();
    loop {
        if s.stopped() {
            break;
        }
        let got = match sound.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };
        part.extend_from_slice(&buf[..got]);
        let whole = part.len() / 4 * 4;
        let frames: Vec<[i16; 2]> = part[..whole].chunks_exact(4)
            .map(|b| [i16::from_le_bytes([b[0], b[1]]), i16::from_le_bytes([b[2], b[3]])]).collect();
        part.drain(..whole);
        s.samples_read.fetch_add(frames.len() as u64, Ordering::Relaxed);
        let Ok(mut ring) = s.ring.lock() else { break };
        ring.data.extend(frames);
        s.moved.notify_all();
        let mut ring = ring;
        while ring.data.len() > AHEAD && !s.stopped() {
            ring = match s.moved.wait_timeout(ring, Duration::from_millis(100)) {
                Ok((r, _)) => r,
                Err(_) => return,
            };
        }
    }
    if let Ok(mut ring) = s.ring.lock() {
        ring.done = true;
    }
    s.moved.notify_all();
}

fn waiting_in(to: &ChildStdin) -> u64 {
    let mut n: libc::c_int = 0;
    let ok = unsafe { libc::ioctl(to.as_raw_fd(), libc::FIONREAD, &mut n) } == 0;
    if ok { n.max(0) as u64 } else { 0 }
}

fn send_sound(mut to: Option<ChildStdin>, s: &Shared) {
    if let Some(t) = &to {
        unsafe { libc::fcntl(t.as_raw_fd(), libc::F_SETPIPE_SZ, PIPE) };
    }
    let lag = (SPEAKER_LAG * RATE as f64) as u64;
    if let Ok(ring) = s.ring.lock() {
        let _ = s.moved.wait_timeout_while(ring, Duration::from_secs(3), |r| r.data.is_empty() && !r.done && !s.stopped());
    }
    let mut sent: u64 = 0;
    let mut paced = Instant::now();
    loop {
        if s.stopped() {
            return;
        }
        let chunk = match next_chunk(s, sent) {
            Chunk::Samples(chunk) => chunk,
            Chunk::End => break,
            Chunk::Quit => return,
        };
        let bytes = loudness(&chunk, gain());
        if let Some(t) = &mut to {
            if t.write_all(&bytes).is_err() {
                to = None;
                paced = Instant::now() - Duration::from_secs_f64(sent as f64 / RATE as f64);
            }
        }
        sent += chunk.len() as u64;
        match &to {
            Some(t) => s.hear(sent.saturating_sub(waiting_in(t) / 4 + lag).max(1)),
            None => {
                let due = paced + Duration::from_secs_f64(sent as f64 / RATE as f64);
                std::thread::sleep(due.saturating_duration_since(Instant::now()));
                s.hear(sent);
            }
        }
    }
    if !hear_the_last(&to, sent, lag, s) {
        return;
    }
    if to.is_some() {
        std::thread::sleep(Duration::from_secs_f64(SPEAKER_LAG));
    }
    s.hear(sent);
    s.sound_done.store(true, Ordering::Relaxed);
    s.wake.store(true, Ordering::Relaxed);
}

enum Chunk {
    Samples(Vec<[i16; 2]>),
    End,
    Quit,
}

fn next_chunk(s: &Shared, sent: u64) -> Chunk {
    let Ok(mut ring) = s.ring.lock() else { return Chunk::Quit };
    if ring.data.is_empty() && !ring.done {
        ring = match s.moved.wait_timeout(ring, Duration::from_millis(15)) {
            Ok((r, _)) => r,
            Err(_) => return Chunk::Quit,
        };
    }
    let got = ring.take(sent, CHUNK);
    s.moved.notify_all();
    if got.is_empty() && ring.done {
        return Chunk::End;
    }
    Chunk::Samples(if got.is_empty() { vec![[0, 0]; CHUNK] } else { got })
}

fn hear_the_last(to: &Option<ChildStdin>, sent: u64, lag: u64, s: &Shared) -> bool {
    while let Some(t) = to {
        if s.stopped() {
            return false;
        }
        let left = waiting_in(t) / 4;
        s.hear(sent.saturating_sub(left + lag).max(1));
        if left == 0 {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_fit_the_room_and_rates_are_capped() {
        assert_eq!(frame_size((1920, 1080), (960.0, 900.0)), (960, 540));
        assert_eq!(frame_size((640, 360), (2000.0, 2000.0)), (640, 360));
        assert_eq!(frame_size((3840, 2160), (5000.0, 5000.0)), (1920, 1080));
        assert_eq!(frame_size((1080, 1920), (800.0, 600.0)), (338, 600));
        assert_eq!(fps_for(Some(59.94)), 30.0);
        assert_eq!(fps_for(Some(24.0)), 24.0);
        assert_eq!(fps_for(None), 30.0);
    }

    #[test]
    fn the_ring_passes_over_late_sound_and_fills_gaps_with_silence() {
        let mut r = Ring { first: 0, data: (0..10).map(|i| [i, -i]).collect(), done: false };
        assert_eq!(r.take(0, 3), vec![[0, 0], [1, -1], [2, -2]]);
        assert_eq!(r.take(6, 2), vec![[6, -6], [7, -7]]);
        assert_eq!(r.first, 8);
        assert!(r.take(20, 4).is_empty() && r.data.is_empty());
        let mut r = Ring { first: 10, data: [[5, 5]; 4].into(), done: false };
        assert_eq!(r.take(8, 4), vec![[0, 0]; 2]);
        assert_eq!(r.take(10, 4), vec![[5, 5]; 4]);
    }

    #[test]
    fn the_volume_scales_every_sample() {
        let f = [[1000i16, -1000], [i16::MAX, i16::MIN]];
        assert_eq!(loudness(&f, 1.0), [232, 3, 24, 252, 255, 127, 0, 128]);
        assert_eq!(loudness(&f, 0.5)[..4], (500i16).to_le_bytes().into_iter().chain((-500i16).to_le_bytes()).collect::<Vec<_>>()[..]);
        assert!(loudness(&f, 0.0).iter().all(|&b| b == 0));
    }

    const PLAYING: [&str; 3] = ["bwrap", "ffmpeg", "ffprobe"];

    fn clip(tag: &str, seconds: u32) -> (PathBuf, PathBuf) {
        crate::common::needs::programs(&PLAYING);
        let d = std::env::temp_dir().join(format!("trindle-play-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        let clip = walled::tests::test_clip(&d, "clip.mkv", seconds).expect("ffmpeg could not make the clip this test plays");
        (d, clip)
    }

    const LONG: Duration = Duration::from_secs(120);

    fn wait_for(mut done: impl FnMut() -> bool, most: Duration) -> bool {
        let start = Instant::now();
        while start.elapsed() < most {
            if done() {
                return true;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        false
    }

    #[test]
    fn a_video_plays_pauses_moves_and_ends() {
        let (d, clip) = clip("video", 3);
        let info = walled::probe(&clip).unwrap();
        let mut p = Player::new(&clip, info, (160.0, 160.0));
        assert_eq!(p.video(), Some((160, 120)));
        assert!(!p.playing());
        p.play();
        assert!(p.playing());
        assert!(wait_for(|| p.position() > 0.0, LONG));
        let (began, from) = (Instant::now(), p.position());
        std::thread::sleep(Duration::from_millis(1000));
        let (moved, took) = (p.position() - from, began.elapsed().as_secs_f64());
        assert!(moved > 0.6 && moved < took + 0.4, "{moved} in {took}");
        let px = p.frame.lock().unwrap().as_ref().map(|px| (px.width(), px.height()));
        assert_eq!(px, Some((160, 120)));
        p.pause();
        assert!(!p.playing() && p.boxed_id().is_none());
        let held = p.position();
        std::thread::sleep(Duration::from_millis(300));
        assert_eq!(p.position(), held);
        std::thread::sleep(SETTLE);
        p.seek(2.0);
        assert!(!p.playing());
        assert!(wait_for(|| { p.tick(); p.boxed_id().is_none() }, LONG));
        assert_eq!(p.position(), 2.0);
        p.play();
        assert!(wait_for(|| { p.tick(); !p.playing() }, LONG));
        assert!(p.position() > 2.9 && !p.failed);
        p.play();
        assert!(p.position() < 1.0, "{}", p.position());
        assert!(wait_for(|| p.boxed_id().is_some(), LONG));
        let pid = p.boxed_id().unwrap();
        drop(p);
        assert!(!Path::new(&format!("/proc/{pid}")).exists());
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_file_that_cannot_be_played_says_so() {
        let (d, clip) = clip("broken", 1);
        let info = walled::probe(&clip).unwrap();
        std::fs::write(&clip, b"not a video at all").unwrap();
        let mut p = Player::new(&clip, info, (160.0, 160.0));
        p.play();
        assert!(wait_for(|| { p.tick(); p.failed || !p.playing() }, LONG));
        assert!(p.failed && !p.playing());
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn a_box_that_gave_nothing_is_judged_by_how_it_ends() {
        crate::common::needs::programs(&PLAYING);
        let d = std::env::temp_dir().join(format!("trindle-play-ends-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        let file = d.join("in.txt");
        std::fs::write(&file, "seen").unwrap();
        let info = Probe { length: Some(3.0), video: Some((320, 240, Some(10.0))), sound: true, cover: false };
        for (script, fails) in [("sleep 2; exit 1", true), ("sleep 2; exit 0", false)] {
            let mut p = Player::new(&file, info.clone(), (160.0, 160.0));
            let mut playing = walled::start(&file, &["/bin/sh".into(), "-c".into(), script.into()], 10, true, true).expect("started");
            let shared = Arc::new(Shared::new(true, p.wake.clone()));
            drop((playing.frames.take(), playing.sound.take()));
            shared.video_done.store(true, Ordering::Relaxed);
            shared.sound_done.store(true, Ordering::Relaxed);
            p.run = Some(Run { from: 0.0, one: false, video: true, playing, speaker: None, shared });
            p.tick();
            assert!(p.playing() && !p.failed && !p.ended, "{script}: judged before the box had ended");
            assert!(p.wake.load(Ordering::Relaxed), "{script}: it is looked at again soon");
            assert!(wait_for(|| { p.tick(); !p.playing() }, LONG), "{script}");
            assert_eq!((p.failed, p.ended), (fails, !fails), "{script}");
        }
        std::fs::remove_dir_all(&d).unwrap();
    }
}
