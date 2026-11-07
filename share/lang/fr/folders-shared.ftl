# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

common-refused-why-cancelled = aucun mot de passe d'administrateur n'a été donné
common-refused-why-wrong = le mot de passe d'administrateur n'a pas été accepté
common-refused-why-not-allowed = cette session n'a pas le droit de le faire, quel que soit le mot de passe donné
common-refused-why-unasked = rien n'était là pour demander le mot de passe d'administrateur
common-refused-why-unclear = le mot de passe d'administrateur n'a pas été accepté, ou cette session n'a pas le droit de le faire
common-refused-sentence-cancelled = Aucun mot de passe d'administrateur n'a été donné
common-refused-sentence-wrong = Le mot de passe d'administrateur n'a pas été accepté
common-refused-sentence-not-allowed = Cette session n'a pas le droit de le faire, quel que soit le mot de passe donné
common-refused-sentence-unasked = Rien n'était là pour demander le mot de passe d'administrateur
common-refused-sentence-unclear = Le mot de passe d'administrateur n'a pas été accepté, ou cette session n'a pas le droit de le faire
common-refused-not-done-cancelled = Non effectué : aucun mot de passe d'administrateur n'a été donné.
common-refused-not-done-wrong = Non effectué : le mot de passe d'administrateur n'a pas été accepté.
common-refused-not-done-not-allowed = Non effectué : cette session n'a pas le droit de le faire, quel que soit le mot de passe donné. Fermez la session, rouvrez-la, puis réessayez.
common-refused-not-done-unasked = Non effectué : rien n'était là pour demander le mot de passe d'administrateur. Fermez la session, rouvrez-la, puis réessayez.
common-refused-not-done-unclear = Non effectué : le mot de passe d'administrateur n'a pas été accepté, ou cette session n'a pas le droit de le faire. Si rien ne l'a demandé, fermez la session, rouvrez-la, puis réessayez.
common-refused-no-reason = Non effectué : l'étape a échoué sans dire pourquoi.
common-keyring-locked = Le trousseau de clés est verrouillé. Fermez la session puis rouvrez-la, ou déverrouillez-le, puis réessayez.
common-keyring-missing = Il n'y a pas de trousseau de clés sur cet ordinateur pour le conserver.
common-keyring-silent = Le trousseau de clés n'a pas répondu à temps.
common-keyring-never = Le trousseau de clés n'a jamais répondu.
common-keyring-turn-failed = L'échange avec le trousseau de clés n'a pas pu avoir lieu ({ $why }), donc rien ne lui a été demandé.
common-keyring-waiting =
    { $action ->
        [removing] Le trousseau de clés n'a pas encore répondu, donc l'élément ({ $what }) n'est pas encore supprimé. S'il le supprime finalement, cette ligne l'indiquera.
       *[other] Le trousseau de clés n'a pas encore répondu, donc l'élément ({ $what }) n'est pas encore conservé. S'il le conserve finalement, cette ligne l'indiquera.
    }
common-keyring-late-done =
    { $action ->
        [removing] Le trousseau de clés a répondu en retard : l'élément ({ $what }) est supprimé.
       *[other] Le trousseau de clés a répondu en retard : l'élément ({ $what }) est conservé.
    }
common-keyring-late-never =
    { $action ->
        [removing] Le trousseau de clés n'a jamais répondu, donc l'élément ({ $what }) n'est pas supprimé.
       *[other] Le trousseau de clés n'a jamais répondu, donc l'élément ({ $what }) n'est pas conservé.
    }
common-keyring-late-refused =
    { $action ->
        [removing] Le trousseau de clés a répondu en retard et n'a pas supprimé l'élément ({ $what }) : { $why }
       *[other] Le trousseau de clés a répondu en retard et n'a pas conservé l'élément ({ $what }) : { $why }
    }
common-more = Plus
common-more-spoken = En savoir plus, ouvre l'aide
folders-volumes-no-password = Aucun mot de passe n'a été donné, rien n'a donc changé.
folders-volumes-nothing-changed = { $reason }, rien n'a donc changé.
folders-volumes-busy = Quelque chose l'utilise encore. Fermez les fenêtres ou programmes qui montrent ses fichiers, puis réessayez.
folders-volumes-no-room = Il n'y a pas assez de place pour ce qu'elle contient à cette taille.
folders-prompt-password = Mot de passe
folders-prompt-unlock = Déverrouiller
folders-prompt-cancel = Annuler
folders-prompt-type-first = Saisissez d'abord le mot de passe
folders-prompt-checking = Vérification…
folders-wallpaper-larger-4k = plus grande que la 4K
folders-wallpaper-larger-screen = plus grande que votre écran
folders-wallpaper-over-20-or-200 = plus de 20 Mo (200 Mo si animée)
folders-wallpaper-over-20 = plus de 20 Mo
folders-wallpaper-over-200 = plus de 200 Mo
folders-wallpaper-reasons-or = { $first } ou { $second }
