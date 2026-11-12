# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-cancelled = du hast das Administratorpasswort nicht eingegeben
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-wrong = das Administratorpasswort wurde nicht akzeptiert
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-not-allowed = dein Konto darf das nicht, auch nicht mit dem Administratorpasswort
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unasked = es gab keine Möglichkeit, dich nach dem Administratorpasswort zu fragen
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unclear = das Administratorpasswort wurde nicht akzeptiert, oder dein Konto darf das nicht
# Sentence of its own saying a step was refused: the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-sentence-cancelled = Du hast das Administratorpasswort nicht eingegeben
# Sentence of its own saying a step was refused: the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-sentence-wrong = Das Administratorpasswort wurde nicht akzeptiert
# Sentence of its own saying a step was refused: this login may never do it.
# SAFETY: needs a human review before it ships
common-refused-sentence-not-allowed = Dein Konto darf das nicht, auch nicht mit dem Administratorpasswort
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
# SAFETY: needs a human review before it ships
common-refused-sentence-unasked = Es gab keine Möglichkeit, dich nach dem Administratorpasswort zu fragen
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
# SAFETY: needs a human review before it ships
common-refused-sentence-unclear = Das Administratorpasswort wurde nicht akzeptiert, oder dein Konto darf das nicht
# Red words when a step was refused because the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-not-done-cancelled = Das wurde nicht ausgeführt, weil du das Administratorpasswort nicht eingegeben hast.
# Red words when a step was refused because the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-not-done-wrong = Das wurde nicht ausgeführt, weil das Administratorpasswort nicht akzeptiert wurde.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-not-allowed = Das wurde nicht ausgeführt. Dein Konto darf das nicht, auch nicht mit dem Administratorpasswort. Melde dich ab und wieder an und versuche es dann noch einmal.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unasked = Das wurde nicht ausgeführt. Es gab keine Möglichkeit, dich nach dem Administratorpasswort zu fragen. Melde dich ab und wieder an und versuche es dann noch einmal.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unclear = Das wurde nicht ausgeführt. Das Administratorpasswort wurde nicht akzeptiert, oder dein Konto darf das nicht. Wenn du nicht nach einem Passwort gefragt wurdest, melde dich ab und wieder an und versuche es dann noch einmal.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Das hat nicht funktioniert, und es wurde kein Grund genannt.
# Red words when the keyring will not open without the person's password.
# SAFETY: needs a human review before it ships
common-keyring-locked = Der Schlüsselbund ist gesperrt. Entsperre ihn oder melde dich ab und wieder an und versuche es dann noch einmal.
# Red words when this computer has no keyring to keep a login or key in.
# SAFETY: needs a human review before it ships
common-keyring-missing = Auf diesem Computer gibt es keinen Schlüsselbund, in dem es gespeichert werden könnte.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Folders konnte den Schlüsselbund nicht rechtzeitig erreichen.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Folders hat aufgegeben, auf den Schlüsselbund zu warten.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Folders konnte nicht auf den Schlüsselbund zugreifen ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Folders wartet noch auf den Schlüsselbund, darum ist { $what } noch nicht entfernt.
       *[other] Folders wartet noch auf den Schlüsselbund, darum ist { $what } noch nicht gespeichert.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] Nach einer Verzögerung wurde { $what } aus dem Schlüsselbund entfernt.
       *[other] Nach einer Verzögerung wurde { $what } im Schlüsselbund gespeichert.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Folders hat aufgegeben, auf den Schlüsselbund zu warten, darum wurde { $what } nicht entfernt.
       *[other] Folders hat aufgegeben, auf den Schlüsselbund zu warten, darum wurde { $what } nicht gespeichert.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Folders konnte { $what } nicht aus dem Schlüsselbund entfernen. { $why }
       *[other] Folders konnte { $what } nicht im Schlüsselbund speichern. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Mehr
common-more-spoken = Hilfe dazu öffnen
# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
folders-volumes-no-password = Du hast kein Passwort eingegeben, daher wurde nichts geändert.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, daher wurde nichts geändert.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = Etwas benutzt sie noch. Schließe alle Fenster oder Programme, die ihre Dateien zeigen, und versuche es dann erneut.
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = In dieser Größe ist nicht genug Platz für das, was darauf liegt.
# Heading over the password box, and the tip shown in the box while it is empty.
# SAFETY: needs a human review before it ships
folders-prompt-password = Passwort
# Main button at the bottom of the password window that checks the password.
# SAFETY: needs a human review before it ships
folders-prompt-unlock = Entsperren
# Button at the bottom of the password window that closes it without unlocking.
# SAFETY: needs a human review before it ships
folders-prompt-cancel = Abbrechen
# Shown under the password box when Unlock is pressed with the box empty.
# SAFETY: needs a human review before it ships
folders-prompt-type-first = Gib zuerst das Passwort ein.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Wird geprüft …
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = größer als 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = größer als dein Bildschirm
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = über 20 MB (200 MB, wenn animiert)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = über 20 MB
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = über 200 MB
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } oder { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Gesperrt: { $name }

# Name given to a drive that has none.
folders-volumes-drive = Laufwerk

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = Diese Passphrase hat das Laufwerk nicht entsperrt.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = Es ist schon geöffnet.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = Zum Öffnen brauchst du eine Passphrase.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = Passphrase für { $name }

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Laufwerk entsperren

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders konnte das Passphrase-Fenster nicht öffnen.
