//! A Folders window run as administrator, started through polkit.

use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};

pub const FLAG: &str = "--as-administrator";

pub const INSTALLED: &str = "/usr/bin/trindle-folders";

pub fn argv(program: &Path, socket: &Path, folder: &Path) -> Option<Vec<String>> {
    let m = std::fs::metadata(program).ok()?;
    if m.uid() != 0 || m.mode() & 0o022 != 0 || !socket.is_absolute() {
        return None;
    }
    Some(vec!["pkexec".into(), program.to_string_lossy().into_owned(), FLAG.into(),
              socket.to_string_lossy().into_owned(), folder.to_string_lossy().into_owned()])
}

pub fn socket() -> Option<PathBuf> {
    let display = PathBuf::from(std::env::var_os("WAYLAND_DISPLAY")?);
    if display.is_absolute() {
        return Some(display);
    }
    Some(PathBuf::from(std::env::var_os("XDG_RUNTIME_DIR")?).join(display))
}

pub fn settle(args: &[String]) -> Option<PathBuf> {
    if unsafe { libc::geteuid() } != 0 {
        return None;
    }
    let mut rest = args.iter().skip_while(|a| *a != FLAG).skip(1);
    let socket = PathBuf::from(rest.next()?);
    let folder = PathBuf::from(rest.next()?);
    let m = std::fs::symlink_metadata(&socket).ok()?;
    if !socket.is_absolute() || !m.file_type().is_socket() || !folder.is_absolute() {
        return None;
    }
    let run = PathBuf::from("/run/user/0");
    let run = if std::fs::metadata(&run).is_ok_and(|m| m.uid() == 0 && m.is_dir() && m.mode() & 0o077 == 0) {
        run
    } else {
        let mut made = *b"/tmp/trindle-folders-root-XXXXXX\0";
        let p = unsafe { libc::mkdtemp(made.as_mut_ptr().cast()) };
        if p.is_null() {
            return None;
        }
        PathBuf::from(std::str::from_utf8(&made[..made.len() - 1]).ok()?)
    };
    std::env::set_var("WAYLAND_DISPLAY", &socket);
    std::env::set_var("XDG_RUNTIME_DIR", &run);
    Some(folder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_root_window_runs_only_the_installed_program_with_nothing_of_the_persons_runtime_folder() {
        let dir = std::env::temp_dir().join(format!("trindle-folders-root-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mine = dir.join("trindle-folders");
        std::fs::write(&mine, b"").unwrap();
        let (socket, folder) = (Path::new("/run/user/1000/wayland-0"), Path::new("/home/user/Documents"));
        assert_eq!(argv(&mine, socket, folder), None, "a program the person owns is run as root");
        let sh = Path::new("/bin/sh");
        let got = argv(sh, socket, folder).unwrap();
        assert_eq!(got, ["pkexec", "/bin/sh", FLAG, "/run/user/1000/wayland-0", "/home/user/Documents"]);
        assert!(!got.iter().any(|a| a.contains("env") || a.contains("XDG_RUNTIME_DIR")), "{got:?}");
        assert_eq!(argv(sh, Path::new("wayland-0"), folder), None);
        assert_eq!(settle(&[FLAG.into(), socket.to_string_lossy().into(), "/".into()]), None, "not root, yet it set itself up as root");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_window_run_as_administrator_opens_no_file_archive_or_preview() {
        use crate::files::app::{Files, Msg};
        use crate::toolkit::ui::App;
        let dir = std::env::temp_dir().join(format!("trindle-folders-root-plain-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("inner")).unwrap();
        std::fs::write(dir.join("note.txt"), b"hello").unwrap();
        let mut f = Files::new(vec![dir.clone()]);
        f.update(Msg::Add(Vec::new()));
        f.as_root = true;
        let p = f.side();
        let i = f.cols[p].entries.iter().position(|e| e.name == "note.txt").unwrap();
        f.update(Msg::Choose(p, i));
        f.open(p, i);
        assert_eq!(f.said, crate::toolkit::t!("folders-app-root-folders-only"));
        f.said.clear();
        f.update(Msg::Preview(crate::files::preview::PreviewMsg::Toggle));
        assert!(f.dialog.is_none() && f.said == crate::toolkit::t!("folders-app-root-folders-only"), "{}", f.said);
        assert!(!f.panel_shows());
        let i = f.cols[p].entries.iter().position(|e| e.name == "inner").unwrap();
        f.open(p, i);
        assert!(matches!(&f.cols[p].at, crate::files::places::Where::Folder(d) if d.ends_with("inner")), "a folder is still opened");
        let _ = std::fs::remove_dir_all(dir);
    }
}
