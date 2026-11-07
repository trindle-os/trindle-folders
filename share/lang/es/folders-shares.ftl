# machine translated (Spanish), not yet reviewed by a person.

folders-shares-nfs-denied = { $server } no deja que este equipo abra { $export }.

folders-shares-nfs-no-share = { $server } no comparte nada llamado { $export }.

folders-shares-left-none = Volverá a preguntar en la próxima apertura

folders-shares-left-under-day = Queda menos de un día

folders-shares-left-days =
    { $n ->
        [one] Queda { $n } día
       *[other] Quedan { $n } días
    }

folders-shares-held =
    { $past ->
        [yes] { $server } responde ahora en { $now }, una dirección distinta de aquella en la que se usó antes tu inicio de sesión para él ({ $before }).
       *[no] { $server } responde en { $now }, una dirección en la que no se había usado antes tu inicio de sesión para él.
    }{ $full ->
        [yes] {" "}Solo se recuerdan las últimas { $most } direcciones en las que se usó.
       *[no] {""}
    } Cualquier dispositivo de una red puede responder a un nombre, así que el inicio de sesión guardado para él no se ha enviado. Si este es tu { $server }, escribe su nombre de usuario y su contraseña para usarlo aquí.

folders-shares-nowhere = No se pudo averiguar de antemano la dirección en la que responde { $server }, así que no se le ha enviado nada y lo que se escriba aquí no tiene adónde ir con seguridad. Ábrelo mejor por su dirección, por ejemplo smb://192.168.1.20/.

folders-shares-lapsed = { $server } responde en { $now }, donde tu inicio de sesión para él se usó por última vez hace más de { $days } días. Desde entonces puede que se haya asignado esa dirección a otro dispositivo, así que el inicio de sesión guardado para él no se ha enviado. Si este es tu { $server }, escribe su nombre de usuario y su contraseña para usarlo aquí.

folders-shares-unknown = { $server } responde en { $now }, donde antes se usó un inicio de sesión para él, pero no se anotó cuándo ni cuál. Desde entonces puede que se haya asignado esa dirección a otro dispositivo, así que el inicio de sesión guardado para él no se ha enviado. Si este es tu { $server }, escribe su nombre de usuario y su contraseña para usarlo aquí.

folders-shares-unshown = { $server } responde en { $now }, la dirección a la que fue antes tu inicio de sesión para él. Esta vez esa dirección solo se detectó en la red, donde cualquier dispositivo puede responder a un nombre, y aquí no se te había mostrado antes, así que el inicio de sesión no se ha enviado. Si este es tu { $server }, escribe su nombre de usuario y su contraseña para usarlo aquí.

folders-shares-asking =
    { $server } pide un nombre de usuario y una contraseña. Son los de una cuenta de ese equipo.{ $heard ->
        [yes] {" "}Responde en { $at }. Cualquier dispositivo de una red puede responder a un nombre, así que comprueba que esa es su dirección antes de escribir.
       *[no] {""}
    }

folders-shares-where =
    Lo que escribas va a { $address }{ $heard ->
        [yes] {" "}y a ningún otro sitio. Con Recordar activado, tu inicio de sesión se guarda solo para esa dirección.
       *[no] {"."}
    }

folders-shares-again = { $server } responde en { $at }, puerto { $port }. Esa dirección solo se detectó en la red, donde cualquier dispositivo puede responder a un nombre, así que tu inicio de sesión para él no se ha enviado. Si esa es su dirección, pulsa Abrir.

folders-shares-keyring-unreachable = No se pudo acceder al llavero: { $why }.

folders-shares-unkept = Está abierto, pero no se guardó el inicio de sesión. { $why }

folders-shares-nfs-version = { $server } no ofrece NFS versión 4, el único tipo que abre Trindle.

folders-shares-nfs-opened-unknown = Se abrió, pero no se sabe dónde.

folders-shares-opened-unknown = El recurso compartido se abrió, pero no se sabe dónde.

folders-shares-not-address = Eso no es una dirección.

folders-shares-seen-label = Direcciones en las que se abrió smb://{ $server }

folders-shares-password-label = Contraseña de { $user } en { $what }

folders-shares-remember = Recordar durante 90 días

folders-shares-remember-line = Vuelve a empezar cada vez que lo usas.

folders-shares-share-on = { $share } en { $server }

folders-shares-card-title = Abrir { $it }

folders-shares-close = Cerrar

folders-shares-user-placeholder = Nombre de usuario, por ejemplo jo, o WORKGROUP\jo

folders-shares-password = Contraseña

folders-shares-forget = Olvidar las direcciones anteriores

folders-shares-forget-spoken = Olvidar las direcciones en las que se usó antes este inicio de sesión

folders-shares-opening = Abriendo

folders-shares-open = Abrir

folders-shares-bad-computer = Ese no es el nombre de un equipo.

folders-shares-bad-path = Esa no es la ruta de una carpeta compartida.

folders-shares-opening-export = Abriendo { $export } en { $host }…

folders-shares-opening-share = Abriendo { $share } en { $server }…

folders-shares-answer-card = Responde primero a la tarjeta de { $it }.

folders-shares-open-waiting = Está abierto. { $words }

folders-shares-none = { $server } no comparte ninguna carpeta contigo.

folders-shares-closing = Cerrando { $name }…

folders-shares-close-failed = No se pudo cerrar { $name }: { $why }.

folders-shares-reconnecting = Volviendo a conectar { $name }…

folders-shares-not-remembered = No se pudieron recordar los recursos compartidos para la próxima vez.
