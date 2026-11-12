
# machine translated (Russian), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = нет сервера с именем { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } не принимает подключения

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } не ответил вовремя

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = не удаётся добраться до { $host } по сети

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
# SAFETY: needs a human review before it ships
folders-remote-ftp-no-protect = Этот сервер не может защитить передачу файлов, поэтому Folders ничего не отправил: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Сервер ответил не так, как ожидалось, когда Folders подключился: { $said }

# Red words when the server does not offer secure FTP.
# SAFETY: needs a human review before it ships
folders-remote-ftp-no-secure = Этот сервер не поддерживает защищённый FTP. Попробуйте адрес с ftp:// в начале.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = защищённого подключения

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = входа

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = подключения для данных

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = двоичного режима передачи

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = стартовой папки

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = Вход с учётной записью FTP

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders не удалось открыть подключение для данных к серверу.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = Защищённый FTP

# Red words when the server stopped answering.
folders-remote-ftp-stopped = Сервер перестал отвечать.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = Подключение к серверу потеряно.

# Red words when the server closed the connection.
folders-remote-ftp-closed = Сервер закрыл подключение.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = Это действие на этом сервере

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Сбой { $what } на сервере: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = сервер прислал что-то, что не похоже на ответ FTP

