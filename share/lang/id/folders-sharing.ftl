# machine translated (Indonesian), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders tidak dapat membaca isi folder ini.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = Hanya folder itu sendiri yang dapat dibagikan, bukan tautan ke folder.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = Hanya folder milik Anda yang dapat dibagikan. Folder ini milik orang lain.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = folder beranda Anda

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = folder ini

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } dan { $second }

# Name of the switch that shares the folder.
# SAFETY: needs a human review before it ships
folders-sharing-share = Bagikan folder ini

# One line under the share switch when the folder is shared.
folders-sharing-shared = Dibagikan di jaringan rumah.


# Tip of the share switch.
# SAFETY: needs a human review before it ships
folders-sharing-share-tip = Bagikan di jaringan rumah

# Label over the box for the name other computers see.
folders-sharing-name = Nama

# Tip of the box for the name other computers see.
folders-sharing-name-tip = Nama yang dilihat komputer lain

# Name of the switch that lets others change files, and its tip.
# SAFETY: needs a human review before it ships
folders-sharing-write = Izinkan orang lain mengubah berkas

# One line under the switch that lets others change files.
# SAFETY: needs a human review before it ships
folders-sharing-write-sub = Mereka dapat menambah, mengubah, dan menghapus berkas di dalamnya.

# Name of the switch that lets anyone in without a password.
# SAFETY: needs a human review before it ships
folders-sharing-guests = Izinkan siapa saja masuk

# One line under the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-sub = Tidak perlu kata sandi.

# Tip of the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-tip = Izinkan siapa saja masuk, tanpa kata sandi

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Setelah dibagikan, folder ini muncul di Jaringan pada komputer Anda yang lain di rumah, termasuk komputer Windows. Pertama kali, Trindle mengunduh Samba, program yang membagikan folder. Ini memakan waktu semenit.
       *[no] Setelah dibagikan, folder ini muncul di Jaringan pada komputer Anda yang lain di rumah, termasuk komputer Windows.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] Di komputer lain, cari di Jaringan untuk { $name } di { $host }.
       *[no] Di komputer lain, cari di Jaringan untuk { $name }.
    }

# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = Komputer lain belum dapat menjangkau folder ini. Di Pengaturan, buka Jaringan dan nyalakan "Izinkan komputer lain terhubung".

# Tip of the button that lets other computers connect.
# SAFETY: needs a human review before it ships
folders-sharing-let-connect = Izinkan komputer lain terhubung

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
folders-sharing-visitors =
    { $n ->
       *[other] Tamu tanpa kata sandi belum dapat membukanya, karena { $names } tertutup bagi akun lain. Jika Anda mengizinkan mereka lewat, mereka dapat menjangkau folder ini tetapi tidak dapat melihat apa pun yang lain di sepanjang jalannya.
    }

# Tip of the button that lets visitors through to the folder.
# SAFETY: needs a human review before it ships
folders-sharing-let-through = Izinkan tamu menjangkau folder ini

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
folders-sharing-sign-in-set = Orang lain masuk sebagai { $user } dengan kata sandi berbagi.

# Tip of the button that changes the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-change-password = Ubah kata sandi berbagi

# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-set-password = Orang lain masuk sebagai { $user }. Tetapkan kata sandi berbagi minimal { $n } karakter. Kata sandi ini terpisah dari kata sandi login Anda.

# Label beside the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password = Kata sandi

# Tip of the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password-tip = Kata sandi berbagi

# Tip of the button that keeps the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-keep-password = Simpan kata sandi

# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = Bagikan { $name } di jaringan

# Red words when the share's name is used by another share.
folders-sharing-name-taken = Anda sudah memiliki folder bersama dengan nama itu. Pilih nama lain.

# Calm words while sharing is being stopped.
folders-sharing-stopping = Menghentikan pembagian…

# Red words when the sharing password is too short; { $n } is the fewest letters.
folders-sharing-password-short = Kata sandi minimal { $n } karakter.

# Calm words while the sharing password is being kept.
folders-sharing-keeping-password = Menyimpan kata sandi. Anda akan diminta memasukkan kata sandi administrator terlebih dahulu.

# Calm words while other computers are being let connect.
folders-sharing-letting-connect = Mengizinkan komputer lain terhubung. Anda akan diminta memasukkan kata sandi administrator terlebih dahulu.

# Calm words while Samba is fetched and set up the first time.
folders-sharing-fetching = Mengunduh dan menyiapkan Samba, program yang membagikan folder. Anda akan diminta memasukkan kata sandi administrator terlebih dahulu. Ini hanya terjadi pertama kali.

# Calm words while sharing is set up the first time and Samba is already there.
folders-sharing-setting-up = Menyiapkan pembagian. Anda akan diminta memasukkan kata sandi administrator terlebih dahulu. Ini hanya terjadi pertama kali.

# Calm words while an existing share is changed.
folders-sharing-changing = Mengubah folder bersama…

# Calm words while a share is made.
folders-sharing-sharing-it = Membagikan folder…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = Beri nama untuk folder bersama ini.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = Nama folder bersama paling banyak { $most } karakter.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = Nama folder bersama tidak boleh memuat salah satu dari % < > * ? | / \ + = ; : " atau koma.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = Nama yang berakhiran $ akan menyembunyikan berbagi ini, jadi tidak dapat dipakai.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = Nama folder bersama tidak boleh diawali tanda hubung.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba sudah memakai nama { $name }. Pilih nama lain.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Dibagikan

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = Hanya folder milik Anda yang dapat dibagikan.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = Nama itu sudah menjadi nama akun di komputer ini, jadi tidak dapat dipakai untuk berbagi. Pilih nama lain.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = Pengaturan Samba di komputer ini tidak mengizinkan berbagi tanpa kata sandi.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = Komputer ini tidak dapat membagikan folder lagi.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = Folder itu sudah tidak ada.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba tidak memberikan alasan.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Pesan dari Samba: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } milik orang lain, jadi Anda tidak dapat memberi tamu akses ke sana dari sini.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders tidak dapat memberi tamu akses ke { $path }: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle tidak dapat mengunduh Samba karena komputer ini tampaknya tidak terhubung. Sambungkan ke internet lalu coba lagi.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Trindle tidak dapat memasang Samba selagi ada pemasangan lain. Coba lagi beberapa menit lagi.
