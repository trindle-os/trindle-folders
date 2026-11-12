
# machine translated (Russian), not yet reviewed by a person.

# Red words when a thing on a server changed just before it was to be removed and was left alone; the variable is its name.
folders-remote-dav-changed-at-removal = { $name } (не удалено, потому что только что изменилось)

# Reason in red words when the server's reply is cut into pieces and one has no size.
folders-remote-dav-chunk = в части ответа сервера не указан размер

# Reason in red words ("Could not reach it: ...") when a server's name has no address; the variables are the name and the system's reason.
folders-remote-dav-not-found = { $host } не найден ({ $why })

# Reason in red words ("Could not reach it: ...") when a server's name gives no address; the variable is the name.
folders-remote-dav-no-address = для { $host } в сети нет адреса

# Red words when the address opened names a file, not a folder; the variable is the path.
folders-remote-dav-is-file = { $path } - это файл, а не папка.

# Red words when a file being sent ended before all of it was sent.
folders-remote-dav-file-ended = Файл закончился раньше, чем был отправлен целиком.

# Red words when the server wants a kind of login Folders cannot do.
folders-remote-dav-unknown-login = Сервер использует способ входа, который Folders не поддерживает.

# Red words when the server does not list folders over WebDAV.
folders-remote-dav-no-listing = Этот сервер не позволяет Folders показывать списки папок по WebDAV.

# Red words when the server did not answer like a web server.
folders-remote-dav-not-web = Сервер ответил не так, как должен отвечать веб-сервер.

# Red words when the server has no room left.
folders-remote-dav-full = На сервере нет места.

# Red words when the server answered with a number Folders has no words for; the variables are the number and the name of the thing.
folders-remote-dav-status = Сервер вернул код { $status } для «{ $name }».

# Red words when the server's folder listing could not be read; the variable is the reason.
folders-remote-dav-listing-unreadable = Folders не удалось прочитать список папок: { $why }

# Red words when a file being fetched ended before all of it came.
folders-remote-dav-cut-short = Пришла только часть файла.

# Red words when a folder to be removed still holds something; the variable is its name.
folders-remote-dav-not-empty = Папка «{ $name }» не пуста.
