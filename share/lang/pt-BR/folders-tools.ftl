# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Choice in the Wipe window: write over each file one time.
folders-tools-pass-once = Uma vez

# Choice in the Wipe window: write over each file three times.
folders-tools-pass-three = 3 vezes

# Choice in the Wipe window: write over each file seven times.
folders-tools-pass-seven = 7 vezes

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = Como estão

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = Tudo em letras minúsculas

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = Tudo maiúsculo

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = Maiúscula em cada palavra

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = Maiúscula na primeira palavra

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = Como está

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = Tudo em letras minúsculas

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = Tudo maiúsculo

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = Nova extensão

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = Sem extensão

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = Arquivo

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = não restaria nenhum nome

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = isso não é permitido como nome

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = esse nome é longo demais

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = dois itens ficariam com este nome

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = outro item já tem este nome

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = o nome antigo tem caracteres que o Folders não consegue ler

# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
# SAFETY: needs a human review before it ships
folders-tools-caveat-flash = Isto é uma unidade flash ou SSD, então cópias antigas podem continuar em partes dela que o Folders não consegue alcançar.

# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
# SAFETY: needs a human review before it ships
folders-tools-caveat-copy-on-write = O sistema de arquivos { $fs } salva as alterações em outro lugar, então os dados antigos ainda podem estar lá.

# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
# SAFETY: needs a human review before it ships
folders-tools-caveat-network = Este é um compartilhamento { $fs } em outro computador, que pode manter as próprias cópias.

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = Só à esquerda

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = Só à direita

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = Igual

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
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

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = Aqui

# Red notice when Wipe is chosen on things that are not files.
# SAFETY: needs a human review before it ships
folders-tools-only-files-wiped = Só arquivos podem ser apagados com segurança.

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = Abra uma pasta em cada painel para compará-las.

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
        [one] Desfazer a renomeação de 1 item
        [many] Desfazer a renomeação de { $n } itens
       *[other] Desfazer a renomeação de { $n } itens
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [one] 1 item renomeado.
        [many] { $n } itens renomeados.
       *[other] { $n } itens renomeados.
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = A renomeação parou no meio do caminho: { $error }.

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = Digite um nome para o arquivo compactado.

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = { $done } verificados

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = { $done } de { $total } lidos

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = Listando arquivos

# Beside the ring while two files are compared.
folders-tools-comparing = Comparando

# Beside the ring while two folders are compared.
folders-tools-looking = Comparando as pastas

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [one] Compactar { $name }
        [many] Compactar { $n } itens
       *[other] Compactar { $n } itens
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = Nome do arquivo compactado

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = Tipo de arquivo compactado

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = O arquivo compactado será salvo em { $place }

# Main button of the Compress window.
folders-tools-compress = Compactar

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = Extrair { $name } para

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = Os arquivos serão colocados em uma nova pasta chamada { $name }.

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [one] Redimensionar { $name }
        [many] Redimensionar { $n } imagens
       *[other] Redimensionar { $n } imagens
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = Novo tamanho

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = O tamanho é o comprimento do lado maior, em pixels. O Folders só reduz as imagens. Cada cópia redimensionada é salva ao lado da original, com o novo tamanho no nome.

# Main button of the Resize window.
folders-tools-resize = Redimensionar

# Title of the Wipe window; { $name } is the one file, { $n } how many.
# SAFETY: needs a human review before it ships
folders-tools-wipe-title =
    { $n ->
        [one] Apagar { $name } com segurança?
        [many] Apagar { $n } arquivos com segurança?
       *[other] Apagar { $n } arquivos com segurança?
    }

# Warning at the top of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-warning = O Folders grava por cima de cada arquivo antes de excluí-lo, então não há como recuperá-lo.

# Name beside the choice of how many times to write over, in the Wipe window.
folders-tools-wipe-write-over = Sobrescrever

# Choice tip in the Wipe window: how many times each file is written over.
folders-tools-wipe-passes-tip = Quantas vezes gravar por cima de cada arquivo

# Red main button of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-button = Apagar com segurança

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = Numerar

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = No início

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = No fim

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = Onde o número fica

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = Primeiro número

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = O que fazer com a extensão

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = Nova extensão, como txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = O Folders não consegue renomear isto: { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = era { $old }

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = continua como está

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [one] Renomear 1 item
        [many] Renomear { $n } itens
       *[other] Renomear { $n } itens
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = Substituir

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = Procurar

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = Substituir por

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = Usar * e ? como curingas

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = Curingas * e ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = Diferenciar maiúsculas de minúsculas

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = Diferenciar maiúsculas

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = Adicionar

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = Antes do nome

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = Depois do nome

# Name of the numbering row in the rename window.
folders-tools-rename-number = Número

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = Maiúsculas

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = Letras maiúsculas nos nomes

# Name over the ending choice in the rename window.
folders-tools-rename-ending = Extensão

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] Em { $place }, alguns nomes não podem ser usados. Veja as linhas marcadas.
       *[no] Em { $place }, { $changes } de { $total } nomes vão mudar.
    }

# Main button of the rename window.
folders-tools-rename-button = Renomear

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = Pasta pessoal

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [one] Duplicados em { $folder }
        [many] Duplicados em { $n } pastas
       *[other] Duplicados em { $n } pastas
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = A busca parou: { $error }.

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = Nenhum duplicado foi encontrado.

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [one] 1 arquivo não pôde ser lido.
        [many] { $n } arquivos não puderam ser lidos.
       *[other] { $n } arquivos não puderam ser lidos.
    }

# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
        [one] 1 conjunto de cópias. As cópias extras estão selecionadas, e movê-las para a Lixeira libera { $size }.
       *[other] { $n } conjuntos de cópias. As cópias extras estão selecionadas, e movê-las para a Lixeira libera { $size }.
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = Conjunto { $set }, { $size }, em { $at }

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 arquivo não pôde ser lido e foi deixado de fora.
        [many] { $n } arquivos não puderam ser lidos e foram deixados de fora.
       *[other] { $n } arquivos não puderam ser lidos e foram deixados de fora.
    }

# Note in the duplicates window when every copy of a set is picked.
folders-tools-dupes-keep-one = Todas as cópias de um conjunto estão selecionadas. Deixe pelo menos uma sem selecionar.

# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = Selecionar as cópias extras

# Main button of the duplicates window: move the picked files to the Trash.
# SAFETY: needs a human review before it ships
folders-tools-dupes-trash = Mover para a Lixeira

# Title of the window that compares two files.
folders-tools-diff-title = Comparar dois arquivos

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = O Folders não conseguiu compará-los: { $error }.

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = Os dois arquivos são exatamente iguais.

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = São iguais até o fim do menor. Um tem { $one } e o outro { $other }.

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = São diferentes a partir do byte { $n }.

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] Os dois arquivos são iguais.
        [one] 1 linha é diferente.
       *[other] { $n } linhas são diferentes.
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = São diferentes demais para comparar linha a linha. Tudo entre a primeira e a última diferença aparece como alterado.

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = Só os primeiros 4 MB de cada arquivo são comparados.

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = Comparar { $left } com { $right }

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = Comparar o conteúdo (mais lento)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = Comparar o que há dentro de cada arquivo

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = Mostrar nomes iguais

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = Listar também os nomes iguais

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = A comparação parou: { $error }.

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = { $left } só à esquerda, { $right } só à direita, { $different } diferentes, { $same } iguais.

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = As duas pastas têm o mesmo conteúdo.

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = Comparar de novo

# Button in the pane compare window; { $n } is how many names are copied to the left folder.
folders-tools-compare-copy-left = Copiar { $n } itens ausentes ou mais novos para a esquerda

# Button in the pane compare window; { $n } is how many names are copied to the right folder.
folders-tools-compare-copy-right = Copiar { $n } itens ausentes ou mais novos para a direita

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = Só arquivos são apagados com segurança, então estas pastas ficarão como estão: { $names }.
