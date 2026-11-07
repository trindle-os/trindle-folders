

folders-outside-wary-hidden = caché, ou dans un dossier caché

folders-outside-wary-holds = contient des éléments cachés

folders-outside-wary-deep = contient trop d’éléments pour être examiné, donc ses éléments cachés ne sont pas apportés

folders-outside-wary-system = en dehors de votre dossier personnel et de vos disques

folders-outside-wary-leads = un lien vers un élément caché, ou en dehors de votre dossier personnel et de vos disques

folders-outside-wary-holds-link = contient un lien vers un élément caché, ou en dehors de votre dossier personnel et de vos disques

folders-outside-title-move =
    { $n ->
        [one] Déplacer { $name } ici ?
        [many] Déplacer ces { $n } éléments ici ?
       *[other] Déplacer ces { $n } éléments ici ?
    }

folders-outside-title-bring =
    { $n ->
        [one] Apporter { $name } ici ?
        [many] Apporter ces { $n } éléments ici ?
       *[other] Apporter ces { $n } éléments ici ?
    }

folders-outside-title-copy =
    { $n ->
        [one] Copier { $name } ici ?
        [many] Copier ces { $n } éléments ici ?
       *[other] Copier ces { $n } éléments ici ?
    }

folders-outside-body-move =
    { $n ->
        [one] Un autre programme l’a déposé sur Dossiers et l’a nommé lui-même. Tout programme peut nommer n’importe quel fichier, et ce qui est nommé ici n’est pas toujours un fichier ordinaire à vous, donc rien n’est déplacé sans votre accord.
        [many] Un autre programme les a déposés sur Dossiers et les a nommés lui-même. Tout programme peut nommer n’importe quel fichier, et ce qui est nommé ici n’est pas toujours un fichier ordinaire à vous, donc rien n’est déplacé sans votre accord.
       *[other] Un autre programme les a déposés sur Dossiers et les a nommés lui-même. Tout programme peut nommer n’importe quel fichier, et ce qui est nommé ici n’est pas toujours un fichier ordinaire à vous, donc rien n’est déplacé sans votre accord.
    }

folders-outside-body-bring =
    { $n ->
        [one] Un autre programme l’a déposé sur Dossiers et l’a nommé lui-même. Tout programme peut nommer n’importe quel fichier, et ce qui est nommé ici n’est pas toujours un fichier ordinaire à vous, donc rien n’est apporté sans votre accord.
        [many] Un autre programme les a déposés sur Dossiers et les a nommés lui-même. Tout programme peut nommer n’importe quel fichier, et ce qui est nommé ici n’est pas toujours un fichier ordinaire à vous, donc rien n’est apporté sans votre accord.
       *[other] Un autre programme les a déposés sur Dossiers et les a nommés lui-même. Tout programme peut nommer n’importe quel fichier, et ce qui est nommé ici n’est pas toujours un fichier ordinaire à vous, donc rien n’est apporté sans votre accord.
    }

folders-outside-body-copy =
    { $n ->
        [one] Un autre programme l’a déposé sur Dossiers et l’a nommé lui-même. Tout programme peut nommer n’importe quel fichier, et ce qui est nommé ici n’est pas toujours un fichier ordinaire à vous, donc rien n’est copié sans votre accord.
        [many] Un autre programme les a déposés sur Dossiers et les a nommés lui-même. Tout programme peut nommer n’importe quel fichier, et ce qui est nommé ici n’est pas toujours un fichier ordinaire à vous, donc rien n’est copié sans votre accord.
       *[other] Un autre programme les a déposés sur Dossiers et les a nommés lui-même. Tout programme peut nommer n’importe quel fichier, et ce qui est nommé ici n’est pas toujours un fichier ordinaire à vous, donc rien n’est copié sans votre accord.
    }

folders-outside-leave =
    { $n ->
        [one] Le laisser
        [many] Les laisser
       *[other] Les laisser
    }

folders-outside-do-move =
    { $n ->
        [one] Le déplacer
        [many] Les déplacer
       *[other] Les déplacer
    }

folders-outside-do-bring =
    { $n ->
        [one] L’apporter
        [many] Les apporter
       *[other] Les apporter
    }

folders-outside-do-copy =
    { $n ->
        [one] Le copier
        [many] Les copier
       *[other] Les copier
    }

folders-outside-named-why = { $name }, depuis { $from } : { $why }

folders-outside-named = { $name }, depuis { $from }

folders-outside-more = Et { $n } de plus.

folders-outside-into = Dans { $into }.

folders-outside-answer-first = Répondez d’abord à la question sur ce qui a été déposé.

folders-outside-still-looking = Le dernier élément déposé est encore en cours d’examen.

folders-outside-only-files = Seuls des fichiers et des dossiers peuvent être déposés ici.

folders-outside-only-folders-pin = Seuls des dossiers peuvent être épinglés à la barre latérale.

folders-outside-only-on-folder = Les éléments ne peuvent être déposés que sur un dossier.

folders-outside-gone = { $name } n’est plus là, donc rien n’a été apporté.

folders-outside-inside-itself = Un dossier ne peut pas être placé dans lui-même.

folders-outside-already-there = Ils sont déjà dans ce dossier.

folders-outside-not-answering = L’endroit où se trouve { $name } ne répond pas, donc rien n’a été apporté.

folders-outside-nothing-brought = Rien n’a été apporté.

folders-outside-items-dragged =
    { $n ->
        [one] 1 élément
        [many] { $n } éléments
       *[other] { $n } éléments
    }

folders-outside-lines-more = et { $n } de plus

folders-outside-stayed-one = { $name } { $why }.

folders-outside-stayed-changed =
    { $n ->
        [one] { $n } élément a changé après avoir été saisi, il est donc resté où il était : { $names }.
        [many] { $n } éléments ont changé après avoir été saisis, ils sont donc restés où ils étaient : { $names }.
       *[other] { $n } éléments ont changé après avoir été saisis, ils sont donc restés où ils étaient : { $names }.
    }

folders-outside-stayed-many =
    { $n ->
        [one] { $n } élément est resté où il était : { $each }.
        [many] { $n } éléments sont restés où ils étaient : { $each }.
       *[other] { $n } éléments sont restés où ils étaient : { $each }.
    }

folders-outside-stayed-each = { $name } ({ $why })

folders-outside-both = { $first } { $second }

folders-outside-unshown = L’autre programme a pris ce qui était glissé d’une manière qui ne prouve pas qu’il l’a lu en entier, donc rien n’a été déplacé ici : ce qu’il a est une copie.

folders-outside-short-move = L’autre programme n’a pas pris tout ce qui était glissé, donc rien n’a été déplacé ici.

folders-outside-short-copy = L’autre programme n’a pas pris tout ce qui était glissé, il n’a donc peut-être pas tout.
