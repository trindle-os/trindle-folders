// Names a server gives: what is made safe before the name of a thing on a server is shown or used.

use std::io::{Read, Write};

use super::{Entry, Error, Progress, Remote};
use crate::common::files::is_one_name;
use crate::toolkit::t;

pub fn not_a_name() -> &'static str {
    t!("folders-remote-screen-not-a-name")
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Listing {
    pub entries: Vec<Entry>,
    pub refused: Vec<String>,
}

impl Listing {
    pub fn held(&self) -> usize {
        self.entries.len() + self.refused.len()
    }
}

fn shown(name: &str) -> String {
    if name.is_empty() {
        return t!("folders-remote-screen-no-name").into();
    }
    name.chars().map(|c| if c.is_control() { '\u{fffd}' } else { c }).collect()
}

pub struct Screened(Box<dyn Remote>);

impl Screened {
    pub fn over(remote: Box<dyn Remote>) -> Screened {
        Screened(remote)
    }

    pub fn list(&mut self, path: &str) -> Result<Listing, Error> {
        let mut out = Listing::default();
        for e in self.0.list(path)? {
            match e.name.as_str() {
                "." | ".." => {}
                name if is_one_name(name.as_bytes()) => out.entries.push(e),
                name => out.refused.push(shown(name)),
            }
        }
        Ok(out)
    }

    pub fn home(&mut self) -> Result<String, Error> {
        self.0.home()
    }

    pub fn stat(&mut self, path: &str) -> Result<Entry, Error> {
        self.0.stat(path)
    }

    pub fn read(&mut self, path: &str, to: &mut dyn Write, progress: Progress) -> Result<u64, Error> {
        self.0.read(path, to, progress)
    }

    pub fn write_new(&mut self, path: &str, from: &mut dyn Read, size: u64, progress: Progress) -> Result<u64, Error> {
        self.0.write_new(path, from, size, progress)
    }

    pub fn refuses_taken(&self) -> bool {
        self.0.refuses_taken()
    }

    pub fn mkdir(&mut self, path: &str) -> Result<(), Error> {
        self.0.mkdir(path)
    }

    pub fn rmdir(&mut self, path: &str) -> Result<(), Error> {
        self.0.rmdir(path)
    }

    pub fn remove(&mut self, path: &str) -> Result<(), Error> {
        self.0.remove(path)
    }

    pub fn remove_if(&mut self, path: &str, size: u64, modified: Option<i64>) -> Result<bool, Error> {
        self.0.remove_if(path, size, modified)
    }

    pub fn rename(&mut self, from: &str, to: &str) -> Result<(), Error> {
        self.0.rename(from, to)
    }

    pub fn follow(&mut self, path: &str) -> Result<Entry, Error> {
        self.0.follow(path)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Kind;
    use super::*;

    struct Says(Vec<&'static str>);

    impl Remote for Says {
        fn home(&mut self) -> Result<String, Error> {
            Ok("/".into())
        }
        fn list(&mut self, _path: &str) -> Result<Vec<Entry>, Error> {
            Ok(self.0.iter().map(|n| Entry { name: n.to_string(), kind: Kind::File, size: 1, modified: None, mode: None }).collect())
        }
        fn stat(&mut self, path: &str) -> Result<Entry, Error> {
            Err(Error::NotFound(path.into()))
        }
        fn read(&mut self, _path: &str, _to: &mut dyn Write, _progress: Progress) -> Result<u64, Error> {
            Ok(0)
        }
        fn write(&mut self, _path: &str, _from: &mut dyn Read, _size: u64, _progress: Progress) -> Result<u64, Error> {
            Ok(0)
        }
        fn mkdir(&mut self, _path: &str) -> Result<(), Error> {
            Ok(())
        }
        fn rmdir(&mut self, _path: &str) -> Result<(), Error> {
            Ok(())
        }
        fn remove(&mut self, _path: &str) -> Result<(), Error> {
            Ok(())
        }
        fn rename(&mut self, _from: &str, _to: &str) -> Result<(), Error> {
            Ok(())
        }
    }

    #[test]
    fn every_name_a_server_gives_is_one_plain_name_or_is_refused() {
        let bad = ["", "/", "/etc/made-up", "../up.txt", "a/b", "a/", "/a", "sub/../../x", "nought\0in.txt", "\0"];
        let good = ["notes.txt", ".hidden", "..two-dots", "...", "a b", "back\\slash", "~", "-rf", "caf\u{e9}.txt", "a\nb"];
        let mut all: Vec<&'static str> = vec![".", ".."];
        all.extend(bad);
        all.extend(good);
        let mut far = Screened::over(Box::new(Says(all)));
        let got = far.list("/").unwrap();
        assert_eq!(got.entries.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), good);
        assert_eq!(got.refused.len(), bad.len(), "{:?}", got.refused);
        assert_eq!(got.held(), bad.len() + good.len(), "the folder itself and the one above it are not things in it");
        assert_eq!(got.refused[0], "a thing with no name");
        assert_eq!(got.refused[3], "../up.txt");
        assert!(got.refused.iter().all(|r| !r.is_empty() && !r.contains('\0')), "{:?}", got.refused);
        for e in &got.entries {
            let joined = std::path::Path::new("/chosen").join(&e.name);
            assert_eq!(joined.parent(), Some(std::path::Path::new("/chosen")), "{}", e.name);
            assert_eq!(joined.file_name().and_then(|n| n.to_str()), Some(e.name.as_str()));
        }
    }
}
