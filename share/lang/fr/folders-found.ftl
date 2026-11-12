
# machine translated (French), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = Saisissez d’abord les mots à chercher dans les fichiers.

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = Saisissez ce que vous cherchez, ou choisissez un type, une taille ou une date.

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = Vous ne pouvez rechercher que dans les dossiers de cet ordinateur.

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $shown } sur { $all } affichés

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = Dans { $folder }

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = Dans le dossier où vous avez cherché

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    Dans { $folder }. Ligne { $n } : { $line }{ $more ->
        [0]
       *[other] { " " }(et { $more } de plus)
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    Dans le dossier où vous avez cherché. Ligne { $n } : { $line }{ $more ->
        [0]
       *[other] { " " }(et { $more } de plus)
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] { $n } sur { $all } trouvés
       *[other] { $n ->
            [one] 1 trouvé
            [many] { $n } trouvés
           *[other] { $n } trouvés
        }
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
