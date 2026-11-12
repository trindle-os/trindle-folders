# Changelog

## 0.1.0

Folders is a program of its own, split out of Trindle's single program.

- Built on the Trindle toolkit and trindle-common from their GitHub repository at a fixed commit, so a clean clone builds.
- It needs nothing of the rest of Trindle to run: the ring, Settings, Apps, Drives, Help and the administrator's helper are looked for by name when it runs, and a menu row that needs a missing part is left out.
- Files for the person's own picks (a key file, a start folder) are asked of the file chooser portal.
- Installed as a Debian package that the main Trindle package depends on; `packaging/build-deb.sh` makes it.
- It installs beside another file manager: its bus service file has a name of its own and it no longer refuses to run beside one. It works only on a Wayland desktop, and the package and README say so.
- Every safety check and its test came across with it: archives, SVGs and films are read in a box, nothing a stranger's file names is followed, nothing is written over where it lies, and nothing is deleted without its proof.
- Renaming a folder or file to change only its letters from capitals to small (or back) works on drives that ignore letter case, instead of saying the name is taken.
- Dead code left from the split was removed (the file chooser lend-outs, the pinned-item looks, a kind-of test, a lock-file writer).
- The fifty-thousand-file timing test (`big_test`) is a speed test, so the usual run skips it; run it alone on a quiet machine with `cargo test --release -- --ignored big_test`.
