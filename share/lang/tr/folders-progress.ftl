

folders-progress-pace-known =
    { $files ->
        [yes] { $of } dosyadan { $done }, saniyede { $rate }, { $left }
       *[no] saniyede { $rate }, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $of } dosyadan { $done }, ne kadar süreceği hesaplanıyor…
       *[no] ne kadar süreceği hesaplanıyor…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } bayt

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = birkaç saniye kaldı

folders-progress-left-seconds =
    { $n ->
        [one] yaklaşık 1 saniye kaldı
       *[other] yaklaşık { $n } saniye kaldı
    }

folders-progress-left-minute = yaklaşık bir dakika kaldı

folders-progress-left-minutes =
    { $n ->
        [one] yaklaşık 1 dakika kaldı
       *[other] yaklaşık { $n } dakika kaldı
    }

folders-progress-left-hours =
    { $n ->
        [one] yaklaşık 1 saat kaldı
       *[other] yaklaşık { $n } saat kaldı
    }

folders-progress-more =
    { $n ->
        [one] 1 iş daha sürüyor
       *[other] { $n } iş daha sürüyor
    }

folders-progress-heavy =
    { $verb ->
        [moving] Taşıma diski yoğun kullanır, bu yüzden bitene kadar diğer programlar yavaşlayabilir.
       *[copying] Kopyalama diski yoğun kullanır, bu yüzden bitene kadar diğer programlar yavaşlayabilir.
    }

folders-progress-waiting = yanıt bekleniyor

folders-progress-paused = duraklatıldı

folders-progress-carry-on = Sürdür

folders-progress-pause = Duraklat

folders-progress-stop = Durdur

folders-progress-mark-newer-larger = daha yeni, daha büyük

folders-progress-mark-newer = daha yeni

folders-progress-mark-larger = daha büyük

folders-progress-changed = { $when } değişti

folders-progress-taken =
    { $kind ->
        [folder] { $name } adlı bir klasör zaten var
        [file] { $name } adlı bir dosya zaten var
       *[unknown] { $name } zaten var
    }

folders-progress-coming = Getirilen

folders-progress-there = Zaten orada olan

folders-progress-skip = Atla

folders-progress-keep-both = İkisini de tut

folders-progress-replace = Değiştir

folders-progress-replace-folder-spoken = Değiştir: zaten orada olan klasör Çöp kutusuna gider

folders-progress-merge = Birleştir

folders-progress-all-folders = Zaten orada olan her klasör için aynı yanıt.

folders-progress-all-files = Zaten orada olan her dosya için aynı yanıt.

folders-progress-same-for-rest = Kalanlar için de aynısını yap

