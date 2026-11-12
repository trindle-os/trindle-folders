
# machine translated (German), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = Gib zuerst die Wörter ein, die in Dateien gesucht werden sollen.

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = Gib etwas zum Suchen ein oder wähle eine Art, eine Größe oder eine Zeit.

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = Du kannst nur Ordner auf diesem Computer durchsuchen.

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $shown } von { $all } angezeigt

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = In { $folder }

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = Im durchsuchten Ordner

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    In { $folder }. Zeile { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(und { $more } weitere)
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    Im durchsuchten Ordner. Zeile { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(und { $more } weitere)
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] { $n } von { $all } gefunden
       *[other] { $n ->
            [one] 1 gefunden
           *[other] { $n } gefunden
        }
    }{ $looking ->
        [yes] , suche noch …
       *[other]
    }{ $full ->
        [yes] . Nur die ersten { $most } werden angezeigt
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } konnten nicht gelesen werden
    }{ $skipped ->
        [0]
       *[other] . { $skipped } übersprungen, weil zu groß oder kein Text
    }

