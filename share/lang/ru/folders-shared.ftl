# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = вы не ввели пароль администратора
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = пароль администратора не был принят
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = вашей учётной записи это запрещено, даже с паролем администратора
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = у вас не было возможности запросить пароль администратора
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = пароль администратора не был принят или вашей учётной записи это запрещено
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = Вы не ввели пароль администратора
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = Пароль администратора не был принят
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = Вашей учётной записи это запрещено, даже с паролем администратора
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = Не было возможности запросить у вас пароль администратора
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = Пароль администратора не был принят или вашей учётной записи это запрещено
# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = Это не выполнено, потому что вы не ввели пароль администратора.
# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = Это не выполнено, потому что пароль администратора не был принят.
# SAFETY: needs a human review before it ships
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = Это не выполнено. Вашей учётной записи это запрещено, даже с паролем администратора. Выйдите из системы и войдите снова, затем повторите попытку.
# SAFETY: needs a human review before it ships
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = Это не выполнено. Не было возможности запросить у вас пароль администратора. Выйдите из системы и войдите снова, затем повторите попытку.
# SAFETY: needs a human review before it ships
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = Это не выполнено. Пароль администратора не был принят, или вашей учётной записи это запрещено. Если пароль у вас не запрашивали, выйдите из системы и войдите снова, затем повторите попытку.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Не получилось, причина не указана.
# SAFETY: needs a human review before it ships
# Red words when the keyring will not open without the person's password.
common-keyring-locked = Связка ключей заблокирована. Разблокируйте её или выйдите из системы и войдите снова, затем повторите попытку.
# SAFETY: needs a human review before it ships
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = На этом компьютере нет связки ключей, чтобы это сохранить.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Folders не смог вовремя связаться со связкой ключей.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Folders перестал ждать ответа связки ключей.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Folders не получил доступ к связке ключей ({ $why }).
# SAFETY: needs a human review before it ships
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Folders всё ещё ждёт ответа связки ключей, поэтому { $what } пока не удалено.
       *[other] Folders всё ещё ждёт ответа связки ключей, поэтому { $what } пока не сохранено.
    }
# SAFETY: needs a human review before it ships
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] После задержки { $what } удалено из связки ключей.
       *[other] После задержки { $what } сохранено в связке ключей.
    }
# SAFETY: needs a human review before it ships
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Folders перестал ждать ответа связки ключей, поэтому { $what } не удалено.
       *[other] Folders перестал ждать ответа связки ключей, поэтому { $what } не сохранено.
    }
# SAFETY: needs a human review before it ships
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Folders не удалось удалить { $what } из связки ключей. { $why }
       *[other] Folders не удалось сохранить { $what } в связке ключей. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Ещё
common-more-spoken = Открыть справку об этом
# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
folders-volumes-no-password = Вы не ввели пароль, поэтому ничего не изменено.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, поэтому ничего не изменилось.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = Что-то ещё использует его. Закройте окна и программы, показывающие его файлы, и повторите.
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = Для того, что на нём есть, не хватит места при таком размере.
# Heading over the password box, and the tip shown in the box while it is empty.
# SAFETY: needs a human review before it ships
folders-prompt-password = Пароль
# Main button at the bottom of the password window that checks the password.
# SAFETY: needs a human review before it ships
folders-prompt-unlock = Разблокировать
# Button at the bottom of the password window that closes it without unlocking.
# SAFETY: needs a human review before it ships
folders-prompt-cancel = Отмена
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = Сначала введите пароль.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Проверка…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = больше 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = больше вашего экрана
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = больше 20 МБ (200 МБ для анимированных)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = больше 20 МБ
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = больше 200 МБ
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } или { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Заперт: { $name }

# Name given to a drive that has none.
folders-volumes-drive = Диск

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = Эта парольная фраза не разблокировала накопитель.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = Он уже открыт.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = Чтобы открыть его, нужна парольная фраза.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = Парольная фраза для { $name }

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Разблокировать диск

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders не удалось открыть окно ввода парольной фразы.
