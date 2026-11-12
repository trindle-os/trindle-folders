
# machine translated (Indonesian), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Potong, salin, dan tempel tidak berfungsi untuk berkas di server. Sebagai gantinya, gunakan tombol salin dan pindah di antara kedua panel.

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = Anda tidak dapat mengubah apa pun di dalam arsip. Ekstrak dulu yang Anda perlukan.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = Item ini dipotong di aplikasi lain, jadi Folders menyalinnya.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = Ini sudah sedang disalin atau dipindahkan.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = Ini masih sedang disalin ke sini.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10.000 item atau lebih

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
       *[other] { $n } item
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
       *[other] { $n } item tidak dapat { $verb }: { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } dan { $more } lagi

# Words on a job card while things are copied: what, and where to.
folders-act-copying = Menyalin { $what } ke { $place }

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = { $what } disalin ke { $place }.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = disalin

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = Urungkan penyalinan { $what }

# Words on a job card while things are moved: what, and where to.
folders-act-moving = Memindahkan { $what } ke { $place }

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = { $what } dipindahkan ke { $place }.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = dipindahkan

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = Urungkan pemindahan { $what }

# Words on a job card while things are put in the Trash.
folders-act-trashing = Memasukkan { $what } ke Tempat Sampah

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = { $what } dimasukkan ke Tempat Sampah.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = dimasukkan ke Tempat Sampah

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = Urungkan pemasukan { $what } ke Tempat Sampah

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = Menghapus { $what }

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = { $what } dihapus permanen.

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = dihapus

# Words on a job card while things are put back from the Trash.
folders-act-restoring = Mengembalikan { $what }

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = { $what } dikembalikan.

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = Urungkan pengembalian { $what }

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = dikembalikan

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Mengurungkan

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Sudah diurungkan.

# Words on a job card while things are packed into an archive.
folders-act-compressing = Memampatkan { $what }

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = { $file } dibuat di { $place }.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = dimampatkan

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = Urungkan pemampatan { $what }

# Words on a job card while an archive is unpacked.
folders-act-extracting = Mengekstrak { $what }

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = { $what } diekstrak ke { $place }.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = diekstrak

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = Urungkan ekstraksi { $what }

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = Menimpa dengan aman { $what }

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = { $what } ditimpa dengan aman.

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = ditimpa dengan aman

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = Mengeluarkan { $what }

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = { $what } dikeluarkan dari arsip ke { $place }.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = dikeluarkan

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = Urungkan pengeluaran { $what }

# Words on a job card while pictures are turned.
folders-act-turning = Memutar { $what }

# Words under the file lines when pictures have been turned.
folders-act-turned = { $what } diputar.

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = diputar

# Words on a job card while pictures are resized.
folders-act-resizing = Mengubah ukuran { $what }

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = Salinan { $what } dengan ukuran baru dibuat.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = diubah ukurannya

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = Urungkan perubahan ukuran { $what }

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = Folder baru

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = Dokumen baru

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders tidak dapat membuat { $base }: { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = Ketik nama.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = Nama tidak boleh memuat garis miring.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = Nama itu dipakai oleh sistem. Pilih nama lain.

# Red words under a name being typed when it is too long.
folders-act-name-long = Nama itu terlalu panjang.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders tidak dapat memulai pekerjaan ini.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = Ulangi { $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = Urungkan { $what }

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = Pekerjaan dihentikan sebelum selesai.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped =
    { $done } { $n ->
       *[other] Folders melewati { $n } item.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Pilih sesuatu dulu.

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = Itu sudah ada di Tempat Sampah.

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = Berkas di server tidak dapat dimasukkan ke Tempat Sampah. Gunakan Hapus permanen sebagai gantinya.

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Pilih sesuatu untuk disalin atau dipindahkan dulu.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = Isi arsip hanya dapat dikeluarkan ke sebuah folder.

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = Anda tidak dapat menyalin item ke Tempat Sampah.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = Panel lain tidak menampilkan folder.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Pilih sesuatu untuk disalin dulu.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = { $what } dipotong.

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = { $what } disalin.

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = Item hanya dapat ditempelkan ke sebuah folder.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = Tidak ada berkas di papan klip.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = Belum ada yang dipotong atau disalin.

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = Lokasi jaringan tidak dapat dihapus dari sini.

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = Tempat Sampah sudah kosong.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Pilih sesuatu di Tempat Sampah dulu.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = Membuat { $name }

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = Anda hanya dapat membuat folder atau dokumen baru di dalam sebuah folder.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = Urungkan pembuatan { $name }

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders tidak dapat mempertahankan warna folder pada item yang dipindahkan.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = Tidak ada yang dapat diulangi.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = Tidak ada yang dapat diurungkan.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = Mengulangi { $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = { $what } diulangi.

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = dilakukan ulang

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = Mengurungkan { $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = { $what } diurungkan.

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Beranda

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Tempat Sampah

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = Item hanya dapat dijatuhkan ke folder atau Tempat Sampah.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
       *[other] { $n } tautan
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = { $what } dibuat.

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
       *[other] { $n } tautan tidak dapat dibuat: { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = Ada pertanyaan yang menunggu jawaban Anda. Jawab, atau hentikan pekerjaannya, sebelum Anda menutup Folders.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = Folders tidak dapat ditutup selama ada pekerjaan yang berjalan. Tunggu sampai selesai, atau hentikan, lalu tutup Folders.
