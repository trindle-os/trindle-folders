
# machine translated (Spanish), not yet reviewed by a person.

# Choice in the Wipe window: write over each file one time.
folders-tools-pass-once = Una vez

# Choice in the Wipe window: write over each file three times.
folders-tools-pass-three = 3 veces

# Choice in the Wipe window: write over each file seven times.
folders-tools-pass-seven = 7 veces

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = Como están

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = Todo en minúsculas

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = Todo en mayúsculas

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = Poner en mayúscula cada palabra

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = Poner en mayúscula la primera palabra

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = Como está

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = Todo en minúsculas

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = Todo en mayúsculas

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = Extensión nueva

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = Sin extensión

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = Archivo

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = no quedaría ningún nombre

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = no se permite como nombre

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = ese nombre es demasiado largo

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = dos elementos tendrían este nombre

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = otra cosa ya tiene este nombre

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = el nombre antiguo tiene caracteres que Folders no puede leer

# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
folders-tools-caveat-flash = Esto es una unidad flash o un SSD, así que pueden quedar copias antiguas en partes a las que Folders no llega.

# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
folders-tools-caveat-copy-on-write = El sistema de archivos { $fs } guarda los cambios en un lugar nuevo, así que los datos antiguos pueden seguir ahí.

# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
folders-tools-caveat-network = Esto es un recurso { $fs } de otro equipo, que puede guardar sus propias copias.

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = Solo a la izquierda

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = Solo a la derecha

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = Igual

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
folders-tools-row-different =
    Distinto: { $why ->
        [kind] uno es una carpeta o un enlace
        [size] los tamaños son distintos
        [date] las fechas son distintas
        [content] los contenidos son distintos
        [target] los enlaces apuntan a otro sitio
        [inside] algo del interior es distinto
       *[unread] no se pudo leer uno de los lados
    }{ $newer ->
        [left] , más reciente a la izquierda
        [right] , más reciente a la derecha
       *[none] { "" }
    }

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = Aquí

# Red notice when Wipe is chosen on things that are not files.
# SAFETY: needs a human review before it ships
folders-tools-only-files-wiped = Solo se pueden borrar de forma segura los archivos.

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = Abre una carpeta en cada panel para compararlas.

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
        [one] Deshacer el cambio de nombre de 1 elemento
       *[other] Deshacer el cambio de nombre de { $n } elementos
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [one] Se cambió el nombre de 1 elemento.
       *[other] Se cambió el nombre de { $n } elementos.
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = El cambio de nombres se detuvo a medias: { $error }.

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = Escribe un nombre para el archivo comprimido.

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = Revisados { $done }

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = Leídos { $done } de { $total }

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = Listando archivos

# Beside the ring while two files are compared.
folders-tools-comparing = Comparando

# Beside the ring while two folders are compared.
folders-tools-looking = Comparando las carpetas

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [one] Comprimir { $name }
       *[other] Comprimir { $n } elementos
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = Nombre del archivo comprimido

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = Tipo de archivo comprimido

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = El archivo comprimido se guardará en { $place }

# Main button of the Compress window.
folders-tools-compress = Comprimir

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = Extraer { $name } en

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = Los archivos se pondrán en una carpeta nueva llamada { $name }.

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [one] Cambiar el tamaño de { $name }
       *[other] Cambiar el tamaño de { $n } imágenes
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = Tamaño nuevo

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = El tamaño es la longitud del lado más largo en píxeles. Folders solo hace las imágenes más pequeñas. Guarda cada copia redimensionada junto al original, con el nuevo tamaño en el nombre.

# Main button of the Resize window.
folders-tools-resize = Cambiar tamaño

# Title of the Wipe window; { $name } is the one file, { $n } how many.
# SAFETY: needs a human review before it ships
folders-tools-wipe-title =
    { $n ->
        [one] ¿Borrar de forma segura { $name }?
       *[other] ¿Borrar de forma segura { $n } archivos?
    }

# Warning at the top of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-warning = Folders sobrescribe cada archivo antes de eliminarlo, así que no podrás recuperarlo.

# Name beside the choice of how many times to write over, in the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-write-over = Sobrescribir

# Choice tip in the Wipe window: how many times each file is written over.
folders-tools-wipe-passes-tip = Cuántas veces sobrescribir cada archivo

# Red main button of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-button = Borrar de forma segura

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = Numerarlos

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = Al principio

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = Al final

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = Dónde va el número

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = Primer número

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = Qué hacer con la extensión

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = Extensión nueva, como txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders no puede cambiar este nombre: { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = antes era { $old }

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = se queda como está

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [one] Renombrar 1 elemento
       *[other] Renombrar { $n } elementos
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = Reemplazar

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = Buscar

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = Reemplazar con

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = Usar * y ? como comodines

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = Comodines * y ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = Distinguir mayúsculas y minúsculas

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = Distinguir mayúsculas

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = Añadir

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = Antes del nombre

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = Después del nombre

# Name of the numbering row in the rename window.
folders-tools-rename-number = Número

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = Mayúsculas

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = Mayúsculas en los nombres

# Name over the ending choice in the rename window.
folders-tools-rename-ending = Extensión

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] En { $place }, algunos nombres no se pueden usar. Mira las filas marcadas.
       *[no] En { $place }, cambiarán { $changes } de { $total } nombres.
    }

# Main button of the rename window.
folders-tools-rename-button = Cambiar nombre

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = Inicio

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [one] Duplicados en { $folder }
       *[other] Duplicados en { $n } carpetas
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = La búsqueda se detuvo: { $error }.

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = No se encontraron duplicados.

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [one] No se pudo leer 1 archivo.
       *[other] No se pudieron leer { $n } archivos.
    }

# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
        [one] 1 grupo de copias. Las copias sobrantes están seleccionadas, y al moverlas a la papelera se liberan { $size }.
       *[other] { $n } grupos de copias. Las copias sobrantes están seleccionadas, y al moverlas a la papelera se liberan { $size }.
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = Grupo { $set }, { $size }, en { $at }

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [one] No se pudo leer 1 archivo y se dejó fuera.
       *[other] No se pudieron leer { $n } archivos y se dejaron fuera.
    }

# Note in the duplicates window when every copy of a set is picked.
# SAFETY: needs a human review before it ships
folders-tools-dupes-keep-one = Todas las copias de un grupo están seleccionadas. Deja al menos una sin seleccionar.

# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = Seleccionar las copias sobrantes

# Main button of the duplicates window: move the picked files to the Trash.
# SAFETY: needs a human review before it ships
folders-tools-dupes-trash = Mover a la papelera

# Title of the window that compares two files.
folders-tools-diff-title = Comparar dos archivos

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders no pudo compararlos: { $error }.

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = Los dos archivos son exactamente iguales.

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = Son iguales hasta el final del más corto. Uno ocupa { $one } y el otro { $other }.

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = Son distintos a partir del byte { $n }.

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] Los dos archivos son iguales.
        [one] Difiere 1 línea.
       *[other] Difieren { $n } líneas.
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = Son demasiado distintos para compararlos línea a línea. Todo lo que hay entre la primera y la última diferencia aparece como cambiado.

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = Solo se comparan los primeros 4 MB de cada archivo.

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = Comparar { $left } con { $right }

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = Comparar contenidos (más lento)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = Comparar lo que hay dentro de cada archivo

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = Mostrar nombres iguales

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = Mostrar también los nombres iguales

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = La comparación se detuvo: { $error }.

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = { $left } solo a la izquierda, { $right } solo a la derecha, { $different } distintos, { $same } iguales.

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = Las dos carpetas tienen el mismo contenido.

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = Comparar de nuevo

# Button in the pane compare window; { $n } is how many names are copied to the left folder.
folders-tools-compare-copy-left = Copiar { $n } elementos que faltan o más recientes a la izquierda

# Button in the pane compare window; { $n } is how many names are copied to the right folder.
folders-tools-compare-copy-right = Copiar { $n } elementos que faltan o más recientes a la derecha

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = Solo se borran del todo los archivos, así que estas carpetas se quedarán como están: { $names }.
