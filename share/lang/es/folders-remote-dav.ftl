
# machine translated (Spanish), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Red words when a thing on a server changed just before it was to be removed and was left alone; the variable is its name.
folders-remote-dav-changed-at-removal = { $name } (no se eliminó porque acababa de cambiar)

# Reason in red words when the server's reply is cut into pieces and one has no size.
folders-remote-dav-chunk = una parte de la respuesta del servidor no tenía tamaño

# Reason in red words ("Could not reach it: ...") when a server's name has no address; the variables are the name and the system's reason.
folders-remote-dav-not-found = No se encontró { $host } ({ $why })

# Reason in red words ("Could not reach it: ...") when a server's name gives no address; the variable is the name.
folders-remote-dav-no-address = no hay ninguna dirección para { $host } en la red

# Red words when the address opened names a file, not a folder; the variable is the path.
folders-remote-dav-is-file = { $path } es un archivo, no una carpeta.

# Red words when a file being sent ended before all of it was sent.
folders-remote-dav-file-ended = El archivo terminó antes de enviarse completo.

# Red words when the server wants a kind of login Folders cannot do.
folders-remote-dav-unknown-login = El servidor usa un tipo de inicio de sesión que Folders no admite.

# Red words when the server does not list folders over WebDAV.
folders-remote-dav-no-listing = Este servidor no deja que Folders liste las carpetas por WebDAV.

# Red words when the server did not answer like a web server.
folders-remote-dav-not-web = El servidor no respondió como debería hacerlo un servidor web.

# Red words when the server has no room left.
folders-remote-dav-full = El servidor está lleno.

# Red words when the server answered with a number Folders has no words for; the variables are the number and the name of the thing.
folders-remote-dav-status = El servidor respondió con el código { $status } para { $name }.

# Red words when the server's folder listing could not be read; the variable is the reason.
folders-remote-dav-listing-unreadable = Folders no pudo leer la lista de la carpeta: { $why }

# Red words when a file being fetched ended before all of it came.
folders-remote-dav-cut-short = Solo llegó una parte del archivo.

# Red words when a folder to be removed still holds something; the variable is its name.
folders-remote-dav-not-empty = { $name } no está vacía.

