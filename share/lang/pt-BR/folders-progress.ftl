

folders-progress-pace-known =
    { $files ->
        [yes] { $done } de { $of } arquivos, { $rate } por segundo, { $left }
       *[no] { $rate } por segundo, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $done } de { $of } arquivos, calculando quanto tempo falta…
       *[no] calculando quanto tempo falta…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } bytes

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = faltam alguns segundos

folders-progress-left-seconds =
    { $n ->
        [one] falta cerca de 1 segundo
       *[other] faltam cerca de { $n } segundos
        [many] faltam cerca de { $n } segundos
    }

folders-progress-left-minute = falta cerca de um minuto

folders-progress-left-minutes =
    { $n ->
        [one] falta cerca de 1 minuto
       *[other] faltam cerca de { $n } minutos
        [many] faltam cerca de { $n } minutos
    }

folders-progress-left-hours =
    { $n ->
        [one] falta cerca de 1 hora
       *[other] faltam cerca de { $n } horas
        [many] faltam cerca de { $n } horas
    }

folders-progress-more =
    { $n ->
        [one] mais 1 trabalho está em andamento
       *[other] mais { $n } trabalhos estão em andamento
        [many] mais { $n } trabalhos estão em andamento
    }

folders-progress-heavy =
    { $verb ->
        [moving] Mover usa muito o disco, então outros programas podem ficar mais lentos até terminar.
       *[copying] Copiar usa muito o disco, então outros programas podem ficar mais lentos até terminar.
    }

folders-progress-waiting = esperando uma resposta

folders-progress-paused = pausado

folders-progress-carry-on = Continuar

folders-progress-pause = Pausar

folders-progress-stop = Parar

folders-progress-mark-newer-larger = mais novo, maior

folders-progress-mark-newer = mais novo

folders-progress-mark-larger = maior

folders-progress-changed = alterado { $when }

folders-progress-taken =
    { $kind ->
        [folder] Já existe ali uma pasta chamada { $name }
        [file] Já existe ali um arquivo chamado { $name }
       *[unknown] { $name } já existe ali
    }

folders-progress-coming = O que está sendo trazido

folders-progress-there = O que já está lá

folders-progress-skip = Ignorar

folders-progress-keep-both = Manter os dois

folders-progress-replace = Substituir

folders-progress-replace-folder-spoken = Substituir: a pasta que já está lá vai para a Lixeira

folders-progress-merge = Mesclar

folders-progress-all-folders = A mesma resposta para toda pasta que já estiver lá.

folders-progress-all-files = A mesma resposta para todo arquivo que já estiver lá.

folders-progress-same-for-rest = Fazer o mesmo com o resto

