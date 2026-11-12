# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = no escribiste la contraseña de administrador
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = no se aceptó la contraseña de administrador
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = tu cuenta no tiene permiso para hacerlo, ni siquiera con la contraseña de administrador
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = no había forma de pedirte la contraseña de administrador
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = no se aceptó la contraseña de administrador, o tu cuenta no tiene permiso para hacerlo
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = No escribiste la contraseña de administrador
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = No se aceptó la contraseña de administrador
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = Tu cuenta no tiene permiso para hacerlo, ni siquiera con la contraseña de administrador
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = No había forma de pedirte la contraseña de administrador
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = No se aceptó la contraseña de administrador, o tu cuenta no tiene permiso para hacerlo
# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = No se hizo porque no escribiste la contraseña de administrador.
# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = No se hizo porque no se aceptó la contraseña de administrador.
# SAFETY: needs a human review before it ships
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = No se hizo. Tu cuenta no tiene permiso para hacerlo, ni siquiera con la contraseña de administrador. Cierra la sesión, vuelve a entrar e inténtalo de nuevo.
# SAFETY: needs a human review before it ships
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = No se hizo. No había forma de pedirte la contraseña de administrador. Cierra la sesión, vuelve a entrar e inténtalo de nuevo.
# SAFETY: needs a human review before it ships
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = No se hizo. No se aceptó la contraseña de administrador, o tu cuenta no tiene permiso para hacerlo. Si no te la pidieron, cierra la sesión, vuelve a entrar e inténtalo de nuevo.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Eso no funcionó y no se indicó el motivo.
# Red words when the keyring will not open without the person's password.
common-keyring-locked = El llavero está bloqueado. Desbloquéalo, o cierra la sesión y vuelve a entrar, e inténtalo de nuevo.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = No hay ningún llavero en este equipo donde guardarlo.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Folders no pudo conectar con el llavero a tiempo.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Folders dejó de esperar al llavero.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Folders no pudo acceder al llavero ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Folders sigue esperando al llavero, así que de momento no se ha eliminado: { $what }.
       *[other] Folders sigue esperando al llavero, así que de momento no se ha guardado: { $what }.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] Tras una demora, se eliminó del llavero: { $what }.
       *[other] Tras una demora, se guardó en el llavero: { $what }.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Folders dejó de esperar al llavero, así que no se eliminó: { $what }.
       *[other] Folders dejó de esperar al llavero, así que no se guardó: { $what }.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Folders no pudo eliminar del llavero: { $what }. { $why }
       *[other] Folders no pudo guardar en el llavero: { $what }. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Más
common-more-spoken = Abrir la Ayuda sobre esto
# SAFETY: needs a human review before it ships
# Red words when the password box was closed with no password. Said whole, as a sentence.
folders-volumes-no-password = No has escrito ninguna contraseña, así que no se cambió nada.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, así que no se cambió nada.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = Algo la sigue usando. Cierra las ventanas o programas que muestren sus archivos y vuelve a intentarlo.
# SAFETY: needs a human review before it ships
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = No hay espacio suficiente para lo que contiene con ese tamaño.
# SAFETY: needs a human review before it ships
# Heading over the password box, and the tip shown in the box while it is empty.
folders-prompt-password = Contraseña
# SAFETY: needs a human review before it ships
# Main button at the bottom of the password window that checks the password.
folders-prompt-unlock = Desbloquear
# SAFETY: needs a human review before it ships
# Button at the bottom of the password window that closes it without unlocking.
folders-prompt-cancel = Cancelar
# SAFETY: needs a human review before it ships
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = Escribe primero la contraseña.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Comprobando…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = mayor que 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = mayor que tu pantalla
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = más de 20 MB (200 MB si es animado)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = más de 20 MB
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = más de 200 MB
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } o { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Bloqueada: { $name }

# Name given to a drive that has none.
folders-volumes-drive = Unidad

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = Esa frase de contraseña no desbloqueó la unidad.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = Ya está abierta.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = Necesitas una frase de contraseña para abrirla.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = Frase de contraseña de { $name }

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Desbloquear unidad

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders no pudo abrir la ventana de la frase de contraseña.
