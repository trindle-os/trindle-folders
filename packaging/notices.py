# Prints the licence notices of every crate built into Folders, for the package's doc folder.
import json, os, re, subprocess, sys

root = sys.argv[1]
meta = json.loads(subprocess.run(
    ["cargo", "metadata", "--format-version", "1", "--offline", "--locked",
     "--filter-platform", "x86_64-unknown-linux-gnu", "--manifest-path", os.path.join(root, "Cargo.toml")],
    check=True, capture_output=True, text=True).stdout)
packages = {p["id"]: p for p in meta["packages"]}
nodes = {n["id"]: n for n in meta["resolve"]["nodes"]}
top = meta["resolve"]["root"]

built, todo = set(), [top]
while todo:
    at = todo.pop()
    for dep in nodes[at]["deps"]:
        if any(k["kind"] is None for k in dep["dep_kinds"]) and dep["pkg"] not in built:
            built.add(dep["pkg"])
            todo.append(dep["pkg"])

texts = re.compile(r"^(licen[cs]e|copying|notice|copyright|unlicense)", re.I)
out = ["Licence notices of the crates built into trindle-folders.", ""]
for p in sorted((packages[i] for i in built), key=lambda p: (p["name"], p["version"])):
    out.append("=" * 72)
    out.append(f"{p['name']} {p['version']}: {p.get('license') or 'see below'}")
    out.append("=" * 72)
    here = os.path.dirname(p["manifest_path"])
    found = sorted(f for f in os.listdir(here) if texts.match(f) and os.path.isfile(os.path.join(here, f)))
    for f in found:
        out.append(f"--- {f}")
        with open(os.path.join(here, f), encoding="utf-8", errors="replace") as t:
            out.append(t.read().rstrip())
    if not found:
        out.append("(the crate ships no licence file of its own; the text of the licence named above is given with other crates here)")
    out.append("")
print("\n".join(out))
