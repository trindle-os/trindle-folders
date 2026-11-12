//! Addresses, command lines and the words they hold no login in.

use crate::files::remote::Address;
use crate::common::smb::Login;

use super::*;

#[test]
fn addresses_name_a_computer_or_a_share() {
    let a = Address::parse("smb://NAS/My%20Photos/2026/June").unwrap();
    assert_eq!(parts(&a), ("NAS".into(), Some("My Photos".into()), "2026/June".into()));
    let b = Address::parse("smb://NAS/").unwrap();
    assert_eq!(parts(&b), ("NAS".into(), None, String::new()));
    let typed = typed_login(&Address::parse("smb://WORK;jo:p%40ss@nas/x").unwrap()).unwrap();
    assert_eq!((typed.user.as_str(), typed.domain.as_str(), typed.password.as_str()), ("jo", "WORK", "p@ss"));
}

#[test]
fn command_lines_hold_no_login() {
    let login = Login::typed("WORKGROUP\\jo", "s3cret");
    let list = list_argv("NAS", Some("192.168.1.20"), false);
    assert_eq!(list, ["smbclient", "-g", "-t", "10", "-L", "//NAS", "-I", "192.168.1.20"]);
    let env = list_env(Some(&login));
    assert!(env.iter().any(|(k, v)| *k == "PASSWD" && v.as_str() == "s3cret"));
    assert!(env.iter().any(|(k, v)| *k == "USER" && v.as_str() == "WORKGROUP\\jo"));
    assert_eq!(list_argv("NAS", None, true)[6..], ["-N", "-U%"]);
    let mount = mount_argv("/usr/lib/trindle/libexec/trindle-admin", "NAS", "My Photos", Some("192.168.1.20"));
    assert_eq!(mount, ["pkexec", "/usr/lib/trindle/libexec/trindle-admin", "smb-mount", "NAS", "My Photos", "192.168.1.20"]);
    for a in list.iter().chain(&mount) {
        assert!(!a.contains("s3cret") && !a.contains("jo"), "{a}");
    }
}
