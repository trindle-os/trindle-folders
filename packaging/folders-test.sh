#!/usr/bin/env bash
# Builds this checkout as "Folders test" and installs it for this person only, beside the installed Folders; reopens it if it was open.
set -euo pipefail

repo="$(cd "$(dirname "$0")/.." && pwd)"
lib="$HOME/.local/lib/folders-test"
apps="$HOME/.local/share/applications"

cd "$repo"
FOLDERS_BUS_NAME=org.trindle.FilesTest CARGO_TARGET_DIR="$repo/target/folders-test" cargo build --release --quiet

mkdir -p "$lib/share/trindle" "$apps" "$HOME/.local/bin"
install -m 755 target/folders-test/release/trindle-folders "$lib/folders-test.new"
rm -rf "$lib/share/trindle/lang"
cp -r share/lang "$lib/share/trindle/lang"

cat > "$HOME/.local/bin/folders-test" <<EOF
#!/bin/sh
XDG_DATA_DIRS="$lib/share:\${XDG_DATA_DIRS:-/usr/local/share:/usr/share}" exec "$lib/folders-test" "\$@"
EOF
chmod 755 "$HOME/.local/bin/folders-test"

cat > "$apps/trindle-folders-test.desktop" <<EOF
[Desktop Entry]
Name=Folders test
Comment=The Folders being worked on, beside the installed one
Exec=$HOME/.local/bin/folders-test %U
Icon=system-file-manager
Terminal=false
Type=Application
Categories=System;FileTools;
StartupNotify=false
StartupWMClass=org.trindle.FilesTest
EOF

was_open=0
if pkill -x folders-test; then
    was_open=1
    sleep 1
fi
mv -f "$lib/folders-test.new" "$lib/folders-test"
if [ "$was_open" = 1 ] && [ -n "${WAYLAND_DISPLAY:-}" ]; then
    setsid "$HOME/.local/bin/folders-test" >/dev/null 2>&1 < /dev/null &
fi
echo "Folders test installed$([ "$was_open" = 1 ] && echo ' and reopened')."
