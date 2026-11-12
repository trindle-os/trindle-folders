//! What each row of the menu does once it is chosen.

use std::path::{Path, PathBuf};

use super::Act;
use crate::common::run::spawn;
use crate::common::selection;
use crate::files::app::{Files, Msg};
use crate::files::places::Where;
use crate::files::work;
use crate::toolkit::t;
use crate::toolkit::ui::App;

pub(super) fn view_msg(act: Act) -> Option<Msg> {
    Some(match act {
        Act::Look(v) => Msg::Look(v),
        Act::Columns => Msg::Ask(crate::files::dialogs::AskMsg::Columns),
        Act::SortBy(i) => Msg::SortBy(i),
        Act::GroupBy(i) => Msg::GroupBy(i),
        Act::Flip => Msg::Flip,
        Act::Undo => Msg::Undo,
        Act::Redo => Msg::Redo,
        Act::Settings => Msg::Own(crate::files::settings::OwnMsg::Open),
        _ => return None,
    })
}

fn act_msg(act: Act) -> Option<Msg> {
    Some(match act {
        Act::Cut => Msg::Cut,
        Act::Copy => Msg::CopyClip,
        Act::Paste => Msg::Paste,
        Act::Trash => Msg::Trash,
        Act::Delete => Msg::Delete,
        Act::Rename => Msg::Rename,
        Act::NewDocument => Msg::NewDocument,
        Act::NewFolder => Msg::NewFolder,
        Act::SelectAll => Msg::All,
        Act::SelectNone => Msg::Unpick,
        Act::Invert => Msg::Invert,
        Act::Restore => Msg::Restore,
        Act::EmptyTrash => Msg::EmptyTrash,
        _ => return None,
    })
}

const TEXT_TYPES: [&str; 3] = ["text/plain;charset=utf-8", "text/plain", "UTF8_STRING"];

pub(in crate::files) fn have_terminal() -> bool {
    !crate::common::apps::terminals(&crate::common::apps::entries()).is_empty()
}

pub(in crate::files) fn run_self(part: &str, args: &[&str]) -> bool {
    let exe = std::env::current_exe().ok().filter(|e| e.file_name().is_some_and(|n| n == "trindle"));
    match exe {
        Some(exe) => {
            let exe = exe.to_string_lossy().into_owned();
            let mut argv = vec![exe.as_str(), part];
            argv.extend_from_slice(args);
            spawn(&argv)
        }
        None => {
            let own = format!("trindle-{part}");
            let mut argv = vec![own.as_str()];
            argv.extend_from_slice(args);
            spawn(&argv)
        }
    }
}

fn run_part_captured(part: &str, args: &[&str]) -> (bool, String, String) {
    let exe = std::env::current_exe().ok().filter(|e| e.file_name().is_some_and(|n| n == "trindle"));
    let mut argv: Vec<String> = match exe {
        Some(exe) => vec![exe.to_string_lossy().into_owned(), part.to_string()],
        None => vec![format!("trindle-{part}")],
    };
    argv.extend(args.iter().map(|a| a.to_string()));
    if !argv.is_empty() && !argv[0].contains('/') {
        if let Some(found) = crate::common::run::find(&argv[0]) {
            argv[0] = found.to_string_lossy().into_owned();
        }
    }
    crate::common::run::captured(&argv, None)
}

pub(in crate::files) fn link_name(dir: &Path, target: &Path) -> PathBuf {
    let name = t!("folders-menu-link-name", name = &work::name_of(target));
    let plain = dir.join(&name);
    if crate::files::reach::meta(&plain, false).is_none() {
        return plain;
    }
    work::free_name(dir, name.as_ref(), crate::files::reach::is_dir(target))
}

impl Files {
    pub(in crate::files) fn do_act(&mut self, act: Act, p: usize, items: Vec<PathBuf>) {
        let here = self.cols[p].at.folder().map(Path::to_path_buf);
        let folder = match items.as_slice() {
            [one] if crate::files::reach::is_dir(one) => Some(one.clone()),
            [] => here.clone(),
            _ => None,
        };
        if self.earlier_refuses(act, p, &items) {
            return;
        }
        if matches!(self.cols[p].at, Where::Inside(..)) {
            match act {
                Act::Open => return self.inside_open(p, items),
                Act::ExtractHere => return self.inside_extract(p, items),
                Act::SelectAll | Act::SelectNone | Act::Invert => {}
                _ => return,
            }
        }
        if let Some(msg) = act_msg(act) {
            return self.update(msg);
        }
        match act {
            Act::Open => self.open_items(p, items),
            Act::OpenTab => {
                let Some(f) = folder else { return };
                self.new_tab(p, Some(Where::Folder(f)));
            }
            Act::OpenWindow => {
                let Some(f) = folder else { return };
                if !run_self("folders", &[crate::files::NEW_WINDOW, &f.to_string_lossy()]) {
                    self.say_help(t!("folders-menu-no-new-window"), crate::files::help::NOT_STARTED);
                }
            }
            Act::CopyPath => self.copy_path(p, items, here),
            Act::Pin => {
                if let Some(f) = folder {
                    self.toggle_pin(f);
                }
            }
            Act::Terminal => {
                let Some(f) = folder.or(here) else { return };
                if !self.parts.has_terminal() || !crate::common::apps::start_terminal(Some(&f), &[]) {
                    self.say_help(crate::files::alone::no_terminal(), crate::files::help::PART_MISSING);
                }
            }
            Act::AsRoot => self.open_as_root(folder),
            Act::Wallpaper => self.set_wallpaper(items),
            Act::Link => self.make_link(p, items, here),
            Act::OpenWith | Act::Properties | Act::SendTo | Act::Colour | Act::ShareOnNetwork => self.ask(act, p, items),
            Act::ShareWithAi => self.share_with_ai(items),
            Act::Preview => self.preview_path(p, items.first()),
            Act::Compress | Act::ExtractHere | Act::ExtractTo => self.pack(act, p, items),
            Act::TurnLeft | Act::TurnRight | Act::TurnOver => self.turn_pictures(act, items, here),
            Act::Resize => self.ask_resize(items),
            Act::InstallFont => self.install_fonts(items),
            Act::Wipe => self.ask_wipe(items),
            Act::PinUp | Act::PinDown => self.move_pinned(act, folder),
            _ => self.act_on_what_is_picked(act, p, items, here),
        }
    }

    fn act_on_what_is_picked(&mut self, act: Act, p: usize, items: Vec<PathBuf>, here: Option<PathBuf>) {
        match act {
            Act::Dupes => self.find_dupes(if items.is_empty() { here.into_iter().collect() } else { items }),
            Act::CompareFiles => {
                if let [left, right] = items.as_slice() {
                    let (left, right) = (left.clone(), right.clone());
                    self.compare_files(left, right);
                }
            }
            Act::ShowFolder => {
                if let [one] = items.as_slice() {
                    self.show_in_folder(p, one);
                }
            }
            Act::RingPin => {
                if let Some(path) = items.first().cloned().or(here) {
                    self.ring_toggle(&path);
                }
            }
            Act::RenamePin => {
                if let Some(one) = items.first().cloned() {
                    self.pin_rename_start(one);
                }
            }
            Act::Earlier => {
                if let [one] = items.as_slice() {
                    self.earlier_open(p, crate::files::earlier::What::Versions(one.clone()));
                }
            }
            Act::DeletedHere => {
                if let Some(here) = here {
                    self.earlier_open(p, crate::files::earlier::What::Deleted(here));
                }
            }
            _ => {}
        }
    }

    fn open_items(&mut self, p: usize, items: Vec<PathBuf>) {
        if items.iter().any(|i| crate::files::far::is_far(i)) {
            let entries = &self.cols[p].entries;
            let folders: Vec<bool> = items.iter().map(|i| entries.iter().any(|e| e.path == *i && e.folder)).collect();
            match (items.as_slice(), folders.as_slice()) {
                ([one], [true]) => self.go(p, Where::Remote(one.to_string_lossy().into_owned())),
                _ => {
                    for (item, folder) in items.iter().zip(folders) {
                        if !folder {
                            self.far_open(item);
                        }
                    }
                }
            }
            return;
        }
        match items.as_slice() {
            [one] if crate::files::reach::is_dir(one) => self.go(p, Where::Folder(one.clone())),
            _ => {
                for item in items.iter().filter(|i| !crate::files::reach::is_dir(i)) {
                    let _ = self.open_file(item);
                }
            }
        }
    }

    fn copy_path(&mut self, p: usize, items: Vec<PathBuf>, here: Option<PathBuf>) {
        let far = match &self.cols[p].at {
            Where::Remote(a) => Some(PathBuf::from(a)),
            _ => None,
        };
        let targets = if items.is_empty() { here.or(far).into_iter().collect() } else { items };
        if targets.is_empty() {
            return;
        }
        let text = targets.iter().map(|t| t.to_string_lossy().into_owned()).collect::<Vec<_>>().join("\n");
        self.say(t!("folders-menu-copied-location", n = targets.len()), false);
        if self.post.is_some() {
            let _ = std::thread::Builder::new().name("folders-clip".into()).spawn(move || {
                selection::offer(text.into_bytes(), &TEXT_TYPES, || {});
            });
        }
    }

    fn open_as_root(&mut self, folder: Option<PathBuf>) {
        let Some(f) = folder else { return };
        if !self.parts.as_root {
            return self.say_help(crate::files::alone::no_root(), crate::files::help::PART_MISSING);
        }
        if !self.as_root(&f) {
            self.say_help(t!("folders-menu-no-admin-window"), crate::files::help::NOT_STARTED);
        }
    }

    fn set_wallpaper(&mut self, items: Vec<PathBuf>) {
        let Some(pic) = items.first() else { return };
        if !self.parts.wallpaper {
            return self.say_help(crate::files::alone::no_wallpaper(), crate::files::help::PART_MISSING);
        }
        let over = crate::common::wallpaper_size::over_quickly(pic);
        if over.any() {
            self.say_help(t!("folders-menu-wallpaper-refused", name = &work::name_of(pic), why = &over.reasons()), crate::files::help::MAKE_FAILED);
        } else if run_self("wallpaper", &["--set", &pic.to_string_lossy()]) {
            self.say(t!("folders-menu-wallpaper-set", name = &work::name_of(pic)), false);
        } else {
            self.say_help(t!("folders-menu-wallpaper-failed"), crate::files::help::NOT_STARTED);
        }
    }

    fn make_link(&mut self, p: usize, items: Vec<PathBuf>, here: Option<PathBuf>) {
        let (Some(target), Some(dir)) = (items.first(), here) else { return };
        if !crate::files::reach::answers(&dir) {
            return self.say(crate::files::reach::not_answering(), false);
        }
        let link = link_name(&dir, target);
        match std::os::unix::fs::symlink(target, &link) {
            Ok(()) => {
                let name = work::name_of(&link);
                self.remember(t!("folders-menu-undo-link", name = &name), vec![work::Step::Remove(link.clone())]);
                self.say(t!("folders-menu-link-made", name = &name), false);
                self.load(self.cols[p].tab, p);
                self.when_read(p, Some(link), None);
            }
            Err(e) => self.say_help(t!("folders-menu-link-failed", why = &e.to_string()), crate::files::help::MAKE_FAILED),
        }
    }

    fn share_with_ai(&mut self, items: Vec<PathBuf>) {
        let Some(path) = items.first().cloned() else { return };
        if !crate::toolkit::window::persons_own(crate::common::person_only::FOLDERS_AI_SHARE) {
            return;
        }
        if !self.parts.ai {
            return self.say_help(crate::files::alone::no_ai_share(), crate::files::help::PART_MISSING);
        }
        let Some(post) = self.post.clone() else { return };
        let name = work::name_of(&path);
        let _ = std::thread::Builder::new().name("folders-ai-share".into()).spawn(move || {
            let (ok, _, err) = run_part_captured("boxed", &["--ai-share", &path.to_string_lossy()]);
            let words = if ok { t!("folders-menu-ai-shared", name = &name) } else { err.trim().to_string() };
            post.send(Msg::Said(words, !ok));
        });
    }

    fn turn_pictures(&mut self, act: Act, items: Vec<PathBuf>, here: Option<PathBuf>) {
        let turn = match act {
            Act::TurnLeft => crate::files::imaging::Turn::Left,
            Act::TurnRight => crate::files::imaging::Turn::Right,
            _ => crate::files::imaging::Turn::Over,
        };
        let to = here.unwrap_or_else(|| self.home.clone());
        self.send_to(items, to, work::Kind::Other(work::Extra::Picture(crate::files::imaging::Work::Turn(turn))));
    }

    fn move_pinned(&mut self, act: Act, folder: Option<PathBuf>) {
        let Some(i) = folder.and_then(|f| self.pinned.iter().position(|p| *p == f)) else { return };
        match act {
            Act::PinUp if i > 0 => self.move_pin(i, i - 1),
            Act::PinDown => self.move_pin(i, i + 2),
            _ => {}
        }
    }

    fn as_root(&self, folder: &Path) -> bool {
        let Some(socket) = crate::files::root::socket() else { return false };
        let Some(argv) = crate::files::root::argv(Path::new(crate::files::root::INSTALLED), &socket, folder) else { return false };
        spawn(&argv.iter().map(String::as_str).collect::<Vec<_>>())
    }
}
