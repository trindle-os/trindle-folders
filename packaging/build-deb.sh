#!/usr/bin/env bash
# Builds the trindle-folders Debian package into OUT (first argument, default deb-out).
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(dirname "$here")"
out="${1:-deb-out}"
version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)"
arch="$(dpkg --print-architecture)"
stage="$out/trindle-folders_${version}_${arch}"

cargo_home="${CARGO_HOME:-$HOME/.cargo}"
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$HOME=/home/user --remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$cargo_home/registry/src=/crates --remap-path-prefix=$cargo_home/git/checkouts=/crates --remap-path-prefix=$root=/src"
cargo build --release --offline --locked --manifest-path "$root/Cargo.toml" --target-dir "$root/target"
if grep -aqF "$HOME" "$root/target/release/trindle-folders"; then echo "trindle-folders still holds the path $HOME of this computer" >&2; exit 1; fi

needs="$(ldd "$root/target/release/trindle-folders" | awk '/=>/ {print $1}' | sort | tr '\n' ' ')"
for lib in $needs; do
    case "$lib" in
        libc.so.6|libffi.so.8|libgcc_s.so.1|libm.so.6|libwayland-client.so.0|libxkbcommon.so.0) ;;
        *) echo "trindle-folders now needs $lib: add its package to packaging/depends.txt and to this list ($needs)" >&2; exit 1 ;;
    esac
done

rm -rf "$stage" "$stage.deb"
install -d "$stage/DEBIAN" "$stage/usr/bin" "$stage/usr/share/applications" "$stage/usr/share/dbus-1/services" "$stage/usr/share/doc/trindle-folders"
install -m 0755 "$root/target/release/trindle-folders" "$stage/usr/bin/trindle-folders"
ln -s trindle-folders "$stage/usr/bin/trindle-files"
install -m 0644 "$root/share/applications/trindle-folders.desktop" "$stage/usr/share/applications/trindle-folders.desktop"
install -m 0644 "$here/dbus/org.trindle.Files.FileManager1.service" "$stage/usr/share/dbus-1/services/org.trindle.Files.FileManager1.service"
install -m 0644 "$root/LICENSE" "$stage/usr/share/doc/trindle-folders/copyright"
for lang in "$root"/share/lang/*; do
    install -d "$stage/usr/share/trindle/lang/$(basename "$lang")"
    install -m 0644 "$lang"/*.ftl "$stage/usr/share/trindle/lang/$(basename "$lang")/"
done

depends="$(sed 's/#.*//; s/[[:space:]]*$//' "$here/depends.txt" | grep -v '^$' | paste -sd, | sed 's/,/, /g')"
cat > "$stage/DEBIAN/control" <<CONTROL
Package: trindle-folders
Version: $version
Section: utils
Priority: optional
Architecture: $arch
Depends: libc6, libffi8, libwayland-client0, libxkbcommon0, $depends
Provides: file-manager
Replaces: trindle (<< 1:0.9.0~beta1+local196)
Breaks: trindle (<< 1:0.9.0~beta1+local196)
Maintainer: Trindle <folders@trindle.org>
Description: Folders for Trindle: a file manager for Wayland desktops only
 Two panes, tabs, archives, drives and network places.
 This works only on a Wayland desktop. It does not open on an X11 desktop.
CONTROL
cat > "$stage/DEBIAN/postinst" <<'POSTINST'
#!/bin/sh
set -e
if [ "$1" = configure ] && command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications || true
fi
POSTINST
chmod 0755 "$stage/DEBIAN/postinst"
dpkg-deb --root-owner-group --build "$stage" "$stage.deb" >/dev/null
echo "$stage.deb"
