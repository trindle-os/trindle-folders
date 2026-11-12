
# machine translated (Spanish), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Title of the question window asked before the Trash is emptied.
folders-view-empty-trash-title = ¿Vaciar la papelera?

# SAFETY: needs a human review before it ships
# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
folders-view-empty-trash-body =
    { $n ->
        [one] El elemento que contiene se eliminará para siempre. Esto no se puede deshacer.
       *[other] Los { $n } elementos que contiene se eliminarán para siempre. Esto no se puede deshacer.
    }

# SAFETY: needs a human review before it ships
# Red button of the question asked before the Trash is emptied: empties it at once.
folders-view-empty-trash = Vaciar la papelera

# SAFETY: needs a human review before it ships
# Title of the question window asked before items are deleted for good; n is how many.
folders-view-delete-title =
    { $n ->
        [one] ¿Eliminar este elemento para siempre?
        [many] ¿Eliminar estos { $n } elementos para siempre?
       *[other] ¿Eliminar estos { $n } elementos para siempre?
    }

# SAFETY: needs a human review before it ships
# Text of the question asked before items are deleted for good.
folders-view-delete-body = Esto no se puede deshacer.

# SAFETY: needs a human review before it ships
# Red button of the question asked before items are deleted for good: deletes them at once.
folders-view-delete-permanently = Eliminar para siempre

# SAFETY: needs a human review before it ships
# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
folders-view-keep-them = Conservarlos

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = ¿Adónde deben ir?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Dejarlos donde están




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = Iconos

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = Lista

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = Compacta

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = Elegir las columnas

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = Nombre

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = Ordenar por

# Name read out for the toolbar's Group by strip.
folders-view-group-by = Agrupar por





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Ocultar el panel de vista previa (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Mostrar el panel de vista previa (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Panel de vista previa

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Pestañas

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Pestañas del panel izquierdo

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Pestañas del panel derecho

# Tip of the button that opens a new tab.
folders-view-tab-new = Pestaña nueva (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = Cerrar pestaña (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = Inicio

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Aún no hay nada fijado. Haz clic derecho en una carpeta para fijarla.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = Fijados

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Lugares

# Small heading in the side bar over the drives plugged in.
folders-view-drives = Unidades

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Recursos compartidos

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = Cerrar { $name }



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Conectar con un servidor

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = Enlazar los dos paneles. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = Enlazar los dos paneles

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] Copiar al panel izquierdo
       *[other] Copiar al panel derecho
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] Mover al panel izquierdo
       *[other] Mover al panel derecho
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Mostrar esta carpeta en el panel izquierdo
       *[other] Mostrar esta carpeta en el panel derecho
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Intercambiar los paneles

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Seleccionar lo que se diferencia del panel izquierdo
       *[other] Seleccionar lo que se diferencia del panel derecho
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] Comparar con el panel izquierdo
       *[other] Comparar con el panel derecho
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Escribe una carpeta, un archivo o la dirección de un servidor

# Name read out for the path above a pane's list.
folders-view-where = Ubicación

# Tip of the pane's Back button.
folders-view-back = Atrás (Alt+Izquierda)

# Tip of the pane's Forward button.
folders-view-forward = Adelante (Alt+Derecha)

# Tip of the pane's button that goes up one folder.
folders-view-up = Subir una carpeta (Alt+Arriba)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Filtra, o pulsa Intro para buscar en esta carpeta y sus subcarpetas

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Buscar nombres, incluidas las subcarpetas

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = Buscar dentro de los archivos, incluidas las subcarpetas


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Tipo

# Name read out for the list that picks which size to show.
folders-view-find-size = Tamaño

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Última modificación

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = Nombre nuevo para { $was }

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } libres

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } seleccionados, calculando el tamaño…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } seleccionados, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Mostrar los archivos de los que trata esto

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Intentar de nuevo

# SAFETY: needs a human review before it ships
# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
folders-view-cancel = Cancelar

# Line on a pane whose filter matches nothing.
folders-view-none-matches = Nada de aquí coincide con el filtro

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Mostrar todo

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = No has abierto nada recientemente

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Ir a Inicio

# Line on the Trash page when it is empty.
folders-view-none-trash = La papelera está vacía

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = No hay unidades que mostrar

# Line on a search page that found nothing.
folders-view-none-found = No se encontró nada

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Borrar la búsqueda

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = Este archivo comprimido está vacío

# Button on an empty archive page: goes up one folder.
folders-view-go-up = Subir una carpeta

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = Esta carpeta está vacía

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = Carpeta nueva

# Button on an empty folder's page that makes a new document.
folders-view-new-document = Documento nuevo

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] 1 elemento
        [many] { $n } elementos
       *[other] { $n } elementos
    }


# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Sin conexión

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = Volver a conectar { $name }

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } no está conectado.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Volver a conectar todos

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] Un recurso compartido no está conectado.
       *[other] { $n } recursos compartidos no están conectados.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = Olvidar { $name }

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = Dos paneles

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = Mostrar dos paneles lado a lado (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Mostrar un panel (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = Ver

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = Nuevo

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = Más opciones

# Row of the More menu that undoes the last change.
folders-view-more-undo = Deshacer

# Row of the More menu that does again what was undone.
folders-view-more-redo = Rehacer

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Ordenar al revés

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Mostrar archivos ocultos (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Ocultar archivos ocultos (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Archivos ocultos

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Equipos cercanos

# Tip of the Search button in the top row while the path shows.
folders-view-search = Buscar (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = Cerrar la búsqueda (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = No se puede buscar en esta página

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Buscar

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = Nombre en la barra lateral para { $was }
