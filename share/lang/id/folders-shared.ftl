# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = Anda tidak memasukkan kata sandi administrator
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = kata sandi administrator tidak diterima
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = akun Anda tidak diizinkan melakukan itu, bahkan dengan kata sandi administrator
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = tidak ada cara untuk meminta kata sandi administrator dari Anda
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = kata sandi administrator tidak diterima, atau akun Anda tidak diizinkan melakukan itu
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = Anda tidak memasukkan kata sandi administrator
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = Kata sandi administrator tidak diterima
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = Akun Anda tidak diizinkan melakukan itu, bahkan dengan kata sandi administrator
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = Tidak ada cara untuk meminta kata sandi administrator dari Anda
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = Kata sandi administrator tidak diterima, atau akun Anda tidak diizinkan melakukan itu
# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = Itu tidak dilakukan karena Anda tidak memasukkan kata sandi administrator.
# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = Itu tidak dilakukan karena kata sandi administrator tidak diterima.
# SAFETY: needs a human review before it ships
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = Itu tidak dilakukan. Akun Anda tidak diizinkan melakukannya, bahkan dengan kata sandi administrator. Keluar lalu masuk kembali, kemudian coba lagi.
# SAFETY: needs a human review before it ships
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = Itu tidak dilakukan. Tidak ada cara untuk meminta kata sandi administrator dari Anda. Keluar lalu masuk kembali, kemudian coba lagi.
# SAFETY: needs a human review before it ships
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = Itu tidak dilakukan. Kata sandi administrator tidak diterima, atau akun Anda tidak diizinkan melakukannya. Jika Anda tidak diminta memasukkan kata sandi, keluar lalu masuk kembali, kemudian coba lagi.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Itu tidak berhasil, dan tidak ada alasan yang diberikan.
# SAFETY: needs a human review before it ships
# Red words when the keyring will not open without the person's password.
common-keyring-locked = Gantungan kunci terkunci. Buka kuncinya, atau keluar lalu masuk lagi, kemudian coba lagi.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = Tidak ada gantungan kunci di komputer ini untuk menyimpannya.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Folders tidak dapat menghubungi gantungan kunci tepat waktu.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Folders berhenti menunggu gantungan kunci.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Folders tidak dapat mengakses gantungan kunci ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Folders masih menunggu gantungan kunci, jadi { $what } belum dihapus.
       *[other] Folders masih menunggu gantungan kunci, jadi { $what } belum disimpan.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] Setelah tertunda, { $what } dihapus dari gantungan kunci.
       *[other] Setelah tertunda, { $what } disimpan di gantungan kunci.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Folders berhenti menunggu gantungan kunci, jadi { $what } tidak dihapus.
       *[other] Folders berhenti menunggu gantungan kunci, jadi { $what } tidak disimpan.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Folders tidak dapat menghapus { $what } dari gantungan kunci. { $why }
       *[other] Folders tidak dapat menyimpan { $what } di gantungan kunci. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Selengkapnya
common-more-spoken = Buka Bantuan tentang ini
# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
folders-volumes-no-password = Anda tidak memasukkan kata sandi, jadi tidak ada yang diubah.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, jadi tidak ada yang berubah.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = Masih ada yang memakainya. Tutup jendela atau program yang menampilkan berkasnya, lalu coba lagi.
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = Tidak cukup ruang untuk isinya pada ukuran itu.
# Heading over the password box, and the tip shown in the box while it is empty.
# SAFETY: needs a human review before it ships
folders-prompt-password = Kata sandi
# Main button at the bottom of the password window that checks the password.
# SAFETY: needs a human review before it ships
folders-prompt-unlock = Buka kunci
# Button at the bottom of the password window that closes it without unlocking.
# SAFETY: needs a human review before it ships
folders-prompt-cancel = Batal
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = Ketik kata sandi terlebih dahulu.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Memeriksa…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = lebih besar dari 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = lebih besar dari layar Anda
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = lebih dari 20 MB (200 MB jika beranimasi)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = lebih dari 20 MB
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = lebih dari 200 MB
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } atau { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Terkunci: { $name }

# Name given to a drive that has none.
folders-volumes-drive = Drive

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = Frasa sandi itu tidak membuka drive.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = Sudah terbuka.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = Anda memerlukan frasa sandi untuk membukanya.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = Frasa sandi untuk { $name }

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Buka kunci drive

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders tidak dapat membuka jendela frasa sandi.
