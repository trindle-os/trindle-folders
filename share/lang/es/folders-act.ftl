
# machine translated (Spanish), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Cortar, copiar y pegar no funcionan con archivos de un servidor. Usa en su lugar los botones de copiar y mover entre los paneles.

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = No se puede cambiar nada dentro de un archivo comprimido. Extrae primero lo que necesites.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = Esto se cortó en otra aplicación, así que Folders lo copió en su lugar.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = Esto ya se está copiando o moviendo.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = Esto aún se está copiando aquí.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10 000 elementos o más

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] 1 elemento
       *[other] { $n } elementos
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] No se pudo { $verb } un elemento: { $names }.
       *[other] No se pudieron { $verb } { $n } elementos: { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } y { $more } más

# Words on a job card while things are copied: what, and where to.
folders-act-copying = Copiando { $what } a { $place }

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = Se copió { $what } a { $place }.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = copiar

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = Deshacer la copia de { $what }

# Words on a job card while things are moved: what, and where to.
folders-act-moving = Moviendo { $what } a { $place }

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = Se movió { $what } a { $place }.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = mover

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = Deshacer el movimiento de { $what }

# Words on a job card while things are put in the Trash.
folders-act-trashing = Enviando { $what } a la Papelera

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = Se envió { $what } a la Papelera.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = enviar a la Papelera

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = Deshacer el envío de { $what } a la Papelera

# Words on a job card while things are deleted for good.
folders-act-deleting = Eliminando { $what }

# SAFETY: needs a human review before it ships
# Words under the file lines when things have been deleted for good.
folders-act-deleted = Se eliminó { $what } de forma permanente.

# The word for what was tried, put in 'could not be ...' when deleting failed.
folders-act-verb-deleted = eliminar

# Words on a job card while things are put back from the Trash.
folders-act-restoring = Restaurando { $what }

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = Se restauró { $what }.

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = Deshacer la restauración de { $what }

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = restaurar

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Deshaciendo

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Deshecho.

# Words on a job card while things are packed into an archive.
folders-act-compressing = Comprimiendo { $what }

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = Se creó { $file } en { $place }.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = comprimir

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = Deshacer la compresión de { $what }

# Words on a job card while an archive is unpacked.
folders-act-extracting = Extrayendo { $what }

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = Se extrajo { $what } en { $place }.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = extraer

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = Deshacer la extracción de { $what }

# Words on a job card while files are wiped securely.
folders-act-wiping = Borrando de forma segura { $what }

# Words under the file lines when files have been wiped securely.
folders-act-wiped = Se borró de forma segura { $what }.

# The word for what was tried, put in 'could not be ...' when wiping failed.
folders-act-verb-wiped = borrar de forma segura

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = Sacando { $what }

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = Se sacó { $what } del archivo comprimido a { $place }.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = sacar

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = Deshacer la salida de { $what }

# Words on a job card while pictures are turned.
folders-act-turning = Girando { $what }

# Words under the file lines when pictures have been turned.
folders-act-turned = Se giró { $what }.

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = girar

# Words on a job card while pictures are resized.
folders-act-resizing = Cambiando el tamaño de { $what }

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = Se creó una copia redimensionada de { $what }.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = redimensionar

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = Deshacer el cambio de tamaño de { $what }

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = Carpeta nueva

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = Documento nuevo

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders no pudo crear { $base }: { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = Escribe un nombre.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = El nombre no puede contener una barra.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = El sistema usa ese nombre. Elige otro.

# Red words under a name being typed when it is too long.
folders-act-name-long = Ese nombre es demasiado largo.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders no pudo iniciar este trabajo.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = Rehacer { $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = Deshacer { $what }

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = El trabajo se detuvo antes de terminar.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [one] Folders omitió un elemento.
       *[other] Folders omitió { $n } elementos.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Selecciona algo primero.

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = Eso ya está en la Papelera.

# Words under the file lines when a server's files are put in the Trash.
folders-act-far-trash = Los archivos de un servidor no pueden ir a la Papelera. Usa Eliminar permanentemente.

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Selecciona primero algo que copiar o mover.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = Solo se puede extraer de un archivo comprimido hacia una carpeta.

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = No puedes copiar cosas a la Papelera.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = El otro panel no muestra una carpeta.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Selecciona primero algo que copiar.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = Se cortó { $what }.

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = Se copió { $what }.

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = Solo se puede pegar en una carpeta.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = No hay archivos en el portapapeles.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = No se ha cortado ni copiado nada.

# Words under the file lines when Delete is used on the Network list.
folders-act-network-delete = Los lugares de red no se pueden eliminar desde aquí.

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = La Papelera ya está vacía.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Selecciona primero algo en la Papelera.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = Creando { $name }

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = Solo puedes crear una carpeta nueva o un documento nuevo dentro de una carpeta.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = Deshacer la creación de { $name }

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders no pudo mantener los colores de carpeta en los elementos que movió.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = No hay nada que rehacer.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = No hay nada que deshacer.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = Rehaciendo { $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = Se rehízo { $what }.

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = rehacer

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = Deshaciendo { $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = Se deshizo { $what }.

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Carpeta personal

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Papelera

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = Solo se puede soltar sobre una carpeta o la Papelera.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] 1 enlace
       *[other] { $n } enlaces
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = Se creó { $what }.

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] No se pudo crear un enlace: { $names }.
       *[other] No se pudieron crear { $n } enlaces: { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = Hay una pregunta esperando tu respuesta. Respóndela o detén el trabajo antes de cerrar Folders.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = Folders no se puede cerrar mientras haya un trabajo en curso. Espera a que termine, o deténlo, y luego cierra Folders.
