
# machine translated (Turkish), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = Önce dosyaların içinde aranacak sözcükleri yazın.

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = Aranacak bir şey yazın ya da bir tür, boyut veya zaman seçin.

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = Yalnızca bu bilgisayardaki klasörlerde arama yapabilirsiniz.

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $all } içinden { $shown } gösteriliyor

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = { $folder } içinde

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = Aradığınız klasörde

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    { $folder } içinde. { $n }. satır: { $line }{ $more ->
        [0]
       *[other] { " " }(ve { $more } tane daha)
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    Aradığınız klasörde. { $n }. satır: { $line }{ $more ->
        [0]
       *[other] { " " }(ve { $more } tane daha)
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] { $all } içinden { $n } bulundu
       *[other] { $n ->
            [one] 1 bulundu
           *[other] { $n } bulundu
        }
    }{ $looking ->
        [yes] , hâlâ aranıyor…
       *[other]
    }{ $full ->
        [yes] . Yalnızca ilk { $most } gösteriliyor
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } okunamadı
    }{ $skipped ->
        [0]
       *[other] . { $skipped } çok büyük ya da metin olmadığı için atlandı
    }

