
# machine translated (Indonesian), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = Ketik dulu kata yang dicari di dalam file.

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = Ketik sesuatu untuk dicari, atau pilih jenis, ukuran, atau waktu.

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = Hanya folder di komputer ini yang bisa dicari.

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $shown } dari { $all } ditampilkan

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = Di { $folder }

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = Di folder yang Anda cari

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    Di { $folder }. Baris { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(dan { $more } lagi)
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    Di folder yang Anda cari. Baris { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(dan { $more } lagi)
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] { $n } dari { $all } ditemukan
       *[other] { $n } ditemukan
    }{ $looking ->
        [yes] , masih mencari…
       *[other]
    }{ $full ->
        [yes] . Hanya { $most } pertama yang ditampilkan
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } tidak bisa dibaca
    }{ $skipped ->
        [0]
       *[other] . { $skipped } dilewati karena terlalu besar atau bukan teks
    }

