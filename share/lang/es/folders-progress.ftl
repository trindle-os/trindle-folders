

folders-progress-pace-known =
    { $files ->
        [yes] { $done } de { $of } archivos, { $rate } por segundo, { $left }
       *[no] { $rate } por segundo, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $done } de { $of } archivos, calculando cuánto falta…
       *[no] calculando cuánto falta…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } bytes

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = faltan unos segundos

folders-progress-left-seconds =
    { $n ->
        [one] falta aproximadamente 1 segundo
        [many] faltan aproximadamente { $n } segundos
       *[other] faltan aproximadamente { $n } segundos
    }

folders-progress-left-minute = falta aproximadamente un minuto

folders-progress-left-minutes =
    { $n ->
        [one] falta aproximadamente 1 minuto
        [many] faltan aproximadamente { $n } minutos
       *[other] faltan aproximadamente { $n } minutos
    }

folders-progress-left-hours =
    { $n ->
        [one] falta aproximadamente 1 hora
        [many] faltan aproximadamente { $n } horas
       *[other] faltan aproximadamente { $n } horas
    }

folders-progress-more =
    { $n ->
        [one] Hay 1 tarea más en marcha
        [many] Hay { $n } tareas más en marcha
       *[other] Hay { $n } tareas más en marcha
    }

folders-progress-heavy =
    { $verb ->
        [moving] Mover usa mucho el disco, así que otros programas pueden ir más lentos hasta que termine.
       *[copying] Copiar usa mucho el disco, así que otros programas pueden ir más lentos hasta que termine.
    }

folders-progress-waiting = esperando una respuesta

folders-progress-paused = en pausa

folders-progress-carry-on = Continuar

folders-progress-pause = Pausar

folders-progress-stop = Detener

folders-progress-mark-newer-larger = más reciente, más grande

folders-progress-mark-newer = más reciente

folders-progress-mark-larger = más grande

folders-progress-changed = modificado { $when }

folders-progress-taken =
    { $kind ->
        [folder] Ya hay una carpeta llamada { $name }
        [file] Ya hay un archivo llamado { $name }
       *[unknown] Ya existe { $name }
    }

folders-progress-coming = El que llega

folders-progress-there = El que ya está

folders-progress-skip = Omitir

folders-progress-keep-both = Conservar ambos

folders-progress-replace = Reemplazar

folders-progress-replace-folder-spoken = Reemplazar: la carpeta que ya está pasa a la papelera

folders-progress-merge = Combinar

folders-progress-all-folders = La misma respuesta para todas las carpetas que ya existen.

folders-progress-all-files = La misma respuesta para todos los archivos que ya existen.

folders-progress-same-for-rest = Hacer lo mismo con el resto

