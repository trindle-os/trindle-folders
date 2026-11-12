
# machine translated (Indonesian), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = tidak ada server bernama { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } tidak menerima sambungan

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } tidak menjawab tepat waktu

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = tidak ada jalur untuk menjangkau { $host } di jaringan

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
# SAFETY: needs a human review before it ships
folders-remote-ftp-no-protect = Server ini tidak dapat menjaga keamanan pengiriman file, jadi Folders tidak mengirim apa pun: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Server tidak membalas seperti yang diharapkan saat Folders terhubung: { $said }

# Red words when the server does not offer secure FTP.
# SAFETY: needs a human review before it ships
folders-remote-ftp-no-secure = Server ini tidak menyediakan FTP aman. Coba alamat dengan ftp:// di awalnya.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = sambungan aman

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = proses masuk

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = sambungan data

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = mode transfer biner

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = folder tempat Anda mulai

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = Masuk dengan akun FTP

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders tidak dapat membuka sambungan data ke server.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = FTP aman

# Red words when the server stopped answering.
folders-remote-ftp-stopped = Server berhenti menjawab.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = Sambungan ke server terputus.

# Red words when the server closed the connection.
folders-remote-ftp-closed = Server menutup sambungan.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = Melakukan itu di server ini

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Terjadi masalah pada { $what } di server: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = server mengirim sesuatu yang bukan balasan FTP

