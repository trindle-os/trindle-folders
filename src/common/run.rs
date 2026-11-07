// Starting other programs without blocking, with the work handed back to the main thread.

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use zeroize::Zeroizing;

pub use crate::toolkit::common::run::*;

pub fn spawn(argv: &[&str]) -> bool {
    let program = argv.first().and_then(|p| match p.contains('/') {
        true => Some(std::path::PathBuf::from(p)).filter(|p| p.is_file()),
        false => find(p),
    });
    let Some(program) = program else {
        eprintln!("trindle: could not run {}: not found", argv.first().unwrap_or(&""));
        return false;
    };
    let args: Vec<String> = argv[1..].iter().map(|a| a.to_string()).collect();
    let start = move |token: Option<String>| {
        let mut command = std::process::Command::new(&program);
        command.args(&args).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
        if let Some(token) = token {
            command.env("XDG_ACTIVATION_TOKEN", token);
        }
        match command.spawn() {
            Ok(mut child) => drop(std::thread::spawn(move || child.wait())),
            Err(e) => eprintln!("trindle: could not run {}: {e}", program.display()),
        }
    };
    vouched(start);
    true
}

#[derive(Clone)]
pub struct Proc {
    gone: Arc<Mutex<bool>>,
}

impl Proc {
}

impl Proc {
}

pub fn child(mut command: Command, ended: impl FnOnce(i32) + 'static) -> Option<Proc> {
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("trindle: could not run {}: {e}", command.get_program().to_string_lossy());
            return None;
        }
    };
    let proc = Proc { gone: Arc::new(Mutex::new(false)) };
    let gone = proc.gone.clone();
    on_thread(move || {
        use std::os::unix::process::ExitStatusExt;
        settled(&mut child, &gone).map(|s| s.into_raw()).unwrap_or(0)
    }, ended);
    Some(proc)
}

fn settled(child: &mut std::process::Child, gone: &Mutex<bool>) -> Option<std::process::ExitStatus> {
    let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
    unsafe { libc::waitid(libc::P_PID, child.id(), &mut info, libc::WEXITED | libc::WNOWAIT) };
    let mut gone = gone.lock().unwrap_or_else(|e| e.into_inner());
    *gone = true;
    child.wait().ok()
}

pub fn captured(argv: &[String], stdin: Option<String>) -> (bool, String, String) {
    run_fed(argv, stdin.map(|s| Zeroizing::new(s.into_bytes())))
}

#[cfg(not(test))]
pub fn captured_secret(argv: &[&str], secret: Zeroizing<String>) -> (bool, String, String) {
    let argv: Vec<String> = argv.iter().map(|a| a.to_string()).collect();
    let mut line = Zeroizing::new(Vec::with_capacity(secret.len() + 1));
    line.extend_from_slice(secret.as_bytes());
    line.push(b'\n');
    run_fed(&argv, Some(line))
}

fn run_fed(argv: &[String], stdin: Option<Zeroizing<Vec<u8>>>) -> (bool, String, String) {
    let Some(program) = argv.first() else { return (false, String::new(), String::new()) };
    asks_polkit(program);
    let child = Command::new(program).args(&argv[1..])
        .stdin(if stdin.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) => return (false, String::new(), e.to_string()),
    };
    if let (Some(text), Some(mut pipe)) = (stdin, child.stdin.take()) {
        std::thread::spawn(move || {
            let _ = pipe.write_all(&text);
        });
    }
    let mut err = String::new();
    let complaints = child.stderr.take().map(|mut e| std::thread::spawn(move || {
        let mut got = Vec::new();
        let _ = e.read_to_end(&mut got);
        String::from_utf8_lossy(&got).to_string()
    }));
    let mut got = Vec::new();
    if let Some(mut o) = child.stdout.take() {
        let _ = o.read_to_end(&mut got);
    }
    if let Some(Ok(e)) = complaints.map(|t| t.join()) {
        err = e;
    }
    let ok = child.wait().map(|s| s.success()).unwrap_or(false);
    (ok, String::from_utf8_lossy(&got).to_string(), err)
}

#[cfg(not(test))]
pub fn have(program: &str) -> bool {
    find(program).is_some()
}

pub fn find(program: &str) -> Option<std::path::PathBuf> {
    std::env::var_os("PATH")
        .and_then(|paths| std::env::split_paths(&paths).map(|d| d.join(program)).find(|p| p.is_file()))
}

pub fn admin_helper() -> String {
    if let Ok(exe) = std::env::current_exe() {
        for dir in exe.ancestors().skip(1).take(3) {
            let p = dir.join("libexec").join("trindle-admin");
            if p.is_file() {
                return p.to_string_lossy().to_string();
            }
        }
    }
    "/usr/lib/trindle/libexec/trindle-admin".to_string()
}

#[cfg(test)]
pub const NO_PASSWORD: &str = "Not done: no administrator's password was given.";

fn asks_polkit(program: &str) {
    if program == "pkexec" || program.ends_with("/pkexec") {
        super::refused::prime(&[super::refused::ADMIN_ACTION]);
    }
}

pub fn admin_error(err: &str) -> String {
    use super::refused;
    let e = err.trim();
    if e.is_empty() {
        return crate::toolkit::t!("common-refused-no-reason").into();
    }
    if let Some(heard) = refused::heard_pkexec(e) {
        return refused::not_done(heard.settled(|| refused::would_ask(&[refused::ADMIN_ACTION])));
    }
    e.lines().last().unwrap_or(e).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn captured_keeps_output_and_complaints_apart() {
        let (ok, out, err) = captured(&argv(&["sh", "-c", "echo hi; echo no >&2; exit 3"]), None);
        assert!(!ok);
        assert_eq!((out.as_str(), err.as_str()), ("hi\n", "no\n"));
        let (ok, out, _) = captured(&argv(&["cat"]), Some("fed in".into()));
        assert!(ok);
        assert_eq!(out, "fed in");
        assert!(!captured(&argv(&["no-such-program-here"]), None).0);
    }

    #[test]
    fn a_step_that_failed_without_a_word_is_not_a_closed_password_box() {
        for silent in ["", "  \n"] {
            let said = admin_error(silent);
            assert_ne!(said, NO_PASSWORD, "{silent:?}");
            assert_eq!(crate::common::refused::of_words(&said), None, "no refusal is read into it");
            assert!(said.starts_with("Not done") && said.ends_with('.'), "{said}");
        }
        assert_eq!(admin_error("Error executing command as another user: Request dismissed"), NO_PASSWORD);
    }

    #[test]
    fn a_child_is_watched_to_its_end() {
        use std::os::unix::process::ExitStatusExt;
        for (script, crashed) in [("kill -SEGV $$", true), ("true", false)] {
            let mut child = Command::new("sh").args(["-c", script]).spawn().unwrap();
            let status = settled(&mut child, &Mutex::new(false)).map(|s| s.into_raw()).unwrap_or(0);
            assert_eq!((status & 0x7f) != 0, crashed, "{script}");
        }
    }
}
