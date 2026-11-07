# machine translated (Brazilian Portuguese), not yet reviewed by a person.

folders-tools-pass-once = Uma vez

folders-tools-pass-three = 3 vezes

folders-tools-pass-seven = 7 vezes

folders-tools-case-keep = Como estão

folders-tools-case-lower = Tudo minúsculo

folders-tools-case-upper = Tudo maiúsculo

folders-tools-case-words = Cada palavra

folders-tools-case-first = Primeira palavra

folders-tools-ending-keep = Como está

folders-tools-ending-lower = Tudo minúsculo

folders-tools-ending-upper = Tudo maiúsculo

folders-tools-ending-new = Uma nova extensão

folders-tools-ending-none = Sem extensão

folders-tools-archive-name = Arquivo

folders-tools-clash-empty = não restaria nenhum nome

folders-tools-clash-bad = isso não é permitido como nome

folders-tools-clash-long = esse nome é longo demais

folders-tools-clash-twice = dois ficariam com este nome

folders-tools-clash-taken = outro item já tem este nome

folders-tools-clash-not-text = o nome antigo não é texto simples

folders-tools-caveat-flash = Este disco é flash, não de rotação, então cópias antigas podem continuar em partes dele que o Trindle não alcança.

folders-tools-caveat-copy-on-write = O sistema de arquivos { $fs } grava as alterações em outro lugar, então os dados antigos podem continuar onde estavam.

folders-tools-caveat-network = Este é um compartilhamento { $fs } em outro computador, que pode manter cópias próprias.

folders-tools-row-only-left = Só à esquerda

folders-tools-row-only-right = Só à direita

folders-tools-row-same = Igual

folders-tools-row-different =
    Diferente: { $why ->
        [kind] um é uma pasta ou um link
        [size] os tamanhos são diferentes
        [date] as datas são diferentes
        [content] o conteúdo é diferente
        [target] os links apontam para lugares diferentes
        [inside] algo dentro é diferente
       *[unread] um dos lados não pôde ser lido
    }{ $newer ->
        [left] , mais recente à esquerda
        [right] , mais recente à direita
       *[none] { "" }
    }

folders-tools-place-here = Aqui

folders-tools-only-files-wiped = Só arquivos podem ser apagados com segurança.

folders-tools-compare-need-folders = Os dois painéis precisam mostrar uma pasta para compará-los.

folders-tools-rename-undo =
    { $n ->
        [one] Desfazer a renomeação de 1 item
        [many] Desfazer a renomeação de { $n } itens
       *[other] Desfazer a renomeação de { $n } itens
    }

folders-tools-rename-done =
    { $n ->
        [one] 1 item renomeado.
        [many] { $n } itens renomeados.
       *[other] { $n } itens renomeados.
    }

folders-tools-rename-stopped = A renomeação parou no meio: { $error }.

folders-tools-name-empty = O nome não pode ficar vazio.

folders-tools-looked-at = { $done } verificados

folders-tools-read-of = { $done } de { $total } lidos

folders-tools-listing-files = Listando arquivos

folders-tools-comparing = Comparando

folders-tools-looking = Verificando

folders-tools-compress-title =
    { $n ->
        [one] Compactar { $name }
        [many] Compactar { $n } itens
       *[other] Compactar { $n } itens
    }

folders-tools-pack-name-tip = Nome do arquivo compactado

folders-tools-pack-kind-tip = Tipo de arquivo compactado

folders-tools-pack-made-in = Criado em { $place }

folders-tools-compress = Compactar

folders-tools-extract-to = Extrair { $name } para

folders-tools-extract-new-folder = Uma nova pasta chamada { $name } é criada lá.

folders-tools-resize-title =
    { $n ->
        [one] Redimensionar { $name }
        [many] Redimensionar { $n } imagens
       *[other] Redimensionar { $n } imagens
    }

folders-tools-resize-size-tip = Novo tamanho

folders-tools-resize-note = Uma porcentagem do tamanho, ou quantos pixels no lado maior; uma imagem nunca é aumentada. Uma cópia é criada ao lado de cada uma, com o novo tamanho no nome; as imagens originais ficam como estão.

folders-tools-resize = Redimensionar

folders-tools-wipe-title =
    { $n ->
        [one] Apagar { $name } com segurança?
        [many] Apagar { $n } arquivos com segurança?
       *[other] Apagar { $n } arquivos com segurança?
    }

folders-tools-wipe-warning = Cada arquivo é sobrescrito antes de ser removido, então não há como recuperá-lo. Isso não pode ser desfeito.

folders-tools-wipe-write-over = Sobrescrever

folders-tools-wipe-passes-tip = Quantas vezes cada arquivo é sobrescrito

folders-tools-wipe-button = Apagar com segurança

folders-tools-rename-number-tip = Numerar

folders-tools-rename-at-start = No início

folders-tools-rename-at-end = No fim

folders-tools-rename-where-tip = Onde o número fica

folders-tools-rename-first-number-tip = Primeiro número

folders-tools-rename-ending-tip = O que acontece com a extensão

folders-tools-rename-new-ending-tip = Nova extensão, como txt

folders-tools-rename-cannot = Não é possível: { $why }

folders-tools-rename-was = era { $old }

folders-tools-rename-stays = continua como está

folders-tools-rename-title =
    { $n ->
        [one] Renomear 1 item
        [many] Renomear { $n } itens
       *[other] Renomear { $n } itens
    }

folders-tools-rename-replace = Substituir

folders-tools-rename-find-tip = Procurar

folders-tools-rename-with-tip = Substituir por

folders-tools-rename-wildcard-tip = Usar * e ? como curingas

folders-tools-rename-wildcard = Curingas * e ?

folders-tools-rename-match-case-tip = Diferenciar maiúsculas de minúsculas

folders-tools-rename-match-case = Diferenciar maiúsculas

folders-tools-rename-add = Adicionar

folders-tools-rename-before-tip = Antes do nome

folders-tools-rename-after-tip = Depois do nome

folders-tools-rename-number = Número

folders-tools-rename-letters = Letras

folders-tools-rename-case-tip = Maiúsculas nos nomes

folders-tools-rename-ending = Extensão

folders-tools-rename-summary =
    { $clash ->
        [yes] Alguns nomes não podem ser usados; veja as linhas marcadas. Em { $place }.
       *[no] { $changes } de { $total } nomes mudam. Em { $place }.
    }

folders-tools-rename-button = Renomear

folders-tools-home = Pasta pessoal

folders-tools-dupes-title =
    { $n ->
        [one] Duplicados em { $folder }
        [many] Duplicados em { $n } pastas
       *[other] Duplicados em { $n } pastas
    }

folders-tools-dupes-stopped = A busca parou: { $error }.

folders-tools-dupes-none = Nenhum duplicado foi encontrado.

folders-tools-dupes-unread =
    { $n ->
        [one] 1 arquivo não pôde ser lido.
        [many] { $n } arquivos não puderam ser lidos.
       *[other] { $n } arquivos não puderam ser lidos.
    }

folders-tools-dupes-sets =
    { $n ->
        [one] 1 conjunto de cópias; mover as extras para a Lixeira libera { $size }. As cópias extras já estão marcadas; Ctrl e um clique marca ou desmarca uma.
        [many] { $n } conjuntos de cópias; mover as extras para a Lixeira libera { $size }. As cópias extras já estão marcadas; Ctrl e um clique marca ou desmarca uma.
       *[other] { $n } conjuntos de cópias; mover as extras para a Lixeira libera { $size }. As cópias extras já estão marcadas; Ctrl e um clique marca ou desmarca uma.
    }

folders-tools-dupes-row = Conjunto { $set }, { $size }, em { $at }

folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 arquivo não pôde ser lido e foi deixado de fora.
        [many] { $n } arquivos não puderam ser lidos e foram deixados de fora.
       *[other] { $n } arquivos não puderam ser lidos e foram deixados de fora.
    }

folders-tools-dupes-keep-one = Todas as cópias de um conjunto estão marcadas; mantenha pelo menos uma.

folders-tools-dupes-pick-extras = Marcar as cópias extras

folders-tools-dupes-trash = Mover para a Lixeira

folders-tools-diff-title = Comparar os dois

folders-tools-diff-failed = Não foi possível compará-los: { $error }.

folders-tools-diff-same-bytes = Os dois são iguais, byte a byte.

folders-tools-diff-same-as-shorter = São iguais até onde o menor vai; um tem { $one } e o outro { $other }.

folders-tools-diff-from-byte = Diferem a partir do byte { $n }.

folders-tools-diff-lines =
    { $n ->
        [0] Os dois são iguais.
        [one] 1 linha é diferente.
        [many] { $n } linhas são diferentes.
       *[other] { $n } linhas são diferentes.
    }

folders-tools-diff-too-different = São diferentes demais para comparar linha a linha, então todas as linhas entre a primeira e a última diferença aparecem como alteradas.

folders-tools-diff-cut = Só os primeiros 4 MB de cada um são comparados.

folders-tools-compare-title = Comparar { $left } com { $right }

folders-tools-compare-bytes = Ler todos os bytes (mais lento)

folders-tools-compare-bytes-tip = Comparar o que há dentro de cada arquivo

folders-tools-compare-show-same = Mostrar o que é igual

folders-tools-compare-show-same-tip = Mostrar também os nomes que são iguais

folders-tools-compare-stopped = A comparação parou: { $error }.

folders-tools-compare-counts = { $left } só à esquerda, { $right } só à direita, { $different } diferentes, { $same } iguais.

folders-tools-compare-all-same = As duas pastas têm o mesmo conteúdo.

folders-tools-compare-again = Comparar de novo

folders-tools-compare-copy-left = Copiar { $n } ausentes ou mais novos para a esquerda

folders-tools-compare-copy-right = Copiar { $n } ausentes ou mais novos para a direita
