//! The Secret Service, asked over D-Bus as any program asks it, with a plain session, as the keyring lives on this computer's own bus.

use std::collections::HashMap;

use zbus::blocking::Connection;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};
use zeroize::{Zeroize, Zeroizing};

use crate::common::keyring_said::{why, locked_words};
use crate::common::smb::Login;
use crate::toolkit::t;
use super::rule::{KeptRow, SEEN_MOST, time_left};

const NAME: &str = "org.freedesktop.secrets";
const ROOT: &str = "/org/freedesktop/secrets";
const SERVICE: &str = "org.freedesktop.Secret.Service";
const COLLECTION: &str = "org.freedesktop.Secret.Collection";
const ITEM: &str = "org.freedesktop.Secret.Item";
const SESSION: &str = "org.freedesktop.Secret.Session";
const DEFAULT: &str = "/org/freedesktop/secrets/aliases/default";
const SCHEMA: &str = "org.gnome.keyring.NetworkPassword";
type Secret = (OwnedObjectPath, Vec<u8>, Vec<u8>, String);

fn call<B: serde::Serialize + zbus::zvariant::DynamicType>(c: &Connection, path: &str, iface: &str, method: &str, body: &B)
    -> zbus::Result<zbus::Message> {
    c.call_method(Some(NAME), path, Some(iface), method, body)
}

fn open(c: &Connection) -> zbus::Result<OwnedObjectPath> {
    let (_, s): (OwnedValue, OwnedObjectPath) = call(c, ROOT, SERVICE, "OpenSession", &("plain", Value::from("")))?.body().deserialize()?;
    Ok(s)
}

fn close(c: &Connection, s: &OwnedObjectPath) {
    let _ = call(c, s.as_str(), SESSION, "Close", &());
}

const SEEN_SCHEMA: &str = "org.trindle.Folders.SmbSeenAt";
const SEEN: &str = "smb-seen-at";
struct Seen {
    at: String,
    when: u64,
    who: String,
}

fn seen_attributes(server: &str) -> HashMap<String, String> {
    HashMap::from([("xdg:schema".to_string(), SEEN_SCHEMA.to_string()), ("protocol".into(), SEEN.into()), ("server".into(), server.to_lowercase())])
}

pub fn seen_for(c: &Connection, server: &str, who: &str) -> Result<Vec<(String, u64)>, String> {
    Ok(seen_all(c, server)?.into_iter().filter(|s| s.who == who || s.who.is_empty())
        .map(|s| (s.at, if s.who.is_empty() { 0 } else { s.when })).take(SEEN_MOST).collect())
}

fn behind_lock<T>(found: Option<T>, locked: &[OwnedObjectPath]) -> Result<Option<T>, String> {
    match found {
        None if !locked.is_empty() => Err(locked_words().to_string()),
        found => Ok(found),
    }
}

fn seen_all(c: &Connection, server: &str) -> Result<Vec<Seen>, String> {
    let s = open(c).map_err(why)?;
    let ask = seen_attributes(server);
    let found = call(c, ROOT, SERVICE, "SearchItems", &(&ask,)).map_err(why)
        .and_then(|m| m.body().deserialize::<(Vec<OwnedObjectPath>, Vec<OwnedObjectPath>)>().map_err(|e| e.to_string()))
        .and_then(|(open, locked)| {
            let got: Option<Secret> = open.into_iter().find_map(|item| call(c, item.as_str(), ITEM, "GetSecret", &(&s,)).ok()?.body().deserialize().ok());
            behind_lock(got, &locked)
        });
    close(c, &s);
    let text = found?.map(|secret| String::from_utf8_lossy(&secret.2).into_owned()).unwrap_or_default();
    Ok(text.lines().filter_map(|l| {
        let mut words = l.split_whitespace();
        let at = words.next().filter(|a| a.parse::<std::net::IpAddr>().is_ok())?;
        let when = words.next().and_then(|w| w.parse().ok()).unwrap_or(0);
        let who = words.next().filter(|w| w.len() <= 64 && w.bytes().all(|b| b.is_ascii_hexdigit())).unwrap_or("");
        Some(Seen { at: at.to_string(), when, who: who.to_string() })
    }).collect())
}

pub fn saw_at(c: &Connection, server: &str, address: &str, now: u64, forget: bool, who: &str) -> Result<(), String> {
    if address.parse::<std::net::IpAddr>().is_err() {
        return Err(t!("folders-shares-not-address").into());
    }
    let mut all = seen_all(c, server)?;
    if forget {
        all.retain(|s| s.who != who || s.at == address);
    }
    all.retain(|s| !(s.at == address && s.who == who));
    all.insert(0, Seen { at: address.to_string(), when: now, who: who.to_string() });
    let mut mine = 0;
    all.retain(|s| s.who != who || {
        mine += 1;
        mine <= SEEN_MOST
    });
    let lines: Vec<String> = all.iter().map(|s| format!("{} {} {}", s.at, s.when, s.who).trim_end().to_string()).collect();
    let s = open(c).map_err(why)?;
    let props: HashMap<String, Value> = HashMap::from([
        ("org.freedesktop.Secret.Item.Label".to_string(), Value::from(t!("folders-shares-seen-label", server = server))),
        ("org.freedesktop.Secret.Item.Attributes".to_string(), Value::from(seen_attributes(server))),
    ]);
    let secret: Secret = (s.clone(), Vec::new(), lines.join("\n").into_bytes(), "text/plain".into());
    let made = call(c, DEFAULT, COLLECTION, "CreateItem", &(props, secret, true));
    close(c, &s);
    let (_, prompt): (OwnedObjectPath, OwnedObjectPath) = made.map_err(why)?.body().deserialize().map_err(|e| e.to_string())?;
    if prompt.as_str() != "/" {
        return Err(locked_words().into());
    }
    Ok(())
}

fn attributes_of(c: &Connection, item: &OwnedObjectPath) -> Option<HashMap<String, String>> {
    let attrs: OwnedValue = call(c, item.as_str(), "org.freedesktop.DBus.Properties", "Get", &(ITEM, "Attributes")).ok()?.body().deserialize().ok()?;
    HashMap::<String, String>::try_from(attrs).ok()
}

pub fn all(c: &Connection, now: u64) -> Result<Vec<KeptRow>, String> {
    let ask = HashMap::from([("protocol", "smb")]);
    let (open, locked) = call(c, ROOT, SERVICE, "SearchItems", &(&ask,)).map_err(why)?
        .body().deserialize::<(Vec<OwnedObjectPath>, Vec<OwnedObjectPath>)>().map_err(|e| e.to_string())?;
    let mut rows: Vec<KeptRow> = Vec::new();
    for item in &open {
        let Some(attrs) = attributes_of(c, item) else { continue };
        let (Some(server), Some(user)) = (attrs.get("server").filter(|s| !s.is_empty()), attrs.get("user").filter(|u| !u.is_empty())) else { continue };
        match rows.iter_mut().find(|r| r.server == *server) {
            Some(row) => {
                if !row.users.split(", ").any(|u| u == user) {
                    row.users = format!("{}, {user}", row.users);
                }
            }
            None => rows.push(KeptRow { server: server.clone(), users: user.clone(), left: 0 }),
        }
    }
    if rows.is_empty() && !locked.is_empty() {
        return Err(locked_words().to_string());
    }
    for row in &mut rows {
        let times: Vec<u64> = seen_all(c, &row.server)?.iter().map(|s| s.when).collect();
        row.left = time_left(&times, now);
    }
    rows.sort_by(|a, b| a.server.cmp(&b.server));
    Ok(rows)
}

pub fn forget(c: &Connection, server: &str) -> Result<(), String> {
    let mut asks = vec![HashMap::from([("protocol".to_string(), "smb".to_string()), ("server".to_string(), server.to_lowercase())]), seen_attributes(server)];
    if server != server.to_lowercase() {
        asks.push(HashMap::from([("protocol".to_string(), "smb".to_string()), ("server".to_string(), server.to_string())]));
    }
    for ask in &asks {
        let (open, locked) = call(c, ROOT, SERVICE, "SearchItems", &(ask,)).map_err(why)?
            .body().deserialize::<(Vec<OwnedObjectPath>, Vec<OwnedObjectPath>)>().map_err(|e| e.to_string())?;
        if !locked.is_empty() {
            return Err(locked_words().to_string());
        }
        for item in &open {
            let (prompt,): (OwnedObjectPath,) = call(c, item.as_str(), ITEM, "Delete", &()).map_err(why)?.body().deserialize().map_err(|e| e.to_string())?;
            if prompt.as_str() != "/" {
                return Err(locked_words().to_string());
            }
        }
    }
    Ok(())
}

fn attributes(server: &str, share: Option<&str>, login: &Login) -> HashMap<String, String> {
    HashMap::from([
        ("xdg:schema".to_string(), SCHEMA.to_string()),
        ("protocol".into(), "smb".into()),
        ("server".into(), server.to_lowercase()),
        ("object".into(), share.unwrap_or("").to_string()),
        ("user".into(), login.user.clone()),
        ("domain".into(), login.domain.clone()),
    ])
}

pub fn find(c: &Connection, server: &str, share: Option<&str>) -> Result<Option<Login>, String> {
    let s = open(c).map_err(why)?;
    let lower = server.to_lowercase();
    let mut asks: Vec<HashMap<&str, &str>> = Vec::new();
    for name in [lower.as_str(), server] {
        if let Some(share) = share {
            asks.push(HashMap::from([("protocol", "smb"), ("server", name), ("object", share)]));
        }
        asks.push(HashMap::from([("protocol", "smb"), ("server", name), ("object", "")]));
        asks.push(HashMap::from([("protocol", "smb"), ("server", name)]));
    }
    let mut found = Ok(None);
    let mut locked: Vec<OwnedObjectPath> = Vec::new();
    for ask in &asks {
        let listed = call(c, ROOT, SERVICE, "SearchItems", &(ask,)).map_err(why)
            .and_then(|m| m.body().deserialize::<(Vec<OwnedObjectPath>, Vec<OwnedObjectPath>)>().map_err(|e| e.to_string()));
        match listed {
            Ok((open, behind)) => {
                locked.extend(behind);
                if let Some(login) = open.into_iter().find_map(|item| read(c, &item, &s)) {
                    found = Ok(Some(login));
                    break;
                }
            }
            Err(why) => {
                found = Err(why);
                break;
            }
        }
    }
    close(c, &s);
    found.and_then(|login| behind_lock(login, &locked))
}

fn read(c: &Connection, item: &OwnedObjectPath, s: &OwnedObjectPath) -> Option<Login> {
    let got: Secret = call(c, item.as_str(), ITEM, "GetSecret", &(s,)).ok()?.body().deserialize().ok()?;
    let attrs: OwnedValue = call(c, item.as_str(), "org.freedesktop.DBus.Properties", "Get", &(ITEM, "Attributes"))
        .ok()?.body().deserialize().ok()?;
    let attrs = HashMap::<String, String>::try_from(attrs).ok()?;
    let password = Zeroizing::new(String::from_utf8(got.2).map_err(|e| e.into_bytes().zeroize()).ok()?);
    let user = attrs.get("user").filter(|u| !u.is_empty())?.clone();
    Some(Login { user, domain: attrs.get("domain").cloned().unwrap_or_default(), password })
}

pub fn keep(c: &Connection, server: &str, share: Option<&str>, login: &Login) -> Result<(), String> {
    let s = open(c).map_err(why)?;
    let what = share.map_or_else(|| format!("smb://{server}"), |sh| format!("smb://{server}/{sh}"));
    let props: HashMap<String, Value> = HashMap::from([
        ("org.freedesktop.Secret.Item.Label".to_string(), Value::from(t!("folders-shares-password-label", user = &login.user, what = &what))),
        ("org.freedesktop.Secret.Item.Attributes".to_string(), Value::from(attributes(server, share, login))),
    ]);
    let mut secret: Secret = (s.clone(), Vec::new(), login.password.as_bytes().to_vec(), "text/plain".into());
    let made = call(c, DEFAULT, COLLECTION, "CreateItem", &(props, &secret, true));
    secret.2.zeroize();
    close(c, &s);
    let (_, prompt): (OwnedObjectPath, OwnedObjectPath) = made.map_err(why)?.body().deserialize().map_err(|e| e.to_string())?;
    if prompt.as_str() != "/" {
        return Err(locked_words().into());
    }
    Ok(())
}
