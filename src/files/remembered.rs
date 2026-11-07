// The network shares the person has mounted, kept so they can be offered again after a restart.

use std::path::{Path, PathBuf};

use crate::common::smb::{self, Mounted};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Remembered {
    pub server: String,
    pub share: String,
    pub at: PathBuf,
}

impl Remembered {
    pub fn is_nfs(&self) -> bool {
        self.share.starts_with('/')
    }

    pub fn name(&self) -> String {
        Mounted { server: self.server.clone(), share: self.share.clone(), at: self.at.clone() }.name()
    }

    pub fn holds(&self, pin: &Path) -> bool {
        pin.starts_with(&self.at)
    }

    pub fn is(&self, server: &str, share: &str) -> bool {
        self.server.eq_ignore_ascii_case(server) && if self.is_nfs() { self.share == share } else { self.share.eq_ignore_ascii_case(share) }
    }
}

#[cfg(not(test))]
pub fn file() -> PathBuf {
    crate::common::files::config_path("folders-shares")
}

pub fn read(file: &Path) -> Vec<Remembered> {
    read_in(file, Path::new(MOUNTS))
}

pub fn from_folder(path: &Path) -> Option<Remembered> {
    let rest = path.strip_prefix(MOUNTS).ok()?;
    let mut parts = rest.components();
    let (user, name) = (parts.next()?, parts.next()?);
    let (share, server) = name.as_os_str().to_str()?.rsplit_once(" on ")?;
    (smb::valid_server(server) && smb::valid_share(share))
        .then(|| Remembered { server: server.into(), share: share.into(), at: Path::new(MOUNTS).join(user).join(name) })
}

const MOUNTS: &str = "/run/media";

fn under(at: &Path, mounts: &Path) -> bool {
    use std::path::Component;
    at.strip_prefix(mounts).is_ok_and(|rest| rest.components().all(|c| matches!(c, Component::Normal(_))) && rest.components().count() >= 2)
}

fn read_in(file: &Path, mounts: &Path) -> Vec<Remembered> {
    let text = crate::common::files::read_file(file).map(|b| String::from_utf8_lossy(&b).into_owned()).unwrap_or_default();
    let mut out: Vec<Remembered> = Vec::new();
    for line in text.lines() {
        let mut f = line.split('\t');
        let (Some(server), Some(share), Some(at), None) = (f.next(), f.next(), f.next(), f.next()) else { continue };
        let good = smb::valid_server(server) && (smb::valid_share(share) || smb::valid_export(share)) && under(Path::new(at), mounts);
        if good && !out.iter().any(|r| r.is(server, share)) {
            out.push(Remembered { server: server.into(), share: share.into(), at: at.into() });
        }
    }
    out
}

pub fn write(file: &Path, list: &[Remembered]) -> std::io::Result<()> {
    if let Some(dir) = file.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let text: String = list.iter().filter(|r| !r.at.to_string_lossy().contains(['\t', '\n']))
        .map(|r| format!("{}\t{}\t{}\n", r.server, r.share, r.at.display())).collect();
    crate::common::files::write_whole(file, text.as_bytes())
}

pub fn note(list: &mut Vec<Remembered>, server: &str, share: &str, at: &Path) -> (Option<PathBuf>, bool) {
    match list.iter_mut().find(|r| r.is(server, share)) {
        Some(r) if r.at == at => (None, false),
        Some(r) => (Some(std::mem::replace(&mut r.at, at.to_path_buf())), true),
        None => {
            list.push(Remembered { server: server.into(), share: share.into(), at: at.into() });
            (None, true)
        }
    }
}

pub fn follow(pin: &Path, old: &Path, new: &Path) -> Option<PathBuf> {
    pin.strip_prefix(old).ok().map(|rest| new.join(rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shares_are_kept_a_line_each_and_bad_lines_are_left_out() {
        let dir = std::env::temp_dir().join(format!("trindle-remembered-{}", std::process::id()));
        let f = dir.join("deeper/folders-shares");
        let list = vec![
            Remembered { server: "nas".into(), share: "Photos".into(), at: "/run/media/jo/Photos on nas".into() },
            Remembered { server: "nas".into(), share: "/srv/media".into(), at: "/run/media/jo/media on nas".into() },
        ];
        write(&f, &list).unwrap();
        assert_eq!(read(&f), list);
        std::fs::write(&f, "nas\tPhotos\t/run/media/jo/a\nbad host\tx\t/run/media/jo/b\nnas\t../x\t/run/media/jo/c\nnas\tPHOTOS\t/run/media/jo/d\nnas\tx\trelative\nnas\tx\n").unwrap();
        let kept = read(&f);
        assert_eq!(kept, [Remembered { server: "nas".into(), share: "Photos".into(), at: "/run/media/jo/a".into() }],
            "the first good line is kept, and the same share twice once");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_folder_outside_the_mounts_is_not_a_remembered_share() {
        let dir = std::env::temp_dir().join(format!("trindle-remembered-out-{}", std::process::id()));
        let f = dir.join("folders-shares");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&f, "nas\tA\t/\nnas\tB\t/home/jo\nnas\tC\t/run/media\nnas\tD\t/run/media/jo\nnas\tE\t/run/media/jo/../../etc\nnas\tF\t/run/media/jo/F on nas\n").unwrap();
        let kept: Vec<String> = read(&f).into_iter().map(|r| r.share).collect();
        assert_eq!(kept, ["F"]);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_share_is_found_by_its_address_and_its_pins_follow_a_new_folder() {
        let mut list = Vec::new();
        let a = Path::new("/run/media/jo/Photos on nas");
        assert_eq!(note(&mut list, "nas", "Photos", a), (None, true));
        assert_eq!(note(&mut list, "NAS", "photos", a), (None, false), "same share, same folder");
        let b = Path::new("/run/media/jo/Photos on nas (2)");
        assert_eq!(note(&mut list, "nas", "Photos", b), (Some(a.to_path_buf()), true));
        assert_eq!(list.len(), 1);
        assert_eq!(follow(&a.join("2024"), a, b), Some(b.join("2024")));
        assert_eq!(follow(Path::new("/home/jo"), a, b), None);
        assert!(list[0].holds(&b.join("x")) && !list[0].holds(a));
    }
}
