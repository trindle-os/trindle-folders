# machine translated (Indonesian), not yet reviewed by a person.

folders-tools-pass-once = Sekali

folders-tools-pass-three = 3 kali

folders-tools-pass-seven = 7 kali

folders-tools-case-keep = Biarkan

folders-tools-case-lower = Semua huruf kecil

folders-tools-case-upper = Semua huruf kapital

folders-tools-case-words = Tiap kata

folders-tools-case-first = Kata pertama

folders-tools-ending-keep = Biarkan

folders-tools-ending-lower = Semua huruf kecil

folders-tools-ending-upper = Semua huruf kapital

folders-tools-ending-new = Ekstensi baru

folders-tools-ending-none = Tanpa ekstensi

folders-tools-archive-name = Arsip

folders-tools-clash-empty = tidak ada nama yang tersisa

folders-tools-clash-bad = itu tidak diizinkan sebagai nama

folders-tools-clash-long = nama itu terlalu panjang

folders-tools-clash-twice = dua berkas akan mendapat nama ini

folders-tools-clash-taken = sudah ada yang memakai nama ini

folders-tools-clash-not-text = nama lamanya bukan teks biasa

folders-tools-caveat-flash = Disk ini berjenis flash, bukan piringan berputar, jadi salinan lama mungkin tetap ada di bagian yang tidak dapat dijangkau Trindle.

folders-tools-caveat-copy-on-write = Sistem berkas { $fs } menulis perubahan ke tempat baru, jadi data lama mungkin tetap berada di tempatnya.

folders-tools-caveat-network = Ini adalah folder bersama { $fs } di komputer lain, yang mungkin menyimpan salinannya sendiri.

folders-tools-row-only-left = Hanya di kiri

folders-tools-row-only-right = Hanya di kanan

folders-tools-row-same = Sama

folders-tools-row-different =
    Berbeda: { $why ->
        [kind] salah satunya folder atau tautan
        [size] ukurannya berbeda
        [date] tanggalnya berbeda
        [content] isinya berbeda
        [target] tautan menunjuk ke tempat lain
        [inside] ada yang berbeda di dalamnya
       *[unread] satu sisi tidak dapat dibaca
    }{ $newer ->
        [left] , lebih baru di kiri
        [right] , lebih baru di kanan
       *[none] { "" }
    }

folders-tools-place-here = Di sini

folders-tools-only-files-wiped = Hanya berkas yang dapat dihapus bersih.

folders-tools-compare-need-folders = Kedua panel harus menampilkan folder untuk dibandingkan.

folders-tools-rename-undo =
    { $n ->
       *[other] Batalkan penggantian nama { $n } item
    }

folders-tools-rename-done =
    { $n ->
       *[other] { $n } item diganti namanya.
    }

folders-tools-rename-stopped = Penggantian nama berhenti di tengah jalan: { $error }.

folders-tools-name-empty = Nama tidak boleh kosong.

folders-tools-looked-at = { $done } diperiksa

folders-tools-read-of = Dibaca { $done } dari { $total }

folders-tools-listing-files = Mendaftar berkas

folders-tools-comparing = Membandingkan

folders-tools-looking = Memeriksa

folders-tools-compress-title =
    { $n ->
        [1] Kompres { $name }
       *[other] Kompres { $n } item
    }

folders-tools-pack-name-tip = Nama arsip

folders-tools-pack-kind-tip = Jenis arsip

folders-tools-pack-made-in = Dibuat di { $place }

folders-tools-compress = Kompres

folders-tools-extract-to = Ekstrak { $name } ke

folders-tools-extract-new-folder = Folder baru bernama { $name } dibuat di sana.

folders-tools-resize-title =
    { $n ->
        [1] Ubah ukuran { $name }
       *[other] Ubah ukuran { $n } gambar
    }

folders-tools-resize-size-tip = Ukuran baru

folders-tools-resize-note = Persentase dari ukurannya, atau jumlah piksel di sisi terpanjangnya; gambar tidak pernah diperbesar. Salinan dibuat di samping masing-masing, dinamai dengan ukuran barunya; gambar aslinya tetap seperti semula.

folders-tools-resize = Ubah ukuran

folders-tools-wipe-title =
    { $n ->
        [1] Hapus bersih { $name }?
       *[other] Hapus bersih { $n } berkas?
    }

folders-tools-wipe-warning = Setiap berkas ditimpa sebelum dihapus, sehingga tidak dapat dikembalikan. Ini tidak dapat dibatalkan.

folders-tools-wipe-write-over = Timpa

folders-tools-wipe-passes-tip = Berapa kali setiap berkas ditimpa

folders-tools-wipe-button = Hapus bersih secara aman

folders-tools-rename-number-tip = Beri nomor

folders-tools-rename-at-start = Di awal

folders-tools-rename-at-end = Di akhir

folders-tools-rename-where-tip = Letak nomor

folders-tools-rename-first-number-tip = Nomor pertama

folders-tools-rename-ending-tip = Perlakuan pada ekstensi

folders-tools-rename-new-ending-tip = Ekstensi baru, misalnya txt

folders-tools-rename-cannot = Tidak bisa: { $why }

folders-tools-rename-was = sebelumnya { $old }

folders-tools-rename-stays = tetap seperti semula

folders-tools-rename-title =
    { $n ->
       *[other] Ganti nama { $n } item
    }

folders-tools-rename-replace = Ganti

folders-tools-rename-find-tip = Cari

folders-tools-rename-with-tip = Ganti dengan

folders-tools-rename-wildcard-tip = Pakai * dan ? sebagai wildcard

folders-tools-rename-wildcard = Wildcard * dan ?

folders-tools-rename-match-case-tip = Cocokkan huruf kapital persis

folders-tools-rename-match-case = Cocokkan huruf besar-kecil

folders-tools-rename-add = Tambah

folders-tools-rename-before-tip = Sebelum nama

folders-tools-rename-after-tip = Sesudah nama

folders-tools-rename-number = Nomor

folders-tools-rename-letters = Huruf

folders-tools-rename-case-tip = Huruf kapital pada nama

folders-tools-rename-ending = Ekstensi

folders-tools-rename-summary =
    { $clash ->
        [yes] Beberapa nama tidak dapat dipakai; lihat baris yang bertanda. Di { $place }.
       *[no] { $changes } dari { $total } nama berubah. Di { $place }.
    }

folders-tools-rename-button = Ganti nama semuanya

folders-tools-home = Beranda

folders-tools-dupes-title =
    { $n ->
        [1] Duplikat di { $folder }
       *[other] Duplikat di { $n } folder
    }

folders-tools-dupes-stopped = Pencarian berhenti: { $error }.

folders-tools-dupes-none = Tidak ditemukan duplikat.

folders-tools-dupes-unread =
    { $n ->
       *[other] { $n } berkas tidak dapat dibaca.
    }

folders-tools-dupes-sets =
    { $n ->
       *[other] { $n } kelompok salinan; memindahkan salinan tambahan ke Sampah membebaskan { $size }. Salinan tambahan sudah dipilih; Ctrl dan klik memilih atau melepas satu.
    }

folders-tools-dupes-row = Kelompok { $set }, { $size }, di { $at }

folders-tools-dupes-unread-left-out =
    { $n ->
       *[other] { $n } berkas tidak dapat dibaca dan dilewati.
    }

folders-tools-dupes-keep-one = Semua salinan dalam satu kelompok terpilih; sisakan setidaknya satu.

folders-tools-dupes-pick-extras = Pilih salinan tambahan

folders-tools-dupes-trash = Pindahkan ke Sampah

folders-tools-diff-title = Bandingkan keduanya

folders-tools-diff-failed = Keduanya tidak dapat dibandingkan: { $error }.

folders-tools-diff-same-bytes = Keduanya sama, byte demi byte.

folders-tools-diff-same-as-shorter = Keduanya sama sejauh yang lebih pendek; satu berukuran { $one } dan yang lain { $other }.

folders-tools-diff-from-byte = Keduanya berbeda mulai dari byte { $n }.

folders-tools-diff-lines =
    { $n ->
        [0] Keduanya sama.
       *[other] { $n } baris berbeda.
    }

folders-tools-diff-too-different = Keduanya terlalu berbeda untuk dicocokkan baris demi baris, jadi setiap baris di antara perbedaan pertama dan terakhir tampil sebagai berubah.

folders-tools-diff-cut = Hanya 4 MB pertama dari masing-masing yang dibandingkan.

folders-tools-compare-title = Bandingkan { $left } dengan { $right }

folders-tools-compare-bytes = Baca setiap byte (lebih lambat)

folders-tools-compare-bytes-tip = Bandingkan isi setiap berkas

folders-tools-compare-show-same = Tampilkan yang sama

folders-tools-compare-show-same-tip = Tampilkan juga nama yang sama

folders-tools-compare-stopped = Perbandingan berhenti: { $error }.

folders-tools-compare-counts = { $left } hanya di kiri, { $right } hanya di kanan, { $different } berbeda, { $same } sama.

folders-tools-compare-all-same = Kedua folder berisi hal yang sama.

folders-tools-compare-again = Bandingkan lagi

folders-tools-compare-copy-left = Salin { $n } yang hilang atau lebih baru ke kiri

folders-tools-compare-copy-right = Salin { $n } yang hilang atau lebih baru ke kanan
