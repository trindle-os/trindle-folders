// Starting the child in its box and listening to what it sends.

use std::ffi::OsString;
use std::io::{self, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::{mpsc, Mutex};
use std::sync::atomic::Ordering;

use crate::files::walled;
use crate::files::archive::{self, trouble, Item, Need, Trouble};
use crate::files::meter::Meter;
use crate::files::unarchive::{CPU_ALL, CPU_NAMES, FINE, MEMORY, SELF, WATCH};
use crate::files::unarchive::child::{ask, inside};
use crate::files::unarchive::frames::{bad_frame, ended, Body, Frames};

pub(super) fn command(parts: &[PathBuf], need: Need) -> io::Result<Command> {
    let (first, rest) = parts.split_first().ok_or_else(|| trouble(Trouble::Unknown))?;
    let mut more: Vec<OsString> = vec!["--ro-bind".into(), walled::this_program()?.into_os_string(), SELF.into()];
    for (i, part) in rest.iter().enumerate() {
        more.extend(["--ro-bind".into(), part.clone().into_os_string(), inside(i + 1).into_os_string()]);
    }
    let asking = ask(need, parts.len());
    #[cfg(not(test))]
    let run: Vec<OsString> = std::iter::once(SELF).chain(["unarchive"]).chain(asking.split(' ')).map(OsString::from).collect();
    #[cfg(test)]
    let run: Vec<OsString> = [SELF, "--exact", "files::unarchive::tests::the_boxed_child", "--nocapture", "--test-threads=1"].map(OsString::from).to_vec();
    #[cfg(test)]
    more.extend(["--setenv", super::tests::CHILD, &asking].map(OsString::from));
    let cpu = if need == Need::Names { CPU_NAMES } else { CPU_ALL };
    walled::boxed_and(first, &more, &run, MEMORY, cpu).ok_or_else(|| trouble(Trouble::NoReader))
}

pub(super) fn run(
    parts: &[PathBuf],
    need: Need,
    chosen: &[u32],
    meter: &Meter,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    let mut cmd = command(parts, need)?;
    cmd.stdin(if need == Need::Some { Stdio::piped() } else { Stdio::null() }).stdout(Stdio::piped()).stderr(Stdio::null());
    let mut child = crate::common::gentle::program(&mut cmd).spawn().map_err(|_| trouble(Trouble::NoReader))?;
    let (stdin, stdout) = (child.stdin.take(), child.stdout.take());
    let child = Mutex::new(child);
    let nameless = archive::stem(&parts[0]);
    let got = std::thread::scope(|s| {
        let (over, watch) = mpsc::channel::<()>();
        let (stop, child) = (meter.switch(), &child);
        s.spawn(move || {
            while watch.recv_timeout(WATCH) == Err(mpsc::RecvTimeoutError::Timeout) {
                if stop.load(Ordering::Relaxed) {
                    end(child);
                    break;
                }
            }
        });
        let got = talk(stdin, stdout, chosen, meter, &nameless, f);
        drop(over);
        got
    });
    end(&child);
    got
}

pub(super) fn end(child: &Mutex<Child>) {
    if let Ok(mut c) = child.lock() {
        let _ = c.kill();
        let _ = c.wait();
    }
}

fn talk(
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
    chosen: &[u32],
    meter: &Meter,
    nameless: &str,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    if let Some(mut to) = stdin {
        let bytes: Vec<u8> = chosen.iter().flat_map(|n| n.to_le_bytes()).collect();
        let _ = to.write_all(&bytes);
    }
    let out = stdout.ok_or_else(|| trouble(Trouble::NoReader))?;
    listen(out, meter, nameless, f)
}

pub(super) fn listen(
    out: impl Read,
    meter: &Meter,
    nameless: &str,
    f: &mut dyn FnMut(&Item, &mut dyn Read) -> io::Result<bool>,
) -> io::Result<()> {
    let mut frames = Frames { r: BufReader::with_capacity(1 << 17, out), meter, eaten: 0 };
    frames.find_mark()?;
    loop {
        match frames.byte()? {
            Some(b'P') => frames.progress()?,
            Some(b'E') => {
                let code = frames.exact::<1>()?[0];
                return if code == FINE { Ok(()) } else { Err(ended(code)) };
            }
            Some(b'H') => {
                let (item, bytes) = frames.head(nameless)?;
                if !bytes {
                    if !f(&item, &mut io::empty())? {
                        return Ok(());
                    }
                    continue;
                }
                let mut body = Body { frames: &mut frames, left: 0, over: false };
                if !f(&item, &mut body)? {
                    return Ok(());
                }
                io::copy(&mut body, &mut io::sink())?;
            }
            Some(_) => return Err(bad_frame()),
            None => return Err(trouble(Trouble::GaveUp)),
        }
    }
}
