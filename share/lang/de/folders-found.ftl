

folders-found-need-inside-words = Gib zuerst die Wörter ein, die in Dateien gesucht werden sollen.

folders-found-need-something = Gib ein, wonach gesucht werden soll, oder wähle eine Art, eine Größe oder eine Zeit.

folders-found-only-here = Es können nur Ordner auf diesem Computer durchsucht werden.

folders-found-shown = { $shown } von { $all } angezeigt

folders-found-in-folder = In { $folder }

folders-found-in-root = Im durchsuchten Ordner

folders-found-in-folder-line =
    In { $folder }. Zeile { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(und { $more } weitere)
    }

folders-found-in-root-line =
    Im durchsuchten Ordner. Zeile { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(und { $more } weitere)
    }

folders-found-status =
    { $n ->
        [one] 1 gefunden
       *[other] { $n } gefunden
    }{ $filtered ->
        [yes] { " " }von { $all }
       *[other]
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

