//! Every language has exactly the messages English has, and uses no placeholder English does not offer.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

fn messages(text: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut key = None::<String>;
    for line in text.lines() {
        let head = line.split_once('=').map(|(k, _)| k.trim_end()).filter(|k| {
            !line.starts_with(|c: char| c.is_whitespace() || c == '#') && !k.is_empty() && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        });
        if let Some(k) = head {
            key = Some(k.to_string());
            out.entry(k.to_string()).or_default();
        } else if line.is_empty() || line.starts_with('#') {
            key = None;
        }
        if let (Some(k), false) = (&key, line.starts_with('#')) {
            let mut rest = line;
            while let Some((_, after)) = rest.split_once('$') {
                let name: String = after.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').collect();
                out.get_mut(k).unwrap().insert(name);
                rest = after;
            }
        }
    }
    out
}

fn files(dir: &Path) -> BTreeMap<String, BTreeMap<String, BTreeSet<String>>> {
    let mut all = BTreeMap::new();
    for f in std::fs::read_dir(dir).unwrap().flatten() {
        let name = f.file_name().to_string_lossy().into_owned();
        if name.ends_with(".ftl") {
            all.insert(name, messages(&std::fs::read_to_string(f.path()).unwrap()));
        }
    }
    all
}

fn differences(lang: &Path, english: &BTreeMap<String, BTreeMap<String, BTreeSet<String>>>) -> Vec<String> {
    let theirs = files(lang);
    let mut bad = Vec::new();
    for (file, en) in english {
        match theirs.get(file) {
            None => bad.push(format!("{file} is missing")),
            Some(own) => {
                for (k, vars) in en {
                    match own.get(k) {
                        None => bad.push(format!("{file}: {k} is missing")),
                        Some(v) if !v.is_subset(vars) => bad.push(format!("{file}: {k} uses a placeholder English does not")),
                        Some(_) => {}
                    }
                }
                bad.extend(own.keys().filter(|k| !en.contains_key(*k)).map(|k| format!("{file}: {k} is not in English")));
            }
        }
    }
    bad.extend(theirs.keys().filter(|f| !english.contains_key(*f)).map(|f| format!("{f} is not in English")));
    bad
}

#[test]
fn every_language_has_the_messages_english_has_and_no_others() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang");
    let english = files(&root.join("en"));
    assert!(english.len() > 40, "English was not read");
    let mut report = Vec::new();
    for dir in std::fs::read_dir(&root).unwrap().flatten().filter(|d| d.file_name() != "en" && d.path().is_dir()) {
        let bad = differences(&dir.path(), &english);
        if !bad.is_empty() {
            report.push(format!("{} ({} differences): {}", dir.file_name().to_string_lossy(), bad.len(), bad.iter().take(6).cloned().collect::<Vec<_>>().join("; ")));
        }
    }
    assert!(report.is_empty(), "languages that do not match English:\n{}", report.join("\n"));
}

#[test]
fn a_missing_message_or_an_extra_one_is_named() {
    let en = messages("a = One\nb = Two { $n }\n");
    let english = BTreeMap::from([("x.ftl".to_string(), en)]);
    let dir = std::env::temp_dir().join(format!("trindle-folders-lang-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("x.ftl"), "a = Eins\nc = Drei\nb = Zwei { $m }\n").unwrap();
    let bad = differences(&dir, &english);
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(bad, ["x.ftl: b uses a placeholder English does not", "x.ftl: c is not in English"]);
}
