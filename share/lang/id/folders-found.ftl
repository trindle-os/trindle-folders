

folders-found-need-inside-words = Ketik dulu kata yang dicari di dalam file.

folders-found-need-something = Ketik apa yang dicari, atau pilih jenis, ukuran, atau waktu.

folders-found-only-here = Hanya folder di komputer ini yang bisa dicari.

folders-found-shown = { $shown } dari { $all } ditampilkan

folders-found-in-folder = Di { $folder }

folders-found-in-root = Di folder yang dicari

folders-found-in-folder-line =
    Di { $folder }. Baris { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(dan { $more } lagi)
    }

folders-found-in-root-line =
    Di folder yang dicari. Baris { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(dan { $more } lagi)
    }

folders-found-status =
    { $n ->
       *[other] { $n } ditemukan
    }{ $filtered ->
        [yes] { " " }dari { $all }
       *[other]
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

