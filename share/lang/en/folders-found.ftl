# Folders' search below a folder: the words in the status line while and after looking, and the red words when a search cannot start (src/files/found.rs).

folders-found-need-inside-words = Type the words to look for inside files first.
folders-found-need-something = Type what to look for, or choose a kind, a size or a time.
folders-found-only-here = Only folders on this computer can be searched.

folders-found-shown = { $shown } of { $all } shown
folders-found-in-folder = In { $folder }
folders-found-in-root = In the folder searched
folders-found-in-folder-line =
    In { $folder }. Line { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(and { $more } more)
    }
folders-found-in-root-line =
    In the folder searched. Line { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(and { $more } more)
    }
folders-found-status =
    { $n ->
        [one] 1 found
       *[other] { $n } found
    }{ $filtered ->
        [yes] { " " }of { $all }
       *[other]
    }{ $looking ->
        [yes] , still looking…
       *[other]
    }{ $full ->
        [yes] . Only the first { $most } are shown
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } could not be read
    }{ $skipped ->
        [0]
       *[other] . { $skipped } passed over as too big or not text
    }
