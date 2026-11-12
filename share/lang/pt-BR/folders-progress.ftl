
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done } de { $of } arquivos, { $rate } por segundo, { $left }
       *[no] { $rate } por segundo, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } de { $of } arquivos, calculando quanto tempo falta…
       *[no] calculando quanto tempo falta…
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
folders-progress-left-few = faltam alguns segundos

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] falta cerca de 1 segundo
       *[other] faltam cerca de { $n } segundos
        [many] faltam cerca de { $n } segundos
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = falta cerca de um minuto

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] falta cerca de 1 minuto
       *[other] faltam cerca de { $n } minutos
        [many] faltam cerca de { $n } minutos
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] falta cerca de 1 hora
       *[other] faltam cerca de { $n } horas
        [many] faltam cerca de { $n } horas
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] mais 1 trabalho em andamento
       *[other] mais { $n } trabalhos em andamento
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Mover mantém o disco ocupado, então outros programas podem ficar mais lentos até terminar.
       *[copying] Copiar mantém o disco ocupado, então outros programas podem ficar mais lentos até terminar.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = esperando uma resposta

# End of a piece of work's line while it is paused.
folders-progress-paused = pausado

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Retomar

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Pausar

# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = Parar

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = mais novo e maior

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = mais novo

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = maior

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = alterado { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] Já existe ali uma pasta chamada { $name }
        [file] Já existe ali um arquivo chamado { $name }
       *[unknown] { $name } já existe ali
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = O novo

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = O que já está lá

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Ignorar

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = Manter os dois

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = Substituir

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = Substituir e mover a pasta que já está lá para a Lixeira

# Main button of the name taken question for a folder: join the two folders.
# SAFETY: needs a human review before it ships
folders-progress-merge = Mesclar

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = Usar esta resposta para todas as pastas que já estiverem lá.

# Line over the switch in the name taken question for files.
folders-progress-all-files = Usar esta resposta para todos os arquivos que já estiverem lá.

# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = Fazer o mesmo com o resto

