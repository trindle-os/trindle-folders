

folders-sharing-cannot-read = No se puede leer esta carpeta.

folders-sharing-only-folder = Solo se puede compartir una carpeta en sí, no un enlace a una.

folders-sharing-only-yours = Solo se pueden compartir las carpetas que son tuyas. Esta es de otra persona.

folders-sharing-your-home = tu carpeta personal

folders-sharing-this-folder = esta carpeta

folders-sharing-and = { $first } y { $second }

folders-sharing-share = Compartir esta carpeta

folders-sharing-shared = Compartida en la red doméstica.

folders-sharing-not-shared = No compartida.

folders-sharing-share-tip = Comparte esta carpeta con otros equipos de la red doméstica

folders-sharing-name = Nombre

folders-sharing-name-tip = El nombre que ven los otros equipos

folders-sharing-write = Los demás pueden cambiar archivos

folders-sharing-write-sub = Pueden añadir, cambiar y eliminar archivos en ella.

folders-sharing-guests = Dejar entrar a cualquiera

folders-sharing-guests-sub = No se pide contraseña.

folders-sharing-guests-tip = Dejar entrar a cualquiera, sin contraseña

folders-sharing-explain =
    { $fetch ->
        [yes] Al compartirla, aparece en Red en los demás equipos de casa, también en los de Windows. La primera vez, Trindle descarga Samba, el programa que comparte carpetas, y tarda un minuto.
       *[no] Al compartirla, aparece en Red en los demás equipos de casa, también en los de Windows.
    }

folders-sharing-find-it =
    { $host_known ->
        [yes] Los demás equipos la encuentran en Red, como { $name } en { $host }.
       *[no] Los demás equipos la encuentran en Red, como { $name }.
    }

folders-sharing-cannot-reach = Los demás equipos todavía no pueden llegar a ella: Permitir que otros equipos se conecten está desactivado en Ajustes, Red.

folders-sharing-let-connect = Permitir que otros equipos se conecten

folders-sharing-visitors =
    { $n ->
        [one] Los visitantes sin contraseña todavía no pueden abrirla: { $names } está cerrada a otras cuentas. Dejarlos pasar solo abre el camino a esta carpeta; no pueden ver qué más hay.
        [many] Los visitantes sin contraseña todavía no pueden abrirla: { $names } están cerradas a otras cuentas. Dejarlos pasar solo abre el camino a esta carpeta; no pueden ver qué más hay.
       *[other] Los visitantes sin contraseña todavía no pueden abrirla: { $names } están cerradas a otras cuentas. Dejarlos pasar solo abre el camino a esta carpeta; no pueden ver qué más hay.
    }

folders-sharing-let-through = Dejar pasar a los visitantes a esta carpeta

folders-sharing-sign-in-set = Los demás inician sesión como { $user }, con la contraseña definida para compartir.

folders-sharing-change-password = Cambiar la contraseña que usan los demás equipos

folders-sharing-set-password = Los demás inician sesión como { $user }. Define la contraseña que usan los demás equipos para abrir tus carpetas compartidas, de al menos { $n } letras. Se guarda aparte de tu contraseña de inicio de sesión.

folders-sharing-password = Contraseña

folders-sharing-password-tip = Contraseña para compartir

folders-sharing-keep-password = Conservar esta contraseña

folders-sharing-card-title = Compartir { $name } en la red

folders-sharing-name-taken = Otra de tus carpetas compartidas ya tiene ese nombre. Elige otro.

folders-sharing-stopping = Dejando de compartir…

folders-sharing-password-short = La contraseña necesita al menos { $n } letras.

folders-sharing-keeping-password = Conservando la contraseña; primero se pide la contraseña del administrador.

folders-sharing-letting-connect = Permitiendo que otros equipos se conecten; primero se pide la contraseña del administrador.

folders-sharing-fetching = Descargando Samba, el programa que comparte carpetas, y configurándolo. Esto ocurre solo la primera vez, y primero se pide la contraseña del administrador.

folders-sharing-setting-up = Configurando el uso compartido. Esto ocurre solo la primera vez, y primero se pide la contraseña del administrador.

folders-sharing-changing = Cambiando lo compartido…

folders-sharing-sharing-it = Compartiéndola…

