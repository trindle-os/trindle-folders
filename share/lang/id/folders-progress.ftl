

folders-progress-pace-known =
    { $files ->
        [yes] { $done } dari { $of } file, { $rate } per detik, { $left }
       *[no] { $rate } per detik, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $done } dari { $of } file, menghitung perkiraan waktu…
       *[no] menghitung perkiraan waktu…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } byte

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = tinggal beberapa detik

folders-progress-left-seconds =
    { $n ->
       *[other] sekitar { $n } detik lagi
    }

folders-progress-left-minute = sekitar satu menit lagi

folders-progress-left-minutes =
    { $n ->
       *[other] sekitar { $n } menit lagi
    }

folders-progress-left-hours =
    { $n ->
       *[other] sekitar { $n } jam lagi
    }

folders-progress-more =
    { $n ->
       *[other] { $n } pekerjaan lagi sedang berjalan
    }

folders-progress-heavy =
    { $verb ->
        [moving] Memindahkan memakai disk dengan berat, jadi program lain mungkin terasa lebih lambat sampai selesai.
       *[copying] Menyalin memakai disk dengan berat, jadi program lain mungkin terasa lebih lambat sampai selesai.
    }

folders-progress-waiting = menunggu jawaban

folders-progress-paused = dijeda

folders-progress-carry-on = Lanjutkan

folders-progress-pause = Jeda

folders-progress-stop = Hentikan

folders-progress-mark-newer-larger = lebih baru, lebih besar

folders-progress-mark-newer = lebih baru

folders-progress-mark-larger = lebih besar

folders-progress-changed = diubah { $when }

folders-progress-taken =
    { $kind ->
        [folder] Folder bernama { $name } sudah ada di sana
        [file] File bernama { $name } sudah ada di sana
       *[unknown] { $name } sudah ada di sana
    }

folders-progress-coming = Yang sedang dibawa

folders-progress-there = Yang sudah ada di sana

folders-progress-skip = Lewati

folders-progress-keep-both = Simpan keduanya

folders-progress-replace = Ganti

folders-progress-replace-folder-spoken = Ganti: folder yang sudah ada dipindahkan ke Tempat sampah

folders-progress-merge = Gabungkan

folders-progress-all-folders = Jawaban yang sama untuk setiap folder yang sudah ada.

folders-progress-all-files = Jawaban yang sama untuk setiap file yang sudah ada.

folders-progress-same-for-rest = Lakukan hal yang sama untuk sisanya
