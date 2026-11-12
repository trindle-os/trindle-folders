
# machine translated (French), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done } fichiers sur { $of }, { $rate } par seconde, { $left }
       *[no] { $rate } par seconde, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } fichiers sur { $of }, calcul de la durée…
       *[no] calcul de la durée…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing } : { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } octets

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } Ko

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } Mo

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } Go

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = quelques secondes restantes

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] environ 1 seconde restante
        [many] environ { $n } secondes restantes
       *[other] environ { $n } secondes restantes
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = environ une minute restante

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] environ 1 minute restante
        [many] environ { $n } minutes restantes
       *[other] environ { $n } minutes restantes
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] environ 1 heure restante
        [many] environ { $n } heures restantes
       *[other] environ { $n } heures restantes
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] 1 autre tâche est en cours
       *[other] { $n } autres tâches sont en cours
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Le déplacement occupe le disque, donc les autres programmes peuvent être plus lents jusqu’à ce qu’il soit terminé.
       *[copying] La copie occupe le disque, donc les autres programmes peuvent être plus lents jusqu’à ce qu’elle soit terminée.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = en attente d’une réponse

# End of a piece of work's line while it is paused.
folders-progress-paused = en pause

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Reprendre

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Pause

# SAFETY: needs a human review before it ships
# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = Arrêter

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = plus récent et plus gros

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = plus récent

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = plus gros

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = modifié { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] Un dossier nommé { $name } existe déjà
        [file] Un fichier nommé { $name } existe déjà
       *[unknown] { $name } existe déjà
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = Le nouveau

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = Celui qui est déjà là

# SAFETY: needs a human review before it ships
# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Ignorer

# SAFETY: needs a human review before it ships
# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = Garder les deux

# SAFETY: needs a human review before it ships
# Word on the button in the name taken question that replaces the one already there.
folders-progress-replace = Remplacer

# SAFETY: needs a human review before it ships
# Screen reader name of the Replace button for a folder in the name taken question.
folders-progress-replace-folder-spoken = Remplacer, et déplacer à la corbeille le dossier déjà présent

# SAFETY: needs a human review before it ships
# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = Fusionner

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = Utiliser cette réponse pour chaque dossier déjà présent.

# Line over the switch in the name taken question for files.
folders-progress-all-files = Utiliser cette réponse pour chaque fichier déjà présent.

# SAFETY: needs a human review before it ships
# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = Faire de même pour le reste
