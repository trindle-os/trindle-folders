
# machine translated (French), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Choice in the Wipe window: write over each file one time.
folders-tools-pass-once = Une fois

# SAFETY: needs a human review before it ships
# Choice in the Wipe window: write over each file three times.
folders-tools-pass-three = 3 fois

# SAFETY: needs a human review before it ships
# Choice in the Wipe window: write over each file seven times.
folders-tools-pass-seven = 7 fois

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = Inchangées

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = Tout en minuscules

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = Tout en majuscules

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = Mettre une majuscule à chaque mot

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = Mettre une majuscule au premier mot

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = Inchangée

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = Tout en minuscules

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = Tout en majuscules

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = Nouvelle extension

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = Aucune extension

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = Archive

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = il ne resterait aucun nom

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = ce n’est pas permis comme nom

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = ce nom est trop long

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = deux éléments recevraient ce nom

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = autre chose porte déjà ce nom

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = l’ancien nom contient des caractères que Folders ne peut pas lire

# SAFETY: needs a human review before it ships
# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
folders-tools-caveat-flash = Il s’agit d’une clé USB ou d’un SSD, donc d’anciennes copies peuvent subsister dans des parties que Folders ne peut pas atteindre.

# SAFETY: needs a human review before it ships
# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
folders-tools-caveat-copy-on-write = Le système de fichiers { $fs } enregistre les changements à un nouvel endroit, donc les anciennes données peuvent encore être là.

# SAFETY: needs a human review before it ships
# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
folders-tools-caveat-network = Il s’agit d’un partage { $fs } sur un autre ordinateur, qui peut garder ses propres copies.

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = Seulement à gauche

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = Seulement à droite

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = Identique

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
folders-tools-row-different =
    Différent : { $why ->
        [kind] l’un est un dossier ou un lien
        [size] les tailles diffèrent
        [date] les dates diffèrent
        [content] les contenus diffèrent
        [target] les liens pointent ailleurs
        [inside] quelque chose à l’intérieur diffère
       *[unread] un côté n’a pas pu être lu
    }{ $newer ->
        [left] , plus récent à gauche
        [right] , plus récent à droite
       *[none] { "" }
    }

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = Ici

# SAFETY: needs a human review before it ships
# Red notice when Wipe is chosen on things that are not files.
folders-tools-only-files-wiped = Seuls des fichiers peuvent être effacés.

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = Ouvrez un dossier dans chaque panneau pour les comparer.

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
        [one] Annuler le renommage de 1 élément
        [many] Annuler le renommage de { $n } éléments
       *[other] Annuler le renommage de { $n } éléments
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [one] 1 élément renommé.
        [many] { $n } éléments renommés.
       *[other] { $n } éléments renommés.
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = Le renommage s’est arrêté en cours de route : { $error }.

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = Saisissez un nom pour l’archive.

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = { $done } vérifiés

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = { $done } lus sur { $total }

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = Liste des fichiers

# Beside the ring while two files are compared.
folders-tools-comparing = Comparaison

# Beside the ring while two folders are compared.
folders-tools-looking = Comparaison des dossiers

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [one] Compresser { $name }
        [many] Compresser { $n } éléments
       *[other] Compresser { $n } éléments
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = Nom de l’archive

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = Type d’archive

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = L’archive sera enregistrée dans { $place }

# Main button of the Compress window.
folders-tools-compress = Compresser

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = Extraire { $name } vers

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = Les fichiers seront placés dans un nouveau dossier nommé { $name }.

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [one] Redimensionner { $name }
        [many] Redimensionner { $n } images
       *[other] Redimensionner { $n } images
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = Nouvelle taille

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = La taille est la longueur du plus grand côté, en pixels. Folders ne fait que réduire les images. Il enregistre chaque copie redimensionnée à côté de l’original, avec la nouvelle taille dans son nom.

# Main button of the Resize window.
folders-tools-resize = Redimensionner

# SAFETY: needs a human review before it ships
# Title of the Wipe window; { $name } is the one file, { $n } how many.
folders-tools-wipe-title =
    { $n ->
        [one] Effacer { $name } de façon sécurisée ?
        [many] Effacer { $n } fichiers de façon sécurisée ?
       *[other] Effacer { $n } fichiers de façon sécurisée ?
    }

# SAFETY: needs a human review before it ships
# Warning at the top of the Wipe window.
folders-tools-wipe-warning = Folders écrase chaque fichier avant de le supprimer, vous ne pourrez donc pas le récupérer.

# SAFETY: needs a human review before it ships
# Name beside the choice of how many times to write over, in the Wipe window.
folders-tools-wipe-write-over = Écraser

# SAFETY: needs a human review before it ships
# Choice tip in the Wipe window: how many times each file is written over.
folders-tools-wipe-passes-tip = Combien de fois écraser chaque fichier

# SAFETY: needs a human review before it ships
# Red main button of the Wipe window.
folders-tools-wipe-button = Effacer de façon sécurisée

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = Les numéroter

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = Au début

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = À la fin

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = Où va le numéro

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = Premier numéro

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = Que faire de l’extension

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = Nouvelle extension, comme txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders ne peut pas renommer ceci : { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = était { $old }

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = reste tel quel

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [one] Renommer 1 élément
        [many] Renommer { $n } éléments
       *[other] Renommer { $n } éléments
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = Remplacer

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = Chercher

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = Remplacer par

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = Utiliser * et ? comme jokers

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = Jokers * et ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = Respecter exactement les majuscules

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = Respecter la casse

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = Ajouter

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = Avant le nom

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = Après le nom

# Name of the numbering row in the rename window.
folders-tools-rename-number = Numéro

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = Majuscules

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = Majuscules dans les noms

# Name over the ending choice in the rename window.
folders-tools-rename-ending = Extension

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] Dans { $place }, certains noms ne peuvent pas être utilisés. Voir les lignes marquées.
       *[no] Dans { $place }, { $changes } noms sur { $total } vont changer.
    }

# Main button of the rename window.
folders-tools-rename-button = Renommer

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = Dossier personnel

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [one] Doublons dans { $folder }
        [many] Doublons dans { $n } dossiers
       *[other] Doublons dans { $n } dossiers
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = La recherche s’est arrêtée : { $error }.

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = Aucun doublon n’a été trouvé.

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [one] 1 fichier n’a pas pu être lu.
        [many] { $n } fichiers n’ont pas pu être lus.
       *[other] { $n } fichiers n’ont pas pu être lus.
    }

# SAFETY: needs a human review before it ships
# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
        [one] 1 série de copies. Les copies en trop sont sélectionnées, et les mettre à la corbeille libère { $size }.
       *[other] { $n } séries de copies. Les copies en trop sont sélectionnées, et les mettre à la corbeille libère { $size }.
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = Série { $set }, { $size }, dans { $at }

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 fichier n’a pas pu être lu et a été laissé de côté.
        [many] { $n } fichiers n’ont pas pu être lus et ont été laissés de côté.
       *[other] { $n } fichiers n’ont pas pu être lus et ont été laissés de côté.
    }

# SAFETY: needs a human review before it ships
# Note in the duplicates window when every copy of a set is picked.
folders-tools-dupes-keep-one = Toutes les copies d’une série sont sélectionnées. Laissez-en au moins une non sélectionnée.

# SAFETY: needs a human review before it ships
# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = Sélectionner les copies en trop

# SAFETY: needs a human review before it ships
# Main button of the duplicates window: move the picked files to the Trash.
folders-tools-dupes-trash = Mettre à la corbeille

# Title of the window that compares two files.
folders-tools-diff-title = Comparer deux fichiers

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders n’a pas pu les comparer : { $error }.

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = Les deux fichiers sont exactement identiques.

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = Ils sont identiques jusqu’à la fin du plus court. L’un fait { $one } et l’autre { $other }.

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = Ils diffèrent à partir de l’octet { $n }.

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] Les deux fichiers sont identiques.
        [one] 1 ligne diffère.
       *[other] { $n } lignes diffèrent.
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = Ils sont trop différents pour être comparés ligne par ligne. Tout ce qui se trouve entre la première et la dernière différence apparaît comme modifié.

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = Seuls les 4 premiers Mo de chaque fichier sont comparés.

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = Comparer { $left } avec { $right }

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = Comparer le contenu (plus lent)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = Comparer le contenu de chaque fichier

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = Afficher les noms identiques

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = Afficher aussi les noms identiques

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = La comparaison s’est arrêtée : { $error }.

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = { $left } seulement à gauche, { $right } seulement à droite, { $different } différents, { $same } identiques.

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = Les deux dossiers ont le même contenu.

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = Comparer de nouveau

# SAFETY: needs a human review before it ships
# Button in the pane compare window; { $n } is how many names are copied to the left folder.
folders-tools-compare-copy-left = Copier { $n } éléments manquants ou plus récents vers la gauche

# SAFETY: needs a human review before it ships
# Button in the pane compare window; { $n } is how many names are copied to the right folder.
folders-tools-compare-copy-right = Copier { $n } éléments manquants ou plus récents vers la droite

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = Seuls les fichiers sont écrasés, donc ces dossiers seront laissés tels quels : { $names }.
