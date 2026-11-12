# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-cancelled = vous n'avez pas saisi le mot de passe d'administrateur
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-wrong = le mot de passe d'administrateur n'a pas été accepté
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-not-allowed = votre compte n'a pas le droit de le faire, même avec le mot de passe d'administrateur
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unasked = il n'y avait aucun moyen de vous demander le mot de passe d'administrateur
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unclear = le mot de passe d'administrateur n'a pas été accepté, ou votre compte n'a pas le droit de le faire
# Sentence of its own saying a step was refused: the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-sentence-cancelled = Vous n'avez pas saisi le mot de passe d'administrateur
# Sentence of its own saying a step was refused: the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-sentence-wrong = Le mot de passe d'administrateur n'a pas été accepté
# Sentence of its own saying a step was refused: this login may never do it.
# SAFETY: needs a human review before it ships
common-refused-sentence-not-allowed = Votre compte n'a pas le droit de le faire, même avec le mot de passe d'administrateur
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
# SAFETY: needs a human review before it ships
common-refused-sentence-unasked = Il n'y avait aucun moyen de vous demander le mot de passe d'administrateur
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
# SAFETY: needs a human review before it ships
common-refused-sentence-unclear = Le mot de passe d'administrateur n'a pas été accepté, ou votre compte n'a pas le droit de le faire
# Red words when a step was refused because the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-not-done-cancelled = Cela n'a pas été fait car vous n'avez pas saisi le mot de passe d'administrateur.
# Red words when a step was refused because the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-not-done-wrong = Cela n'a pas été fait car le mot de passe d'administrateur n'a pas été accepté.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-not-allowed = Cela n'a pas été fait. Votre compte n'a pas le droit de le faire, même avec le mot de passe d'administrateur. Fermez la session, rouvrez-la, puis réessayez.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unasked = Cela n'a pas été fait. Il n'y avait aucun moyen de vous demander le mot de passe d'administrateur. Fermez la session, rouvrez-la, puis réessayez.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unclear = Cela n'a pas été fait. Le mot de passe d'administrateur n'a pas été accepté, ou votre compte n'a pas le droit de le faire. Si on ne vous a pas demandé de mot de passe, fermez la session, rouvrez-la, puis réessayez.
# Red words when a step behind the administrator's password failed and said nothing of why.
# SAFETY: needs a human review before it ships
common-refused-no-reason = Cela n'a pas fonctionné, et aucune raison n'a été donnée.
# Red words when the keyring will not open without the person's password.
# SAFETY: needs a human review before it ships
common-keyring-locked = Le trousseau de clés est verrouillé. Déverrouillez-le, ou fermez puis rouvrez votre session, puis réessayez.
# Red words when this computer has no keyring to keep a login or key in.
# SAFETY: needs a human review before it ships
common-keyring-missing = Il n'y a pas de trousseau de clés sur cet ordinateur pour l'y enregistrer.
# Red words when the keyring is there and gives no answer in time.
# SAFETY: needs a human review before it ships
common-keyring-silent = Folders n'a pas pu joindre le trousseau de clés à temps.
# Red words when work with the keyring never ended, long after its deadline.
# SAFETY: needs a human review before it ships
common-keyring-never = Folders a renoncé à attendre le trousseau de clés.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
common-keyring-turn-failed = Folders n'a pas pu accéder au trousseau de clés ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
# SAFETY: needs a human review before it ships
common-keyring-waiting =
    { $action ->
        [removing] Folders attend encore le trousseau de clés, donc l'élément ({ $what }) n'est pas encore supprimé.
       *[other] Folders attend encore le trousseau de clés, donc l'élément ({ $what }) n'est pas encore enregistré.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
# SAFETY: needs a human review before it ships
common-keyring-late-done =
    { $action ->
        [removing] Après un délai, l'élément ({ $what }) a été supprimé du trousseau de clés.
       *[other] Après un délai, l'élément ({ $what }) a été enregistré dans le trousseau de clés.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
# SAFETY: needs a human review before it ships
common-keyring-late-never =
    { $action ->
        [removing] Folders a renoncé à attendre le trousseau de clés, donc l'élément ({ $what }) n'a pas été supprimé.
       *[other] Folders a renoncé à attendre le trousseau de clés, donc l'élément ({ $what }) n'a pas été enregistré.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
# SAFETY: needs a human review before it ships
common-keyring-late-refused =
    { $action ->
        [removing] Folders n'a pas pu supprimer l'élément ({ $what }) du trousseau de clés. { $why }
       *[other] Folders n'a pas pu enregistrer l'élément ({ $what }) dans le trousseau de clés. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Plus
common-more-spoken = Ouvrir l'aide à ce sujet
# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
folders-volumes-no-password = Vous n'avez pas saisi de mot de passe, rien n'a donc été modifié.
# Red words for a refusal. { $reason } says why, without a full stop.
# SAFETY: needs a human review before it ships
folders-volumes-nothing-changed = { $reason }, rien n'a donc été modifié.
# Red words when a drive is in use and cannot be unmounted.
# SAFETY: needs a human review before it ships
folders-volumes-busy = Quelque chose l'utilise encore. Fermez les fenêtres ou programmes qui montrent ses fichiers, puis réessayez.
# Red words when there is not enough room for what is on a part at the size asked for.
# SAFETY: needs a human review before it ships
folders-volumes-no-room = Il n'y a pas assez de place pour ce qu'elle contient à cette taille.
# Heading over the password box, and the tip shown in the box while it is empty.
folders-prompt-password = Mot de passe
# SAFETY: needs a human review before it ships
# Main button at the bottom of the password window that checks the password.
folders-prompt-unlock = Déverrouiller
# SAFETY: needs a human review before it ships
# Button at the bottom of the password window that closes it without unlocking.
folders-prompt-cancel = Annuler
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = Saisissez d'abord le mot de passe.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Vérification…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = plus grande que la 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = plus grande que votre écran
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = plus de 20 Mo (200 Mo si animée)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = plus de 20 Mo
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = plus de 200 Mo
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } ou { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Verrouillé : { $name }

# Name given to a drive that has none.
folders-volumes-drive = Disque

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = Cette phrase secrète n'a pas déverrouillé le disque.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = Il est déjà ouvert.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = Il faut une phrase secrète pour l'ouvrir.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = Phrase secrète pour { $name }

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Déverrouiller le disque

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders n'a pas pu ouvrir la fenêtre de phrase secrète.
