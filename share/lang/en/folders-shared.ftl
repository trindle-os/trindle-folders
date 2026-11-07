# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

folders-prompt-password = Password
folders-prompt-unlock = Unlock
folders-prompt-cancel = Cancel
folders-prompt-type-first = Type the password first
folders-prompt-checking = Checking…
folders-volumes-no-password = No password was given, so nothing changed.
folders-volumes-nothing-changed = { $reason }, so nothing changed.
folders-volumes-busy = Something is still using it. Close any windows or programs showing its files, then try again.
folders-volumes-no-room = There is not enough room for what is on it at that size.
common-refused-why-cancelled = no administrator's password was given
common-refused-why-wrong = the administrator's password was not accepted
common-refused-why-not-allowed = this login is not allowed to do that, whatever password is given
common-refused-why-unasked = nothing was there to ask for the administrator's password
common-refused-why-unclear = the administrator's password was not accepted, or this login is not allowed to do that
common-refused-sentence-cancelled = No administrator's password was given
common-refused-sentence-wrong = The administrator's password was not accepted
common-refused-sentence-not-allowed = This login is not allowed to do that, whatever password is given
common-refused-sentence-unasked = Nothing was there to ask for the administrator's password
common-refused-sentence-unclear = The administrator's password was not accepted, or this login is not allowed to do that
common-refused-not-done-cancelled = Not done: no administrator's password was given.
common-refused-not-done-wrong = Not done: the administrator's password was not accepted.
common-refused-not-done-not-allowed = Not done: this login is not allowed to do that, whatever password is given. Log out and back in, then try again.
common-refused-not-done-unasked = Not done: nothing was there to ask for the administrator's password. Log out and back in, then try again.
common-refused-not-done-unclear = Not done: the administrator's password was not accepted, or this login is not allowed to do that. If nothing asked for it, log out and back in, then try again.
common-refused-no-reason = Not done: the step failed and did not say why.
common-keyring-locked = The keyring is locked. Log out and in again, or unlock it, then try again.
common-keyring-missing = There is no keyring on this computer to keep it in.
common-keyring-silent = The keyring did not answer in time.
common-keyring-never = The keyring never answered.
common-keyring-turn-failed = The turn with the keyring could not be taken ({ $why }), so nothing was asked of it.
common-keyring-waiting =
    { $action ->
        [removing] The keyring has not answered yet, so the { $what } is not removed so far. If the keyring removes it after all, this line will say so.
       *[other] The keyring has not answered yet, so the { $what } is not kept so far. If the keyring keeps it after all, this line will say so.
    }
common-keyring-late-done =
    { $action ->
        [removing] The keyring answered late: the { $what } is removed.
       *[other] The keyring answered late: the { $what } is kept.
    }
common-keyring-late-never =
    { $action ->
        [removing] The keyring never answered, so the { $what } is not removed.
       *[other] The keyring never answered, so the { $what } is not kept.
    }
common-keyring-late-refused =
    { $action ->
        [removing] The keyring answered late and did not remove the { $what }: { $why }
       *[other] The keyring answered late and did not keep the { $what }: { $why }
    }
common-more = More
common-more-spoken = More about this, opens Help
folders-wallpaper-larger-4k = larger than 4K
folders-wallpaper-larger-screen = larger than your screen
folders-wallpaper-over-20-or-200 = over 20 MB (200 MB if moving)
folders-wallpaper-over-20 = over 20 MB
folders-wallpaper-over-200 = over 200 MB
folders-wallpaper-reasons-or = { $first } or { $second }
