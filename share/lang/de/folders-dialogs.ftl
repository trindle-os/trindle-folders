
# machine translated (German), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = Schließen

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = Persönlicher Ordner

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = { $name } öffnen mit

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = Aktuelle App

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = Keine App auf diesem Computer ist dafür eingerichtet, diese Art von Datei zu öffnen.

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = Diese App immer verwenden

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = Für jede Datei vom Typ { $kind }.

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = Als Standard-App festlegen

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = Nur die vorgeschlagenen Apps zeigen

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = Alle Apps zeigen

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = Öffnen

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = Wähle zuerst eine App aus.

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders konnte { $name } nicht zur Standard-App machen.

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders konnte { $name } nicht starten.

# Heading of the Properties window.
folders-dialogs-properties = Eigenschaften

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = Seiten der Eigenschaften

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = Allgemein

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = Details

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = Freigabe



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = Name

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = Ort

# Label in Properties of the kind of file.
folders-dialogs-row-kind = Art

# Label in Properties of where a link points.
folders-dialogs-row-link-to = Verweist auf

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = Geändert

# Label in Properties of who owns the item.
folders-dialogs-row-owner = Besitzer

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = Berechtigungen

# Label in Properties of the size of the item.
folders-dialogs-row-size = Größe

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = Unbekannt

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, du kannst es öffnen und ändern

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, du kannst es öffnen, aber nicht ändern

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, du kannst es ändern, aber nicht öffnen

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, du kannst es nicht öffnen

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , einiges konnte nicht gelesen werden
       *[other]
    }{ $adding ->
        [yes] , wird noch zusammengezählt
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = Größe wird berechnet

# Label in Properties of the row about checksums.
folders-dialogs-checksums = Prüfsummen

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = Prüfsummen berechnen

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = Prüfsummen werden berechnet

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders konnte die Datei nicht lesen.

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = Prüfsumme zum Vergleichen einfügen

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } stimmt überein

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = Keine Übereinstimmung

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = Prüfen


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [one] { $name } senden an
       *[other] { $n } Objekte senden an
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = Dadurch entsteht dort eine Kopie. Die Originale bleiben, wo sie sind.

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = Der andere Bereich, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = Spalten der Listenansicht

# Note in the Columns window.
folders-dialogs-columns-note = Name wird immer angezeigt. Spalten, die nicht passen, werden ausgeblendet, bis der Bereich breiter ist.

# Heading over the columns that suit every file.
folders-dialogs-columns-files = Dateien

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = Musik, Fotos, PDFs

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = { $name } anzeigen

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders konnte die Spalten nicht speichern.

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [one] Farbe für { $name }
       *[other] Farbe für { $n } Ordner
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = Keine Farbe

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders konnte die Ordnerfarbe nicht speichern.


folders-dialogs-no-match = Keine App passt zu diesen Wörtern.


folders-dialogs-find-app = App suchen


folders-dialogs-find-place = Ort suchen


folders-dialogs-no-match-place = Kein Ort passt zu diesen Wörtern.
