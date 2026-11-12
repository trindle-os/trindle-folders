# machine translated (Russian), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = сервер «{ $host }» не найден

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = файл дошёл не весь, поэтому оригинал остался на месте

# Reason a link was not sent to a server.
folders-far-not-followed = это ссылка, поэтому она не отправлена

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = это не файл и не папка, поэтому оно не отправлено

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = не удалось определить, что это за элемент, поэтому он оставлен на месте

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = На сервере такого типа один файл может заменить другой с тем же именем.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = Так нельзя поступать с файлами на сервере

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = не удалось его прочитать

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Не удалось открыть.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (папку нельзя поместить внутрь самой себя)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (там уже есть папка с таким именем)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (там уже есть файл с таким именем)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders не знает адрес сервера.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = отправленная копия оказалась другого размера

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = Это работает только с файлами на сервере.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders не удалось поставить новый файл на место: старый файл остался и теперь называется { $old }, а новый называется { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders не удалось удалить старый файл. Он остался на месте и теперь называется { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders не удалось удалить старый файл, он остался на месте и теперь называется { $old }: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = Папка { $name } не пуста.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders пропустил файл, так как его имя небезопасно.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
        [one] Folders пропустил { $n } элемент из-за небезопасного имени: { $names }.
        [few] Folders пропустил { $n } элемента из-за небезопасных имён: { $names }.
        [many] Folders пропустил { $n } элементов из-за небезопасных имён: { $names }.
       *[other] Folders пропустил { $n } элемента из-за небезопасных имён: { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = Загружается { $name }…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Не удалось загрузить { $name }: { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = Открыта копия файла { $name }. Когда вы сохраните её, изменения отправятся обратно на сервер.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = Файл { $name } сохранён на сервере.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Не удалось сохранить { $name } на сервере: { $why }. Ваши изменения остались в копии на этом компьютере.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } переименовано в { $now }.

# Heading of the Connect to a server card.
folders-far-connect-title = Подключиться к серверу

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Закрыть

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Адрес, например sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
# SAFETY: needs a human review before it ships
folders-far-connect-password = Пароль, если он нужен

# Placeholder of the box on the Connect to a server card where the key file is named.
# SAFETY: needs a human review before it ships
folders-far-connect-key = Файл ключа, если вы им пользуетесь

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = Выбрать файл ключа


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders использует ваши настройки ssh, агент ssh, ключи и список известных серверов.

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = К серверу { $host } вы ещё не подключались, поэтому Folders не может проверить, тот ли это сервер. Доверяйте ему, только если вы ожидали подключения именно к нему.

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = У сервера { $host } другая идентификация, чем в прошлый раз. Возможно, кто-то пытается перехватить ваше соединение, поэтому Folders не подключился. Если владелец её изменил, сначала удалите старую запись из списка известных серверов.

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-password = Этому серверу нужен пароль. Введите его выше и попробуйте снова.

# Note on the Connect to a server card when a server asks for a key or password.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-key = Этому серверу нужен ключ или пароль. Выберите файл ключа выше или добавьте его в агент ssh, затем повторите попытку.

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = Folders не удалось проверить сертификат сервера { $host }: { $why }. Если возможно, сравните его отпечаток с тем, который дал вам владелец сервера. Доверяйте сертификату, только если они совпадают.

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = Сертификат сервера { $host } изменился с тех пор, как вы ему доверились. Возможно, владелец его обновил, а возможно, кто-то пытается перехватить ваше соединение. Доверяйте новому сертификату, только если владелец подтвердил этот отпечаток.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Подключение

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = Доверять и подключиться

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Подключиться

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Телефоны появляются на боковой панели, когда вы их подключаете и разблокируете.
