
# machine translated (French), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Couper, copier et coller ne fonctionnent pas avec les fichiers d'un serveur. Utilisez plutôt les boutons de copie et de déplacement entre les volets.

# Words under the file lines when something is tried that would change what is inside an archive.
# SAFETY: needs a human review before it ships
folders-act-shut = Vous ne pouvez rien modifier dans une archive. Extrayez d'abord ce dont vous avez besoin.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = Ces éléments ont été coupés dans une autre application, donc Folders les a copiés à la place.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = Cet élément est déjà en cours de copie ou de déplacement.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = Cet élément est encore en cours de copie ici.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10 000 éléments ou plus

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] 1 élément
        [many] { $n } d'éléments
       *[other] { $n } éléments
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] Impossible de { $verb } un élément : { $names }.
        [many] Impossible de { $verb } { $n } d'éléments : { $names }.
       *[other] Impossible de { $verb } { $n } éléments : { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept } : { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } et { $more } de plus

# Words on a job card while things are copied: what, and where to.
folders-act-copying = Copie de { $what } vers { $place }

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = { $what } copié vers { $place }.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = copier

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = Annuler la copie de { $what }

# Words on a job card while things are moved: what, and where to.
folders-act-moving = Déplacement de { $what } vers { $place }

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = { $what } déplacé vers { $place }.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = déplacer

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = Annuler le déplacement de { $what }

# Words on a job card while things are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-trashing = Mise de { $what } à la corbeille

# Words under the file lines when things have been put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-trashed = { $what } mis à la corbeille.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
# SAFETY: needs a human review before it ships
folders-act-verb-trashed = mettre à la corbeille

# Tip of the Undo button after things were put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-undo-trashing = Annuler la mise à la corbeille de { $what }

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = Suppression de { $what }

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = { $what } supprimé définitivement.

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = supprimer

# Words on a job card while things are put back from the Trash.
folders-act-restoring = Remise en place de { $what }

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = { $what } remis en place.

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = Annuler la remise en place de { $what }

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = remettre en place

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Annulation en cours

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Annulé.

# Words on a job card while things are packed into an archive.
folders-act-compressing = Compression de { $what }

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = { $file } créé dans { $place }.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = compresser

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = Annuler la compression de { $what }

# Words on a job card while an archive is unpacked.
folders-act-extracting = Extraction de { $what }

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = { $what } extrait dans { $place }.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = extraire

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = Annuler l'extraction de { $what }

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = Effacement sécurisé de { $what }

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = { $what } effacé de façon sécurisée.

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = effacer de façon sécurisée

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = Extraction de { $what }

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = { $what } extrait de l'archive dans { $place }.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = extraire

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = Annuler l'extraction de { $what }

# Words on a job card while pictures are turned.
folders-act-turning = Rotation de { $what }

# Words under the file lines when pictures have been turned.
folders-act-turned = Rotation de { $what } effectuée.

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = faire pivoter

# Words on a job card while pictures are resized.
folders-act-resizing = Redimensionnement de { $what }

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = Copie redimensionnée de { $what } créée.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = redimensionner

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = Annuler le redimensionnement de { $what }

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = Nouveau dossier

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = Nouveau document

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders n'a pas pu créer { $base } : { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = Saisissez un nom.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = Un nom ne peut pas contenir de barre oblique.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = Le système utilise ce nom. Choisissez-en un autre.

# Red words under a name being typed when it is too long.
folders-act-name-long = Ce nom est trop long.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders n'a pas pu démarrer cette tâche.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = Rétablir { $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = Annuler { $what }

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = La tâche a été arrêtée avant la fin.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [one] Folders a ignoré un élément.
        [many] Folders a ignoré { $n } d'éléments.
       *[other] Folders a ignoré { $n } éléments.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Sélectionnez d'abord un élément.

# Words under the file lines when something already in the Trash is put there again.
# SAFETY: needs a human review before it ships
folders-act-in-trash = C'est déjà dans la corbeille.

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = Les fichiers d'un serveur ne peuvent pas être mis à la corbeille. Utilisez plutôt Supprimer définitivement.

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Sélectionnez d'abord un élément à copier ou à déplacer.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = Les éléments d'une archive ne peuvent être extraits que dans un dossier.

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = Vous ne pouvez pas copier des éléments dans la corbeille.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = L'autre volet n'affiche pas de dossier.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Sélectionnez d'abord un élément à copier.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = { $what } coupé.

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = { $what } copié.

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = Vous ne pouvez coller que dans un dossier.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = Il n'y a pas de fichiers dans le presse-papiers.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = Rien n'a été coupé ni copié.

# Words under the file lines when Delete is used on the Network list.
folders-act-network-delete = Vous ne pouvez pas supprimer des emplacements réseau d'ici.

# Words under the file lines when Empty the Trash is used on an empty Trash.
# SAFETY: needs a human review before it ships
folders-act-trash-empty = La corbeille est déjà vide.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Sélectionnez d'abord un élément dans la corbeille.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = Création de { $name }

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = Vous ne pouvez créer un nouveau dossier ou document que dans un dossier.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = Annuler la création de { $name }

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders n'a pas pu conserver les couleurs des dossiers sur les éléments déplacés.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = Il n'y a rien à rétablir.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = Il n'y a rien à annuler.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = Rétablissement de { $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = { $what } rétabli.

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = rétablir

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = Annulation de { $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = { $what } annulé.

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Dossier personnel

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Corbeille

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = Vous ne pouvez déposer des éléments que sur un dossier ou la corbeille.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] 1 lien
        [many] { $n } de liens
       *[other] { $n } liens
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = { $what } créé.

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] Impossible de créer un lien : { $names }.
        [many] Impossible de créer { $n } de liens : { $names }.
       *[other] Impossible de créer { $n } liens : { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
# SAFETY: needs a human review before it ships
folders-act-closing-waits = Une question attend votre réponse. Répondez-y, ou arrêtez la tâche, avant de fermer Folders.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = Folders ne peut pas se fermer tant qu'une tâche est en cours. Attendez qu'elle se termine, ou arrêtez-la, puis fermez Folders.
