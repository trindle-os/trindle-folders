// The password and passphrase questions that ssh asks, answered in a Folders window.

use std::cell::RefCell;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use zeroize::Zeroizing;

use crate::toolkit::event::Loop;
use crate::toolkit::t;
use crate::toolkit::window::Display;

pub const NAME: &str = "trindle-askpass";

pub fn program() -> Option<PathBuf> {
    let me = std::env::current_exe().ok()?;
    let link = crate::common::files::scratch().join(NAME);
    if std::fs::read_link(&link).ok().as_deref() == Some(me.as_path()) {
        return Some(link);
    }
    let _ = std::fs::remove_file(&link);
    std::os::unix::fs::symlink(&me, &link).ok()?;
    Some(link)
}

fn question(args: &[String]) -> String {
    let q = args.get(1).map(String::as_str).unwrap_or("").trim().trim_end_matches(':').trim();
    if q.is_empty() { t!("folders-askpass-default-question").into() } else { q.to_string() }
}

fn title_for(q: &str) -> &'static str {
    if q.to_lowercase().contains("passphrase") { t!("folders-askpass-title-passphrase") } else { t!("folders-askpass-title-password") }
}

pub fn main(args: Vec<String>) -> i32 {
    let q = question(&args);
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else { return 1 };
    let answer: Rc<RefCell<Option<Zeroizing<String>>>> = Rc::new(RefCell::new(None));
    let (a, ev) = (answer.clone(), events.clone());
    let check = move |password: Zeroizing<String>, ok: crate::common::prompt::Answer| {
        *a.borrow_mut() = Some(password);
        ok(Ok(()));
    };
    let shown = crate::common::prompt::ask(&events, &display, title_for(&q), &q, check, move |_| ev.quit());
    if !shown {
        return 1;
    }
    events.run();
    let Some(password) = answer.borrow_mut().take() else { return 1 };
    let mut out = std::io::stdout().lock();
    if writeln!(out, "{}", password.as_str()).and_then(|()| out.flush()).is_err() {
        return 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_question_reads_well() {
        let q = |s: &str| question(&["x".into(), s.into()]);
        assert_eq!(q("john@example.com's password: "), "john@example.com's password");
        assert_eq!(title_for(&q("Enter passphrase for key '/k': ")), "Key passphrase");
        assert_eq!(title_for(&q("john@example.com's password: ")), "Server password");
        assert_eq!(question(&["x".into()]), "The server asks for a password.");
    }
}
