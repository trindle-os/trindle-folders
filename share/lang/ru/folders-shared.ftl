# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

common-refused-why-cancelled = пароль администратора не был введён
common-refused-why-wrong = пароль администратора не был принят
common-refused-why-not-allowed = этой учётной записи это запрещено, какой бы пароль ни ввели
common-refused-why-unasked = некому было запросить пароль администратора
common-refused-why-unclear = пароль администратора не был принят или этой учётной записи это запрещено
common-refused-sentence-cancelled = Пароль администратора не был введён
common-refused-sentence-wrong = Пароль администратора не был принят
common-refused-sentence-not-allowed = Этой учётной записи это запрещено, какой бы пароль ни ввели
common-refused-sentence-unasked = Некому было запросить пароль администратора
common-refused-sentence-unclear = Пароль администратора не был принят или этой учётной записи это запрещено
common-refused-not-done-cancelled = Не выполнено: пароль администратора не был введён.
common-refused-not-done-wrong = Не выполнено: пароль администратора не был принят.
common-refused-not-done-not-allowed = Не выполнено: этой учётной записи это запрещено, какой бы пароль ни ввели. Выйдите из системы и войдите снова, затем повторите попытку.
common-refused-not-done-unasked = Не выполнено: некому было запросить пароль администратора. Выйдите из системы и войдите снова, затем повторите попытку.
common-refused-not-done-unclear = Не выполнено: пароль администратора не был принят или этой учётной записи это запрещено. Если пароль не запрашивали, выйдите из системы и войдите снова, затем повторите попытку.
common-refused-no-reason = Не выполнено: шаг не удался, причина не указана.
common-keyring-locked = Связка ключей заблокирована. Выйдите из системы и войдите снова или разблокируйте её, затем повторите попытку.
common-keyring-missing = На этом компьютере нет связки ключей, чтобы хранить это.
common-keyring-silent = Связка ключей не ответила вовремя.
common-keyring-never = Связка ключей так и не ответила.
common-keyring-turn-failed = Не удалось обратиться к связке ключей ({ $why }), поэтому ничего у неё не запрошено.
common-keyring-waiting =
    { $action ->
        [removing] Связка ключей пока не ответила, поэтому { $what } пока не удалено. Если связка всё же удалит это, здесь будет сказано.
       *[other] Связка ключей пока не ответила, поэтому { $what } пока не сохранено. Если связка всё же сохранит это, здесь будет сказано.
    }
common-keyring-late-done =
    { $action ->
        [removing] Связка ключей ответила с опозданием: { $what } удалено.
       *[other] Связка ключей ответила с опозданием: { $what } сохранено.
    }
common-keyring-late-never =
    { $action ->
        [removing] Связка ключей так и не ответила, поэтому { $what } не удалено.
       *[other] Связка ключей так и не ответила, поэтому { $what } не сохранено.
    }
common-keyring-late-refused =
    { $action ->
        [removing] Связка ключей ответила с опозданием и не удалила { $what }: { $why }
       *[other] Связка ключей ответила с опозданием и не сохранила { $what }: { $why }
    }
common-more = Ещё
common-more-spoken = Подробнее об этом, открывает справку
folders-volumes-no-password = Пароль не введён, поэтому ничего не изменилось.
folders-volumes-nothing-changed = { $reason }, поэтому ничего не изменилось.
folders-volumes-busy = Что-то ещё использует его. Закройте окна и программы, показывающие его файлы, и повторите.
folders-volumes-no-room = Для того, что на нём есть, не хватит места при таком размере.
folders-prompt-password = Пароль
folders-prompt-unlock = Разблокировать
folders-prompt-cancel = Отмена
folders-prompt-type-first = Сначала введите пароль
folders-prompt-checking = Проверка…
folders-wallpaper-larger-4k = больше 4K
folders-wallpaper-larger-screen = больше вашего экрана
folders-wallpaper-over-20-or-200 = больше 20 МБ (200 МБ для движущихся)
folders-wallpaper-over-20 = больше 20 МБ
folders-wallpaper-over-200 = больше 200 МБ
folders-wallpaper-reasons-or = { $first } или { $second }
