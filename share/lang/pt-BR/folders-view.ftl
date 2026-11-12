# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = Esvaziar a Lixeira?

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
        [one] O único item que ela contém será excluído permanentemente. Isso não pode ser desfeito.
       *[other] Todos os { $n } itens que ela contém serão excluídos permanentemente. Isso não pode ser desfeito.
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = Esvaziar a Lixeira

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
        [one] Excluir este item permanentemente?
        [many] Excluir estes { $n } itens permanentemente?
       *[other] Excluir estes { $n } itens permanentemente?
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = Isso não pode ser desfeito.

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = Excluir permanentemente

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = Manter

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = Para onde devem ir?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Deixar onde estão




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = Ícones

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = Lista

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = Compacta

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = Escolher as colunas

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = Nome

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = Ordenar por

# Name read out for the toolbar's Group by strip.
folders-view-group-by = Agrupar por





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Ocultar o painel de visualização (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Mostrar o painel de visualização (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Painel de visualização

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Abas

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Abas do painel esquerdo

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Abas do painel direito

# Tip of the button that opens a new tab.
folders-view-tab-new = Nova aba (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = Fechar aba (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = Pasta pessoal

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Nada fixado ainda. Clique com o botão direito em uma pasta para fixá-la.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = Fixados

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Locais

# Small heading in the side bar over the drives plugged in.
folders-view-drives = Unidades

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Compartilhamentos

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = Fechar { $name }



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Conectar a um servidor

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = Vincular os dois painéis. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = Vincular os dois painéis

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] Copiar para o painel esquerdo
       *[other] Copiar para o painel direito
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] Mover para o painel esquerdo
       *[other] Mover para o painel direito
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Mostrar esta pasta no painel esquerdo
       *[other] Mostrar esta pasta no painel direito
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Trocar os painéis

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Selecionar o que difere do painel esquerdo
       *[other] Selecionar o que difere do painel direito
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] Comparar com o painel esquerdo
       *[other] Comparar com o painel direito
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Digite uma pasta, um arquivo ou um endereço de servidor

# Name read out for the path above a pane's list.
folders-view-where = Local

# Tip of the pane's Back button.
folders-view-back = Voltar (Alt+Esquerda)

# Tip of the pane's Forward button.
folders-view-forward = Avançar (Alt+Direita)

# Tip of the pane's button that goes up one folder.
folders-view-up = Subir uma pasta (Alt+Cima)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Filtre, ou pressione Enter para pesquisar nesta pasta e nas subpastas

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Pesquisar nomes, incluindo subpastas

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = Pesquisar dentro dos arquivos, incluindo subpastas


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Tipo

# Name read out for the list that picks which size to show.
folders-view-find-size = Tamanho

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Última alteração

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = Novo nome para { $was }

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } livres

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } selecionados, calculando o tamanho…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } selecionados, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Mostrar os arquivos a que isto se refere

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Tentar de novo

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
folders-view-cancel = Cancelar

# Line on a pane whose filter matches nothing.
folders-view-none-matches = Nada aqui corresponde ao filtro

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Mostrar tudo

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = Você não abriu nada recentemente

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Ir para a Pasta pessoal

# Line on the Trash page when it is empty.
folders-view-none-trash = A Lixeira está vazia

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = Nenhuma unidade para mostrar

# Line on a search page that found nothing.
folders-view-none-found = Nada encontrado

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Limpar a pesquisa

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = Este arquivo compactado está vazio

# Button on an empty archive page: goes up one folder.
folders-view-go-up = Subir uma pasta

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = Esta pasta está vazia

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = Nova pasta

# Button on an empty folder's page that makes a new document.
folders-view-new-document = Novo documento

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] 1 item
        [many] { $n } itens
       *[other] { $n } itens
    }


# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Não conectado

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = Reconectar { $name }

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } não está conectado.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Reconectar tudo


# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] Um compartilhamento não está conectado.
       *[other] { $n } compartilhamentos não estão conectados.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = Esquecer { $name }

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = Dois painéis

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = Mostrar dois painéis lado a lado (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Mostrar um painel (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = Visualizar

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = Novo

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = Mais opções

# Row of the More menu that undoes the last change.
folders-view-more-undo = Desfazer

# Row of the More menu that does again what was undone.
folders-view-more-redo = Refazer

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Ordenar no sentido oposto

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Mostrar arquivos ocultos (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Ocultar arquivos ocultos (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Arquivos ocultos

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Computadores próximos

# Tip of the Search button in the top row while the path shows.
folders-view-search = Pesquisar (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = Fechar a pesquisa (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = Não é possível pesquisar nesta página

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Pesquisar

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = Nome na barra lateral para { $was }
