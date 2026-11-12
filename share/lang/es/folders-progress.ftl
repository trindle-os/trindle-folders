
# machine translated (Spanish), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done } de { $of } archivos, { $rate } por segundo, { $left }
       *[no] { $rate } por segundo, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } de { $of } archivos, calculando cuánto falta…
       *[no] calculando cuánto falta…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } bytes

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = faltan unos segundos

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] falta aproximadamente 1 segundo
        [many] faltan aproximadamente { $n } segundos
       *[other] faltan aproximadamente { $n } segundos
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = falta aproximadamente un minuto

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] falta aproximadamente 1 minuto
        [many] faltan aproximadamente { $n } minutos
       *[other] faltan aproximadamente { $n } minutos
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] falta aproximadamente 1 hora
        [many] faltan aproximadamente { $n } horas
       *[other] faltan aproximadamente { $n } horas
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] Hay 1 tarea más en marcha
       *[other] Hay { $n } tareas más en marcha
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Mover usa mucho el disco, así que otros programas pueden ir más lentos hasta que termine.
       *[copying] Copiar usa mucho el disco, así que otros programas pueden ir más lentos hasta que termine.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = esperando una respuesta

# End of a piece of work's line while it is paused.
folders-progress-paused = en pausa

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Reanudar

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Pausar

# SAFETY: needs a human review before it ships
# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = Detener

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = más reciente y más grande

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = más reciente

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = más grande

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = modificado { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] Ya hay una carpeta llamada { $name }
        [file] Ya hay un archivo llamado { $name }
       *[unknown] Ya existe { $name }
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = El nuevo

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = El que ya está

# SAFETY: needs a human review before it ships
# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Omitir

# SAFETY: needs a human review before it ships
# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = Conservar ambos

# SAFETY: needs a human review before it ships
# Word on the button in the name taken question that replaces the one already there.
folders-progress-replace = Reemplazar

# SAFETY: needs a human review before it ships
# Screen reader name of the Replace button for a folder in the name taken question.
folders-progress-replace-folder-spoken = Reemplazar y mover a la papelera la carpeta que ya está

# SAFETY: needs a human review before it ships
# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = Combinar

# SAFETY: needs a human review before it ships
# Line over the switch in the name taken question for folders.
folders-progress-all-folders = Usar esta respuesta con todas las carpetas que ya existen.

# SAFETY: needs a human review before it ships
# Line over the switch in the name taken question for files.
folders-progress-all-files = Usar esta respuesta con todos los archivos que ya existen.

# SAFETY: needs a human review before it ships
# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = Hacer lo mismo con el resto

