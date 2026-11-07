

folders-tools-pass-once = Einmal

folders-tools-pass-three = 3-mal

folders-tools-pass-seven = 7-mal

folders-tools-case-keep = Unverändert

folders-tools-case-lower = Alles klein

folders-tools-case-upper = Alles groß

folders-tools-case-words = Jedes Wort

folders-tools-case-first = Erstes Wort

folders-tools-ending-keep = Unverändert

folders-tools-ending-lower = Alles klein

folders-tools-ending-upper = Alles groß

folders-tools-ending-new = Eine neue Endung

folders-tools-ending-none = Keine Endung

folders-tools-archive-name = Archiv

folders-tools-clash-empty = es bliebe kein Name übrig

folders-tools-clash-bad = das ist als Name nicht erlaubt

folders-tools-clash-long = der Name ist zu lang

folders-tools-clash-twice = zwei würden diesen Namen bekommen

folders-tools-clash-taken = etwas anderes hat diesen Namen

folders-tools-clash-not-text = der alte Name ist kein einfacher Text

folders-tools-caveat-flash = Dieser Datenträger ist ein Flash-Speicher und keine drehende Platte, deshalb können alte Kopien in Teilen weiterleben, die Trindle nicht erreichen kann.

folders-tools-caveat-copy-on-write = Das Dateisystem { $fs } schreibt Änderungen an eine neue Stelle, deshalb können die alten Daten dort bleiben, wo sie waren.

folders-tools-caveat-network = Dies ist eine { $fs }-Freigabe auf einem anderen Computer, der eigene Kopien behalten kann.

folders-tools-row-only-left = Nur links

folders-tools-row-only-right = Nur rechts

folders-tools-row-same = Gleich

folders-tools-row-different =
    Unterschiedlich: { $why ->
        [kind] eines ist ein Ordner oder Link
        [size] die Größen unterscheiden sich
        [date] die Daten unterscheiden sich
        [content] die Inhalte unterscheiden sich
        [target] die Links zeigen woandershin
        [inside] etwas darin unterscheidet sich
       *[unread] eine Seite konnte nicht gelesen werden
    }{ $newer ->
        [left] , links neuer
        [right] , rechts neuer
       *[none] { "" }
    }

folders-tools-place-here = Hier

folders-tools-only-files-wiped = Nur Dateien können sicher gelöscht werden.

folders-tools-compare-need-folders = Beide Bereiche müssen einen Ordner zeigen, um sie zu vergleichen.

folders-tools-rename-undo =
    { $n ->
        [one] Umbenennen von 1 Objekt rückgängig machen
       *[other] Umbenennen von { $n } Objekten rückgängig machen
    }

folders-tools-rename-done =
    { $n ->
        [one] 1 Objekt umbenannt.
       *[other] { $n } Objekte umbenannt.
    }

folders-tools-rename-stopped = Das Umbenennen wurde mittendrin angehalten: { $error }.

folders-tools-name-empty = Ein Name darf nicht leer sein.

folders-tools-looked-at = { $done } angesehen

folders-tools-read-of = { $done } von { $total } gelesen

folders-tools-listing-files = Dateien werden aufgelistet

folders-tools-comparing = Wird verglichen

folders-tools-looking = Wird angesehen

folders-tools-compress-title =
    { $n ->
        [one] { $name } komprimieren
       *[other] { $n } Objekte komprimieren
    }

folders-tools-pack-name-tip = Name des Archivs

folders-tools-pack-kind-tip = Art des Archivs

folders-tools-pack-made-in = Wird erstellt in { $place }

folders-tools-compress = Komprimieren

folders-tools-extract-to = { $name } entpacken nach

folders-tools-extract-new-folder = Dort wird ein neuer Ordner namens { $name } angelegt.

folders-tools-resize-title =
    { $n ->
        [one] Größe von { $name } ändern
       *[other] Größe von { $n } Bildern ändern
    }

folders-tools-resize-size-tip = Neue Größe

folders-tools-resize-note = Ein Anteil der Größe oder die Anzahl der Pixel entlang der längeren Seite; ein Bild wird nie vergrößert. Neben jedem wird eine Kopie mit der neuen Größe im Namen angelegt; die Bilder selbst bleiben, wie sie sind.

folders-tools-resize = Größe ändern

folders-tools-wipe-title =
    { $n ->
        [one] { $name } sicher löschen?
       *[other] { $n } Dateien sicher löschen?
    }

folders-tools-wipe-warning = Jede Datei wird überschrieben, bevor sie verschwindet, und kann deshalb nicht wiederhergestellt werden. Das lässt sich nicht rückgängig machen.

folders-tools-wipe-write-over = Überschreiben

folders-tools-wipe-passes-tip = Wie oft jede Datei überschrieben wird

folders-tools-wipe-button = Sicher löschen

folders-tools-rename-number-tip = Nummerieren

folders-tools-rename-at-start = Am Anfang

folders-tools-rename-at-end = Am Ende

folders-tools-rename-where-tip = Wo die Nummer steht

folders-tools-rename-first-number-tip = Erste Nummer

folders-tools-rename-ending-tip = Was mit der Endung geschieht

folders-tools-rename-new-ending-tip = Neue Endung, zum Beispiel txt

folders-tools-rename-cannot = Nicht möglich: { $why }

folders-tools-rename-was = war { $old }

folders-tools-rename-stays = bleibt, wie es ist

folders-tools-rename-title =
    { $n ->
        [one] 1 Objekt umbenennen
       *[other] { $n } Objekte umbenennen
    }

folders-tools-rename-replace = Ersetzen

folders-tools-rename-find-tip = Suchen

folders-tools-rename-with-tip = Ersetzen durch

folders-tools-rename-wildcard-tip = * und ? als Platzhalter verwenden

folders-tools-rename-wildcard = Platzhalter * und ?

folders-tools-rename-match-case-tip = Groß- und Kleinschreibung genau beachten

folders-tools-rename-match-case = Groß-/Kleinschreibung beachten

folders-tools-rename-add = Hinzufügen

folders-tools-rename-before-tip = Vor dem Namen

folders-tools-rename-after-tip = Nach dem Namen

folders-tools-rename-number = Nummer

folders-tools-rename-letters = Buchstaben

folders-tools-rename-case-tip = Großbuchstaben in den Namen

folders-tools-rename-ending = Endung

folders-tools-rename-summary =
    { $clash ->
        [yes] Einige Namen können nicht verwendet werden; siehe die markierten Zeilen. In { $place }.
       *[no] { $changes } von { $total } Namen ändern sich. In { $place }.
    }

folders-tools-rename-button = Alle umbenennen

folders-tools-home = Home

folders-tools-dupes-title =
    { $n ->
        [one] Duplikate in { $folder }
       *[other] Duplikate in { $n } Ordnern
    }

folders-tools-dupes-stopped = Die Suche wurde angehalten: { $error }.

folders-tools-dupes-none = Es wurden keine Duplikate gefunden.

folders-tools-dupes-unread =
    { $n ->
        [one] 1 Datei konnte nicht gelesen werden.
       *[other] { $n } Dateien konnten nicht gelesen werden.
    }

folders-tools-dupes-sets =
    { $n ->
        [one] 1 Gruppe von Kopien; die überzähligen in den Papierkorb zu legen, gibt { $size } frei. Die überzähligen Kopien sind ausgewählt; Strg und Klick wählt eine aus oder ab.
       *[other] { $n } Gruppen von Kopien; die überzähligen in den Papierkorb zu legen, gibt { $size } frei. Die überzähligen Kopien sind ausgewählt; Strg und Klick wählt eine aus oder ab.
    }

folders-tools-dupes-row = Gruppe { $set }, { $size }, in { $at }

folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 Datei konnte nicht gelesen werden und wurde ausgelassen.
       *[other] { $n } Dateien konnten nicht gelesen werden und wurden ausgelassen.
    }

folders-tools-dupes-keep-one = Alle Kopien einer Gruppe sind ausgewählt; behalte mindestens eine.

folders-tools-dupes-pick-extras = Die überzähligen Kopien auswählen

folders-tools-dupes-trash = In den Papierkorb verschieben

folders-tools-diff-title = Beide vergleichen

folders-tools-diff-failed = Sie konnten nicht verglichen werden: { $error }.

folders-tools-diff-same-bytes = Beide sind gleich, Byte für Byte.

folders-tools-diff-same-as-shorter = Sie sind gleich, soweit die kürzere reicht; eine ist { $one } groß und die andere { $other }.

folders-tools-diff-from-byte = Sie unterscheiden sich ab Byte { $n }.

folders-tools-diff-lines =
    { $n ->
        [0] Beide sind gleich.
        [one] 1 Zeile unterscheidet sich.
       *[other] { $n } Zeilen unterscheiden sich.
    }

folders-tools-diff-too-different = Sie sind zu verschieden, um Zeile für Zeile verglichen zu werden, deshalb wird jede Zeile zwischen dem ersten und dem letzten Unterschied als geändert angezeigt.

folders-tools-diff-cut = Nur die ersten 4 MB jeder Datei werden verglichen.

folders-tools-compare-title = { $left } mit { $right } vergleichen

folders-tools-compare-bytes = Jedes Byte lesen (langsamer)

folders-tools-compare-bytes-tip = Den Inhalt jeder Datei vergleichen

folders-tools-compare-show-same = Gleiches anzeigen

folders-tools-compare-show-same-tip = Auch die gleichen Namen anzeigen

folders-tools-compare-stopped = Der Vergleich wurde angehalten: { $error }.

folders-tools-compare-counts = { $left } nur links, { $right } nur rechts, { $different } unterschiedlich, { $same } gleich.

folders-tools-compare-all-same = Beide Ordner enthalten dasselbe.

folders-tools-compare-again = Erneut vergleichen

folders-tools-compare-copy-left = { $n } fehlende oder neuere nach links kopieren

folders-tools-compare-copy-right = { $n } fehlende oder neuere nach rechts kopieren
