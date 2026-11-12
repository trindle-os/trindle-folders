
# machine translated (German), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = es gibt keinen Server namens { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } nimmt keine Verbindungen an

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } hat nicht rechtzeitig geantwortet

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = { $host } ist im Netzwerk nicht erreichbar

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
folders-remote-ftp-no-protect = Dieser Server kann Dateiübertragungen nicht absichern, deshalb hat Folders nichts gesendet: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Der Server hat beim Verbinden mit Folders nicht wie erwartet geantwortet: { $said }

# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = Dieser Server bietet kein sicheres FTP an. Versuche die Adresse mit ftp:// am Anfang.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = eine sichere Verbindung

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = die Anmeldung

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = eine Datenverbindung

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = den binären Übertragungsmodus

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = den Ordner, in dem du startest

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = Die Anmeldung mit einem FTP-Konto

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders konnte keine Datenverbindung zum Server öffnen.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = Sicheres FTP

# Red words when the server stopped answering.
folders-remote-ftp-stopped = Der Server antwortet nicht mehr.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = Die Verbindung zum Server ist abgebrochen.

# Red words when the server closed the connection.
folders-remote-ftp-closed = Der Server hat die Verbindung geschlossen.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = Dieser Vorgang auf diesem Server

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Auf dem Server ist etwas schiefgelaufen (es ging um { $what }): { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = der Server hat etwas gesendet, das keine FTP-Antwort ist

