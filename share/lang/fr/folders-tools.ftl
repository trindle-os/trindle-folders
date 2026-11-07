

folders-tools-pass-once = Une fois

folders-tools-pass-three = 3 fois

folders-tools-pass-seven = 7 fois

folders-tools-case-keep = Inchangées

folders-tools-case-lower = Tout en minuscules

folders-tools-case-upper = Tout en majuscules

folders-tools-case-words = Chaque mot

folders-tools-case-first = Premier mot

folders-tools-ending-keep = Inchangée

folders-tools-ending-lower = Tout en minuscules

folders-tools-ending-upper = Tout en majuscules

folders-tools-ending-new = Une nouvelle extension

folders-tools-ending-none = Aucune extension

folders-tools-archive-name = Archive

folders-tools-clash-empty = il ne resterait aucun nom

folders-tools-clash-bad = ce n’est pas permis comme nom

folders-tools-clash-long = ce nom est trop long

folders-tools-clash-twice = deux recevraient ce nom

folders-tools-clash-taken = autre chose porte ce nom

folders-tools-clash-not-text = l’ancien nom n’est pas du texte simple

folders-tools-caveat-flash = Ce disque est de type flash, pas à plateaux tournants, donc d’anciennes copies peuvent subsister dans des parties que Trindle ne peut pas atteindre.

folders-tools-caveat-copy-on-write = Le système de fichiers { $fs } écrit les changements à un nouvel endroit, donc les anciennes données peuvent rester où elles étaient.

folders-tools-caveat-network = Il s’agit d’un partage { $fs } sur un autre ordinateur, qui peut garder ses propres copies.

folders-tools-row-only-left = Seulement à gauche

folders-tools-row-only-right = Seulement à droite

folders-tools-row-same = Identique

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

folders-tools-place-here = Ici

folders-tools-only-files-wiped = Seuls des fichiers peuvent être effacés.

folders-tools-compare-need-folders = Les deux panneaux doivent afficher un dossier pour les comparer.

folders-tools-rename-undo =
    { $n ->
        [one] Annuler le renommage de 1 élément
        [many] Annuler le renommage de { $n } éléments
       *[other] Annuler le renommage de { $n } éléments
    }

folders-tools-rename-done =
    { $n ->
        [one] 1 élément renommé.
        [many] { $n } éléments renommés.
       *[other] { $n } éléments renommés.
    }

folders-tools-rename-stopped = Le renommage s’est arrêté en cours de route : { $error }.

folders-tools-name-empty = Un nom ne peut pas être vide.

folders-tools-looked-at = { $done } examinés

folders-tools-read-of = { $done } lus sur { $total }

folders-tools-listing-files = Liste des fichiers

folders-tools-comparing = Comparaison

folders-tools-looking = Examen

folders-tools-compress-title =
    { $n ->
        [one] Compresser { $name }
        [many] Compresser { $n } éléments
       *[other] Compresser { $n } éléments
    }

folders-tools-pack-name-tip = Nom de l’archive

folders-tools-pack-kind-tip = Type d’archive

folders-tools-pack-made-in = Créée dans { $place }

folders-tools-compress = Compresser

folders-tools-extract-to = Extraire { $name } vers

folders-tools-extract-new-folder = Un nouveau dossier nommé { $name } y est créé.

folders-tools-resize-title =
    { $n ->
        [one] Redimensionner { $name }
        [many] Redimensionner { $n } images
       *[other] Redimensionner { $n } images
    }

folders-tools-resize-size-tip = Nouvelle taille

folders-tools-resize-note = Une part de sa taille, ou un nombre de pixels le long de son plus grand côté ; une image n’est jamais agrandie. Une copie est créée à côté de chacune, nommée avec sa nouvelle taille ; les images elles-mêmes restent telles quelles.

folders-tools-resize = Redimensionner

folders-tools-wipe-title =
    { $n ->
        [one] Effacer { $name } de façon sécurisée ?
        [many] Effacer { $n } fichiers de façon sécurisée ?
       *[other] Effacer { $n } fichiers de façon sécurisée ?
    }

folders-tools-wipe-warning = Chaque fichier est écrasé avant de disparaître, il ne pourra donc pas être récupéré. Cette action est irréversible.

folders-tools-wipe-write-over = Écraser

folders-tools-wipe-passes-tip = Combien de fois chaque fichier est écrasé

folders-tools-wipe-button = Effacer de façon sécurisée

folders-tools-rename-number-tip = Les numéroter

folders-tools-rename-at-start = Au début

folders-tools-rename-at-end = À la fin

folders-tools-rename-where-tip = Où va le numéro

folders-tools-rename-first-number-tip = Premier numéro

folders-tools-rename-ending-tip = Ce qui arrive à l’extension

folders-tools-rename-new-ending-tip = Nouvelle extension, comme txt

folders-tools-rename-cannot = Impossible : { $why }

folders-tools-rename-was = était { $old }

folders-tools-rename-stays = reste tel quel

folders-tools-rename-title =
    { $n ->
        [one] Renommer 1 élément
        [many] Renommer { $n } éléments
       *[other] Renommer { $n } éléments
    }

folders-tools-rename-replace = Remplacer

folders-tools-rename-find-tip = Chercher

folders-tools-rename-with-tip = Remplacer par

folders-tools-rename-wildcard-tip = Utiliser * et ? comme jokers

folders-tools-rename-wildcard = Jokers * et ?

folders-tools-rename-match-case-tip = Respecter exactement les majuscules

folders-tools-rename-match-case = Respecter la casse

folders-tools-rename-add = Ajouter

folders-tools-rename-before-tip = Avant le nom

folders-tools-rename-after-tip = Après le nom

folders-tools-rename-number = Numéro

folders-tools-rename-letters = Lettres

folders-tools-rename-case-tip = Majuscules dans les noms

folders-tools-rename-ending = Extension

folders-tools-rename-summary =
    { $clash ->
        [yes] Certains noms ne peuvent pas être utilisés ; voir les lignes marquées. Dans { $place }.
       *[no] { $changes } noms sur { $total } changent. Dans { $place }.
    }

folders-tools-rename-button = Les renommer

folders-tools-home = Dossier personnel

folders-tools-dupes-title =
    { $n ->
        [one] Doublons dans { $folder }
        [many] Doublons dans { $n } dossiers
       *[other] Doublons dans { $n } dossiers
    }

folders-tools-dupes-stopped = La recherche s’est arrêtée : { $error }.

folders-tools-dupes-none = Aucun doublon n’a été trouvé.

folders-tools-dupes-unread =
    { $n ->
        [one] 1 fichier n’a pas pu être lu.
        [many] { $n } fichiers n’ont pas pu être lus.
       *[other] { $n } fichiers n’ont pas pu être lus.
    }

folders-tools-dupes-sets =
    { $n ->
        [one] 1 série de copies ; mettre les copies en trop à la corbeille libère { $size }. Les copies en trop sont sélectionnées ; Ctrl et un clic en sélectionne ou en désélectionne une.
        [many] { $n } séries de copies ; mettre les copies en trop à la corbeille libère { $size }. Les copies en trop sont sélectionnées ; Ctrl et un clic en sélectionne ou en désélectionne une.
       *[other] { $n } séries de copies ; mettre les copies en trop à la corbeille libère { $size }. Les copies en trop sont sélectionnées ; Ctrl et un clic en sélectionne ou en désélectionne une.
    }

folders-tools-dupes-row = Série { $set }, { $size }, dans { $at }

folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 fichier n’a pas pu être lu et a été laissé de côté.
        [many] { $n } fichiers n’ont pas pu être lus et ont été laissés de côté.
       *[other] { $n } fichiers n’ont pas pu être lus et ont été laissés de côté.
    }

folders-tools-dupes-keep-one = Toutes les copies d’une série sont sélectionnées ; gardez-en au moins une.

folders-tools-dupes-pick-extras = Sélectionner les copies en trop

folders-tools-dupes-trash = Mettre à la corbeille

folders-tools-diff-title = Comparer les deux

folders-tools-diff-failed = Ils n’ont pas pu être comparés : { $error }.

folders-tools-diff-same-bytes = Les deux sont identiques, octet pour octet.

folders-tools-diff-same-as-shorter = Ils sont identiques jusqu’où va le plus court ; l’un fait { $one } et l’autre { $other }.

folders-tools-diff-from-byte = Ils diffèrent à partir de l’octet { $n }.

folders-tools-diff-lines =
    { $n ->
        [0] Les deux sont identiques.
        [one] 1 ligne diffère.
        [many] { $n } lignes diffèrent.
       *[other] { $n } lignes diffèrent.
    }

folders-tools-diff-too-different = Ils sont trop différents pour être comparés ligne par ligne, donc chaque ligne entre leur première et leur dernière différence apparaît comme modifiée.

folders-tools-diff-cut = Seuls les 4 premiers Mo de chacun sont comparés.

folders-tools-compare-title = Comparer { $left } avec { $right }

folders-tools-compare-bytes = Lire chaque octet (plus lent)

folders-tools-compare-bytes-tip = Comparer le contenu de chaque fichier

folders-tools-compare-show-same = Afficher ce qui est identique

folders-tools-compare-show-same-tip = Afficher aussi les noms identiques

folders-tools-compare-stopped = La comparaison s’est arrêtée : { $error }.

folders-tools-compare-counts = { $left } seulement à gauche, { $right } seulement à droite, { $different } différents, { $same } identiques.

folders-tools-compare-all-same = Les deux dossiers contiennent la même chose.

folders-tools-compare-again = Comparer de nouveau

folders-tools-compare-copy-left = Copier { $n } manquants ou plus récents vers la gauche

folders-tools-compare-copy-right = Copier { $n } manquants ou plus récents vers la droite
