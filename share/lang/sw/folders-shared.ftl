# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-cancelled = hukuweka nenosiri la msimamizi
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-wrong = nenosiri la msimamizi halikukubaliwa
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-not-allowed = akaunti yako hairuhusiwi kufanya hivyo, hata kwa nenosiri la msimamizi
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unasked = hakukuwa na njia ya kukuomba nenosiri la msimamizi
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unclear = nenosiri la msimamizi halikukubaliwa, au akaunti yako hairuhusiwi kufanya hivyo
# Sentence of its own saying a step was refused: the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-sentence-cancelled = Hukuweka nenosiri la msimamizi
# Sentence of its own saying a step was refused: the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-sentence-wrong = Nenosiri la msimamizi halikukubaliwa
# Sentence of its own saying a step was refused: this login may never do it.
# SAFETY: needs a human review before it ships
common-refused-sentence-not-allowed = Akaunti yako hairuhusiwi kufanya hivyo, hata kwa nenosiri la msimamizi
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
# SAFETY: needs a human review before it ships
common-refused-sentence-unasked = Hakukuwa na njia ya kukuomba nenosiri la msimamizi
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
# SAFETY: needs a human review before it ships
common-refused-sentence-unclear = Nenosiri la msimamizi halikukubaliwa, au akaunti yako hairuhusiwi kufanya hivyo
# Red words when a step was refused because the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-not-done-cancelled = Hilo halikufanyika kwa sababu hukuweka nenosiri la msimamizi.
# Red words when a step was refused because the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-not-done-wrong = Hilo halikufanyika kwa sababu nenosiri la msimamizi halikukubaliwa.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-not-allowed = Hilo halikufanyika. Akaunti yako hairuhusiwi kulifanya, hata kwa nenosiri la msimamizi. Toka kisha ingia tena, kisha jaribu tena.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unasked = Hilo halikufanyika. Hakukuwa na njia ya kukuomba nenosiri la msimamizi. Toka kisha ingia tena, kisha jaribu tena.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unclear = Hilo halikufanyika. Nenosiri la msimamizi halikukubaliwa, au akaunti yako hairuhusiwi kulifanya. Ikiwa hukuombwa nenosiri, toka kisha ingia tena, kisha jaribu tena.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Hilo halikufanya kazi, na hakuna sababu iliyotolewa.
# Red words when the keyring will not open without the person's password.
# SAFETY: needs a human review before it ships
common-keyring-locked = Hifadhi ya funguo imefungwa. Ifungue, au toka kisha ingia tena, kisha jaribu tena.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = Hakuna hifadhi ya funguo kwenye kompyuta hii ya kuihifadhia.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Folders haikuweza kufikia hifadhi ya funguo kwa wakati.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Folders iliacha kusubiri hifadhi ya funguo.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Folders haikuweza kupata nafasi ya kutumia hifadhi ya funguo ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Folders bado inasubiri hifadhi ya funguo, kwa hiyo { $what } haijaondolewa.
       *[other] Folders bado inasubiri hifadhi ya funguo, kwa hiyo { $what } haijahifadhiwa.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] Baada ya kuchelewa, { $what } iliondolewa kwenye hifadhi ya funguo.
       *[other] Baada ya kuchelewa, { $what } ilihifadhiwa kwenye hifadhi ya funguo.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Folders iliacha kusubiri hifadhi ya funguo, kwa hiyo { $what } haikuondolewa.
       *[other] Folders iliacha kusubiri hifadhi ya funguo, kwa hiyo { $what } haikuhifadhiwa.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Folders haikuweza kuondoa { $what } kwenye hifadhi ya funguo. { $why }
       *[other] Folders haikuweza kuhifadhi { $what } kwenye hifadhi ya funguo. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Zaidi
common-more-spoken = Fungua Msaada kuhusu hili
# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
folders-volumes-no-password = Hukuweka nenosiri, kwa hiyo hakuna kilichobadilishwa.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, kwa hiyo hakuna kilichobadilishwa.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = Kuna kitu bado kinaitumia. Funga madirisha au programu zinazoonyesha faili zake, kisha jaribu tena.
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = Hakuna nafasi ya kutosha kwa kilichomo kwa ukubwa huo.
# Heading over the password box, and the tip shown in the box while it is empty.
# SAFETY: needs a human review before it ships
folders-prompt-password = Nenosiri
# Main button at the bottom of the password window that checks the password.
# SAFETY: needs a human review before it ships
folders-prompt-unlock = Fungua
# Button at the bottom of the password window that closes it without unlocking.
# SAFETY: needs a human review before it ships
folders-prompt-cancel = Ghairi
# Shown under the password box when Unlock is pressed with the box empty.
# SAFETY: needs a human review before it ships
folders-prompt-type-first = Andika nenosiri kwanza.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Inakagua…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = kubwa kuliko 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = kubwa kuliko skrini yako
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = zaidi ya MB 20 (MB 200 ikisogea)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = zaidi ya MB 20
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = zaidi ya MB 200
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } au { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Imefungwa: { $name }

# Name given to a drive that has none.
folders-volumes-drive = Diski

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = Nenosiri hilo halikufungua diski.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = Tayari imefunguliwa.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = Unahitaji nenosiri ili kuifungua.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = Nenosiri la { $name }

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Fungua diski

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders haikuweza kufungua dirisha la nenosiri.
