# Folders' search below a folder: the words in the status line while and after looking, and the red words when a search cannot start (src/files/found.rs).

## Search (src/files/found.rs)

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = Type the words to look for inside files first.
# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = Type something to look for, or choose a kind, a size or a time.
# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = You can only search folders on this computer.

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $shown } of { $all } shown
# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = In { $folder }
# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = In the folder you searched
# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    In { $folder }. Line { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(and { $more } more)
    }
# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    In the folder you searched. Line { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(and { $more } more)
    }
# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] { $n } of { $all } found
       *[other] { $n ->
            [one] 1 found
           *[other] { $n } found
        }
    }{ $looking ->
        [yes] , still looking…
       *[other]
    }{ $full ->
        [yes] . Showing only the first { $most }
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } couldn't be read
    }{ $skipped ->
        [0]
       *[other] . { $skipped } skipped as too big or not text
    }
