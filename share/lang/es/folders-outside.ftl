
# machine translated (Spanish), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = oculto, o en una carpeta oculta

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = contiene elementos ocultos

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = contiene demasiados elementos para revisarlos, así que Folders dejará fuera sus elementos ocultos

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = fuera de tu carpeta personal y de tus unidades

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = un enlace a algo oculto, o fuera de tu carpeta personal y de tus unidades

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = contiene un enlace a algo oculto, o fuera de tu carpeta personal y de tus unidades

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [one] ¿Mover { $name } aquí?
        [many] ¿Mover estos { $n } elementos aquí?
       *[other] ¿Mover estos { $n } elementos aquí?
    }

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [one] ¿Traer { $name } aquí?
        [many] ¿Traer estos { $n } elementos aquí?
       *[other] ¿Traer estos { $n } elementos aquí?
    }

# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [one] ¿Copiar { $name } aquí?
        [many] ¿Copiar estos { $n } elementos aquí?
       *[other] ¿Copiar estos { $n } elementos aquí?
    }

# Words on the question card when the drop would be moved: why it is asked.
folders-outside-body-move =
    { $n ->
        [one] Otro programa soltó esto en Folders. Cualquier programa puede soltar cualquier archivo, y esto no es uno de tus archivos normales. Folders no lo moverá hasta que tú lo digas.
       *[other] Otro programa soltó estos elementos en Folders. Cualquier programa puede soltar cualquier archivo, y algunos de estos no son tus archivos normales. Folders no los moverá hasta que tú lo digas.
    }

# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
        [one] Otro programa soltó esto en Folders. Cualquier programa puede soltar cualquier archivo, y esto no es uno de tus archivos normales. Folders no lo traerá hasta que tú lo digas.
       *[other] Otro programa soltó estos elementos en Folders. Cualquier programa puede soltar cualquier archivo, y algunos de estos no son tus archivos normales. Folders no los traerá hasta que tú lo digas.
    }

# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
        [one] Otro programa soltó esto en Folders. Cualquier programa puede soltar cualquier archivo, y esto no es uno de tus archivos normales. Folders no lo copiará hasta que tú lo digas.
       *[other] Otro programa soltó estos elementos en Folders. Cualquier programa puede soltar cualquier archivo, y algunos de estos no son tus archivos normales. Folders no los copiará hasta que tú lo digas.
    }

# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
        [one] Dejarlo
        [many] Dejarlos
       *[other] Dejarlos
    }

# SAFETY: needs a human review before it ships
# Button on the question card that moves what was dropped.
folders-outside-do-move =
    { $n ->
        [one] Moverlo
        [many] Moverlos
       *[other] Moverlos
    }

# SAFETY: needs a human review before it ships
# Button on the question card that brings what was dropped.
folders-outside-do-bring =
    { $n ->
        [one] Traerlo
        [many] Traerlos
       *[other] Traerlos
    }

# Button on the question card that copies what was dropped.
folders-outside-do-copy =
    { $n ->
        [one] Copiarlo
        [many] Copiarlos
       *[other] Copiarlos
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, de { $from }: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, de { $from }

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = Y { $n } más.

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = En { $into }.

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Responde primero a la pregunta sobre lo que soltaste.

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders aún está revisando lo último que soltaste.

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Aquí solo se pueden soltar archivos y carpetas.

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = Solo se pueden fijar carpetas en la barra lateral.

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = Solo se puede soltar sobre una carpeta.

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } ya no está ahí, así que no se copió ni se movió nada.

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = Una carpeta no se puede poner dentro de sí misma.

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = Ya están en esa carpeta.

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders no pudo acceder a tiempo a la unidad donde está { $name }, así que no se copió ni se movió nada.

# Words when the question card is answered with no.
folders-outside-nothing-brought = No se copió ni se movió nada.

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] 1 elemento
        [many] { $n } elementos
       *[other] { $n } elementos
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = y { $n } más

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] { $n } elemento cambió después de que empezaras a arrastrarlo, así que Folders lo dejó donde estaba: { $names }.
       *[other] { $n } elementos cambiaron después de que empezaras a arrastrarlos, así que Folders los dejó donde estaban: { $names }.
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] { $n } elemento sigue donde estaba: { $each }.
       *[other] { $n } elementos siguen donde estaban: { $each }.
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders no puede saber si el otro programa recibió todo lo que arrastraste, así que dejó aquí los originales. El otro programa tiene una copia.

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = El otro programa no tomó todo lo que arrastraste, así que Folders no movió nada.

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = El otro programa no tomó todo lo que arrastraste, así que su copia puede estar incompleta.

