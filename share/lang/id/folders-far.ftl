# machine translated (Indonesian), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = tidak ada server bernama { $host }

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = tidak semuanya sampai, jadi aslinya belum dipindahkan

# Reason a link was not sent to a server.
folders-far-not-followed = ini tautan, jadi Folders tidak mengirimnya

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = ini bukan file atau folder, jadi Folders tidak mengirimnya

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders tidak dapat mengetahui jenis item ini, jadi dibiarkan di sana

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = Di server jenis ini, sebuah file bisa menimpa file lain yang bernama sama.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = Itu tidak dapat dilakukan pada berkas di server

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders tidak dapat membacanya

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders tidak dapat membukanya.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (folder tidak dapat dimasukkan ke dalam dirinya sendiri)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (sudah ada folder dengan nama itu)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (sudah ada berkas dengan nama itu)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders tidak tahu alamat server.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = salinan yang dikirim berukuran salah

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = Ini hanya berfungsi dengan file di server.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders tidak dapat menempatkan file baru, jadi yang lama masih ada, sekarang bernama { $old }, dan yang baru bernama { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders tidak dapat menghapus file lama. File itu masih ada, sekarang bernama { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders tidak dapat menghapus file lama, jadi file itu masih ada, sekarang bernama { $old }: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } tidak kosong.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders melewati sebuah file karena namanya tidak aman dipakai.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
       *[other] Folders melewati { $n } item karena namanya tidak aman dipakai: { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = Mengunduh { $name }…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders tidak dapat mengunduh { $name }: { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = Salinan { $name } dibuka. Saat Anda menyimpannya, perubahan Anda dikirim kembali ke server.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = { $name } disimpan di server.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders tidak dapat menyimpan { $name } di server: { $why }. Perubahan Anda masih ada di salinan di komputer ini.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } diganti namanya menjadi { $now }.

# Heading of the Connect to a server card.
folders-far-connect-title = Hubungkan ke server

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Tutup

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Alamat, misalnya sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
folders-far-connect-password = Kata sandi, jika perlu

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = Berkas kunci, jika Anda memakainya

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = Pilih berkas kunci


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders memakai pengaturan ssh, agen ssh, kunci, dan daftar server yang dikenal milik Anda.

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = Anda belum pernah terhubung ke { $host }, jadi Folders tidak dapat memastikan bahwa itu server yang benar. Percayai hanya jika Anda memang akan terhubung ke sana.

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = { $host } memiliki identitas yang berbeda dari terakhir kali. Mungkin ada pihak yang mencoba mencegat koneksi Anda, jadi Folders tidak terhubung. Jika pemiliknya mengubahnya, hapus entri lamanya dari daftar host yang dikenal terlebih dahulu.

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
folders-far-connect-needs-password = Server ini meminta kata sandi. Ketik di atas, lalu coba lagi.

# Note on the Connect to a server card when a server asks for a key or password.
folders-far-connect-needs-key = Server ini memerlukan kunci atau kata sandi. Pilih berkas kunci Anda di atas, atau tambahkan ke agen ssh Anda, lalu coba lagi.

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = Folders tidak dapat memeriksa sertifikat { $host }: { $why }. Jika bisa, bandingkan sidik jarinya dengan yang diberikan pemiliknya kepada Anda. Percayai hanya jika cocok.

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = Sertifikat { $host } berubah sejak Anda mempercayainya. Pemiliknya mungkin memperbaruinya, atau ada pihak yang mencoba mencegat koneksi Anda. Percayai hanya jika pemiliknya membenarkan sidik jari ini.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Menghubungkan

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = Percayai dan hubungkan

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Hubungkan

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Ponsel muncul di bilah samping saat Anda mencolokkannya dan membuka kuncinya.
