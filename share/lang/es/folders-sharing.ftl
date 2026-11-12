
# machine translated (Spanish), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders no puede leer el contenido de esta carpeta.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = Solo puedes compartir una carpeta en sí, no un enlace a una.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = Solo puedes compartir las carpetas que son tuyas. Esta es de otra persona.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = tu carpeta personal

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = esta carpeta

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } y { $second }

# Name of the switch that shares the folder.
folders-sharing-share = Compartir esta carpeta

# One line under the share switch when the folder is shared.
folders-sharing-shared = Compartida en la red doméstica.


# Tip of the share switch.
folders-sharing-share-tip = Compartir en la red doméstica

# Label over the box for the name other computers see.
folders-sharing-name = Nombre

# Tip of the box for the name other computers see.
folders-sharing-name-tip = El nombre que ven los otros equipos

# SAFETY: needs a human review before it ships
# Name of the switch that lets others change files, and its tip.
folders-sharing-write = Dejar que los demás cambien archivos

# One line under the switch that lets others change files.
folders-sharing-write-sub = Pueden añadir, cambiar y eliminar archivos en ella.

# SAFETY: needs a human review before it ships
# Name of the switch that lets anyone in without a password.
folders-sharing-guests = Dejar entrar a cualquiera

# One line under the let in anyone switch.
folders-sharing-guests-sub = No hace falta contraseña.

# SAFETY: needs a human review before it ships
# Tip of the let in anyone switch.
folders-sharing-guests-tip = Dejar entrar a cualquiera sin contraseña

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Una vez compartida, aparece en Red en tus otros equipos de casa, también en los de Windows. La primera vez, Trindle descarga Samba, el programa que comparte carpetas. Tarda un minuto.
       *[no] Una vez compartida, aparece en Red en tus otros equipos de casa, también en los de Windows.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] En los demás equipos, busca en Red { $name } en { $host }.
       *[no] En los demás equipos, busca en Red { $name }.
    }

# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = Los demás equipos todavía no pueden acceder a esta carpeta. En Ajustes, abre Red y activa “Permitir que otros equipos se conecten”.

# Tip of the button that lets other computers connect.
folders-sharing-let-connect = Permitir que otros equipos se conecten

# SAFETY: needs a human review before it ships
# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
folders-sharing-visitors =
    { $n ->
        [one] Los invitados sin contraseña todavía no pueden abrirla, porque { $names } está cerrada a otras cuentas. Si los dejas pasar, podrán llegar a esta carpeta pero no ver nada más por el camino.
       *[other] Los invitados sin contraseña todavía no pueden abrirla, porque { $names } están cerradas a otras cuentas. Si los dejas pasar, podrán llegar a esta carpeta pero no ver nada más por el camino.
    }

# SAFETY: needs a human review before it ships
# Tip of the button that lets visitors through to the folder.
folders-sharing-let-through = Dejar que los invitados accedan a esta carpeta

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
folders-sharing-sign-in-set = Los demás inician sesión como { $user } con la contraseña de uso compartido.

# Tip of the button that changes the sharing password.
folders-sharing-change-password = Cambiar la contraseña de uso compartido

# SAFETY: needs a human review before it ships
# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
folders-sharing-set-password = Los demás inician sesión como { $user }. Define una contraseña de uso compartido de al menos { $n } caracteres. Es independiente de tu contraseña de inicio de sesión.

# Label beside the sharing password box.
folders-sharing-password = Contraseña

# Tip of the sharing password box.
folders-sharing-password-tip = Contraseña para compartir

# Tip of the button that keeps the sharing password.
folders-sharing-keep-password = Guardar contraseña

# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = Compartir { $name } en la red

# Red words when the share's name is used by another share.
folders-sharing-name-taken = Ya tienes una carpeta compartida con ese nombre. Elige otro.

# Calm words while sharing is being stopped.
folders-sharing-stopping = Dejando de compartir…

# Red words when the sharing password is too short; { $n } is the fewest letters.
folders-sharing-password-short = La contraseña necesita al menos { $n } caracteres.

# Calm words while the sharing password is being kept.
folders-sharing-keeping-password = Guardando la contraseña. Primero se te pedirá la contraseña del administrador.

# Calm words while other computers are being let connect.
folders-sharing-letting-connect = Permitiendo que otros equipos se conecten. Primero se te pedirá la contraseña del administrador.

# Calm words while Samba is fetched and set up the first time.
folders-sharing-fetching = Descargando y configurando Samba, el programa que comparte carpetas. Primero se te pedirá la contraseña del administrador. Esto solo ocurre la primera vez.

# Calm words while sharing is set up the first time and Samba is already there.
folders-sharing-setting-up = Configurando el uso compartido. Primero se te pedirá la contraseña del administrador. Esto solo ocurre la primera vez.

# Calm words while an existing share is changed.
folders-sharing-changing = Cambiando lo compartido…

# Calm words while a share is made.
folders-sharing-sharing-it = Compartiendo la carpeta…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = Dale un nombre a la carpeta compartida.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = El nombre de una carpeta compartida no puede tener más de { $most } caracteres.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = El nombre de una carpeta compartida no puede contener ninguno de estos: % < > * ? | / \ + = ; : " ni una coma.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = Un nombre que termina en $ ocultaría la carpeta compartida, así que no se puede usar.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = El nombre de una carpeta compartida no puede empezar con un guion.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba ya usa el nombre { $name }. Elige otro.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Compartido

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = Solo puedes compartir las carpetas que te pertenecen.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = Ese nombre ya es el nombre de una cuenta en este equipo, así que no se puede usar para una carpeta compartida. Elige otro.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = La configuración de Samba de este equipo no permite recursos compartidos sin contraseña.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = Este equipo no puede compartir más carpetas.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = Esa carpeta ya no está.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba no dio ningún motivo.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Mensaje de Samba: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } pertenece a otra persona, así que desde aquí no puedes dar acceso a invitados.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders no pudo dar acceso a invitados a { $path }: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle no pudo descargar Samba porque este equipo parece no tener conexión. Conéctate a internet e inténtalo de nuevo.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Trindle no pudo instalar Samba mientras se instala otra cosa. Inténtalo de nuevo en unos minutos.
