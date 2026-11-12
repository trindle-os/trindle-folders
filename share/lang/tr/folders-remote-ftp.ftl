
# machine translated (Turkish), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = { $host } adında bir sunucu yok

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } bağlantı kabul etmiyor

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } zamanında yanıt vermedi

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = ağda { $host } sunucusuna ulaşmanın bir yolu yok

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
# SAFETY: needs a human review before it ships
folders-remote-ftp-no-protect = Bu sunucu dosya aktarımlarını güvenli tutamıyor, bu yüzden Folders hiçbir şey göndermedi: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Folders bağlandığında sunucu beklenen şekilde yanıt vermedi: { $said }

# Red words when the server does not offer secure FTP.
# SAFETY: needs a human review before it ships
folders-remote-ftp-no-secure = Bu sunucu güvenli FTP sunmuyor. Adresin başına ftp:// yazarak deneyin.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = güvenli bir bağlantı

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = oturum açma

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = bir veri bağlantısı

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = ikili aktarım kipi

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = başladığınız klasör

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = FTP hesabıyla oturum açma

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders sunucuyla veri bağlantısı açamadı.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = Güvenli FTP

# Red words when the server stopped answering.
folders-remote-ftp-stopped = Sunucu yanıt vermeyi bıraktı.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = Sunucuyla bağlantı koptu.

# Red words when the server closed the connection.
folders-remote-ftp-closed = Sunucu bağlantıyı kapattı.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = Bunu bu sunucuda yapmak

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Sunucuda { $what } ile ilgili bir sorun oluştu: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = sunucu FTP yanıtı olmayan bir şey gönderdi

