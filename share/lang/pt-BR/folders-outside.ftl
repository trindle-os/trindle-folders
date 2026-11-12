
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = oculto, ou em uma pasta oculta

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = tem itens ocultos dentro

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = tem itens demais para verificar, então o Folders deixará de fora os itens ocultos dele

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = fora da sua pasta pessoal e das suas unidades

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = um atalho para algo oculto, ou fora da sua pasta pessoal e das suas unidades

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = tem dentro um atalho para algo oculto, ou fora da sua pasta pessoal e das suas unidades

# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
# SAFETY: needs a human review before it ships
folders-outside-title-move =
    { $n ->
        [one] Mover { $name } para cá?
       *[other] Mover estes { $n } itens para cá?
        [many] Mover estes { $n } itens para cá?
    }

# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
# SAFETY: needs a human review before it ships
folders-outside-title-bring =
    { $n ->
        [one] Trazer { $name } para cá?
       *[other] Trazer estes { $n } itens para cá?
        [many] Trazer estes { $n } itens para cá?
    }

# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
# SAFETY: needs a human review before it ships
folders-outside-title-copy =
    { $n ->
        [one] Copiar { $name } para cá?
       *[other] Copiar estes { $n } itens para cá?
        [many] Copiar estes { $n } itens para cá?
    }

# Words on the question card when the drop would be moved: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-move =
    { $n ->
        [one] Outro programa soltou este item no Folders. Qualquer programa pode soltar qualquer arquivo, e este não é um dos seus arquivos comuns. O Folders não vai movê-lo até você autorizar.
       *[other] Outro programa soltou estes itens no Folders. Qualquer programa pode soltar qualquer arquivo, e alguns destes não são seus arquivos comuns. O Folders não vai movê-los até você autorizar.
    }

# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
        [one] Outro programa soltou este item no Folders. Qualquer programa pode soltar qualquer arquivo, e este não é um dos seus arquivos comuns. O Folders não vai trazê-lo até você autorizar.
       *[other] Outro programa soltou estes itens no Folders. Qualquer programa pode soltar qualquer arquivo, e alguns destes não são seus arquivos comuns. O Folders não vai trazê-los até você autorizar.
    }

# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
        [one] Outro programa soltou este item no Folders. Qualquer programa pode soltar qualquer arquivo, e este não é um dos seus arquivos comuns. O Folders não vai copiá-lo até você autorizar.
       *[other] Outro programa soltou estes itens no Folders. Qualquer programa pode soltar qualquer arquivo, e alguns destes não são seus arquivos comuns. O Folders não vai copiá-los até você autorizar.
    }

# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
        [one] Deixá-lo
       *[other] Deixá-los
        [many] Deixá-los
    }

# Button on the question card that moves what was dropped.
folders-outside-do-move =
    { $n ->
        [one] Movê-lo
       *[other] Movê-los
        [many] Movê-los
    }

# Button on the question card that brings what was dropped.
folders-outside-do-bring =
    { $n ->
        [one] Trazê-lo
       *[other] Trazê-los
        [many] Trazê-los
    }

# Button on the question card that copies what was dropped.
folders-outside-do-copy =
    { $n ->
        [one] Copiá-lo
       *[other] Copiá-los
        [many] Copiá-los
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, de { $from }: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, de { $from }

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = E mais { $n }.

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = Para { $into }.

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Responda primeiro à pergunta sobre o que você soltou.

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = O Folders ainda está verificando a última coisa que você soltou.

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Só arquivos e pastas podem ser soltos aqui.

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = Só pastas podem ser fixadas na barra lateral.

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = Você só pode soltar itens em uma pasta.

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } não está mais lá, então nada foi copiado nem movido.

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = Você não pode colocar uma pasta dentro dela mesma.

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = Eles já estão nessa pasta.

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = O Folders não conseguiu acessar a tempo a unidade em que { $name } está, então nada foi copiado nem movido.

# Words when the question card is answered with no.
folders-outside-nothing-brought = Nada foi copiado nem movido.

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] 1 item
       *[other] { $n } itens
        [many] { $n } itens
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = e mais { $n }

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] { $n } item mudou depois que você começou a arrastá-lo, então o Folders o deixou onde estava: { $names }.
       *[other] { $n } itens mudaram depois que você começou a arrastá-los, então o Folders os deixou onde estavam: { $names }.
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] { $n } item continua onde estava: { $each }.
       *[other] { $n } itens continuam onde estavam: { $each }.
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = O Folders não consegue saber se o outro programa recebeu tudo o que você arrastou, então deixou os originais aqui. O outro programa tem uma cópia.

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = O outro programa não recebeu tudo o que você arrastou, então o Folders não moveu nada.

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = O outro programa não recebeu tudo o que você arrastou, então a cópia dele pode estar incompleta.

