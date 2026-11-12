# machine translated (Spanish), not yet reviewed by a person.

# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
folders-remote-mod-heard-card = Folders encontró { $host } en la red local, en { $at }, puerto { $port }. Cualquier dispositivo puede usar ese nombre allí, así que Folders no ha enviado tu contraseña. Si esta es la dirección correcta, pulsa Conectar.

# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
folders-remote-mod-heard-stopped = Folders encontró { $host } en la red local, en { $at }. Cualquier dispositivo puede usar ese nombre allí, así que Folders no ha enviado tu contraseña. Abre { $host } de nuevo para comprobar su dirección.

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders no pudo leer esa dirección: { $why }.

# Red words when a server needs a password or key and none could be asked for.
folders-remote-mod-needs-password = Este servidor necesita una contraseña o una clave.

# Red words when a server turned down the login.
folders-remote-mod-login-refused = El servidor no aceptó ese inicio de sesión.

# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
folders-remote-mod-host-key-unknown = No te has conectado antes a { $host }, así que Folders aún no conoce su identidad.

# Red words when a server's identity differs from the one remembered; the variable is the server's name.
folders-remote-mod-host-key-changed = { $host } tiene una identidad distinta a la de la última vez. Puede que lo hayan reconstruido, o que alguien esté interceptando la conexión.

# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
folders-remote-mod-cert-unknown = Folders no pudo comprobar el certificado de { $host }: { $why }.

# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
folders-remote-mod-cert-changed = El certificado de { $host } ha cambiado desde que lo aceptaste.

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders no pudo conectarse: { $why }.

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders no encuentra { $what }.

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } ya existe.

# Red words when a server or phone refused permission; the variable is what it was refused for.
folders-remote-mod-denied = No tienes permiso para usar { $what }.

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } está en uso por otro programa.

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } no es compatible.

# Red words when a copy or other work with a server was cancelled.
folders-remote-mod-cancelled = Cancelado.

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = Seguir enlaces

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = Copiar a un recurso compartido de Windows, o desde él, antes de abrirlo

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = Copiar a un recurso compartido NFS, o desde él, antes de abrirlo

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = no empieza por sftp://, ftp:// ni mtp://

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = debe empezar por sftp://, ftp://, ftps://, ftpes://, dav://, davs://, smb://, nfs:// o mtp://

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = a un [ le falta su ]

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = no indica ningún servidor

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = el nombre del servidor no es válido

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = la ruta contiene un salto de línea

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = hay algo extra después del nombre del servidor

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } no es un número de puerto válido

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = omitido porque el servidor envió un nombre no seguro

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = algo sin nombre

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = La respuesta era demasiado grande, así que Folders la detuvo.

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } ahora es una carpeta, así que no se eliminó.
