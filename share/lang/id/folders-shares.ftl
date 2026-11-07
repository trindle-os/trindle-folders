

folders-shares-nfs-denied = { $server } tidak mengizinkan komputer ini membuka { $export }.

folders-shares-nfs-no-share = { $server } tidak membagikan apa pun bernama { $export }.

folders-shares-left-none = Akan ditanyakan lagi pada pembukaan berikutnya

folders-shares-left-under-day = Kurang dari sehari lagi

folders-shares-left-days =
    { $n ->
       *[other] Tinggal { $n } hari
    }

folders-shares-held =
    { $past ->
        [yes] { $server } sekarang menjawab di { $now }, alamat yang berbeda dari alamat tempat login Anda untuk komputer itu digunakan sebelumnya ({ $before }).
       *[no] { $server } menjawab di { $now }, alamat tempat login Anda untuk komputer itu belum pernah digunakan.
    }{ $full ->
        [yes] {" "}Hanya { $most } alamat terakhir tempat login itu digunakan yang diingat.
       *[no] {""}
    } Perangkat mana pun di jaringan dapat menjawab atas sebuah nama, jadi login yang tersimpan untuknya tidak dikirim. Jika ini { $server } milik Anda, ketik nama pengguna dan kata sandinya untuk menggunakannya di sini.

folders-shares-nowhere = Alamat tempat { $server } menjawab tidak dapat ditemukan sebelumnya, jadi tidak ada yang dikirim ke sana dan apa yang diketik di sini tidak punya tujuan yang aman. Buka melalui alamatnya, misalnya smb://192.168.1.20/.

folders-shares-lapsed = { $server } menjawab di { $now }, tempat login Anda untuk komputer itu terakhir digunakan lebih dari { $days } hari lalu. Perangkat lain mungkin sudah mendapat alamat itu sejak saat itu, jadi login yang tersimpan untuknya tidak dikirim. Jika ini { $server } milik Anda, ketik nama pengguna dan kata sandinya untuk menggunakannya di sini.

folders-shares-unknown = { $server } menjawab di { $now }, tempat sebuah login untuknya pernah digunakan, tetapi kapan atau login yang mana tidak dicatat. Perangkat lain mungkin sudah mendapat alamat itu sejak saat itu, jadi login yang tersimpan untuknya tidak dikirim. Jika ini { $server } milik Anda, ketik nama pengguna dan kata sandinya untuk menggunakannya di sini.

folders-shares-unshown = { $server } menjawab di { $now }, alamat tempat login Anda untuk komputer itu dikirim sebelumnya. Kali ini alamat itu hanya terdengar di jaringan, tempat perangkat mana pun dapat menjawab atas sebuah nama, dan belum pernah ditampilkan kepada Anda di sini, jadi login tidak dikirim. Jika ini { $server } milik Anda, ketik nama pengguna dan kata sandinya untuk menggunakannya di sini.

folders-shares-asking =
    { $server } meminta nama pengguna dan kata sandi. Isinya adalah untuk akun di komputer itu.{ $heard ->
        [yes] {" "}Komputer itu menjawab di { $at }. Perangkat mana pun di jaringan dapat menjawab atas sebuah nama, jadi pastikan itu alamatnya sebelum Anda mengetik.
       *[no] {""}
    }

folders-shares-where =
    Yang Anda ketik dikirim ke { $address }{ $heard ->
        [yes] {" "}dan tidak ke tempat lain. Dengan Ingat menyala, login Anda disimpan hanya untuk alamat itu.
       *[no] {"."}
    }

folders-shares-again = { $server } menjawab di { $at }, port { $port }. Alamat itu hanya terdengar di jaringan, tempat perangkat mana pun dapat menjawab atas sebuah nama, jadi login Anda untuknya tidak dikirim. Jika itu alamatnya, tekan Buka.

folders-shares-keyring-unreachable = Gantungan kunci tidak dapat dijangkau: { $why }.

folders-shares-unkept = Terbuka, tetapi login tidak disimpan. { $why }

folders-shares-nfs-version = { $server } tidak menyediakan NFS versi 4, satu-satunya jenis yang dibuka Trindle.

folders-shares-nfs-opened-unknown = Terbuka, tetapi lokasinya tidak diketahui.

folders-shares-opened-unknown = Berbagi terbuka, tetapi lokasinya tidak diketahui.

folders-shares-not-address = Itu bukan alamat.

folders-shares-seen-label = Alamat tempat smb://{ $server } dibuka

folders-shares-password-label = Kata sandi untuk { $user } di { $what }

folders-shares-remember = Ingat selama 90 hari

folders-shares-remember-line = Dihitung ulang setiap kali Anda menggunakannya.

folders-shares-share-on = { $share } di { $server }

folders-shares-card-title = Buka { $it }

folders-shares-close = Tutup

folders-shares-user-placeholder = Nama pengguna, misalnya jo, atau WORKGROUP\jo

folders-shares-password = Kata sandi

folders-shares-forget = Lupakan alamat sebelumnya

folders-shares-forget-spoken = Lupakan alamat tempat login ini digunakan sebelumnya

folders-shares-opening = Membuka

folders-shares-open = Buka

folders-shares-bad-computer = Itu bukan nama komputer.

folders-shares-bad-path = Itu bukan jalur folder bersama.

folders-shares-opening-export = Membuka { $export } di { $host }…

folders-shares-opening-share = Membuka { $share } di { $server }…

folders-shares-answer-card = Jawab dulu kartu untuk { $it }.

folders-shares-open-waiting = Terbuka. { $words }

folders-shares-none = { $server } tidak membagikan folder apa pun kepada Anda.

folders-shares-closing = Menutup { $name }…

folders-shares-close-failed = { $name } tidak dapat ditutup: { $why }.

folders-shares-reconnecting = Menghubungkan kembali { $name }…

folders-shares-not-remembered = Berbagi jaringan tidak dapat diingat untuk lain kali.
