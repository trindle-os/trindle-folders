# Trindle Folders

Folders is Trindle's file manager. It's written in Rust on the Trindle toolkit. You can use two panes with tabs, and open archives as if they were folders. It lists your drives, phones and network places. It can also find files, compare two folders and get back an earlier copy of a file.

It's a program of its own. The main Trindle package needs it, but Folders doesn't need the rest of Trindle to run. It runs on Debian 13 or later, and on distributions built on Debian of the same age. It can sit beside any other file manager you have installed.

**It works only on a Wayland desktop.** It won't open on an X11 desktop.

When Folders starts, it looks for the other Trindle parts it can use: the ring, Apps, Settings, Help and the administrator's helper. If a part is missing, Folders leaves out the menu rows that need it.

## Build

```
cargo build --release
cargo test --release
packaging/build-deb.sh
```

You need Rust 1.89 or newer (it's checked with 1.89). The compiler that comes with Debian 13 is 1.85, which is too old. The package is built with Rust 1.98.1.

The tests need a session bus (`dbus-run-session -- cargo test --release`), bubblewrap, ffmpeg and poppler-utils.

## Safety

Folders opens files that strangers made. It reads archives, SVGs and films inside a bubblewrap box with no network. It never follows anything a stranger's file points to. It never writes over a file in place. It removes a file of yours only after checking that it was copied whole, or that it's the very file you chose to delete. The tests for these checks are in `src/common` and `src/files`.

## Reporting problems

Please send bugs and ideas to the [Buildmorphic community](https://trindle.buildmorphic.com). For security problems, see [SECURITY.md](SECURITY.md). The code and releases are on GitHub, under trindle-os.

Folders is licensed under GPL-3.0-or-later.

Comments are stripped from the published code to keep it lean.
