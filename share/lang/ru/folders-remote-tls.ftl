# machine translated (Russian), not yet reviewed by a person.

# Reason shown on the certificate card when this computer has no trusted certificates to check against; a sentence follows "could not be checked:".
folders-remote-tls-no-roots = на этом компьютере нет доверенных сертификатов для проверки

# Reason shown on the certificate card when nobody this computer trusts signed the certificate.
folders-remote-tls-unknown-issuer = он не подписан никем, кому доверяет этот компьютер. Так бывает, когда сервер выпускает сертификат сам для себя

# Reason shown on the certificate card when the certificate has run out.
folders-remote-tls-expired = срок его действия истёк

# Reason shown on the certificate card when the certificate is not valid yet.
folders-remote-tls-not-yet = он ещё не действует

# Reason shown on the certificate card when the certificate was made for another server name.
folders-remote-tls-wrong-name = он выпущен для другого имени сервера

# Reason shown on the certificate card when the certificate has been withdrawn.
folders-remote-tls-revoked = он аннулирован

# Reason shown on the certificate card when the certificate failed the usual checks for another reason.
folders-remote-tls-failed-checks = он не прошёл обычные проверки

# Red words when the secure connection settings could not be made; the variable is the reason the system gave.
folders-remote-tls-setup-failed = Folders не удалось подготовить защищённые подключения: { $why }

# Red words when the server name cannot go in a certificate check; the variable is the server name.
folders-remote-tls-bad-name = { $host } не подходит как имя сервера для защищённого подключения

# Red words when a secure connection could not be started; the variable is the reason the system gave.
folders-remote-tls-start-failed = Folders не удалось начать защищённое подключение: { $why }

# Red words when the server stopped answering while the secure link was being set up.
folders-remote-tls-stopped = Сервер перестал отвечать, пока Folders устанавливал защищённое подключение.

# Red words when the server closed the connection while the secure link was being set up.
folders-remote-tls-closed = Сервер закрыл соединение, пока Folders устанавливал защищённое подключение.

# Red words when the secure link could not be set up for another reason; the variable is the reason the system gave.
folders-remote-tls-could-not-set-up = Folders не удалось установить защищённое подключение: { $why }

