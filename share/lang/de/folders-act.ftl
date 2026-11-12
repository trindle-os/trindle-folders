# machine translated (German), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Ausschneiden, Kopieren und Einfügen funktionieren bei Dateien auf einem Server nicht. Verwende stattdessen die Schaltflächen Kopieren und Verschieben zwischen den Fenstern.

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = In einem Archiv kannst du nichts ändern. Entpacke zuerst, was du brauchst.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = Diese wurden in einer anderen App ausgeschnitten, deshalb hat Folders sie kopiert.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = Das wird bereits kopiert oder verschoben.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = Das wird noch hierher kopiert.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10.000 Einträge oder mehr

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] 1 Eintrag
       *[other] { $n } Einträge
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] Ein Eintrag konnte nicht { $verb } werden: { $names }.
       *[other] { $n } Einträge konnten nicht { $verb } werden: { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } und { $more } weitere

# Words on a job card while things are copied: what, and where to.
folders-act-copying = { $what } wird nach { $place } kopiert

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = { $what } nach { $place } kopiert.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = kopiert

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = Kopieren von { $what } rückgängig machen

# Words on a job card while things are moved: what, and where to.
folders-act-moving = { $what } wird nach { $place } verschoben

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = { $what } nach { $place } verschoben.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = verschoben

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = Verschieben von { $what } rückgängig machen

# Words on a job card while things are put in the Trash.
folders-act-trashing = { $what } wird in den Papierkorb gelegt

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = { $what } in den Papierkorb gelegt.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = in den Papierkorb gelegt

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = Ablegen von { $what } im Papierkorb rückgängig machen

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = { $what } wird gelöscht

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = { $what } endgültig gelöscht.

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = gelöscht

# Words on a job card while things are put back from the Trash.
folders-act-restoring = { $what } wird zurückgelegt

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = { $what } zurückgelegt.

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = Zurücklegen von { $what } rückgängig machen

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = zurückgelegt

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Wird rückgängig gemacht

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Rückgängig gemacht.

# Words on a job card while things are packed into an archive.
folders-act-compressing = { $what } wird komprimiert

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = { $file } in { $place } erstellt.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = komprimiert

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = Komprimieren von { $what } rückgängig machen

# Words on a job card while an archive is unpacked.
folders-act-extracting = { $what } wird entpackt

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = { $what } in { $place } entpackt.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = entpackt

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = Entpacken von { $what } rückgängig machen

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = { $what } wird sicher gelöscht

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = { $what } sicher gelöscht.

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = sicher gelöscht

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = { $what } wird herausgenommen

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = { $what } aus dem Archiv in { $place } entpackt.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = herausgenommen

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = Herausnehmen von { $what } rückgängig machen

# Words on a job card while pictures are turned.
folders-act-turning = { $what } wird gedreht

# Words under the file lines when pictures have been turned.
folders-act-turned = { $what } gedreht.

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = gedreht

# Words on a job card while pictures are resized.
folders-act-resizing = Größe von { $what } wird geändert

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = Eine Kopie von { $what } in anderer Größe erstellt.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = in der Größe geändert

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = Größenänderung von { $what } rückgängig machen

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = Neuer Ordner

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = Neues Dokument

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders konnte { $base } nicht erstellen: { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = Gib einen Namen ein.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = Ein Name darf keinen Schrägstrich enthalten.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = Dieser Name wird vom System verwendet. Wähle einen anderen.

# Red words under a name being typed when it is too long.
folders-act-name-long = Dieser Name ist zu lang.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders konnte diese Aufgabe nicht starten.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = { $what } wiederholen

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = { $what } rückgängig machen

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = Die Aufgabe wurde angehalten, bevor sie fertig war.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped =
    { $done } { $n ->
        [one] Folders hat einen Eintrag übersprungen.
       *[other] Folders hat { $n } Einträge übersprungen.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Wähle zuerst etwas aus.

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = Das ist schon im Papierkorb.

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = Die Dateien eines Servers können nicht in den Papierkorb gelegt werden. Verwende stattdessen „Endgültig löschen“.

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Wähle zuerst etwas zum Kopieren oder Verschieben aus.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = Du kannst Dinge nur aus einem Archiv in einen Ordner entpacken.

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = Du kannst nichts in den Papierkorb kopieren.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = Das andere Fenster zeigt keinen Ordner.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Wähle zuerst etwas zum Kopieren aus.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = { $what } ausgeschnitten.

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = { $what } kopiert.

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = Du kannst nur in einen Ordner einfügen.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = In der Zwischenablage sind keine Dateien.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = Es wurde nichts ausgeschnitten oder kopiert.

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = Du kannst Netzwerkorte nicht von hier aus löschen.

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = Der Papierkorb ist schon leer.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Wähle zuerst etwas im Papierkorb aus.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = { $name } wird erstellt

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = Du kannst einen neuen Ordner oder ein neues Dokument nur in einem Ordner erstellen.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = Erstellen von { $name } rückgängig machen

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders konnte die Ordnerfarben bei den verschobenen Einträgen nicht beibehalten.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = Es gibt nichts zu wiederholen.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = Es gibt nichts rückgängig zu machen.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = { $what } wird wiederholt

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = { $what } wiederholt.

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = wiederholt

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = { $what } wird rückgängig gemacht

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = { $what } rückgängig gemacht.

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Persönlicher Ordner

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Papierkorb

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = Du kannst Dinge nur auf einem Ordner oder dem Papierkorb ablegen.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] 1 Verknüpfung
       *[other] { $n } Verknüpfungen
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = { $what } erstellt.

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] Eine Verknüpfung konnte nicht erstellt werden: { $names }.
       *[other] { $n } Verknüpfungen konnten nicht erstellt werden: { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = Eine Frage wartet auf deine Antwort. Beantworte sie oder brich die Aufgabe ab, bevor du Folders schließt.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = Folders lässt sich nicht schließen, solange eine Aufgabe läuft. Warte, bis sie fertig ist, oder brich sie ab und schließe Folders dann.
