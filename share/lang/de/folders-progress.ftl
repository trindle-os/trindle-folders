
# machine translated (German), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done } von { $of } Dateien, { $rate } pro Sekunde, { $left }
       *[no] { $rate } pro Sekunde, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } von { $of } Dateien, die Dauer wird berechnet…
       *[no] die Dauer wird berechnet…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } Bytes

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = noch einige Sekunden

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] noch etwa 1 Sekunde
       *[other] noch etwa { $n } Sekunden
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = noch etwa eine Minute

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] noch etwa 1 Minute
       *[other] noch etwa { $n } Minuten
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] noch etwa 1 Stunde
       *[other] noch etwa { $n } Stunden
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] Eine weitere Aufgabe läuft
       *[other] { $n } weitere Aufgaben laufen
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Beim Verschieben ist der Datenträger stark ausgelastet, daher können andere Programme langsamer sein, bis es fertig ist.
       *[copying] Beim Kopieren ist der Datenträger stark ausgelastet, daher können andere Programme langsamer sein, bis es fertig ist.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = wartet auf eine Antwort

# End of a piece of work's line while it is paused.
folders-progress-paused = angehalten

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Fortsetzen

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Anhalten

# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = Stoppen

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = neuer und größer

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = neuer

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = größer

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = geändert { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] Ein Ordner namens { $name } ist bereits vorhanden
        [file] Eine Datei namens { $name } ist bereits vorhanden
       *[unknown] { $name } ist bereits vorhanden
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = Das Neue

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = Das bereits Vorhandene

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Überspringen

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = Beide behalten

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = Ersetzen

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = Ersetzen und den bereits vorhandenen Ordner in den Papierkorb verschieben

# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = Zusammenführen

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = Diese Antwort für jeden bereits vorhandenen Ordner verwenden.

# Line over the switch in the name taken question for files.
folders-progress-all-files = Diese Antwort für jede bereits vorhandene Datei verwenden.

# Name and tip of the switch in the name taken question that uses the answer for the rest.
# SAFETY: needs a human review before it ships
folders-progress-same-for-rest = Für den Rest genauso verfahren
