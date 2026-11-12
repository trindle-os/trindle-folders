# machine translated (Spanish), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = Este equipo no tiene permiso para abrir { $export } en { $server }.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } no comparte ninguna carpeta llamada { $export }.

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = Se te volverá a preguntar la próxima vez

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = Queda menos de un día

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] Queda { $n } día
       *[other] Quedan { $n } días
    }

# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
folders-shares-held =
    { $past ->
        [yes] { $server } está ahora en { $now }. Tu inicio de sesión para él se usó antes en una dirección distinta ({ $before }).
       *[no] { $server } está en { $now }. Tu inicio de sesión para él no se ha usado antes en esa dirección.
    }{ $full ->
        [yes] {" "}Folders solo recuerda las últimas { $most } direcciones.
       *[no] {""}
    } Cualquier dispositivo de una red puede usar un nombre, así que Folders no ha enviado tu inicio de sesión guardado. Si este es tu { $server }, escribe su nombre de usuario y su contraseña para usarlo aquí.

# Words on the sign-in card when no address could be found for the computer to show.
folders-shares-nowhere = Folders no pudo averiguar la dirección de { $server }, así que no se le ha enviado nada. Lo que escribas aquí no se puede enviar de forma segura. Ábrelo mejor por su dirección, por ejemplo smb://192.168.1.20/.

# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
folders-shares-lapsed = { $server } está en { $now }. Tu inicio de sesión para él se usó allí por última vez hace más de { $days } días. Desde entonces puede que se haya asignado esa dirección a otro dispositivo, así que Folders no ha enviado tu inicio de sesión guardado. Si este es tu { $server }, escribe su nombre de usuario y su contraseña para usarlo aquí.

# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
folders-shares-unknown = { $server } está en { $now }. Antes se usó allí un inicio de sesión para él, pero Folders no anotó cuándo ni cuál. Desde entonces puede que se haya asignado esa dirección a otro dispositivo, así que Folders no ha enviado tu inicio de sesión guardado. Si este es tu { $server }, escribe su nombre de usuario y su contraseña para usarlo aquí.

# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
folders-shares-unshown = { $server } está en { $now }, donde antes se usó tu inicio de sesión para él. Esta vez Folders lo encontró en la red local, donde cualquier dispositivo puede usar ese nombre. No has visto antes esta dirección, así que Folders no ha enviado tu inicio de sesión. Si este es tu { $server }, escribe su nombre de usuario y su contraseña para usarlo aquí.

# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
folders-shares-asking =
    Escribe el nombre de usuario y la contraseña de una cuenta de { $server }.{ $heard ->
        [yes] {" "}Está en { $at }. Cualquier dispositivo de una red puede usar un nombre, así que comprueba que esta es su dirección antes de escribir.
       *[no] {""}
    }

# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
folders-shares-where =
    Lo que escribas va a { $address }{ $heard ->
        [yes] {" "}y a ningún otro sitio. Con Recordar activado, Folders guarda tu inicio de sesión solo para esta dirección.
       *[no] {"."}
    }

# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
folders-shares-again = Folders encontró { $server } en la red local, en { $at }, puerto { $port }. Cualquier dispositivo puede usar ese nombre allí, así que Folders no ha enviado tu inicio de sesión. Si esta es la dirección correcta, pulsa Abrir.

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders no pudo acceder al llavero: { $why }.

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = Está abierto, pero Folders no pudo guardar el inicio de sesión. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } no ofrece NFS versión 4, la única versión que Folders puede abrir.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = El recurso compartido se abrió, pero Folders no pudo encontrar dónde está en este equipo.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = El recurso compartido se abrió, pero Folders no pudo encontrar dónde está en este equipo.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = Eso no es una dirección.

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = Direcciones usadas para smb://{ $server }

# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
folders-shares-password-label = Contraseña de { $user } en { $what }

# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
folders-shares-remember = Recordar durante 90 días

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = Los 90 días vuelven a empezar cada vez que lo usas.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } en { $server }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = Abrir { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Cerrar

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = Nombre de usuario, por ejemplo jo o WORKGROUP\jo

# Hint in the empty password box of the sign-in card.
folders-shares-password = Contraseña

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
folders-shares-forget = Olvidar las direcciones anteriores

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
folders-shares-forget-spoken = Olvidar las direcciones anteriores de este inicio de sesión

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Abriendo…

# Name of the main button of the sign-in card.
folders-shares-open = Abrir

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = Ese no es un nombre de equipo.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = Esa no es una ruta a una carpeta compartida.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = Abriendo { $export } en { $host }…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = Abriendo { $share } en { $server }…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Termina primero de iniciar sesión en { $it }.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = Está abierto. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } no comparte ninguna carpeta contigo.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = Cerrando { $name }…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders no pudo cerrar { $name }: { $why }.


# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = Volviendo a conectar { $name }…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders no pudo guardar tus recursos compartidos para la próxima vez.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = Ese nombre de usuario no se puede usar para un recurso compartido.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = Ese dominio no se puede usar para un recurso compartido.

# Red words when a share's password holds a line break.
folders-shares-bad-password = La contraseña de un recurso compartido no puede tener saltos de línea.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } necesita un nombre de usuario y una contraseña para { $it }.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = El nombre de usuario o la contraseña de { $it } no son correctos.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = Tu cuenta en { $server } está bloqueada o desactivada, o su contraseña ha caducado.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } no tiene ninguna carpeta compartida llamada { $share }.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = Esa cuenta no puede abrir { $it }.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } no respondió. Puede que esté suspendido o apagado. Despiértalo e inténtalo de nuevo.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders no encontró ningún equipo llamado { $server } en esta red.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } solo usa SMB1, una forma antigua de compartir que ya no es segura. Folders no la usa.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = Esto necesita el paquete { $package } de Debian, que no está instalado.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders no pudo abrir el recurso compartido.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders no pudo abrir la carpeta compartida.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders no pudo listar sus carpetas compartidas.
