

folders-progress-pace-known =
    { $files ->
        [yes] { $done } fichiers sur { $of }, { $rate } par seconde, { $left }
       *[no] { $rate } par seconde, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $done } fichiers sur { $of }, calcul de la durée…
       *[no] calcul de la durée…
    }

folders-progress-line = { $doing } : { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } octets

folders-progress-speed-kb = { $n } Ko

folders-progress-speed-mb = { $n } Mo

folders-progress-speed-gb = { $n } Go

folders-progress-left-few = quelques secondes restantes

folders-progress-left-seconds =
    { $n ->
        [one] environ 1 seconde restante
        [many] environ { $n } secondes restantes
       *[other] environ { $n } secondes restantes
    }

folders-progress-left-minute = environ une minute restante

folders-progress-left-minutes =
    { $n ->
        [one] environ 1 minute restante
        [many] environ { $n } minutes restantes
       *[other] environ { $n } minutes restantes
    }

folders-progress-left-hours =
    { $n ->
        [one] environ 1 heure restante
        [many] environ { $n } heures restantes
       *[other] environ { $n } heures restantes
    }

folders-progress-more =
    { $n ->
        [one] 1 autre tâche est en cours
        [many] { $n } autres tâches sont en cours
       *[other] { $n } autres tâches sont en cours
    }

folders-progress-heavy =
    { $verb ->
        [moving] Le déplacement sollicite beaucoup le disque, donc les autres programmes peuvent sembler plus lents jusqu’à la fin.
       *[copying] La copie sollicite beaucoup le disque, donc les autres programmes peuvent sembler plus lents jusqu’à la fin.
    }

folders-progress-waiting = en attente d’une réponse

folders-progress-paused = en pause

folders-progress-carry-on = Reprendre

folders-progress-pause = Pause

folders-progress-stop = Arrêter

folders-progress-mark-newer-larger = plus récent, plus gros

folders-progress-mark-newer = plus récent

folders-progress-mark-larger = plus gros

folders-progress-changed = modifié { $when }

folders-progress-taken =
    { $kind ->
        [folder] Un dossier nommé { $name } existe déjà
        [file] Un fichier nommé { $name } existe déjà
       *[unknown] { $name } existe déjà
    }

folders-progress-coming = Celui qui arrive

folders-progress-there = Celui qui est déjà là

folders-progress-skip = Ignorer

folders-progress-keep-both = Garder les deux

folders-progress-replace = Remplacer

folders-progress-replace-folder-spoken = Remplacer : le dossier déjà là va à la corbeille

folders-progress-merge = Fusionner

folders-progress-all-folders = La même réponse pour chaque dossier déjà présent.

folders-progress-all-files = La même réponse pour chaque fichier déjà présent.

folders-progress-same-for-rest = Faire de même pour le reste
