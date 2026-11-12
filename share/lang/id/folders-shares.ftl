
# machine translated (Indonesian), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = Komputer ini tidak diizinkan membuka { $export } di { $server }.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } tidak membagikan folder bernama { $export }.

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = Anda akan ditanya lagi lain kali

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = Kurang dari sehari lagi

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
       *[other] Tinggal { $n } hari
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
folders-shares-held =
    { $past ->
        [yes] { $server } sekarang berada di { $now }. Login Anda untuknya sebelumnya dipakai di alamat yang berbeda ({ $before }).
       *[no] { $server } berada di { $now }. Login Anda untuknya belum pernah dipakai di alamat itu.
    }{ $full ->
        [yes] {" "}Folders hanya mengingat { $most } alamat terakhir.
       *[no] {""}
    } Perangkat mana pun di jaringan dapat memakai sebuah nama, jadi Folders belum mengirim login tersimpan Anda. Jika ini { $server } milik Anda, ketik nama pengguna dan kata sandinya untuk menggunakannya di sini.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when no address could be found for the computer to show.
folders-shares-nowhere = Folders tidak dapat menemukan alamat { $server }, jadi tidak ada yang dikirim ke sana. Apa pun yang Anda ketik di sini tidak dapat dikirim dengan aman. Buka melalui alamatnya, misalnya smb://192.168.1.20/.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
folders-shares-lapsed = { $server } berada di { $now }. Login Anda untuknya terakhir dipakai di sana lebih dari { $days } hari lalu. Perangkat lain mungkin sudah mendapat alamat itu sejak saat itu, jadi Folders belum mengirim login tersimpan Anda. Jika ini { $server } milik Anda, ketik nama pengguna dan kata sandinya untuk menggunakannya di sini.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
folders-shares-unknown = { $server } berada di { $now }. Sebuah login untuknya pernah dipakai di sana, tetapi Folders tidak mencatat kapan atau login yang mana. Perangkat lain mungkin sudah mendapat alamat itu sejak saat itu, jadi Folders belum mengirim login tersimpan Anda. Jika ini { $server } milik Anda, ketik nama pengguna dan kata sandinya untuk menggunakannya di sini.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
folders-shares-unshown = { $server } berada di { $now }, tempat login Anda untuknya pernah dipakai. Kali ini Folders menemukannya di jaringan lokal, tempat perangkat mana pun dapat memakai nama itu. Anda belum pernah melihat alamat ini, jadi Folders belum mengirim login Anda. Jika ini { $server } milik Anda, ketik nama pengguna dan kata sandinya untuk menggunakannya di sini.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
folders-shares-asking =
    Ketik nama pengguna dan kata sandi untuk sebuah akun di { $server }.{ $heard ->
        [yes] {" "}Komputer itu berada di { $at }. Perangkat mana pun di jaringan dapat memakai sebuah nama, jadi pastikan ini alamatnya sebelum Anda mengetik.
       *[no] {""}
    }

# SAFETY: needs a human review before it ships
# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
folders-shares-where =
    Yang Anda ketik dikirim ke { $address }{ $heard ->
        [yes] {" "}dan tidak ke tempat lain. Dengan Ingat menyala, Folders menyimpan login Anda hanya untuk alamat ini.
       *[no] {"."}
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
folders-shares-again = Folders menemukan { $server } di jaringan lokal pada { $at }, port { $port }. Perangkat mana pun dapat memakai nama itu di sana, jadi Folders belum mengirim login Anda. Jika ini alamat yang benar, tekan Buka.

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders tidak dapat menghubungi gantungan kunci: { $why }.

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = Terbuka, tetapi Folders tidak dapat menyimpan login. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } tidak menyediakan NFS versi 4, satu-satunya versi yang dapat dibuka Folders.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = Terbuka, tetapi Folders tidak dapat menemukan lokasinya di komputer ini.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = Berbagi terbuka, tetapi Folders tidak dapat menemukan lokasinya di komputer ini.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = Itu bukan alamat.

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = Alamat yang dipakai untuk smb://{ $server }

# SAFETY: needs a human review before it ships
# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
folders-shares-password-label = Kata sandi untuk { $user } di { $what }

# SAFETY: needs a human review before it ships
# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
folders-shares-remember = Ingat selama 90 hari

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = Hitungan 90 hari dimulai lagi setiap kali Anda menggunakannya.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } di { $server }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = Buka { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Tutup

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = Nama pengguna, misalnya jo atau WORKGROUP\jo

# SAFETY: needs a human review before it ships
# Hint in the empty password box of the sign-in card.
folders-shares-password = Kata sandi

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
folders-shares-forget = Lupakan alamat sebelumnya

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
folders-shares-forget-spoken = Lupakan alamat sebelumnya untuk login ini

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Membuka…

# Name of the main button of the sign-in card.
folders-shares-open = Buka

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = Itu bukan nama komputer.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = Itu bukan jalur folder bersama.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = Membuka { $export } di { $host }…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = Membuka { $share } di { $server }…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Selesaikan dulu proses login ke { $it }.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = Terbuka. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } tidak membagikan folder apa pun kepada Anda.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = Menutup { $name }…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders tidak dapat menutup { $name }: { $why }.

# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = Menghubungkan kembali { $name }…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders tidak dapat menyimpan berbagi Anda untuk lain kali.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = Nama pengguna itu tidak dapat dipakai untuk sebuah berbagi.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = Domain itu tidak dapat dipakai untuk sebuah berbagi.

# Red words when a share's password holds a line break.
folders-shares-bad-password = Kata sandi sebuah berbagi tidak boleh memuat pergantian baris.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } memerlukan nama pengguna dan kata sandi untuk { $it }.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = Nama pengguna atau kata sandi untuk { $it } salah.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = Akun Anda di { $server } terkunci atau dimatikan, atau kata sandinya sudah kedaluwarsa.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } tidak memiliki folder bersama bernama { $share }.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = Akun itu tidak diizinkan membuka { $it }.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } tidak merespons. Mungkin sedang tidur atau dimatikan. Bangunkan lalu coba lagi.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders tidak menemukan komputer bernama { $server } di jaringan ini.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } hanya memakai SMB1, cara berbagi lama yang tidak lagi aman. Folders tidak memakainya.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = Ini memerlukan paket { $package } dari Debian, yang belum terpasang.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders tidak dapat membuka berbagi.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders tidak dapat membuka folder bersama.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders tidak dapat menampilkan daftar folder bersamanya.
