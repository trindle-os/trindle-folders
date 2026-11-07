

folders-tools-pass-once = Una vez

folders-tools-pass-three = 3 veces

folders-tools-pass-seven = 7 veces

folders-tools-case-keep = Como están

folders-tools-case-lower = Todo en minúsculas

folders-tools-case-upper = Todo en mayúsculas

folders-tools-case-words = Cada palabra

folders-tools-case-first = Primera palabra

folders-tools-ending-keep = Como está

folders-tools-ending-lower = Todo en minúsculas

folders-tools-ending-upper = Todo en mayúsculas

folders-tools-ending-new = Una extensión nueva

folders-tools-ending-none = Sin extensión

folders-tools-archive-name = Archivo

folders-tools-clash-empty = no quedaría ningún nombre

folders-tools-clash-bad = no se permite como nombre

folders-tools-clash-long = ese nombre es demasiado largo

folders-tools-clash-twice = dos tendrían este nombre

folders-tools-clash-taken = otra cosa ya tiene este nombre

folders-tools-clash-not-text = el nombre antiguo no es texto sin formato

folders-tools-caveat-flash = Este disco es flash, no giratorio, así que pueden quedar copias antiguas en partes a las que Trindle no llega.

folders-tools-caveat-copy-on-write = El sistema de archivos { $fs } escribe los cambios en un lugar nuevo, así que los datos antiguos pueden seguir donde estaban.

folders-tools-caveat-network = Esto es un recurso { $fs } de otro equipo, que puede guardar sus propias copias.

folders-tools-row-only-left = Solo a la izquierda

folders-tools-row-only-right = Solo a la derecha

folders-tools-row-same = Igual

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

folders-tools-place-here = Aquí

folders-tools-only-files-wiped = Solo se pueden borrar de forma segura los archivos.

folders-tools-compare-need-folders = Los dos paneles deben mostrar una carpeta para compararlos.

folders-tools-rename-undo =
    { $n ->
        [one] Deshacer el cambio de nombre de 1 elemento
       *[other] Deshacer el cambio de nombre de { $n } elementos
    }

folders-tools-rename-done =
    { $n ->
        [one] Se cambió el nombre de 1 elemento.
       *[other] Se cambió el nombre de { $n } elementos.
    }

folders-tools-rename-stopped = El cambio de nombres se detuvo a medias: { $error }.

folders-tools-name-empty = El nombre no puede estar vacío.

folders-tools-looked-at = Revisados { $done }

folders-tools-read-of = Leídos { $done } de { $total }

folders-tools-listing-files = Listando archivos

folders-tools-comparing = Comparando

folders-tools-looking = Mirando

folders-tools-compress-title =
    { $n ->
        [one] Comprimir { $name }
       *[other] Comprimir { $n } elementos
    }

folders-tools-pack-name-tip = Nombre del archivo comprimido

folders-tools-pack-kind-tip = Tipo de archivo comprimido

folders-tools-pack-made-in = Se crea en { $place }

folders-tools-compress = Comprimir

folders-tools-extract-to = Extraer { $name } en

folders-tools-extract-new-folder = Allí se crea una carpeta nueva llamada { $name }.

folders-tools-resize-title =
    { $n ->
        [one] Cambiar el tamaño de { $name }
       *[other] Cambiar el tamaño de { $n } imágenes
    }

folders-tools-resize-size-tip = Tamaño nuevo

folders-tools-resize-note = Un porcentaje de su tamaño, o cuántos píxeles tiene su lado más largo; una imagen nunca se agranda. Se hace una copia junto a cada una, con su nuevo tamaño en el nombre; las imágenes originales no cambian.

folders-tools-resize = Cambiar tamaño

folders-tools-wipe-title =
    { $n ->
        [one] ¿Borrar de forma segura { $name }?
       *[other] ¿Borrar de forma segura { $n } archivos?
    }

folders-tools-wipe-warning = Cada archivo se sobrescribe antes de eliminarse, así que no se puede recuperar. Esto no se puede deshacer.

folders-tools-wipe-write-over = Sobrescribir

folders-tools-wipe-passes-tip = Cuántas veces se sobrescribe cada archivo

folders-tools-wipe-button = Borrar de forma segura

folders-tools-rename-number-tip = Numerarlos

folders-tools-rename-at-start = Al principio

folders-tools-rename-at-end = Al final

folders-tools-rename-where-tip = Dónde va el número

folders-tools-rename-first-number-tip = Primer número

folders-tools-rename-ending-tip = Qué pasa con la extensión

folders-tools-rename-new-ending-tip = Extensión nueva, como txt

folders-tools-rename-cannot = No se puede: { $why }

folders-tools-rename-was = antes era { $old }

folders-tools-rename-stays = se queda como está

folders-tools-rename-title =
    { $n ->
        [one] Renombrar 1 elemento
       *[other] Renombrar { $n } elementos
    }

folders-tools-rename-replace = Reemplazar

folders-tools-rename-find-tip = Buscar

folders-tools-rename-with-tip = Reemplazar con

folders-tools-rename-wildcard-tip = Usar * y ? como comodines

folders-tools-rename-wildcard = Comodines * y ?

folders-tools-rename-match-case-tip = Distinguir mayúsculas y minúsculas

folders-tools-rename-match-case = Distinguir mayúsculas

folders-tools-rename-add = Añadir

folders-tools-rename-before-tip = Antes del nombre

folders-tools-rename-after-tip = Después del nombre

folders-tools-rename-number = Número

folders-tools-rename-letters = Letras

folders-tools-rename-case-tip = Mayúsculas en los nombres

folders-tools-rename-ending = Extensión

folders-tools-rename-summary =
    { $clash ->
        [yes] Algunos nombres no se pueden usar; mira las filas marcadas. En { $place }.
       *[no] Cambian { $changes } de { $total } nombres. En { $place }.
    }

folders-tools-rename-button = Renombrarlos

folders-tools-home = Inicio

folders-tools-dupes-title =
    { $n ->
        [one] Duplicados en { $folder }
       *[other] Duplicados en { $n } carpetas
    }

folders-tools-dupes-stopped = La búsqueda se detuvo: { $error }.

folders-tools-dupes-none = No se encontraron duplicados.

folders-tools-dupes-unread =
    { $n ->
        [one] No se pudo leer 1 archivo.
       *[other] No se pudieron leer { $n } archivos.
    }

folders-tools-dupes-sets =
    { $n ->
        [one] 1 grupo de copias; al mover las copias sobrantes a la papelera se liberan { $size }. Las copias sobrantes están marcadas; Ctrl y clic marca o desmarca una.
       *[other] { $n } grupos de copias; al mover las copias sobrantes a la papelera se liberan { $size }. Las copias sobrantes están marcadas; Ctrl y clic marca o desmarca una.
    }

folders-tools-dupes-row = Grupo { $set }, { $size }, en { $at }

folders-tools-dupes-unread-left-out =
    { $n ->
        [one] No se pudo leer 1 archivo y se dejó fuera.
       *[other] No se pudieron leer { $n } archivos y se dejaron fuera.
    }

folders-tools-dupes-keep-one = Están marcadas todas las copias de un grupo; conserva al menos una.

folders-tools-dupes-pick-extras = Marcar las copias sobrantes

folders-tools-dupes-trash = Mover a la papelera

folders-tools-diff-title = Comparar los dos

folders-tools-diff-failed = No se pudieron comparar: { $error }.

folders-tools-diff-same-bytes = Los dos son iguales, byte a byte.

folders-tools-diff-same-as-shorter = Son iguales hasta donde llega el más corto; uno es de { $one } y el otro de { $other }.

folders-tools-diff-from-byte = Se diferencian a partir del byte { $n }.

folders-tools-diff-lines =
    { $n ->
        [0] Los dos son iguales.
        [one] Difiere 1 línea.
       *[other] Difieren { $n } líneas.
    }

folders-tools-diff-too-different = Son demasiado distintos para compararlos línea a línea, así que todas las líneas entre la primera y la última diferencia aparecen como cambiadas.

folders-tools-diff-cut = Solo se comparan los primeros 4 MB de cada uno.

folders-tools-compare-title = Comparar { $left } con { $right }

folders-tools-compare-bytes = Leer cada byte (más lento)

folders-tools-compare-bytes-tip = Comparar el contenido de cada archivo

folders-tools-compare-show-same = Mostrar lo que es igual

folders-tools-compare-show-same-tip = Mostrar también los nombres que son iguales

folders-tools-compare-stopped = La comparación se detuvo: { $error }.

folders-tools-compare-counts = { $left } solo a la izquierda, { $right } solo a la derecha, { $different } distintos, { $same } iguales.

folders-tools-compare-all-same = Las dos carpetas contienen lo mismo.

folders-tools-compare-again = Comparar de nuevo

folders-tools-compare-copy-left = Copiar { $n } que faltan o más recientes a la izquierda

folders-tools-compare-copy-right = Copiar { $n } que faltan o más recientes a la derecha

