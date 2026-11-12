//! Thumbnails of files Trindle does not read itself, made by a program shut inside bubblewrap.

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdout, Command, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use image::{DynamicImage, ImageFormat};

pub const TIME: Duration = Duration::from_secs(10);
const LONGEST: u32 = 12;
const TASKS: usize = 64;
const LOOK: Duration = Duration::from_millis(100);
pub const MEMORY: u64 = 768 << 20;
const TMP: u64 = 16 << 20;
const MOST_OUT: u64 = 4 << 20;
pub(super) const INSIDE: &str = "/thumb/in";

fn program(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH").unwrap_or_else(|| "/usr/bin:/bin".into());
    std::env::split_paths(&path).map(|d| d.join(name)).find(|p| p.is_file())
        .and_then(|p| p.canonicalize().ok())
        .filter(|p| p.starts_with("/usr"))
}

fn bwrap() -> Option<&'static PathBuf> {
    static B: OnceLock<Option<PathBuf>> = OnceLock::new();
    B.get_or_init(|| program("bwrap")).as_ref()
}

fn ffmpeg() -> Option<&'static PathBuf> {
    static F: OnceLock<Option<PathBuf>> = OnceLock::new();
    F.get_or_init(|| program("ffmpeg")).as_ref()
}

fn ffprobe() -> Option<&'static PathBuf> {
    static F: OnceLock<Option<PathBuf>> = OnceLock::new();
    F.get_or_init(|| program("ffprobe")).as_ref()
}

fn pdftoppm() -> Option<&'static PathBuf> {
    static P: OnceLock<Option<PathBuf>> = OnceLock::new();
    P.get_or_init(|| program("pdftoppm")).as_ref()
}

pub fn can_pdf() -> bool {
    bwrap().is_some() && pdftoppm().is_some()
}

pub fn can_video() -> bool {
    bwrap().is_some() && ffmpeg().is_some()
}

pub fn can_play() -> bool {
    can_video() && ffprobe().is_some()
}

#[cfg(test)]
pub fn walls(file: &Path, command: &[OsString]) -> Vec<OsString> {
    walls_and(file, &[], command)
}

pub(super) fn walls_and(file: &Path, more: &[OsString], command: &[OsString]) -> Vec<OsString> {
    walls_of(Some(file), more, command)
}

fn walls_of(file: Option<&Path>, more: &[OsString], command: &[OsString]) -> Vec<OsString> {
    let mut a: Vec<OsString> = Vec::new();
    let mut add = |args: &[&str]| a.extend(args.iter().map(OsString::from));
    add(&["--unshare-all", "--new-session", "--die-with-parent", "--clearenv",
          "--setenv", "PATH", "/usr/bin", "--setenv", "LANG", "C.UTF-8",
          "--setenv", "MALLOC_ARENA_MAX", "2",
          "--ro-bind", "/usr", "/usr", "--ro-bind-try", "/etc/ld.so.cache", "/etc/ld.so.cache",
          "--ro-bind-try", "/etc/alternatives", "/etc/alternatives",
          "--ro-bind-try", "/etc/fonts", "/etc/fonts"]);
    for (link, to) in [("/bin", "usr/bin"), ("/sbin", "usr/sbin"), ("/lib", "usr/lib"), ("/lib64", "usr/lib64")] {
        add(&["--symlink", to, link]);
    }
    let tmp = TMP.to_string();
    add(&["--proc", "/proc", "--dev", "/dev", "--size", &tmp, "--tmpfs", "/tmp", "--dir", "/thumb"]);
    if let Some(file) = file {
        a.push("--ro-bind".into());
        a.push(file.as_os_str().to_owned());
        a.push(INSIDE.into());
    }
    a.extend(more.iter().cloned());
    a.extend(["--chdir", "/tmp", "--"].map(OsString::from));
    a.extend(command.iter().cloned());
    a
}

fn video_command(ffmpeg: &Path, most: u32) -> Vec<OsString> {
    let fit = format!("scale={most}:{most}:force_original_aspect_ratio=decrease,thumbnail=30");
    let mut c = vec![ffmpeg.as_os_str().to_owned()];
    c.extend(["-nostdin", "-hide_banner", "-loglevel", "error", "-threads", "1", "-i", INSIDE,
              "-an", "-sn", "-dn", "-filter_threads", "1", "-vf", &fit, "-frames:v", "1",
              "-f", "image2pipe", "-c:v", "png", "-"].map(OsString::from));
    c
}

fn pdf_command(pdftoppm: &Path, page: u32, most: u32) -> Vec<OsString> {
    let (most, page) = (most.to_string(), page.max(1).to_string());
    let mut c = vec![pdftoppm.as_os_str().to_owned()];
    c.extend(["-f", &page, "-l", &page, "-singlefile", "-scale-to", &most, "-png", INSIDE].map(OsString::from));
    c
}

pub fn video(file: &Path, most: u32) -> Option<DynamicImage> {
    png(&run(file, &video_command(ffmpeg()?, most))?, most)
}

pub fn pdf(file: &Path, most: u32) -> Option<DynamicImage> {
    pdf_page(file, 1, most)
}

pub fn pdf_page(file: &Path, page: u32, most: u32) -> Option<DynamicImage> {
    png(&run(file, &pdf_command(pdftoppm()?, page, most))?, most)
}

const SVG_MARK: [u8; 8] = *b"\x00\xffTSVG\x01\n";
const SVG_MARK_WITHIN: usize = 1 << 16;
pub const SVG_SIDE: u32 = 2048;
const SVG_OUT: u64 = 24 << 20;
#[cfg(test)]
const SVG_CHILD: &str = "TRINDLE_SVG_CHILD";

pub fn can_svg() -> bool {
    bwrap().is_some()
}

pub fn svg(file: &Path, most: u32) -> Option<DynamicImage> {
    let most = most.clamp(1, SVG_SIDE);
    let data = crate::toolkit::picture::svg_bytes(file)?;
    #[cfg(not(test))]
    let (more, run): (Vec<OsString>, Vec<OsString>) = (Vec::new(), vec!["draw-svg".into(), most.to_string().into()]);
    #[cfg(test)]
    let (more, run): (Vec<OsString>, Vec<OsString>) = (vec!["--setenv".into(), SVG_CHILD.into(), most.to_string().into()],
        ["--exact", "files::walled::tests::the_svg_child", "--nocapture", "--test-threads=1"].map(OsString::from).to_vec());
    let out = run_boxed(boxed_self(&more, &run, MEMORY, TIME.as_secs())?, Some(data), TIME, SVG_OUT)?;
    let at = out.windows(SVG_MARK.len()).take(SVG_MARK_WITHIN).position(|w| w == SVG_MARK)? + SVG_MARK.len();
    png(&out[at..], most)
}

pub fn svg_main(args: Vec<String>) -> i32 {
    match args.get(1).and_then(|most| most.parse().ok()) {
        Some(most) => svg_child(most),
        None => 2,
    }
}

fn svg_child(most: u32) -> i32 {
    use crate::toolkit::picture;
    use image::ImageEncoder;
    use std::io::Write;
    let mut data = Vec::new();
    if std::io::Read::take(std::io::stdin().lock(), picture::SVG_MOST + 1).read_to_end(&mut data).is_err() || data.len() as u64 > picture::SVG_MOST {
        return 1;
    }
    let Some(img) = picture::svg(&data, most.clamp(1, SVG_SIDE)) else { return 1 };
    let rgba = img.to_rgba8();
    let mut out = SVG_MARK.to_vec();
    let coded = image::codecs::png::PngEncoder::new_with_quality(&mut out, image::codecs::png::CompressionType::Fast, image::codecs::png::FilterType::NoFilter)
        .write_image(rgba.as_raw(), rgba.width(), rgba.height(), image::ExtendedColorType::Rgba8);
    let mut to = std::io::stdout().lock();
    if coded.is_ok() && to.write_all(&out).and_then(|()| to.flush()).is_ok() { 0 } else { 1 }
}

fn boxed(file: &Path, more: &[OsString], command: &[OsString], cpu: u64) -> Option<Command> {
    boxed_and(file, more, command, MEMORY, cpu)
}

pub(super) fn boxed_and(file: &Path, more: &[OsString], command: &[OsString], memory: u64, cpu: u64) -> Option<Command> {
    held(walls_and(file, more, command), memory, cpu)
}

pub(crate) const SELF: &str = "/thumb/trindle";

pub(crate) fn this_program() -> std::io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    match exe.to_str().and_then(|e| e.strip_suffix(" (deleted)")) {
        Some(now) if !exe.exists() => Ok(PathBuf::from(now)),
        _ => Ok(exe),
    }
}

pub(crate) fn boxed_self(more: &[OsString], run: &[OsString], memory: u64, cpu: u64) -> Option<Command> {
    let mut all: Vec<OsString> = vec!["--ro-bind".into(), this_program().ok()?.into_os_string(), SELF.into()];
    all.extend(more.iter().cloned());
    let command: Vec<OsString> = std::iter::once(OsString::from(SELF)).chain(run.iter().cloned()).collect();
    held(walls_of(None, &all, &command), memory, cpu)
}

fn held(walls: Vec<OsString>, memory: u64, cpu: u64) -> Option<Command> {
    let mut cmd = Command::new(bwrap()?);
    let filter = seccomp_fd()?;
    let number = filter.as_raw_fd();
    cmd.args(["--unshare-user", "--disable-userns", "--seccomp"]).arg(number.to_string());
    cmd.args(walls);
    let memory = libc::rlimit { rlim_cur: memory, rlim_max: memory };
    let cpu = libc::rlimit { rlim_cur: cpu, rlim_max: cpu };
    unsafe {
        cmd.pre_exec(move || {
            let _keep = &filter;
            libc::setrlimit(libc::RLIMIT_AS, &memory);
            libc::setrlimit(libc::RLIMIT_CPU, &cpu);
            if libc::fcntl(number, libc::F_SETFD, 0) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    Some(cmd)
}

fn seccomp_fd() -> Option<OwnedFd> {
    let made = crate::common::seccomp::memfd()?;
    let high = unsafe { libc::fcntl(made.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 10) };
    if high < 0 {
        return None;
    }
    Some(unsafe { OwnedFd::from_raw_fd(high) })
}

fn png(bytes: &[u8], most: u32) -> Option<DynamicImage> {
    let img = image::load_from_memory_with_format(bytes, ImageFormat::Png).ok()?;
    (img.width() <= most && img.height() <= most && img.width() > 0).then_some(img)
}

fn run(file: &Path, command: &[OsString]) -> Option<Vec<u8>> {
    run_for(file, command, TIME)
}

fn run_for(file: &Path, command: &[OsString], time: Duration) -> Option<Vec<u8>> {
    run_with(file, &[], command, time, MOST_OUT)
}

fn run_with(file: &Path, more: &[OsString], command: &[OsString], time: Duration, most_out: u64) -> Option<Vec<u8>> {
    run_boxed(boxed(file, more, command, TIME.as_secs())?, None, time, most_out)
}

fn run_boxed(mut cmd: Command, input: Option<Vec<u8>>, time: Duration, most_out: u64) -> Option<Vec<u8>> {
    cmd.stdin(if input.is_some() { Stdio::piped() } else { Stdio::null() }).stdout(Stdio::piped()).stderr(Stdio::null());
    let mut child = cmd.spawn().ok()?;
    if let (Some(input), Some(mut to)) = (input, child.stdin.take()) {
        std::thread::spawn(move || {
            let _ = std::io::Write::write_all(&mut to, &input);
        });
    }
    let out = child.stdout.take()?;
    let reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = out.take(most_out + 1).read_to_end(&mut buf);
        buf
    });
    let start = Instant::now();
    let mut had = Had::new(start);
    let ok = loop {
        let late = match child.try_wait() {
            Ok(Some(status)) => break status.success(),
            Ok(None) => {
                let now = Instant::now();
                had.look(child.id(), now);
                out_of_time(had.own, now.duration_since(start), time)
            }
            Err(_) => true,
        };
        if late {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let buf = reader.join().ok()?;
    (ok && !buf.is_empty() && buf.len() as u64 <= most_out).then_some(buf)
}

fn out_of_time(own: Duration, gone: Duration, time: Duration) -> bool {
    own >= time || gone >= time * LONGEST
}

fn own_of(gone: Duration, worked: Duration, waited: Duration) -> Duration {
    worked.min(gone).max(gone.saturating_sub(waited))
}

fn tasks_of(pid: u32) -> HashMap<u32, (u64, u64)> {
    let mut found = HashMap::new();
    let mut todo = vec![pid];
    let mut looked = 0;
    while let Some(p) = todo.pop() {
        looked += 1;
        if looked > TASKS {
            break;
        }
        let Ok(tasks) = std::fs::read_dir(format!("/proc/{p}/task")) else { continue };
        for task in tasks.flatten() {
            let dir = task.path();
            let counts = std::fs::read_to_string(dir.join("schedstat")).ok().and_then(|s| {
                let mut figures = s.split_whitespace().map(|w| w.parse::<u64>().ok());
                Some((figures.next()??, figures.next()??))
            });
            if let (Some(counts), Some(tid)) = (counts, task.file_name().to_str().and_then(|n| n.parse::<u32>().ok())) {
                found.insert(tid, counts);
            }
            if let Ok(children) = std::fs::read_to_string(dir.join("children")) {
                todo.extend(children.split_whitespace().filter_map(|w| w.parse::<u32>().ok()));
            }
        }
    }
    found
}

struct Had {
    own: Duration,
    at: Instant,
    tasks: HashMap<u32, (u64, u64)>,
}

impl Had {
    fn new(start: Instant) -> Had {
        Had { own: Duration::ZERO, at: start, tasks: HashMap::new() }
    }

    fn look(&mut self, pid: u32, now: Instant) {
        let gone = now.duration_since(self.at);
        if gone < LOOK {
            return;
        }
        let tasks = tasks_of(pid);
        let (mut worked, mut waited) = (0u64, 0u64);
        for (tid, (on, wait)) in &tasks {
            let (was_on, was_waiting) = self.tasks.get(tid).copied().unwrap_or((0, 0));
            worked += on.saturating_sub(was_on);
            waited += wait.saturating_sub(was_waiting);
        }
        self.own += own_of(gone, Duration::from_nanos(worked), Duration::from_nanos(waited));
        self.at = now;
        self.tasks = tasks;
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Probe {
    pub length: Option<f64>,
    pub video: Option<(u32, u32, Option<f64>)>,
    pub sound: bool,
    pub cover: bool,
}

fn probe_command(ffprobe: &Path) -> Vec<OsString> {
    let mut c = vec![ffprobe.as_os_str().to_owned()];
    c.extend(["-v", "error", "-of", "json", "-show_format", "-show_streams", INSIDE].map(OsString::from));
    c
}

pub fn probe(file: &Path) -> Option<Probe> {
    read_probe(&run(file, &probe_command(ffprobe()?))?)
}

pub fn read_probe(json: &[u8]) -> Option<Probe> {
    let v: serde_json::Value = serde_json::from_slice(json).ok()?;
    let num = |x: &serde_json::Value| x.as_f64().or_else(|| x.as_str().and_then(|s| s.trim().parse().ok()));
    let ratio = |x: &serde_json::Value| {
        let (a, b) = x.as_str()?.split_once(['/', ':'])?;
        let (a, b) = (a.trim().parse::<f64>().ok()?, b.trim().parse::<f64>().ok()?);
        (a > 0.0 && b > 0.0).then_some(a / b)
    };
    let streams = v.get("streams")?.as_array()?;
    let kind = |s: &serde_json::Value, k: &str| s.get("codec_type").and_then(|t| t.as_str()) == Some(k);
    let still = |s: &serde_json::Value| s.pointer("/disposition/attached_pic").and_then(|a| a.as_i64()) == Some(1);
    let mut p = Probe { sound: streams.iter().any(|s| kind(s, "audio")), ..Probe::default() };
    p.cover = streams.iter().any(|s| kind(s, "video") && still(s));
    if let Some(s) = streams.iter().find(|s| kind(s, "video") && !still(s)) {
        let (w, h) = (s.get("width").and_then(num).unwrap_or(0.0), s.get("height").and_then(num).unwrap_or(0.0));
        let shape = s.get("sample_aspect_ratio").and_then(ratio).unwrap_or(1.0);
        let turn = s.get("side_data_list").and_then(|l| l.as_array()).into_iter().flatten()
            .find_map(|d| d.get("rotation").and_then(num))
            .or_else(|| s.pointer("/tags/rotate").and_then(num)).unwrap_or(0.0);
        let (mut w, mut h) = ((w * shape).round(), h);
        if (turn.abs().round() as i64) % 180 == 90 {
            (w, h) = (h, w);
        }
        let fps = s.get("avg_frame_rate").and_then(ratio).or_else(|| s.get("r_frame_rate").and_then(ratio));
        if (1.0..100_000.0).contains(&w) && (1.0..100_000.0).contains(&h) {
            p.video = Some((w as u32, h as u32, fps));
        }
    }
    p.length = v.pointer("/format/duration").and_then(num).filter(|d| d.is_finite() && *d > 0.0);
    (p.video.is_some() || p.sound).then_some(p)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ask {
    pub from: f64,
    pub frames: Option<(u32, u32, f64)>,
    pub sound: bool,
    pub one: bool,
}

pub const RATE: u32 = 48_000;

fn play_command(ffmpeg: &Path, ask: &Ask) -> Vec<OsString> {
    let mut c = vec![ffmpeg.as_os_str().to_owned()];
    let from = format!("{:.3}", ask.from.max(0.0));
    c.extend(["-nostdin", "-hide_banner", "-loglevel", "error", "-threads", "2", "-ss", &from, "-i", INSIDE, "-sn", "-dn"]
        .map(OsString::from));
    if let Some((w, h, fps)) = ask.frames {
        let fit = format!("fps=fps={fps:.3}:start_time=0,scale={w}:{h}:force_original_aspect_ratio=decrease,\
                           pad={w}:{h}:(ow-iw)/2:(oh-ih)/2,setsar=1");
        c.extend(["-map", "0:V:0", "-filter_threads", "1", "-vf", &fit].map(OsString::from));
        if ask.one {
            c.extend(["-frames:v", "1"].map(OsString::from));
        }
        c.extend(["-f", "rawvideo", "-pix_fmt", "bgra", "pipe:1"].map(OsString::from));
    }
    if ask.sound && !ask.one {
        let rate = RATE.to_string();
        c.extend(["-map", "0:a:0", "-af", "aresample=async=1:first_pts=0", "-f", "s16le", "-ac", "2", "-ar", &rate, "pipe:3"]
            .map(OsString::from));
    }
    c
}

pub struct Playing {
    child: Child,
    pub frames: Option<ChildStdout>,
    pub sound: Option<File>,
}

impl Playing {
    pub fn ended(&mut self) -> Option<bool> {
        self.child.try_wait().ok().flatten().map(|s| s.success())
    }

    #[cfg(test)]
    pub fn id(&self) -> u32 {
        self.child.id()
    }
}

impl Drop for Playing {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub fn play(file: &Path, ask: &Ask, length: Option<f64>) -> Option<Playing> {
    let command = play_command(ffmpeg()?, ask);
    let left = length.map_or(3600.0, |l| (l - ask.from).max(0.0));
    start(file, &command, (left * 4.0) as u64 + 60, ask.frames.is_some(), ask.sound && !ask.one)
}

pub(super) fn start(file: &Path, command: &[OsString], cpu: u64, frames: bool, sound_too: bool) -> Option<Playing> {
    let mut cmd = boxed(file, &[], command, cpu)?;
    cmd.stdin(Stdio::null()).stderr(Stdio::null())
        .stdout(if frames { Stdio::piped() } else { Stdio::null() });
    let mut sound = None;
    if sound_too {
        let mut fds = [0; 2];
        if unsafe { libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
            return None;
        }
        let (read, write) = unsafe { (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])) };
        let w = write.as_raw_fd();
        unsafe {
            cmd.pre_exec(move || {
                if w != 3 && libc::dup2(w, 3) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::fcntl(3, libc::F_SETFD, 0) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        sound = Some((read, write));
    }
    let mut child = cmd.spawn().ok()?;
    let frames = child.stdout.take();
    let sound = sound.map(|(read, write)| {
        drop(write);
        File::from(read)
    });
    Some(Playing { child, frames, sound })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("trindle-walled-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn the_svg_child() {
        if let Some(most) = std::env::var(SVG_CHILD).ok().and_then(|m| m.parse().ok()) {
            std::process::exit(svg_child(most));
        }
    }

    #[test]
    fn an_svg_is_drawn_in_the_box() {
        crate::common::needs::program("bwrap");
        let d = scratch("svg");
        let file = d.join("wide.svg");
        std::fs::write(&file, r##"<svg xmlns="http://www.w3.org/2000/svg" width="40" height="20"><rect width="40" height="20" fill="#0000ff"/></svg>"##).unwrap();
        let img = svg(&file, 20).expect("drawn");
        assert_eq!((img.width(), img.height()), (20, 10));
        assert_eq!(img.to_rgba8().get_pixel(5, 5).0, [0, 0, 255, 255]);
        let img = svg(&file, 4000).expect("drawn");
        assert_eq!((img.width(), img.height()), (40, 20), "never enlarged");
        std::fs::write(&file, "not a picture").unwrap();
        assert!(svg(&file, 20).is_none());
        assert_eq!(svg_main(vec!["trindle".into(), "many".into()]), 2);
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn the_box_shows_only_the_one_file() {
        let file = Path::new("/home/john/Videos/holiday.mp4");
        let a: Vec<String> = walls(file, &["true".into()]).iter().map(|s| s.to_string_lossy().into_owned()).collect();
        let has = |w: &[&str]| a.windows(w.len()).any(|x| x == w);
        for w in ["--unshare-all", "--die-with-parent", "--clearenv", "--new-session"] {
            assert!(a.contains(&w.to_string()), "{w}");
        }
        assert!(!a.contains(&"--share-net".to_string()));
        assert!(has(&["--ro-bind", "/home/john/Videos/holiday.mp4", INSIDE]));
        assert!(has(&["--size", &TMP.to_string(), "--tmpfs", "/tmp"]));
        assert!(!a.iter().any(|x| x == "--bind" || x == "--dev-bind" || x.contains("/run/user") || x.contains("DBUS")));
        assert!(!a.iter().any(|x| x.starts_with("/home") && x != "/home/john/Videos/holiday.mp4"));
        assert_eq!(a.last().map(String::as_str), Some("true"));
    }

    #[test]
    fn the_box_for_trindle_itself_names_no_file_of_the_persons() {
        crate::common::needs::program("bwrap");
        let cmd = boxed_self(&[], &["decode-film".into()], MEMORY, 10).expect("a box");
        let a: Vec<String> = cmd.get_args().map(|s| s.to_string_lossy().into_owned()).collect();
        let has = |w: &[&str]| a.windows(w.len()).any(|x| x == w);
        let me = this_program().unwrap().to_string_lossy().into_owned();
        for w in ["--unshare-all", "--die-with-parent", "--clearenv", "--new-session"] {
            assert!(a.contains(&w.to_string()), "{w}");
        }
        assert!(has(&["--ro-bind", &me, SELF]) && a.ends_with(&[SELF.to_string(), "decode-film".to_string()]));
        let bound: Vec<&String> = a.windows(2).filter(|x| x[0].starts_with("--ro-bind")).map(|x| &x[1]).collect();
        assert!(bound.iter().all(|b| b.starts_with("/usr") || b.starts_with("/etc/") || b.as_str() == me.as_str()), "{bound:?}");
        assert!(!a.contains(&INSIDE.to_string()) && !a.contains(&"--share-net".to_string()));
        assert!(!a.iter().any(|x| x == "--bind" || x == "--dev-bind" || x.contains("/run/user") || x.contains("DBUS")));
    }

    #[test]
    fn every_box_is_given_the_filter_of_refused_calls() {
        crate::common::needs::program("bwrap");
        let file = Path::new("/etc/hostname");
        let run: Vec<OsString> = vec!["/bin/true".into()];
        let all = [boxed(file, &[], &run, 5), boxed_and(file, &[], &run, MEMORY, 5), boxed_self(&[], &run, MEMORY, 5)];
        for (i, cmd) in all.into_iter().enumerate() {
            let args: Vec<String> = cmd.expect("a box").get_args().map(|a| a.to_string_lossy().into_owned()).collect();
            let at = args.iter().position(|a| a == "--seccomp").unwrap_or_else(|| panic!("box {i} has no filter: {args:?}"));
            assert!(args[at + 1].parse::<i32>().is_ok_and(|n| n >= 10), "box {i}: {args:?}");
            assert!(args.iter().any(|a| a == "--disable-userns"), "box {i} lets a program make a user space of its own");
        }
    }

    #[test]
    fn inside_the_box_little_is_reachable() {
        crate::common::needs::program("bwrap");
        let d = scratch("walls");
        let file = d.join("in.txt");
        std::fs::write(&file, "seen").unwrap();
        let sh = |script: &str| -> String {
            let out = run(&file, &["/bin/sh".into(), "-c".into(), script.into()]).unwrap_or_default();
            String::from_utf8_lossy(&out).trim().to_string()
        };
        assert_eq!(sh(&format!("cat {INSIDE}")), "seen");
        assert_eq!(sh(&format!("echo x 2>/dev/null >> {INSIDE} || echo shut")), "shut");
        assert_eq!(sh(&format!("ls {} 2>/dev/null || echo gone", d.display())), "gone");
        assert_eq!(sh("ls /home /root /run 2>/dev/null | head -1; echo end"), "end");
        assert_eq!(sh("echo ${DBUS_SESSION_BUS_ADDRESS:-none}"), "none");
        assert_eq!(sh("ls /sys/class/net 2>/dev/null; cat /proc/net/dev | tail -n +3 | cut -d: -f1 | tr -d ' '"), "lo");
        assert_eq!(sh("touch /tmp/a && echo made"), "made");
        assert_eq!(sh("grep ^Seccomp: /proc/self/status | tr -d '[:space:]'"), "Seccomp:2");
        assert_eq!(sh("touch /usr/a 2>/dev/null || echo shut"), "shut");
        let start = Instant::now();
        let late = run_for(&file, &["/bin/sh".into(), "-c".into(), "sleep 60; echo late".into()], Duration::from_secs(1));
        assert!(late.is_none());
        assert!(start.elapsed() < Duration::from_secs(30), "{:?}", start.elapsed());
        assert_eq!(sh("ulimit -v; ulimit -t"), format!("{}\n{}", MEMORY / 1024, TIME.as_secs()));
        std::fs::remove_dir_all(&d).unwrap();
    }

    fn allowed() -> Option<libc::cpu_set_t> {
        unsafe {
            let mut set: libc::cpu_set_t = std::mem::zeroed();
            (libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &mut set) == 0).then_some(set)
        }
    }

    fn keep_to(set: &libc::cpu_set_t) -> bool {
        unsafe { libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), set) == 0 }
    }

    fn on_a_busy_processor(test: impl FnOnce(bool)) {
        let Some(all) = allowed() else { return test(false) };
        let Some(one) = (0..libc::CPU_SETSIZE as usize).rev().find(|n| unsafe { libc::CPU_ISSET(*n, &all) }) else { return test(false) };
        let mut only: libc::cpu_set_t = unsafe { std::mem::zeroed() };
        unsafe { libc::CPU_SET(one, &mut only) };
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let busy: Vec<_> = (0..2).map(|_| {
            let stop = stop.clone();
            std::thread::spawn(move || {
                let kept = keep_to(&only);
                while kept && !stop.load(std::sync::atomic::Ordering::Relaxed) {
                    std::hint::spin_loop();
                }
            })
        }).collect();
        let kept = keep_to(&only);
        let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| test(kept)));
        stop.store(true, std::sync::atomic::Ordering::Relaxed);
        for b in busy {
            let _ = b.join();
        }
        keep_to(&all);
        if let Err(e) = ran {
            std::panic::resume_unwind(e);
        }
    }

    #[test]
    fn a_boxed_program_is_timed_by_its_own_time_on_a_busy_computer() {
        crate::common::needs::program("bwrap");
        let d = scratch("busy");
        let file = d.join("in.txt");
        std::fs::write(&file, "seen").unwrap();
        on_a_busy_processor(|busy| {
            let work = "for n in 1 2 3; do (ulimit -t 1; while :; do :; done); done; echo done";
            let (limit, start) = (Duration::from_secs(4), Instant::now());
            let out = run_for(&file, &["/bin/sh".into(), "-c".into(), work.into()], limit);
            let took = start.elapsed();
            assert_eq!(out.as_deref(), Some(b"done\n".as_slice()), "three seconds of its own work did not fit a limit of four: stopped after {took:?} by the clock");
            assert!(took >= Duration::from_secs(3), "{took:?}");
            if busy {
                assert!(took > limit, "the processor was not busy, so this showed nothing: {took:?}");
            }
            let (limit, start) = (Duration::from_secs(1), Instant::now());
            let out = run_for(&file, &["/bin/sh".into(), "-c".into(), "while :; do :; done".into()], limit);
            let took = start.elapsed();
            assert!(out.is_none(), "work that never ends was not stopped");
            assert!(took >= limit && took < limit * (LONGEST - 1), "stopped after {took:?} by the clock, not when it had had its {limit:?} of work");
        });
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn time_kept_waiting_for_a_processor_is_not_held_against_it() {
        let (s, ms, none) = (Duration::from_secs, Duration::from_millis, Duration::ZERO);
        assert_eq!(own_of(s(10), none, none), s(10));
        assert_eq!(own_of(s(10), s(10), none), s(10));
        assert_eq!(own_of(s(10), s(80), none), s(10));
        assert_eq!(own_of(s(10), s(3), s(7)), s(3));
        assert_eq!(own_of(s(10), s(10), s(70)), s(10));
        assert_eq!(own_of(s(10), ms(1), ms(1)), s(10) - ms(1));
        assert_eq!(own_of(s(10), s(2), s(5)), s(5));
        assert!(!out_of_time(s(9), s(9), s(10)));
        assert!(out_of_time(s(10), s(10), s(10)), "asleep or at work, it has had its time");
        assert!(!out_of_time(s(9), s(60), s(10)), "fifty-one seconds of it were spent waiting for a turn");
        assert!(out_of_time(s(10), s(61), s(10)));
        assert!(!out_of_time(s(1), s(119), s(10)));
        assert!(out_of_time(s(1), s(120), s(10)), "but never longer than twelve times the limit");
        let start = Instant::now();
        let mut had = Had::new(start);
        had.look(u32::MAX, start + LOOK / 2);
        assert_eq!((had.own, had.at), (none, start));
        had.look(u32::MAX, start + s(2));
        assert_eq!((had.own, had.at), (s(2), start + s(2)));
        assert!(tasks_of(u32::MAX).is_empty());
        let me = std::process::id();
        if std::fs::read_to_string("/proc/self/schedstat").is_ok_and(|w| w.split_whitespace().count() >= 3) {
            let start = Instant::now();
            let mut had = Had { own: none, at: start, tasks: tasks_of(me) };
            assert!(had.tasks.values().any(|(worked, _)| *worked > 0), "{:?}", had.tasks);
            while start.elapsed() < LOOK * 2 {
                std::hint::spin_loop();
            }
            let now = Instant::now();
            had.look(me, now);
            assert!(had.own > none && had.own <= now.duration_since(start), "{:?} of {:?}", had.own, now.duration_since(start));
        }
    }

    #[test]
    fn a_video_gives_a_frame_or_quietly_none() {
        crate::common::needs::programs(&["bwrap", "ffmpeg"]);
        assert!(video(Path::new("/no/such/video.mp4"), 128).is_none());
        let d = scratch("video");
        let clip = d.join("clip.mp4");
        let made = Command::new(crate::common::needs::program("ffmpeg"))
            .args(["-nostdin", "-loglevel", "error", "-f", "lavfi", "-i", "testsrc=size=640x360:rate=10:duration=2",
                   "-pix_fmt", "yuv420p"]).arg(&clip).status();
        assert!(made.is_ok_and(|s| s.success()), "ffmpeg could not make the clip this test is about");
        let img = video(&clip, 128).expect("a frame");
        assert_eq!((img.width(), img.height()), (128, 72));
        let big = d.join("big.mkv");
        let made = Command::new(crate::common::needs::program("ffmpeg"))
            .args(["-nostdin", "-loglevel", "error", "-f", "lavfi", "-i", "testsrc2=size=3840x2160:rate=10:duration=4",
                   "-c:v", "libx264", "-preset", "ultrafast", "-pix_fmt", "yuv420p"]).arg(&big).status();
        if made.is_ok_and(|s| s.success()) {
            let img = video(&big, 256).expect("a frame of a 4K video");
            assert_eq!((img.width(), img.height()), (256, 144));
        }
        let broken = d.join("broken.mp4");
        std::fs::write(&broken, b"not a video at all").unwrap();
        assert!(video(&broken, 128).is_none());
        std::fs::remove_dir_all(&d).unwrap();
    }

    pub(crate) fn a4_pdf(page: &str) -> Vec<u8> {
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
            "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Contents 4 0 R >>".to_string(),
            format!("<< /Length {} >>\nstream\n{page}\nendstream", page.len()),
        ];
        let mut out = b"%PDF-1.4\n".to_vec();
        let mut at = Vec::new();
        for (i, o) in objects.iter().enumerate() {
            at.push(out.len());
            out.extend(format!("{} 0 obj\n{o}\nendobj\n", i + 1).bytes());
        }
        let xref = out.len();
        out.extend(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).bytes());
        for a in at {
            out.extend(format!("{a:010} 00000 n \n").bytes());
        }
        out.extend(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objects.len() + 1).bytes());
        out
    }

    #[test]
    fn a_pdf_gives_its_first_page_or_quietly_none() {
        crate::common::needs::programs(&["bwrap", "pdftoppm"]);
        assert!(pdf(Path::new("/no/such/file.pdf"), 128).is_none());
        let d = scratch("pdf");
        let file = d.join("page.pdf");
        std::fs::write(&file, a4_pdf("0.2 0.4 0.8 rg 100 300 300 300 re f")).unwrap();
        let img = pdf(&file, 128).expect("a page").to_rgb8();
        assert_eq!(img.height(), 128);
        assert!((88..=92).contains(&img.width()), "{}", img.width());
        assert_eq!(img.get_pixel(2, 2).0, [255, 255, 255]);
        let mid = img.get_pixel(img.width() / 2, img.height() / 2).0;
        assert!(mid[2] > 150 && mid[0] < 100, "{mid:?}");
        let mut heavy = String::new();
        for i in 0..25_000u32 {
            let (x, y) = ((i * 37) % 595, (i * 53) % 842);
            heavy.push_str(&format!("{} {} {} RG {x} {y} m {} {} {} {} {} {} c S\n",
                (i % 7) as f32 / 7.0, (i % 5) as f32 / 5.0, (i % 3) as f32 / 3.0,
                (x + 90) % 595, (y + 40) % 842, (x + 20) % 595, (y + 300) % 842, (x + 200) % 595, y));
        }
        let big = d.join("heavy.pdf");
        std::fs::write(&big, a4_pdf(&heavy)).unwrap();
        assert!(pdf(&big, 256).is_some());
        let broken = d.join("broken.pdf");
        std::fs::write(&broken, b"%PDF-1.4\nand then nothing that makes a page").unwrap();
        assert!(pdf(&broken, 128).is_none());
        std::fs::remove_dir_all(&d).unwrap();
    }

    fn words(c: &[OsString]) -> Vec<String> {
        c.iter().map(|s| s.to_string_lossy().into_owned()).collect()
    }

    #[test]
    fn playing_reads_only_the_one_file_and_gives_frames_and_sound() {
        let ask = Ask { from: 12.5, frames: Some((640, 360, 25.0)), sound: true, one: false };
        let c = words(&play_command(Path::new("/usr/bin/ffmpeg"), &ask));
        let has = |w: &[&str]| c.windows(w.len()).any(|x| x == w);
        assert!(has(&["-ss", "12.500", "-i", INSIDE]));
        assert_eq!(c.iter().filter(|x| *x == "-i").count(), 1);
        assert!(!c.iter().any(|x| x.contains("://") || x.starts_with("/home") || x == "-y"));
        assert!(has(&["-map", "0:V:0"]) && has(&["-f", "rawvideo", "-pix_fmt", "bgra", "pipe:1"]));
        assert!(c.iter().any(|x| x.starts_with("fps=fps=25.000:start_time=0,scale=640:360") && x.contains("pad=640:360")));
        assert!(has(&["-map", "0:a:0"]) && has(&["-f", "s16le", "-ac", "2", "-ar", "48000", "pipe:3"]));
        assert!(has(&["-nostdin"]));
        let c = words(&play_command(Path::new("/usr/bin/ffmpeg"), &Ask { one: true, ..ask.clone() }));
        assert!(c.windows(2).any(|w| w == ["-frames:v", "1"]) && !c.contains(&"pipe:3".to_string()));
        let c = words(&play_command(Path::new("/usr/bin/ffmpeg"), &Ask { frames: None, ..ask.clone() }));
        assert!(!c.contains(&"pipe:1".to_string()) && c.contains(&"pipe:3".to_string()));
        let a = words(&walls(Path::new("/home/john/Videos/holiday.mp4"), &play_command(Path::new("/usr/bin/ffmpeg"), &ask)));
        assert!(a.contains(&"--unshare-all".to_string()) && !a.contains(&"--share-net".to_string()));
        assert!(a.windows(3).any(|w| w == ["--ro-bind", "/home/john/Videos/holiday.mp4", INSIDE]));
        assert!(!a.iter().any(|x| x == "--bind" || x.contains("/run/user") || x.contains("pulse") || x.contains("pipewire")));
    }

    #[test]
    fn the_sound_pipe_reaches_into_the_box_and_nothing_else_does() {
        crate::common::needs::program("bwrap");
        let d = scratch("pipe");
        let file = d.join("in.txt");
        std::fs::write(&file, "seen").unwrap();
        let script = "echo frames; echo sound >&3; ls /run/user /run/pulse 2>/dev/null || echo none >&3; \
                      env | grep -ci 'pulse\\|pipewire\\|wayland\\|dbus' >&3";
        let mut p = start(&file, &["/bin/sh".into(), "-c".into(), script.into()], 10, true, true).expect("started");
        let mut out = String::new();
        p.frames.take().unwrap().read_to_string(&mut out).unwrap();
        let mut sound = String::new();
        p.sound.take().unwrap().read_to_string(&mut sound).unwrap();
        assert_eq!(out, "frames\n");
        assert_eq!(sound, "sound\nnone\n0\n");
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn ffprobe_is_read_for_size_turning_cover_and_rate() {
        let json = br#"{"streams":[
            {"codec_type":"video","width":1920,"height":1080,"avg_frame_rate":"30000/1001","sample_aspect_ratio":"1:1",
             "side_data_list":[{"side_data_type":"Display Matrix","rotation":-90}],"disposition":{"attached_pic":0}},
            {"codec_type":"audio","sample_rate":"44100"}],
            "format":{"duration":"62.500000"}}"#;
        let p = read_probe(json).unwrap();
        assert_eq!(p.video.map(|v| (v.0, v.1)), Some((1080, 1920)));
        assert!((p.video.unwrap().2.unwrap() - 29.97).abs() < 0.01);
        assert!(p.sound && !p.cover);
        assert_eq!(p.length, Some(62.5));
        let song = br#"{"streams":[{"codec_type":"audio"},
            {"codec_type":"video","width":500,"height":500,"avg_frame_rate":"0/0","disposition":{"attached_pic":1}}],
            "format":{"duration":"200.1"}}"#;
        let p = read_probe(song).unwrap();
        assert!(p.sound && p.cover && p.video.is_none());
        let wide = br#"{"streams":[{"codec_type":"video","width":720,"height":576,"sample_aspect_ratio":"16:11","r_frame_rate":"25/1"}],"format":{}}"#;
        assert_eq!(read_probe(wide).unwrap().video, Some((1047, 576, Some(25.0))));
        assert_eq!(read_probe(br#"{"streams":[{"codec_type":"subtitle"}],"format":{}}"#), None);
        assert_eq!(read_probe(b"not json"), None);
    }

    pub(crate) fn test_clip(dir: &Path, name: &str, seconds: u32) -> Option<PathBuf> {
        let clip = dir.join(name);
        let made = Command::new(crate::common::needs::program("ffmpeg"))
            .args(["-nostdin", "-loglevel", "error", "-y", "-f", "lavfi", "-i", &format!("testsrc=size=320x240:rate=10:duration={seconds}"),
                   "-f", "lavfi", "-i", &format!("sine=frequency=440:duration={seconds}"), "-pix_fmt", "yuv420p", "-shortest"])
            .arg(&clip).status();
        made.is_ok_and(|s| s.success()).then_some(clip)
    }

    #[test]
    fn a_video_plays_as_frames_and_sound_from_the_box() {
        crate::common::needs::programs(&["bwrap", "ffmpeg", "ffprobe"]);
        let d = scratch("play");
        let clip = test_clip(&d, "clip.mkv", 2).expect("ffmpeg could not make the clip this test plays");
        let info = probe(&clip).expect("what it holds");
        assert_eq!(info.video.map(|v| (v.0, v.1)), Some((320, 240)));
        assert!(info.sound && (info.length.unwrap() - 2.0).abs() < 0.1);
        let ask = Ask { from: 0.5, frames: Some((160, 120, 10.0)), sound: true, one: false };
        let mut p = play(&clip, &ask, info.length).expect("playing");
        let mut sound = p.sound.take().unwrap();
        let reader = std::thread::spawn(move || {
            let mut b = Vec::new();
            sound.read_to_end(&mut b).unwrap();
            b.len()
        });
        let mut frames = Vec::new();
        p.frames.take().unwrap().read_to_end(&mut frames).unwrap();
        let bytes = reader.join().unwrap();
        assert_eq!(frames.len() % (160 * 120 * 4), 0);
        assert!((14..=16).contains(&(frames.len() / (160 * 120 * 4))), "{}", frames.len());
        assert!((bytes as f64 / (RATE * 4) as f64 - 1.5).abs() < 0.1, "{bytes}");
        assert!(frames.chunks_exact(4).all(|px| px[3] == 255));
        assert!(frames.chunks_exact(4).any(|px| px[2] > 200 && px[1] < 50));
        let mut p = play(&clip, &Ask { one: true, ..ask }, info.length).unwrap();
        assert!(p.sound.is_none());
        let mut one = Vec::new();
        p.frames.take().unwrap().read_to_end(&mut one).unwrap();
        assert_eq!(one.len(), 160 * 120 * 4);
        let p = play(&clip, &Ask { from: 0.0, frames: Some((160, 120, 10.0)), sound: true, one: false }, info.length).unwrap();
        let pid = p.id();
        drop(p);
        assert!(!Path::new(&format!("/proc/{pid}")).exists());
        let broken = d.join("broken.mp4");
        std::fs::write(&broken, b"not a video at all").unwrap();
        assert!(probe(&broken).is_none());
        std::fs::remove_dir_all(&d).unwrap();
    }
}
