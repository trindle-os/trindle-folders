
# machine translated (French), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = caché, ou dans un dossier caché

# SAFETY: needs a human review before it ships
# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = contient des éléments cachés

# SAFETY: needs a human review before it ships
# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = contient trop d’éléments pour être vérifié, donc Folders laissera de côté ses éléments cachés

# SAFETY: needs a human review before it ships
# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = en dehors de votre dossier personnel et de vos disques

# SAFETY: needs a human review before it ships
# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = un lien vers un élément caché, ou en dehors de votre dossier personnel et de vos disques

# SAFETY: needs a human review before it ships
# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = contient un lien vers un élément caché, ou en dehors de votre dossier personnel et de vos disques

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [one] Déplacer { $name } ici ?
        [many] Déplacer ces { $n } éléments ici ?
       *[other] Déplacer ces { $n } éléments ici ?
    }

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [one] Apporter { $name } ici ?
        [many] Apporter ces { $n } éléments ici ?
       *[other] Apporter ces { $n } éléments ici ?
    }

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [one] Copier { $name } ici ?
        [many] Copier ces { $n } éléments ici ?
       *[other] Copier ces { $n } éléments ici ?
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be moved: why it is asked.
folders-outside-body-move =
    { $n ->
        [one] Un autre programme a déposé cet élément sur Folders. N’importe quel programme peut déposer n’importe quel fichier, et ceci n’est pas l’un de vos fichiers ordinaires. Folders ne le déplacera pas tant que vous ne l’aurez pas demandé.
        [many] Un autre programme a déposé ces éléments sur Folders. N’importe quel programme peut déposer n’importe quel fichier, et certains d’entre eux ne sont pas vos fichiers ordinaires. Folders ne les déplacera pas tant que vous ne l’aurez pas demandé.
       *[other] Un autre programme a déposé ces éléments sur Folders. N’importe quel programme peut déposer n’importe quel fichier, et certains d’entre eux ne sont pas vos fichiers ordinaires. Folders ne les déplacera pas tant que vous ne l’aurez pas demandé.
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
        [one] Un autre programme a déposé cet élément sur Folders. N’importe quel programme peut déposer n’importe quel fichier, et ceci n’est pas l’un de vos fichiers ordinaires. Folders ne l’apportera pas ici tant que vous ne l’aurez pas demandé.
        [many] Un autre programme a déposé ces éléments sur Folders. N’importe quel programme peut déposer n’importe quel fichier, et certains d’entre eux ne sont pas vos fichiers ordinaires. Folders ne les apportera pas ici tant que vous ne l’aurez pas demandé.
       *[other] Un autre programme a déposé ces éléments sur Folders. N’importe quel programme peut déposer n’importe quel fichier, et certains d’entre eux ne sont pas vos fichiers ordinaires. Folders ne les apportera pas ici tant que vous ne l’aurez pas demandé.
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
        [one] Un autre programme a déposé cet élément sur Folders. N’importe quel programme peut déposer n’importe quel fichier, et ceci n’est pas l’un de vos fichiers ordinaires. Folders ne le copiera pas tant que vous ne l’aurez pas demandé.
        [many] Un autre programme a déposé ces éléments sur Folders. N’importe quel programme peut déposer n’importe quel fichier, et certains d’entre eux ne sont pas vos fichiers ordinaires. Folders ne les copiera pas tant que vous ne l’aurez pas demandé.
       *[other] Un autre programme a déposé ces éléments sur Folders. N’importe quel programme peut déposer n’importe quel fichier, et certains d’entre eux ne sont pas vos fichiers ordinaires. Folders ne les copiera pas tant que vous ne l’aurez pas demandé.
    }

# SAFETY: needs a human review before it ships
# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
        [one] Le laisser
        [many] Les laisser
       *[other] Les laisser
    }

# SAFETY: needs a human review before it ships
# Button on the question card that moves what was dropped.
folders-outside-do-move =
    { $n ->
        [one] Le déplacer
        [many] Les déplacer
       *[other] Les déplacer
    }

# SAFETY: needs a human review before it ships
# Button on the question card that brings what was dropped.
folders-outside-do-bring =
    { $n ->
        [one] L’apporter
        [many] Les apporter
       *[other] Les apporter
    }

# SAFETY: needs a human review before it ships
# Button on the question card that copies what was dropped.
folders-outside-do-copy =
    { $n ->
        [one] Le copier
        [many] Les copier
       *[other] Les copier
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, depuis { $from } : { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, depuis { $from }

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = Et { $n } de plus.

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = Dans { $into }.

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Répondez d’abord à la question sur ce que vous avez déposé.

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders est encore en train de vérifier le dernier élément que vous avez déposé.

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Seuls des fichiers et des dossiers peuvent être déposés ici.

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = Seuls des dossiers peuvent être épinglés à la barre latérale.

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = Vous ne pouvez déposer des éléments que sur un dossier.

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } n’existe plus, donc rien n’a été copié ni déplacé.

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = Vous ne pouvez pas placer un dossier dans lui-même.

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = Ils sont déjà dans ce dossier.

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders n’a pas pu joindre à temps le disque où se trouve { $name }, donc rien n’a été copié ni déplacé.

# Words when the question card is answered with no.
folders-outside-nothing-brought = Rien n’a été copié ni déplacé.

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] 1 élément
        [many] { $n } éléments
       *[other] { $n } éléments
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = et { $n } de plus

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] { $n } élément a changé après que vous avez commencé à le faire glisser, donc Folders l’a laissé où il était : { $names }.
        [many] { $n } d’éléments ont changé après que vous avez commencé à les faire glisser, donc Folders les a laissés où ils étaient : { $names }.
       *[other] { $n } éléments ont changé après que vous avez commencé à les faire glisser, donc Folders les a laissés où ils étaient : { $names }.
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] { $n } élément est resté où il était : { $each }.
        [many] { $n } éléments sont restés où ils étaient : { $each }.
       *[other] { $n } éléments sont restés où ils étaient : { $each }.
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders ne peut pas savoir si l’autre programme a tout reçu de ce que vous avez glissé, donc il a laissé les originaux ici. L’autre programme en a une copie.

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = L’autre programme n’a pas pris tout ce que vous avez glissé, donc Folders n’a rien déplacé.

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = L’autre programme n’a pas pris tout ce que vous avez glissé, donc sa copie peut être incomplète.
