# machine translated (Russian), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = Этому компьютеру не разрешено открывать { $export } на сервере { $server }.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = На сервере { $server } нет общей папки с именем { $export }.

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = Вас спросят снова в следующий раз

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = Осталось меньше суток

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] Остался { $n } день
        [few] Осталось { $n } дня
        [many] Осталось { $n } дней
       *[other] Осталось { $n } дня
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
folders-shares-held =
    { $past ->
        [yes] Сейчас { $server } находится по адресу { $now }. Раньше ваш вход для него использовался по другому адресу ({ $before }).
       *[no] { $server } находится по адресу { $now }. Ваш вход для него по этому адресу раньше не использовался.
    }{ $full ->
        [yes] {" "}Folders запоминает только последние адреса (не больше { $most }).
       *[no] {""}
    } Любое устройство в сети может назваться этим именем, поэтому Folders не отправил ваш сохранённый вход. Если это ваш { $server }, введите имя пользователя и пароль, чтобы использовать его здесь.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when no address could be found for the computer to show.
folders-shares-nowhere = Folders не удалось найти адрес { $server }, поэтому ему ничего не отправлено. Всё, что вы здесь введёте, нельзя отправить безопасно. Откройте его по адресу, например smb://192.168.1.20/.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
folders-shares-lapsed = { $server } находится по адресу { $now }. Ваш вход для него в последний раз использовался там более { $days } дней назад. С тех пор этот адрес мог достаться другому устройству, поэтому Folders не отправил ваш сохранённый вход. Если это ваш { $server }, введите имя пользователя и пароль, чтобы использовать его здесь.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
folders-shares-unknown = { $server } находится по адресу { $now }. Вход для него там использовался раньше, но Folders не записал, когда и какой именно. С тех пор этот адрес мог достаться другому устройству, поэтому Folders не отправил ваш сохранённый вход. Если это ваш { $server }, введите имя пользователя и пароль, чтобы использовать его здесь.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
folders-shares-unshown = { $server } находится по адресу { $now }, где раньше использовался ваш вход для него. На этот раз Folders нашёл его в локальной сети, где любое устройство может назваться этим именем. Вы раньше не видели этот адрес, поэтому Folders не отправил ваш вход. Если это ваш { $server }, введите имя пользователя и пароль, чтобы использовать его здесь.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
folders-shares-asking =
    Введите имя пользователя и пароль учётной записи на { $server }.{ $heard ->
        [yes] {" "}Он находится по адресу { $at }. Любое устройство в сети может назваться этим именем, поэтому перед вводом убедитесь, что это его адрес.
       *[no] {""}
    }

# SAFETY: needs a human review before it ships
# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
folders-shares-where =
    Введённое уходит на адрес { $address }{ $heard ->
        [yes] {" "}и больше никуда. Если включено «Запомнить», Folders сохраняет ваш вход только для этого адреса.
       *[no] {"."}
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
folders-shares-again = Folders нашёл { $server } в локальной сети по адресу { $at }, порт { $port }. Любое устройство может назваться этим именем, поэтому Folders не отправил ваш вход. Если адрес верный, нажмите «Открыть».

# SAFETY: needs a human review before it ships
# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders не удалось связаться со связкой ключей: { $why }.

# SAFETY: needs a human review before it ships
# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = Ресурс открыт, но Folders не удалось сохранить вход. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = Сервер { $server } не поддерживает NFS версии 4, а Folders открывает только её.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = Папка открыта, но Folders не удалось найти, где она находится на этом компьютере.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = Общий ресурс открыт, но Folders не удалось найти, где он находится на этом компьютере.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = Это не адрес.

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = Адреса, использованные для smb://{ $server }

# SAFETY: needs a human review before it ships
# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
folders-shares-password-label = Пароль для { $user } на { $what }

# SAFETY: needs a human review before it ships
# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
folders-shares-remember = Запомнить на 90 дней

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = 90 дней отсчитываются заново при каждом использовании.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } на { $server }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = Открыть { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Закрыть

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = Имя пользователя, например jo или WORKGROUP\jo

# SAFETY: needs a human review before it ships
# Hint in the empty password box of the sign-in card.
folders-shares-password = Пароль

# SAFETY: needs a human review before it ships
# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
folders-shares-forget = Забыть прежние адреса

# SAFETY: needs a human review before it ships
# Name a screen reader gives the switch that forgets the earlier addresses of a login.
folders-shares-forget-spoken = Забыть прежние адреса для этого входа

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Открывается…

# Name of the main button of the sign-in card.
folders-shares-open = Открыть

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = Это не имя компьютера.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = Это не путь к общей папке.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = Открывается { $export } на { $host }…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = Открывается { $share } на { $server }…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Сначала завершите вход в { $it }.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = Ресурс открыт. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } не предоставляет вам общих папок.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = Закрывается { $name }…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders не удалось закрыть { $name }: { $why }.


# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = Повторное подключение { $name }…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders не удалось сохранить ваши сетевые папки на следующий раз.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = Такое имя пользователя для общей папки не подходит.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = Такой домен для общей папки не подходит.

# Red words when a share's password holds a line break.
folders-shares-bad-password = Пароль общей папки не может содержать перевод строки.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } запрашивает имя пользователя и пароль для { $it }.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = Имя пользователя или пароль для { $it } неверны.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = Ваша учётная запись на { $server } заблокирована или отключена, либо истёк срок действия пароля.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = На { $server } нет общей папки с именем { $share }.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = Этой учётной записи нельзя открывать { $it }.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } не ответил. Возможно, он в спящем режиме или выключен. Разбудите его и попробуйте снова.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders не удалось найти в этой сети компьютер с именем { $server }.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } использует только SMB1, старый и небезопасный способ общего доступа. Folders его не использует.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = Для этого нужен пакет Debian { $package }, но он не установлен.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders не удалось открыть общую папку.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders не удалось открыть общую папку.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders не удалось получить список его общих папок.
