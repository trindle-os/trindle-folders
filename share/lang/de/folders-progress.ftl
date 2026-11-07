

folders-progress-pace-known =
    { $files ->
        [yes] { $done } von { $of } Dateien, { $rate } pro Sekunde, { $left }
       *[no] { $rate } pro Sekunde, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $done } von { $of } Dateien, die Dauer wird berechnet…
       *[no] die Dauer wird berechnet…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } Bytes

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = noch einige Sekunden

folders-progress-left-seconds =
    { $n ->
        [one] noch etwa 1 Sekunde
       *[other] noch etwa { $n } Sekunden
    }

folders-progress-left-minute = noch etwa eine Minute

folders-progress-left-minutes =
    { $n ->
        [one] noch etwa 1 Minute
       *[other] noch etwa { $n } Minuten
    }

folders-progress-left-hours =
    { $n ->
        [one] noch etwa 1 Stunde
       *[other] noch etwa { $n } Stunden
    }

folders-progress-more =
    { $n ->
        [one] Eine weitere Aufgabe läuft
       *[other] { $n } weitere Aufgaben laufen
    }

folders-progress-heavy =
    { $verb ->
        [moving] Das Verschieben belastet den Datenträger stark, daher können andere Programme bis zum Ende langsamer wirken.
       *[copying] Das Kopieren belastet den Datenträger stark, daher können andere Programme bis zum Ende langsamer wirken.
    }

folders-progress-waiting = wartet auf eine Antwort

folders-progress-paused = angehalten

folders-progress-carry-on = Fortsetzen

folders-progress-pause = Anhalten

folders-progress-stop = Stoppen

folders-progress-mark-newer-larger = neuer, größer

folders-progress-mark-newer = neuer

folders-progress-mark-larger = größer

folders-progress-changed = geändert { $when }

folders-progress-taken =
    { $kind ->
        [folder] Ein Ordner namens { $name } ist bereits vorhanden
        [file] Eine Datei namens { $name } ist bereits vorhanden
       *[unknown] { $name } ist bereits vorhanden
    }

folders-progress-coming = Das Neue

folders-progress-there = Das bereits Vorhandene

folders-progress-skip = Überspringen

folders-progress-keep-both = Beide behalten

folders-progress-replace = Ersetzen

folders-progress-replace-folder-spoken = Ersetzen: Der bereits vorhandene Ordner kommt in den Papierkorb

folders-progress-merge = Zusammenführen

folders-progress-all-folders = Dieselbe Antwort für jeden bereits vorhandenen Ordner.

folders-progress-all-files = Dieselbe Antwort für jede bereits vorhandene Datei.

folders-progress-same-for-rest = Für den Rest genauso verfahren
