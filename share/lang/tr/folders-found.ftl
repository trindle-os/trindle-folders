

folders-found-need-inside-words = Önce dosyaların içinde aranacak sözcükleri yazın.

folders-found-need-something = Aranacak şeyi yazın ya da bir tür, boyut veya zaman seçin.

folders-found-only-here = Yalnızca bu bilgisayardaki klasörlerde arama yapılabilir.

folders-found-shown = { $all } içinden { $shown } gösteriliyor

folders-found-in-folder = { $folder } içinde

folders-found-in-root = Aranan klasörde

folders-found-in-folder-line =
    { $folder } içinde. { $n }. satır: { $line }{ $more ->
        [0]
       *[other] { " " }(ve { $more } tane daha)
    }

folders-found-in-root-line =
    Aranan klasörde. { $n }. satır: { $line }{ $more ->
        [0]
       *[other] { " " }(ve { $more } tane daha)
    }

folders-found-status =
    { $n ->
        [one] 1 bulundu
       *[other] { $n } bulundu
    }{ $filtered ->
        [yes] { " " }/ { $all }
       *[other]
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

