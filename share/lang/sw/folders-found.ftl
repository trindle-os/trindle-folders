# machine translated (Swahili), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = Andika kwanza maneno ya kutafuta ndani ya faili.

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = Andika kitu cha kutafuta, au chagua aina, ukubwa au muda.

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = Unaweza kutafuta folda za kompyuta hii pekee.

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $shown } kati ya { $all } vinaonyeshwa

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = Ndani ya { $folder }

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = Ndani ya folda uliyotafuta

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    Ndani ya { $folder }. Mstari wa { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(na { $more } zaidi)
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    Ndani ya folda uliyotafuta. Mstari wa { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(na { $more } zaidi)
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] { $n } kati ya { $all } zimepatikana
       *[other] { $n ->
            [one] 1 imepatikana
           *[other] { $n } zimepatikana
        }
    }{ $looking ->
        [yes] , bado inatafuta…
       *[other]
    }{ $full ->
        [yes] . Inaonyesha { $most } za kwanza tu
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } hazikuweza kusomwa
    }{ $skipped ->
        [0]
       *[other] . { $skipped } zimerukwa kwa kuwa kubwa mno au si maandishi
    }
