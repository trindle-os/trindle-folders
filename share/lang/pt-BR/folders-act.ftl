
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Recortar, copiar e colar não funcionam com arquivos de um servidor. Use os botões de copiar e mover entre os painéis.

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = Você não pode alterar nada dentro de um arquivo compactado. Extraia primeiro o que precisar.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = Esses itens foram recortados em outro aplicativo, então o Folders os copiou.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = Isso já está sendo copiado ou movido.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = Isso ainda está sendo copiado para cá.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10.000 itens ou mais

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] 1 item
       *[other] { $n } itens
        [many] { $n } itens
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] Não foi possível { $verb } um item: { $names }.
       *[other] Não foi possível { $verb } { $n } itens: { $names }.
        [many] Não foi possível { $verb } { $n } itens: { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } e mais { $more }

# Words on a job card while things are copied: what, and where to.
folders-act-copying = Copiando { $what } para { $place }

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = { $what } copiado(s) para { $place }.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = copiar

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = Desfazer a cópia de { $what }

# Words on a job card while things are moved: what, and where to.
folders-act-moving = Movendo { $what } para { $place }

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = { $what } movido(s) para { $place }.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = mover

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = Desfazer a movimentação de { $what }

# Words on a job card while things are put in the Trash.
folders-act-trashing = Pondo { $what } na Lixeira

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = { $what } posto(s) na Lixeira.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = pôr na Lixeira

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = Desfazer pôr { $what } na Lixeira

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = Excluindo { $what }

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = { $what } excluído(s) permanentemente.

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = excluir

# Words on a job card while things are put back from the Trash.
folders-act-restoring = Recolocando { $what }

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = { $what } recolocado(s).

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = Desfazer recolocar { $what }

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = recolocar

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Desfazendo

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Desfeito.

# Words on a job card while things are packed into an archive.
folders-act-compressing = Compactando { $what }

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = { $file } criado em { $place }.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = compactar

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = Desfazer a compactação de { $what }

# Words on a job card while an archive is unpacked.
folders-act-extracting = Extraindo { $what }

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = { $what } extraído(s) para { $place }.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = extrair

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = Desfazer a extração de { $what }

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = Apagando { $what } sem volta

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = { $what } apagado(s) sem volta.

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = apagar sem volta

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = Tirando { $what }

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = { $what } extraído(s) do arquivo compactado para { $place }.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = tirar

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = Desfazer tirar { $what }

# Words on a job card while pictures are turned.
folders-act-turning = Girando { $what }

# Words under the file lines when pictures have been turned.
folders-act-turned = { $what } girado(s).

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = girar

# Words on a job card while pictures are resized.
folders-act-resizing = Redimensionando { $what }

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = Foi feita uma cópia redimensionada de { $what }.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = redimensionar

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = Desfazer o redimensionamento de { $what }

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = Nova pasta

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = Novo documento

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = O Folders não conseguiu criar { $base }: { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = Digite um nome.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = Um nome não pode ter uma barra.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = O sistema usa esse nome. Escolha outro.

# Red words under a name being typed when it is too long.
folders-act-name-long = Esse nome é longo demais.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = O Folders não conseguiu iniciar este trabalho.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = Refazer { $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = Desfazer { $what }

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = O trabalho foi parado antes de terminar.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [one] O Folders ignorou um item.
       *[other] O Folders ignorou { $n } itens.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Selecione algo primeiro.

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = Isso já está na Lixeira.

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = Arquivos em um servidor não podem ir para a Lixeira. Use Excluir permanentemente.

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Selecione algo para copiar ou mover primeiro.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = Você só pode extrair itens de um arquivo compactado para uma pasta.

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = Você não pode copiar itens para a Lixeira.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = O outro painel não está mostrando uma pasta.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Selecione algo para copiar primeiro.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = { $what } recortado(s).

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = { $what } copiado(s).

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = Você só pode colar em uma pasta.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = Não há arquivos na área de transferência.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = Nada foi recortado nem copiado.

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = Você não pode excluir locais de rede por aqui.

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = A Lixeira já está vazia.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Selecione algo na Lixeira primeiro.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = Criando { $name }

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = Você só pode criar uma nova pasta ou um novo documento dentro de uma pasta.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = Desfazer a criação de { $name }

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = O Folders não conseguiu manter as cores das pastas nos itens que foram movidos.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = Não há nada para refazer.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = Não há nada para desfazer.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = Refazendo { $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = { $what } refeito(s).

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = refazer

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = Desfazendo { $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = { $what } desfeito(s).

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Início

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Lixeira

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = Você só pode soltar itens em uma pasta ou na Lixeira.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] 1 atalho
       *[other] { $n } atalhos
        [many] { $n } atalhos
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = { $what } criado(s).

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] Não foi possível criar um atalho: { $names }.
       *[other] Não foi possível criar { $n } atalhos: { $names }.
        [many] Não foi possível criar { $n } atalhos: { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = Há uma pergunta esperando sua resposta. Responda a ela, ou pare o trabalho, antes de fechar o Folders.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = O Folders não pode fechar enquanto um trabalho está em andamento. Espere terminar, ou pare o trabalho, e depois feche o Folders.
