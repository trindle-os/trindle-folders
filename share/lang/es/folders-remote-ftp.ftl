
# machine translated (Spanish), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = no existe ningún servidor llamado { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } no acepta conexiones

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } no respondió a tiempo

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = no hay forma de llegar a { $host } en la red

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
folders-remote-ftp-no-protect = Este servidor no puede mantener seguras las transferencias de archivos, así que Folders no envió nada: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = El servidor no respondió como se esperaba cuando Folders se conectó: { $said }

# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = Este servidor no ofrece FTP seguro. Prueba la dirección con ftp:// al principio.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = una conexión segura

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = el inicio de sesión

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = una conexión de datos

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = el modo de transferencia binario

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = la carpeta en la que empiezas

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = Iniciar sesión con una cuenta FTP

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders no pudo abrir una conexión de datos con el servidor.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = FTP seguro

# Red words when the server stopped answering.
folders-remote-ftp-stopped = El servidor dejó de responder.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = Se perdió la conexión con el servidor.

# Red words when the server closed the connection.
folders-remote-ftp-closed = El servidor cerró la conexión.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = Hacer eso en este servidor

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Algo salió mal con { $what } en el servidor: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = el servidor envió algo que no es una respuesta FTP

