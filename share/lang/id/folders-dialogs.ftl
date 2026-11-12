
# machine translated (Indonesian), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = Tutup

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = Beranda

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = Buka { $name } dengan

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = Aplikasi saat ini

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = Tidak ada aplikasi di komputer ini yang diatur untuk membuka jenis file ini.

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = Selalu gunakan aplikasi ini

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = Gunakan untuk setiap file { $kind }.

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = Jadikan aplikasi default

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = Tampilkan hanya aplikasi yang disarankan

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = Tampilkan semua aplikasi

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = Buka

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = Pilih aplikasi dulu.

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders tidak dapat menjadikan { $name } aplikasi default.

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders tidak dapat menjalankan { $name }.

# Heading of the Properties window.
folders-dialogs-properties = Properti

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = Halaman Properti

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = Umum

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = Detail

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = Berbagi



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = Nama

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = Lokasi

# Label in Properties of the kind of file.
folders-dialogs-row-kind = Jenis

# Label in Properties of where a link points.
folders-dialogs-row-link-to = Tautan ke

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = Diubah

# Label in Properties of who owns the item.
folders-dialogs-row-owner = Pemilik

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = Hak akses

# Label in Properties of the size of the item.
folders-dialogs-row-size = Ukuran

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = Tidak diketahui

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, Anda dapat membuka dan mengubahnya

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, Anda dapat membukanya, tetapi tidak mengubahnya

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, Anda dapat mengubahnya, tetapi tidak membukanya

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, Anda tidak dapat membukanya

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , sebagian tidak dapat dibaca
       *[other]
    }{ $adding ->
        [yes] , masih dihitung
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = Menghitung ukuran

# Label in Properties of the row about checksums.
folders-dialogs-checksums = Checksum

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = Hitung checksum

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = Sedang menghitung checksum

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders tidak dapat membaca file.

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = Tempel checksum untuk dibandingkan

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } cocok

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = Tidak cocok

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = Periksa


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [1] Kirim { $name } ke
       *[other] Kirim { $n } item ke
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = Ini membuat salinan di sana. Aslinya tetap di tempatnya.

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = Panel lain, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = Kolom tampilan Daftar

# Note in the Columns window.
folders-dialogs-columns-note = Nama selalu ditampilkan. Kolom yang tidak muat disembunyikan sampai panel lebih lebar.

# Heading over the columns that suit every file.
folders-dialogs-columns-files = File

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = Musik, foto, PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = Tampilkan { $name }

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders tidak dapat menyimpan kolom.

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [1] Warna untuk { $name }
       *[other] Warna untuk { $n } folder
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = Tanpa warna

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders tidak dapat menyimpan warna folder.


folders-dialogs-no-match = Tidak ada aplikasi yang cocok dengan kata-kata itu.


folders-dialogs-find-app = Cari aplikasi


folders-dialogs-find-place = Cari tempat


folders-dialogs-no-match-place = Tidak ada tempat yang cocok dengan kata-kata itu.
