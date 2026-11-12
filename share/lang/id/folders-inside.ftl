
# machine translated (Indonesian), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = Pilih sesuatu di dalam arsip terlebih dahulu.

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = Isi arsip hanya dapat diekstrak ke folder di komputer ini.

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = Folders tidak dapat membaca { $name }: { $why }.

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
       *[other] Folders tidak dapat mengekstrak { $n } item, karena namanya mengarah ke luar folder atau ada yang menghalangi.
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = Folders tidak dapat mengekstraknya dari arsip: { $why }.
