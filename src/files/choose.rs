// Folders as the window another program chooses files in: `trindle-folders --choose`, started by Trindle's portal.

use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::app::{Files, Msg};
use super::kinds::{file_uri, glob_matches, Mime};
use super::places::{Entry, Where};
use crate::toolkit::t;
use crate::toolkit::widget::{column, dropdown, entry, form_label, label, main_action, main_word, row, space, switch, word_button, Length, Node, Tone};

pub const FLAG: &str = "--choose";

const ASK_MOST: u64 = 1 << 20;
const NAME_W: f32 = 320.0;
const GAP: f32 = 8.0;
const SAID_H: f32 = 20.0;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Mode {
    Open { many: bool, folder: bool },
    Save,
    SaveMany(Vec<String>),
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Open { many: false, folder: false }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Filter {
    pub name: String,
    pub rules: Vec<(u32, String)>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct Pick {
    pub id: String,
    pub label: String,
    pub options: Vec<(String, String)>,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Ask {
    pub title: String,
    pub accept: Option<String>,
    pub mode: Mode,
    pub filters: Vec<Filter>,
    pub filter: Option<usize>,
    pub choices: Vec<Pick>,
    pub folder: Option<PathBuf>,
    pub name: String,
    pub app_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Outcome {
    Chosen { uris: Vec<String>, choices: Vec<(String, String)>, filter: Option<Filter> },
    Cancelled,
}

#[derive(Clone, Debug)]
pub enum ChooseMsg {
    Name(String),
    Kind(usize),
    Choice(usize, String),
    Accept,
    Cancel,
}

#[derive(Clone, Debug, PartialEq)]
enum Said {
    Taken(PathBuf),
    NoName,
    Slash,
}

pub struct Choosing {
    pub ask: Ask,
    name: String,
    filter: Option<usize>,
    values: Vec<String>,
    said: Option<Said>,
    last: Option<PathBuf>,
    answer: Option<Box<dyn FnOnce(Outcome)>>,
}

pub fn read_ask(from: impl Read) -> Option<Ask> {
    serde_json::from_reader(from.take(ASK_MOST)).ok()
}

pub fn answer_line(outcome: &Outcome) -> String {
    serde_json::to_string(outcome).unwrap_or_else(|_| "\"Cancelled\"".into())
}

pub fn print(outcome: &Outcome) {
    let mut out = std::io::stdout().lock();
    if let Err(e) = writeln!(out, "{}", answer_line(outcome)).and_then(|()| out.flush()) {
        eprintln!("trindle-folders: the answer could not be given: {e}");
    }
}

impl Choosing {
    pub fn new(ask: Ask, answer: impl FnOnce(Outcome) + 'static) -> Choosing {
        let filter = ask.filter.or(Some(0)).filter(|&i| i < ask.filters.len());
        let values = ask.choices.iter().map(|c| c.value.clone()).collect();
        let name = if ask.mode == Mode::Save { plain_name(&ask.name).unwrap_or_default() } else { String::new() };
        Choosing { ask, name, filter, values, said: None, last: None, answer: Some(Box::new(answer)) }
    }

    fn saving(&self) -> bool {
        self.ask.mode == Mode::Save
    }

    fn folders_only(&self) -> bool {
        matches!(self.ask.mode, Mode::Open { folder: true, .. } | Mode::SaveMany(_))
    }

    pub fn shows(&self, e: &Entry, mime: &Mime) -> bool {
        if e.folder {
            return true;
        }
        if self.folders_only() {
            return false;
        }
        self.filter.and_then(|i| self.ask.filters.get(i)).is_none_or(|f| passes(f, mime, &e.name, e.kind.as_deref()))
    }
}

fn passes(filter: &Filter, mime: &Mime, name: &str, kind: Option<&str>) -> bool {
    let lower = name.to_lowercase();
    filter.rules.iter().any(|(how, rule)| match how {
        0 => glob_matches(&rule.to_lowercase(), &lower),
        _ => rule == "application/octet-stream" || kind.is_some_and(|k| mime.is_a(k, rule)),
    })
}

fn plain_name(name: &str) -> Option<String> {
    Path::new(name).file_name().map(|n| n.to_string_lossy().into_owned()).filter(|n| !n.is_empty())
}

fn taken(path: &Path) -> bool {
    let p = path.to_path_buf();
    super::reach::within(path, move || std::fs::symlink_metadata(&p).is_ok()).unwrap_or(true)
}

fn free(into: &Path, name: &str, given: &HashSet<PathBuf>) -> PathBuf {
    let first = into.join(name);
    if !given.contains(&first) && !taken(&first) {
        return first;
    }
    let (stem, ending) = match name.rfind('.').filter(|&i| i > 0) {
        Some(i) => name.split_at(i),
        None => (name, ""),
    };
    (2..).map(|n| into.join(format!("{stem} ({n}){ending}"))).find(|p| !given.contains(p) && !taken(p)).unwrap_or(first)
}

fn own_folder(at: &Where) -> Option<&Path> {
    at.folder().filter(|p| !super::far::is_far(p) && super::inside::within(p).is_none())
}

fn own_items(at: &Where) -> bool {
    match at {
        Where::Recent => true,
        Where::Found(dir, ..) => !super::far::is_far(dir),
        _ => own_folder(at).is_some(),
    }
}

impl Files {
    pub fn choosing_for(&mut self, choosing: Choosing) {
        self.choosing = Some(choosing);
        for p in 0..2 {
            self.load(self.cols[p].tab, p);
        }
    }

    pub fn choose_closed(&mut self) {
        self.answer(Outcome::Cancelled);
    }

    pub(super) fn choose_gives_folders(&self) -> bool {
        self.choosing.as_ref().is_some_and(Choosing::folders_only)
    }

    pub(super) fn choose_escapes(&self) -> bool {
        self.choosing.is_some() && self.find.open.is_none() && self.locate.is_none() && self.pin_rename.is_none() && self.renaming.is_none()
    }

    pub(super) fn choose(&mut self, m: ChooseMsg) -> bool {
        let Some(c) = &mut self.choosing else { return false };
        match m {
            ChooseMsg::Name(name) => {
                c.name = name;
                c.said = None;
            }
            ChooseMsg::Kind(i) => {
                c.filter = Some(i).filter(|&i| i < c.ask.filters.len());
                return true;
            }
            ChooseMsg::Choice(i, value) => {
                if let Some(v) = c.values.get_mut(i) {
                    *v = value;
                }
            }
            ChooseMsg::Accept => self.choose_accept(),
            ChooseMsg::Cancel => self.answer(Outcome::Cancelled),
        }
        false
    }

    pub(super) fn choose_follow(&mut self) {
        let pane = &self.cols[self.side()];
        let Some(e) = pane.chosen.and_then(|i| pane.entries.get(i)).filter(|e| !e.folder) else { return };
        let path = e.path.clone();
        let Some(c) = self.choosing.as_mut().filter(|c| c.saving()) else { return };
        if c.last.as_ref() != Some(&path) {
            c.name = plain_name(&e.name).unwrap_or_default();
            c.said = None;
            c.last = Some(path);
        }
    }

    pub(super) fn choose_opened(&mut self, path: &Path) -> bool {
        let own = own_items(&self.cols[self.side()].at);
        let Some(c) = self.choosing.as_mut().filter(|_| own) else { return false };
        match c.ask.mode {
            Mode::Open { folder: false, .. } => {
                let choices = c.ask.choices.iter().zip(&c.values).map(|(p, v)| (p.id.clone(), v.clone())).collect();
                let filter = c.filter.and_then(|i| c.ask.filters.get(i)).cloned();
                self.answer(Outcome::Chosen { uris: vec![file_uri(path)], choices, filter });
            }
            Mode::Save => {
                c.name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                c.last = Some(path.to_path_buf());
                self.choose_accept();
            }
            _ => return false,
        }
        true
    }

    fn choose_picked(&self) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let pane = &self.cols[self.side()];
        if !own_items(&pane.at) {
            return (Vec::new(), Vec::new());
        }
        let at: Vec<usize> = if pane.picked.is_empty() { pane.chosen.into_iter().collect() } else { pane.picked.clone() };
        let (folders, files): (Vec<&Entry>, Vec<&Entry>) = at.iter().filter_map(|&i| pane.entries.get(i)).partition(|e| e.folder);
        (files.into_iter().map(|e| e.path.clone()).collect(), folders.into_iter().map(|e| e.path.clone()).collect())
    }

    fn choose_here(&self) -> Option<PathBuf> {
        own_folder(&self.cols[self.side()].at).map(Path::to_path_buf)
    }

    fn choose_ready(&self) -> bool {
        let Some(c) = &self.choosing else { return false };
        let (files, folders) = self.choose_picked();
        match c.ask.mode {
            Mode::Open { folder: true, .. } => !folders.is_empty() || self.choose_here().is_some(),
            Mode::Open { .. } => !files.is_empty() || folders.len() == 1,
            Mode::Save => self.choose_here().is_some(),
            Mode::SaveMany(_) => folders.len() == 1 || self.choose_here().is_some(),
        }
    }

    fn choose_accept(&mut self) {
        if !self.choose_ready() {
            return;
        }
        let (files, folders) = self.choose_picked();
        let here = self.choose_here();
        let Some(c) = &mut self.choosing else { return };
        let chosen = match &c.ask.mode {
            Mode::Open { folder: true, many } => {
                let mut v = if folders.is_empty() { here.into_iter().collect() } else { folders };
                if !many {
                    v.truncate(1);
                }
                v
            }
            Mode::Open { many, .. } => {
                if files.is_empty() {
                    let p = self.side();
                    self.go(p, Where::Folder(folders[0].clone()));
                    return;
                }
                let mut v = files;
                if !many {
                    v.truncate(1);
                }
                v
            }
            Mode::Save => {
                let Some(here) = here else { return };
                let typed = c.name.trim();
                if typed.is_empty() {
                    c.said = Some(Said::NoName);
                    return;
                }
                if typed.contains('/') || typed == "." || typed == ".." {
                    c.said = Some(Said::Slash);
                    return;
                }
                let target = here.join(typed);
                if taken(&target) && c.said != Some(Said::Taken(target.clone())) {
                    c.said = Some(Said::Taken(target));
                    return;
                }
                vec![target]
            }
            Mode::SaveMany(names) => {
                let Some(into) = (if folders.len() == 1 { Some(folders[0].clone()) } else { here }) else { return };
                let mut given = HashSet::new();
                for name in names.iter().filter_map(|n| plain_name(n)) {
                    given.insert(free(&into, &name, &given));
                }
                let mut v: Vec<PathBuf> = given.into_iter().collect();
                v.sort();
                v
            }
        };
        let choices = c.ask.choices.iter().zip(&c.values).map(|(p, v)| (p.id.clone(), v.clone())).collect();
        let filter = c.filter.and_then(|i| c.ask.filters.get(i)).cloned();
        self.answer(Outcome::Chosen { uris: chosen.iter().map(|p| file_uri(p)).collect(), choices, filter });
    }

    fn answer(&mut self, outcome: Outcome) {
        let Some(answer) = self.choosing.as_mut().and_then(|c| c.answer.take()) else { return };
        self.choosing = None;
        answer(outcome);
        if let Some(hook) = &self.close_hook {
            hook();
        }
    }
}

pub fn title(ask: &Ask) -> String {
    if !ask.title.trim().is_empty() {
        return ask.title.clone();
    }
    match ask.mode {
        Mode::Open { folder: true, .. } => t!("folders-choose-title-folder").into(),
        Mode::Open { .. } => t!("folders-choose-title-open").into(),
        Mode::Save | Mode::SaveMany(_) => t!("folders-choose-title-save").into(),
    }
}

pub fn bar(f: &Files) -> Option<Node<Msg>> {
    let c = f.choosing.as_ref()?;
    let send = |m: ChooseMsg| Msg::Chooser(m);
    let mut parts = Vec::new();
    if c.saving() {
        parts.push(form_label(t!("folders-choose-name")));
        parts.push(entry(c.name.clone(), t!("folders-choose-name"), move |w| send(ChooseMsg::Name(w)))
            .on_submit(send(ChooseMsg::Accept)).width(Length::Fixed(NAME_W)));
    }
    parts.push(space().width(Length::Fill));
    if c.ask.filters.len() > 1 {
        let names = c.ask.filters.iter().map(|f| f.name.clone()).collect();
        parts.push(dropdown(names, c.filter, t!("folders-choose-kinds"), move |i| send(ChooseMsg::Kind(i))));
    }
    for (i, (pick, value)) in c.ask.choices.iter().zip(&c.values).enumerate() {
        parts.push(form_label(pick.label.clone()));
        if pick.options.is_empty() {
            parts.push(switch(value == "true", pick.label.clone(), move |on| send(ChooseMsg::Choice(i, on.to_string()))));
        } else {
            let ids: Vec<String> = pick.options.iter().map(|(id, _)| id.clone()).collect();
            let at = ids.iter().position(|id| id == value);
            let words = pick.options.iter().map(|(_, w)| w.clone()).collect();
            parts.push(dropdown(words, at, pick.label.clone(), move |k| send(ChooseMsg::Choice(i, ids.get(k).cloned().unwrap_or_default()))));
        }
    }
    parts.push(word_button(t!("folders-choose-cancel")).on_press(send(ChooseMsg::Cancel)));
    let replacing = matches!(c.said, Some(Said::Taken(_)));
    let (icon, words) = match c.ask.mode {
        _ if replacing => (None, t!("folders-choose-replace").to_string()),
        Mode::Open { folder: true, .. } => (Some("folder-symbolic"), t!("folders-choose-folder").to_string()),
        Mode::Open { .. } => (Some("document-open-symbolic"), t!("folders-choose-open").to_string()),
        Mode::Save | Mode::SaveMany(_) => (None, t!("folders-choose-save").to_string()),
    };
    let words = c.ask.accept.clone().filter(|w| !w.trim().is_empty() && !replacing).unwrap_or(words);
    let main = match icon {
        Some(icon) => main_action(icon, words),
        None => main_word(words),
    };
    parts.push(main.on_press(send(ChooseMsg::Accept)).off(!f.choose_ready()));
    let said = match &c.said {
        Some(Said::Taken(p)) => label(t!("folders-choose-taken", name = &p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default())),
        Some(Said::NoName) => label(t!("folders-choose-no-name")).tone(Tone::Problem),
        Some(Said::Slash) => label(t!("folders-choose-slash")).tone(Tone::Problem),
        None => space(),
    };
    Some(column(vec![row(parts).gap(GAP).width(Length::Fill), said.height(Length::Fixed(SAID_H))]).gap(GAP / 2.0).width(Length::Fill))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_request_is_read_with_the_portal_s_own_names_and_what_is_left_out_is_empty() {
        let ask = read_ask(r#"{"title":"Save a picture","mode":"Save","name":"../beach.png","filters":[{"name":"Pictures","rules":[[0,"*.png"],[1,"image/*"]]}]}"#.as_bytes()).unwrap();
        assert_eq!(ask.mode, Mode::Save);
        assert_eq!(ask.filters[0].rules[1], (1, "image/*".to_string()));
        assert!(ask.choices.is_empty() && ask.folder.is_none() && ask.accept.is_none());
        let c = Choosing::new(ask, |_| {});
        assert_eq!((c.name.as_str(), c.filter), ("beach.png", Some(0)), "a program's name for a file never names another folder");
        let many = read_ask(r#"{"mode":{"Open":{"many":true,"folder":false}},"choices":[{"id":"enc","label":"Encoding","options":[["u","UTF-8"]],"value":"u"}]}"#.as_bytes()).unwrap();
        assert_eq!(many.mode, Mode::Open { many: true, folder: false });
        assert_eq!(many.choices[0].options[0].1, "UTF-8");
        assert!(read_ask("not a request".as_bytes()).is_none());
    }

    #[test]
    fn the_answer_is_one_line_the_portal_reads() {
        assert_eq!(answer_line(&Outcome::Cancelled), "\"Cancelled\"");
        let chosen = Outcome::Chosen { uris: vec!["file:///home/john/a%20b.txt".into()], choices: vec![("enc".into(), "u".into())], filter: None };
        assert_eq!(answer_line(&chosen), r#"{"Chosen":{"uris":["file:///home/john/a%20b.txt"],"choices":[["enc","u"]],"filter":null}}"#);
    }

    #[test]
    fn names_already_taken_or_given_are_numbered() {
        let dir = std::env::temp_dir().join(format!("trindle-folders-choose-free-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.txt"), b"made up").unwrap();
        std::os::unix::fs::symlink(dir.join("nowhere"), dir.join("link.txt")).unwrap();
        let mut given = HashSet::new();
        assert_eq!(free(&dir, "a.txt", &given), dir.join("a (2).txt"));
        assert_eq!(free(&dir, "link.txt", &given), dir.join("link (2).txt"), "a link that leads nowhere is still taken");
        given.insert(dir.join("b"));
        assert_eq!(free(&dir, "b", &given), dir.join("b (2)"));
        assert_eq!(free(&dir, ".hidden", &given), dir.join(".hidden"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    use crate::toolkit::ui::App;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn made_up(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-folders-choose-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Sub")).unwrap();
        std::fs::write(dir.join("a.txt"), b"made up").unwrap();
        std::fs::write(dir.join("b.png"), b"made up").unwrap();
        dir
    }

    fn window(dir: &Path, ask: Ask) -> (Files, Rc<RefCell<Vec<Outcome>>>) {
        let told = Rc::new(RefCell::new(Vec::new()));
        let t = told.clone();
        let mut f = Files::new(vec![dir.to_path_buf()]);
        f.update(Msg::PanesAre(false));
        f.choosing_for(Choosing::new(ask, move |o| t.borrow_mut().push(o)));
        (f, told)
    }

    fn names(f: &Files) -> Vec<String> {
        f.cols[0].entries.iter().map(|e| e.name.clone()).collect()
    }

    fn pick(f: &mut Files, name: &str) {
        let i = f.cols[0].entries.iter().position(|e| e.name == name).unwrap();
        f.update(Msg::Choose(0, i));
        f.update(Msg::Pick(0, vec![i]));
    }

    fn chosen(told: &Rc<RefCell<Vec<Outcome>>>) -> Vec<String> {
        match told.borrow().as_slice() {
            [Outcome::Chosen { uris, .. }] => uris.clone(),
            other => panic!("one answer was wanted, not {other:?}"),
        }
    }

    #[test]
    fn opening_shows_the_kind_chosen_and_answers_the_files_picked() {
        let dir = made_up("open");
        let filters = vec![Filter { name: "Pictures".into(), rules: vec![(0, "*.PNG".into())] }, Filter { name: "All".into(), rules: vec![(0, "*".into())] }];
        let (mut f, told) = window(&dir, Ask { filters, ..Ask::default() });
        assert_eq!(names(&f), ["Sub", "b.png"], "folders always, files only of the kind chosen");
        f.update(Msg::Chooser(ChooseMsg::Kind(1)));
        assert_eq!(names(&f), ["Sub", "a.txt", "b.png"]);
        assert!(!f.choose_ready(), "nothing picked");
        pick(&mut f, "a.txt");
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(chosen(&told), [file_uri(&dir.join("a.txt"))]);
        match &told.borrow()[0] {
            Outcome::Chosen { filter, .. } => assert_eq!(filter.as_ref().map(|f| f.name.as_str()), Some("All")),
            _ => unreachable!(),
        }
        f.choose_closed();
        assert_eq!(told.borrow().len(), 1, "answered once");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_double_click_chooses_a_file_and_open_goes_into_a_folder() {
        let dir = made_up("double");
        let (mut f, told) = window(&dir, Ask::default());
        pick(&mut f, "Sub");
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Sub")));
        assert!(told.borrow().is_empty());
        f.go(0, Where::Folder(dir.clone()));
        let i = f.cols[0].entries.iter().position(|e| e.name == "b.png").unwrap();
        f.open(0, i);
        assert_eq!(chosen(&told), [file_uri(&dir.join("b.png"))]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn saving_asks_before_a_name_already_there_and_says_what_is_wrong_with_a_name() {
        let dir = made_up("save");
        let (mut f, told) = window(&dir, Ask { mode: Mode::Save, name: "new.txt".into(), ..Ask::default() });
        pick(&mut f, "a.txt");
        assert_eq!(f.choosing.as_ref().unwrap().name, "a.txt", "a file clicked gives its name");
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(f.choosing.as_ref().unwrap().said, Some(Said::Taken(dir.join("a.txt"))));
        assert!(told.borrow().is_empty(), "nothing is answered over a file without asking");
        for (typed, said) in [("", Said::NoName), ("x/y.txt", Said::Slash), ("..", Said::Slash)] {
            f.update(Msg::Chooser(ChooseMsg::Name(typed.into())));
            f.update(Msg::Chooser(ChooseMsg::Accept));
            assert_eq!(f.choosing.as_ref().unwrap().said, Some(said));
        }
        assert!(told.borrow().is_empty());
        f.update(Msg::Chooser(ChooseMsg::Name("a.txt".into())));
        f.update(Msg::Chooser(ChooseMsg::Accept));
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(chosen(&told), [file_uri(&dir.join("a.txt"))], "the second press replaces");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn typing_a_name_never_narrows_the_list() {
        let dir = made_up("typing");
        let (mut f, _) = window(&dir, Ask { mode: Mode::Save, ..Ask::default() });
        let before = names(&f);
        f.update(Msg::Chooser(ChooseMsg::Name("b".into())));
        assert_eq!(names(&f), before);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn choosing_a_folder_shows_only_folders_and_answers_the_one_shown() {
        let dir = made_up("folder");
        let (mut f, told) = window(&dir, Ask { mode: Mode::Open { many: false, folder: true }, ..Ask::default() });
        assert_eq!(names(&f), ["Sub"]);
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(chosen(&told), [file_uri(&dir)]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_folder_browsed_to_and_not_picked_is_the_folder_answered() {
        let dir = made_up("browsed");
        std::fs::create_dir_all(dir.join("Sub/Deep")).unwrap();
        let ask = Ask { mode: Mode::Open { many: false, folder: true }, folder: Some(dir.join("Sub")), ..Ask::default() };
        let (mut f, told) = window(&dir.join("Sub"), ask.clone());
        assert_eq!(names(&f), ["Deep"]);
        f.update(Msg::Choose(0, 0));
        f.update(Msg::Pick(0, vec![0]));
        f.open(0, 0);
        assert_eq!(f.cols[0].at, Where::Folder(dir.join("Sub/Deep")));
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(chosen(&told), [file_uri(&dir.join("Sub/Deep"))]);
        let (mut f, told) = window(&dir.join("Sub"), ask);
        f.update(Msg::Up);
        assert_eq!(f.cols[0].at, Where::Folder(dir.clone()));
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(chosen(&told), [file_uri(&dir)], "not the folder just left");
        let ask = Ask { mode: Mode::SaveMany(vec!["new.txt".into()]), ..Ask::default() };
        let (mut f, told) = window(&dir.join("Sub"), ask);
        f.update(Msg::Up);
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(chosen(&told), [file_uri(&dir.join("new.txt"))]);
        let ask = Ask { mode: Mode::Open { many: false, folder: true }, ..Ask::default() };
        let (mut f, told) = window(&dir, ask);
        pick(&mut f, "Sub");
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(chosen(&told), [file_uri(&dir.join("Sub"))]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_answered_chooser_is_an_ordinary_window_that_ends_with_the_work_it_leaves_going() {
        use super::super::{on_close, Next};
        let dir = made_up("over");
        for cancel in [false, true] {
            let (mut f, told) = window(&dir, Ask::default());
            f.jobs.push(super::super::act::Job::sample(1, "Copying 3 items to Documents", 300, 120));
            pick(&mut f, "a.txt");
            f.update(Msg::Chooser(if cancel { ChooseMsg::Cancel } else { ChooseMsg::Accept }));
            assert_eq!(told.borrow().len(), 1, "answered once");
            assert!(f.choosing.is_none(), "no chooser line left to press with nothing behind it");
            assert!(!f.choose_escapes());
            assert_eq!(f.jobs.len(), 1, "answering stops no work");
            f.choose_closed();
            assert_eq!(told.borrow().len(), 1);
            assert_eq!(on_close(f.busy()), Next::Hide, "the copy goes on out of sight, and the window ends with it");
            f.jobs.clear();
            assert_eq!(on_close(f.busy()), Next::Quit, "with no work it ends at once");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn saving_several_numbers_the_names_taken_and_escape_or_closing_cancels() {
        let dir = made_up("many");
        let (mut f, told) = window(&dir, Ask { mode: Mode::SaveMany(vec!["a.txt".into(), "../c.txt".into()]), ..Ask::default() });
        f.update(Msg::Chooser(ChooseMsg::Accept));
        assert_eq!(chosen(&told), [file_uri(&dir.join("a (2).txt")), file_uri(&dir.join("c.txt"))]);
        let (mut f, told) = window(&dir, Ask::default());
        assert!(f.choose_escapes());
        f.update(Msg::Chooser(ChooseMsg::Cancel));
        f.choose_closed();
        assert_eq!(*told.borrow(), [Outcome::Cancelled]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
