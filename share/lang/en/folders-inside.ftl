# Folders' notices about taking things out of an archive and reading one (src/files/inside.rs).

## Inside an archive (src/files/inside.rs)

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = Select something in the archive first.
# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = You can only take things out of an archive into a folder on this computer.
# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = Folders couldn't read { $name }: { $why }.
# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
        [one] Folders couldn't take out 1 item, because its name points outside the folder or something is in the way.
       *[other] Folders couldn't take out { $n } items, because their names point outside the folder or something is in the way.
    }
# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = Folders couldn't take it out of the archive: { $why }.
