
# machine translated (Russian), not yet reviewed by a person.

# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-card = Folders нашёл { $host } в локальной сети по адресу { $at }, порт { $port }. Любое устройство может назваться так в сети, поэтому Folders не отправил ваш пароль. Если адрес верный, нажмите «Подключить».

# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-stopped = Folders нашёл { $host } в локальной сети по адресу { $at }. Любое устройство может назваться так в сети, поэтому Folders не отправил ваш пароль. Откройте { $host } снова, чтобы проверить его адрес.

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders не удалось разобрать этот адрес: { $why }.

# Red words when a server needs a password or key and none could be asked for.
# SAFETY: needs a human review before it ships
folders-remote-mod-needs-password = Этому серверу нужен пароль или ключ.

# Red words when a server turned down the login.
# SAFETY: needs a human review before it ships
folders-remote-mod-login-refused = Сервер не принял этот вход.

# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-unknown = Вы раньше не подключались к { $host }, поэтому Folders пока не знает его подлинность.

# Red words when a server's identity differs from the one remembered; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-changed = У { $host } другая подлинность, чем в прошлый раз. Возможно, его переустановили, или кто-то перехватывает соединение.

# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-unknown = Folders не удалось проверить сертификат { $host }: { $why }.

# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-changed = Сертификат { $host } изменился с тех пор, как вы ему доверились.

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders не удалось подключиться: { $why }.

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders не удаётся найти { $what }.

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } уже существует.

# Red words when a server or phone refused permission; the variable is what it was refused for.
# SAFETY: needs a human review before it ships
folders-remote-mod-denied = Нет доступа: { $what }.

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } занят другой программой.

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } не поддерживается.

# Red words when a copy or other work with a server was cancelled.
folders-remote-mod-cancelled = Отменено.

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = Переход по ссылкам

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = Копирование в общую папку Windows или из неё до её открытия

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = Копирование в общую папку NFS или из неё до её открытия

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = он не начинается с sftp://, ftp:// или mtp://

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = он должен начинаться с sftp://, ftp://, ftps://, ftpes://, dav://, davs://, smb://, nfs:// или mtp://

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = после [ не хватает ]

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = в нём нет имени сервера

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = имя сервера недопустимо

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = в пути есть перевод строки

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = после имени сервера есть что-то лишнее

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } не подходит как номер порта

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = пропущено, потому что сервер прислал небезопасное имя

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = объект без имени

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = Ответ был слишком большим, поэтому Folders его остановил.

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } теперь папка, поэтому Folders не стал её удалять.
