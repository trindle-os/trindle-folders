# machine translated (Spanish), not yet reviewed by a person.

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = no se encontró ningún servidor llamado { $host }

folders-far-not-all-arrived = no llegó completo, así que sigue donde estaba

folders-far-not-followed = es un enlace, que no se sigue, así que no se envió

folders-far-not-sent = no es un archivo ni una carpeta, así que no se envió

folders-far-not-told = el servidor no dice qué es, así que se dejó

folders-far-ftp-cannot-refuse = Este servidor puede dar el mismo nombre a dos archivos.

folders-far-cannot-do = Eso no se puede hacer con los archivos de un servidor

folders-far-not-looked-at = no se pudo examinar

folders-far-not-opened = No se pudo abrir.

folders-far-inside-itself = { $name } (una carpeta no puede ir dentro de sí misma)

folders-far-taken-by-folder = { $name } (ya hay una carpeta con ese nombre)

folders-far-taken-by-file = { $name } (ya hay un archivo con ese nombre)

folders-far-address-unknown = No se conoce la dirección del servidor.

folders-far-copy-wrong-length = la copia enviada no tenía el tamaño que debía

folders-far-only-server-file = Solo se puede cambiar así el nombre de un archivo de un servidor.

folders-far-aside-both = el anterior está completo como { $old }, y el nuevo como { $new }: { $why }

folders-far-aside-kept = no se pudo quitar el anterior, que está completo como { $old }

folders-far-aside-kept-why = no se pudo quitar el anterior, que está completo como { $old }: { $why }

folders-far-not-empty = { $name } no está vacía.

folders-far-unsafe-name = Omitido: el servidor envió un nombre no seguro.

folders-far-unsafe-names =
    { $n ->
        [one] Omitido: el servidor envió un nombre no seguro: { $names }.
       *[other] Omitidos { $n }: el servidor envió nombres no seguros: { $names }.
    }

folders-far-fetching = Obteniendo { $name }…

folders-far-cannot-fetch = No se puede obtener { $name }: { $why }.

folders-far-opened-copy = Se abrió una copia de { $name }. Al guardarla, los cambios se envían de vuelta.

folders-far-saved = Se guardó { $name } en el servidor.

folders-far-not-saved = No se pudo guardar { $name } en el servidor: { $why }. La copia de aquí conserva los cambios.

folders-far-renamed = { $was } ahora se llama { $now }.

folders-far-connect-title = Conectar con un servidor

folders-far-connect-close = Cerrar

folders-far-connect-address = Dirección, por ejemplo sftp://jo@example.com

folders-far-connect-password = Contraseña, si la necesita

folders-far-connect-key = Archivo de clave, si eliges uno

folders-far-connect-pick-key = Elegir el archivo de clave

folders-far-connect-choose = Elegir

folders-far-connect-ssh-setup = Se usa tu propia configuración de ssh: su configuración, agente, claves y servidores conocidos.

folders-far-connect-new-host = { $host } no se ha visitado antes, así que Trindle no puede saber si es el correcto. Confía en él si esperabas llegar a este servidor.

folders-far-connect-host-changed = { $host } ya no tiene la identidad que tenía antes. Esto puede significar que alguien se está interponiendo, así que no se abre. Si su propietario la cambió, quita primero el anterior de tus servidores conocidos.

folders-far-connect-needs-password = Este servidor pide una contraseña. Escríbela arriba y vuelve a intentarlo.

folders-far-connect-needs-key = Este servidor pide una clave o una contraseña. Indica arriba tu archivo de clave, o añádelo a tu agente ssh, y vuelve a intentarlo.

folders-far-connect-cert-new = No se pudo comprobar el certificado de { $host }: { $why }. Si puedes, compara su huella digital con la que te dé el propietario del servidor, y confía en él solo si coinciden.

folders-far-connect-cert-changed = El certificado de { $host } no es el que aceptaste antes. Puede que su propietario lo haya renovado, o que alguien se esté interponiendo. Confía en el nuevo solo si su propietario confirma esta huella digital.

folders-far-connect-connecting = Conectando

folders-far-connect-trust = Confiar y conectar

folders-far-connect-go = Conectar

folders-far-phones = Los teléfonos aparecen solos en la barra lateral cuando están conectados y desbloqueados.

