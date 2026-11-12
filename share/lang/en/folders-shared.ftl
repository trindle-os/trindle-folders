# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# Heading over the password box, and the tip shown in the box while it is empty.
folders-prompt-password = Password
# Main button at the bottom of the password window that checks the password.
folders-prompt-unlock = Unlock
# Button at the bottom of the password window that closes it without unlocking.
folders-prompt-cancel = Cancel
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = Type the password first.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Checking…
# Red words when the password box was closed with no password. Said whole, as a sentence.
folders-volumes-no-password = You didn't enter a password, so nothing was changed.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, so nothing was changed.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = Something is still using it. Close any windows or programs showing its files, then try again.
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = There isn't enough space for what's on it at that size.
# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = you didn't enter the administrator's password
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = the administrator's password wasn't accepted
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = your account isn't allowed to do that, even with the administrator's password
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = there was no way to ask you for the administrator's password
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = the administrator's password wasn't accepted, or your account isn't allowed to do that
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = You didn't enter the administrator's password
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = The administrator's password wasn't accepted
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = Your account isn't allowed to do that, even with the administrator's password
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = There was no way to ask you for the administrator's password
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = The administrator's password wasn't accepted, or your account isn't allowed to do that
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = That wasn't done because you didn't enter the administrator's password.
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = That wasn't done because the administrator's password wasn't accepted.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = That wasn't done. Your account isn't allowed to do it, even with the administrator's password. Log out and back in, then try again.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = That wasn't done. There was no way to ask you for the administrator's password. Log out and back in, then try again.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = That wasn't done. The administrator's password wasn't accepted, or your account isn't allowed to do it. If you weren't asked for a password, log out and back in, then try again.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = That didn't work, and no reason was given.
# Red words when the keyring will not open without the person's password.
common-keyring-locked = The keyring is locked. Unlock it, or log out and back in, then try again.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = There's no keyring on this computer to save it in.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Folders couldn't reach the keyring in time.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Folders gave up waiting for the keyring.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Folders couldn't get access to the keyring ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Folders is still waiting for the keyring, so the { $what } isn't removed yet.
       *[other] Folders is still waiting for the keyring, so the { $what } isn't saved yet.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] After a delay, the { $what } was removed from the keyring.
       *[other] After a delay, the { $what } was saved in the keyring.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Folders gave up waiting for the keyring, so the { $what } wasn't removed.
       *[other] Folders gave up waiting for the keyring, so the { $what } wasn't saved.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Folders couldn't remove the { $what } from the keyring. { $why }
       *[other] Folders couldn't save the { $what } in the keyring. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = More
common-more-spoken = Open Help about this
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = larger than 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = larger than your screen
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = over 20 MB (200 MB if animated)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = over 20 MB
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = over 200 MB
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } or { $second }
# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Locked: { $name }
# Name given to a drive that has none.
folders-volumes-drive = Drive
# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = That passphrase didn't unlock the drive.
# Said when a drive that is open already is opened again.
folders-volumes-already-open = It's already open.
# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = You need a passphrase to open it.
# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = Passphrase for { $name }
# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Unlock drive
# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders couldn't open the passphrase window.
