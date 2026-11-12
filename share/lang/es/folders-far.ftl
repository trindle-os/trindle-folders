# machine translated (Spanish), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = no existe ningún servidor llamado { $host }

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = no llegó completo, así que el original no se ha movido

# Reason a link was not sent to a server.
folders-far-not-followed = es un enlace, así que Folders no lo envió

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = no es un archivo ni una carpeta, así que Folders no lo envió

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders no pudo saber qué tipo de elemento es, así que lo dejó donde estaba

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = En este tipo de servidor, un archivo podría reemplazar a otro con el mismo nombre.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = No puedes hacer eso con archivos de un servidor

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders no pudo leerlo

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders no pudo abrirlo.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (una carpeta no puede ir dentro de sí misma)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (ya hay una carpeta con ese nombre)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (ya hay un archivo con ese nombre)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders no conoce la dirección del servidor.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = la copia enviada tenía un tamaño incorrecto

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = Esto solo funciona con archivos de un servidor.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders no pudo colocar el archivo nuevo, así que el anterior sigue ahí, ahora llamado { $old }, y el nuevo se llama { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders no pudo quitar el archivo anterior. Sigue ahí, ahora llamado { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders no pudo quitar el archivo anterior, así que sigue ahí, ahora llamado { $old }: { $why }

# SAFETY: needs a human review before it ships
# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } no está vacía.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders omitió un archivo porque su nombre no es seguro.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
        [one] Folders omitió un elemento porque su nombre no es seguro: { $names }.
       *[other] Folders omitió { $n } elementos porque sus nombres no son seguros: { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = Descargando { $name }…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders no pudo descargar { $name }: { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = Se abrió una copia de { $name }. Al guardarla, tus cambios se envían de vuelta al servidor.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = Se guardó { $name } en el servidor.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders no pudo guardar { $name } en el servidor: { $why }. Tus cambios siguen en la copia de este equipo.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } ahora se llama { $now }.

# Heading of the Connect to a server card.
folders-far-connect-title = Conectar con un servidor

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Cerrar

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Dirección, por ejemplo sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
folders-far-connect-password = Contraseña, si hace falta

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = Archivo de clave, si usas uno

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = Elegir el archivo de clave


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders usa tu configuración de ssh, tu agente ssh, tus claves y tu lista de servidores conocidos.

# Note on the Connect to a server card for a server not visited before; the server's name.
folders-far-connect-new-host = No te has conectado antes a { $host }, así que Folders no puede comprobar que sea el correcto. Confía en él solo si esperabas conectarte a este servidor.

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
folders-far-connect-host-changed = { $host } tiene una identidad distinta de la última vez. Puede que alguien esté intentando interceptar tu conexión, así que Folders no se conectó. Si su propietario la cambió, quita antes su entrada antigua de tus servidores conocidos.

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
folders-far-connect-needs-password = Este servidor necesita una contraseña. Escríbela arriba y vuelve a intentarlo.

# Note on the Connect to a server card when a server asks for a key or password.
folders-far-connect-needs-key = Este servidor necesita una clave o una contraseña. Elige arriba tu archivo de clave, o añádelo a tu agente ssh, y vuelve a intentarlo.

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
folders-far-connect-cert-new = Folders no pudo comprobar el certificado de { $host }: { $why }. Si puedes, compara su huella digital con la que te dé el propietario. Confía en él solo si coinciden.

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
folders-far-connect-cert-changed = El certificado de { $host } ha cambiado desde que lo aceptaste. Puede que su propietario lo haya renovado, o que alguien esté intentando interceptar tu conexión. Confía en él solo si el propietario confirma esta huella digital.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Conectando

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
folders-far-connect-trust = Confiar y conectar

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Conectar

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Los teléfonos aparecen en la barra lateral cuando los conectas y los desbloqueas.

