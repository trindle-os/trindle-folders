#!/bin/sh
# Install, remove and purge test for trindle-folders. Run as root on the desktop:
set -u
deb=${1:?give the .deb}
fail=0
ok() { echo "PASS $1"; }
bad() { echo "FAIL $1"; fail=1; }
check() { desc=$1; shift; if "$@" >/dev/null 2>&1; then ok "$desc"; else bad "$desc"; fi; }

log=$(mktemp)
trap 'rm -f "$log"' EXIT
dpkg -i "$deb" >"$log" 2>&1 || { echo "FAIL dpkg -i"; cat "$log"; exit 1; }
check "program installed" test -x /usr/bin/trindle-folders
check "old name links to it" test "$(readlink /usr/bin/trindle-files)" = trindle-folders
check "launcher installed" test -f /usr/share/applications/trindle-folders.desktop
check "file manager service installed" grep -q 'trindle-folders' /usr/share/dbus-1/services/org.trindle.Files.FileManager1.service
check "language files installed" test -f /usr/share/trindle/lang/en/folders-act.ftl
check "dpkg owns the launcher" sh -c 'dpkg -S /usr/share/applications/trindle-folders.desktop | grep -q "^trindle-folders:"'
check "dpkg owns the service" sh -c 'dpkg -S /usr/share/dbus-1/services/org.trindle.Files.FileManager1.service | grep -q "^trindle-folders:"'
check "no missing libraries" sh -c '! ldd /usr/bin/trindle-folders | grep -q "not found"'
check "main package still installed" sh -c 'dpkg -s trindle | grep -q "^Status: install ok installed"'
check "package database is sound" sh -c '! dpkg --audit | grep .'

dpkg -r trindle-folders >/dev/null 2>&1 || bad "dpkg -r"
check "program gone after remove" sh -c '! test -e /usr/bin/trindle-folders'
check "service gone after remove" sh -c '! test -e /usr/share/dbus-1/services/org.trindle.Files.FileManager1.service || dpkg -S /usr/share/dbus-1/services/org.trindle.Files.FileManager1.service | grep -q "^trindle:"'
check "main package untouched by remove" sh -c 'dpkg -s trindle | grep -q "^Status: install ok installed"'

dpkg -i "$deb" >/dev/null 2>&1
dpkg -P trindle-folders >/dev/null 2>&1 || bad "dpkg -P"
check "nothing left after purge" sh -c '! test -e /usr/share/trindle/lang/en/folders-act.ftl && ! test -e /usr/bin/trindle-folders'
check "package not known after purge" sh -c '! dpkg -s trindle-folders 2>/dev/null | grep -q "^Status: install"'

dpkg -i "$deb" >/dev/null 2>&1 && ok "installed again and left in place"
[ $fail = 0 ] && { echo "ALL PASSED"; exit 0; }
echo "SOME FAILED"
exit 1
