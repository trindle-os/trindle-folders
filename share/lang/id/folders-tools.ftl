# machine translated (Indonesian), not yet reviewed by a person.

# Choice in the Wipe window: write over each file one time.
# SAFETY: needs a human review before it ships
folders-tools-pass-once = Sekali

# Choice in the Wipe window: write over each file three times.
# SAFETY: needs a human review before it ships
folders-tools-pass-three = 3 kali

# Choice in the Wipe window: write over each file seven times.
# SAFETY: needs a human review before it ships
folders-tools-pass-seven = 7 kali

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = Biarkan

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = Semua huruf kecil

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = Semua huruf kapital

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = Kapitalkan tiap kata

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = Kapitalkan kata pertama

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = Biarkan

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = Semua huruf kecil

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = Semua huruf kapital

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = Ekstensi baru

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = Tanpa ekstensi

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = Arsip

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = tidak ada nama yang tersisa

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = itu tidak diizinkan sebagai nama

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = nama itu terlalu panjang

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = dua item akan mendapat nama ini

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = sudah ada yang memakai nama ini

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = nama lamanya memuat karakter yang tidak dapat dibaca Folders

# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
# SAFETY: needs a human review before it ships
folders-tools-caveat-flash = Ini flash disk atau SSD, jadi salinan lama mungkin tetap ada di bagian yang tidak dapat dijangkau Folders.

# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
# SAFETY: needs a human review before it ships
folders-tools-caveat-copy-on-write = Sistem berkas { $fs } menyimpan perubahan di tempat baru, jadi data lama mungkin masih ada.

# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
# SAFETY: needs a human review before it ships
folders-tools-caveat-network = Ini adalah folder bersama { $fs } di komputer lain, yang mungkin menyimpan salinannya sendiri.

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = Hanya di kiri

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = Hanya di kanan

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = Sama

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
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

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = Di sini

# Red notice when Wipe is chosen on things that are not files.
# SAFETY: needs a human review before it ships
folders-tools-only-files-wiped = Hanya berkas yang dapat dihapus bersih.

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = Buka folder di setiap panel untuk membandingkannya.

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
       *[other] Batalkan penggantian nama { $n } item
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
       *[other] { $n } item diganti namanya.
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = Penggantian nama berhenti di tengah proses: { $error }.

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = Ketik nama untuk arsip.

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = { $done } diperiksa

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = Dibaca { $done } dari { $total }

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = Mendaftar berkas

# Beside the ring while two files are compared.
folders-tools-comparing = Membandingkan

# Beside the ring while two folders are compared.
folders-tools-looking = Membandingkan folder

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [1] Kompres { $name }
       *[other] Kompres { $n } item
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = Nama arsip

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = Jenis arsip

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = Arsip akan disimpan di { $place }

# Main button of the Compress window.
folders-tools-compress = Kompres

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = Ekstrak { $name } ke

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = Berkas akan ditaruh di folder baru bernama { $name }.

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [1] Ubah ukuran { $name }
       *[other] Ubah ukuran { $n } gambar
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = Ukuran baru

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = Ukuran adalah panjang sisi terpanjang dalam piksel. Folders hanya memperkecil gambar. Folders menyimpan setiap salinan yang diubah ukurannya di samping aslinya, dengan ukuran baru pada namanya.

# Main button of the Resize window.
folders-tools-resize = Ubah ukuran

# Title of the Wipe window; { $name } is the one file, { $n } how many.
# SAFETY: needs a human review before it ships
folders-tools-wipe-title =
    { $n ->
        [1] Hapus bersih { $name }?
       *[other] Hapus bersih { $n } berkas?
    }

# Warning at the top of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-warning = Folders menimpa setiap berkas sebelum menghapusnya, jadi Anda tidak dapat mengembalikannya.

# Name beside the choice of how many times to write over, in the Wipe window.
folders-tools-wipe-write-over = Timpa

# Choice tip in the Wipe window: how many times each file is written over.
folders-tools-wipe-passes-tip = Berapa kali menimpa setiap berkas

# Red main button of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-button = Hapus bersih secara aman

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = Beri nomor

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = Di awal

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = Di akhir

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = Letak nomor

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = Nomor pertama

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = Apa yang dilakukan pada ekstensi

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = Ekstensi baru, misalnya txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders tidak dapat mengganti nama ini: { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = sebelumnya { $old }

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = tetap seperti semula

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
       *[other] Ganti nama { $n } item
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = Ganti

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = Cari

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = Ganti dengan

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = Pakai * dan ? sebagai wildcard

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = Wildcard * dan ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = Cocokkan huruf kapital persis

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = Cocokkan huruf besar-kecil

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = Tambah

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = Sebelum nama

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = Sesudah nama

# Name of the numbering row in the rename window.
folders-tools-rename-number = Nomor

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = Huruf kapital

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = Huruf kapital pada nama

# Name over the ending choice in the rename window.
folders-tools-rename-ending = Ekstensi

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] Di { $place }, beberapa nama tidak dapat dipakai. Lihat baris yang bertanda.
       *[no] Di { $place }, { $changes } dari { $total } nama akan berubah.
    }

# Main button of the rename window.
folders-tools-rename-button = Ganti nama

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = Beranda

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [1] Duplikat di { $folder }
       *[other] Duplikat di { $n } folder
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = Pencarian berhenti: { $error }.

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = Tidak ditemukan duplikat.

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
       *[other] { $n } berkas tidak dapat dibaca.
    }

# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
# SAFETY: needs a human review before it ships
folders-tools-dupes-sets =
    { $n ->
       *[other] { $n } kelompok salinan. Salinan tambahan sudah dipilih, dan memindahkannya ke Sampah membebaskan { $size }.
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = Kelompok { $set }, { $size }, di { $at }

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
       *[other] { $n } berkas tidak dapat dibaca dan dilewati.
    }

# Note in the duplicates window when every copy of a set is picked.
# SAFETY: needs a human review before it ships
folders-tools-dupes-keep-one = Semua salinan dalam satu kelompok terpilih. Sisakan setidaknya satu yang tidak dipilih.

# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = Pilih salinan tambahan

# Main button of the duplicates window: move the picked files to the Trash.
# SAFETY: needs a human review before it ships
folders-tools-dupes-trash = Pindahkan ke Sampah

# Title of the window that compares two files.
folders-tools-diff-title = Bandingkan dua berkas

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders tidak dapat membandingkannya: { $error }.

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = Kedua berkas persis sama.

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = Keduanya sama sampai akhir yang lebih pendek. Satu berukuran { $one } dan yang lain { $other }.

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = Keduanya berbeda mulai dari byte { $n }.

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] Kedua berkas sama.
       *[other] { $n } baris berbeda.
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = Keduanya terlalu berbeda untuk dibandingkan baris demi baris. Semua yang ada di antara perbedaan pertama dan terakhir ditampilkan sebagai berubah.

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = Hanya 4 MB pertama dari setiap berkas yang dibandingkan.

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = Bandingkan { $left } dengan { $right }

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = Bandingkan isi (lebih lambat)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = Bandingkan isi setiap berkas

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = Tampilkan nama yang sama

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = Tampilkan juga nama yang sama

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = Perbandingan berhenti: { $error }.

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = { $left } hanya di kiri, { $right } hanya di kanan, { $different } berbeda, { $same } sama.

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = Kedua folder memiliki isi yang sama.

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = Bandingkan lagi

# Button in the pane compare window; { $n } is how many names are copied to the left folder.
# SAFETY: needs a human review before it ships
folders-tools-compare-copy-left = Salin { $n } item yang hilang atau lebih baru ke kiri

# Button in the pane compare window; { $n } is how many names are copied to the right folder.
# SAFETY: needs a human review before it ships
folders-tools-compare-copy-right = Salin { $n } item yang hilang atau lebih baru ke kanan

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = Hanya berkas yang dihapus tuntas, jadi folder ini dibiarkan apa adanya: { $names }.
