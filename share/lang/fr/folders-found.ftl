

folders-found-need-inside-words = Saisissez d’abord les mots à chercher dans les fichiers.

folders-found-need-something = Saisissez ce que vous cherchez, ou choisissez un type, une taille ou une date.

folders-found-only-here = Seuls les dossiers de cet ordinateur peuvent être fouillés.

folders-found-shown = { $shown } sur { $all } affichés

folders-found-in-folder = Dans { $folder }

folders-found-in-root = Dans le dossier fouillé

folders-found-in-folder-line =
    Dans { $folder }. Ligne { $n } : { $line }{ $more ->
        [0]
       *[other] { " " }(et { $more } de plus)
    }

folders-found-in-root-line =
    Dans le dossier fouillé. Ligne { $n } : { $line }{ $more ->
        [0]
       *[other] { " " }(et { $more } de plus)
    }

folders-found-status =
    { $n ->
        [one] 1 trouvé
        [many] { $n } trouvés
       *[other] { $n } trouvés
    }{ $filtered ->
        [yes] { " " }sur { $all }
       *[other]
    }{ $looking ->
        [yes] , recherche en cours…
       *[other]
    }{ $full ->
        [yes] . Seuls les { $most } premiers sont affichés
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } illisibles
    }{ $skipped ->
        [0]
       *[other] . { $skipped } ignorés car trop gros ou sans texte
    }
