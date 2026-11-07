# Trindle Folders

Folders is the file manager of Trindle, written in Rust on the Trindle toolkit. It shows two panes with tabs, opens archives like folders, lists drives, phones and network places, finds files, compares folders and keeps an earlier copy of a file within reach.

It is a program of its own. The main Trindle package depends on it, but it needs nothing of the rest of Trindle to run, and it runs on Debian 13 and Ubuntu 24.04 or later, beside whatever other file manager is installed.

**It works only on a Wayland desktop.** It does not open on an X11 desktop.

What it can use when it is there (the ring, Apps, Settings, Help, the administrator's helper) is looked for when it runs; a row that needs a missing part is left out of the menus.

## Build

```
cargo build --release
cargo test --release
packaging/build-deb.sh
```

It needs Rust 1.89 or newer (checked with 1.89), so the compiler that comes with Debian 13 (1.85) is too old. The package is built with Rust 1.98.1.

The tests need a session bus (`dbus-run-session -- cargo test --release`), bubblewrap, ffmpeg and poppler-utils. `PLAN.md` lists what Folders takes from the rest of Trindle and how; `docs/MAIN-REMOVAL.md` lists what the main program stops containing.

## Safety

Folders handles files that strangers made. Archives, SVGs and films are read in a bubblewrap box with no network. Nothing a stranger's file names is followed. A file is never written over where it lies, and a person's own file is deleted only with its proof or into the Trash. The tests that hold these are in `src/common` and `src/files`.

## Reporting problems

Bugs and ideas: the [Buildmorphic community](https://trindle.buildmorphic.com). Security: see [SECURITY.md](SECURITY.md). Code and releases are on GitHub, under trindle-os.

GPL-3.0-or-later.

Comments are stripped from the published code to keep it lean.
