//! Whether this computer is a laptop running on its battery.

use std::path::Path;

const SUPPLIES: &str = "/sys/class/power_supply";

fn laptop() -> bool {
    laptop_in(Path::new(SUPPLIES))
}

fn laptop_in(root: &Path) -> bool {
    let Ok(dir) = std::fs::read_dir(root) else { return false };
    dir.flatten().map(|e| e.path()).any(|p| {
        std::fs::read_to_string(p.join("type")).is_ok_and(|t| t.trim() == "Battery")
            && !std::fs::read_to_string(p.join("scope")).is_ok_and(|s| s.trim() == "Device")
    })
}

pub fn on_battery() -> bool {
    laptop() && !plugged_in(Path::new(SUPPLIES))
}

fn plugged_in(root: &Path) -> bool {
    let Ok(dir) = std::fs::read_dir(root) else { return true };
    dir.flatten().map(|e| e.path()).any(|p| {
        std::fs::read_to_string(p.join("type")).is_ok_and(|t| matches!(t.trim(), "Mains" | "USB"))
            && std::fs::read_to_string(p.join("online")).is_ok_and(|o| o.trim() == "1")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_laptop_on_the_charger_is_found() {
        let root = std::env::temp_dir().join(format!("trindle-charge-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (name, kind) in [("BAT0", "Battery"), ("AC", "Mains")] {
            let d = root.join(name);
            std::fs::create_dir_all(&d).unwrap();
            std::fs::write(d.join("type"), format!("{kind}\n")).unwrap();
        }
        assert!(laptop_in(&root));
        std::fs::write(root.join("AC").join("online"), "0\n").unwrap();
        assert!(!plugged_in(&root));
        std::fs::write(root.join("AC").join("online"), "1\n").unwrap();
        assert!(plugged_in(&root));
        let _ = std::fs::remove_dir_all(&root);
    }
}
