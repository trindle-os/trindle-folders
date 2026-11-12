
# machine translated (Russian), not yet reviewed by a person.

# Red words when the server sent a message Folders could not read.
folders-remote-sftp-nonsense = Сервер прислал сообщение, которое Folders не смог прочитать.

# Red words when the ssh program is not on this computer.
folders-remote-sftp-no-ssh = Программа ssh не установлена.

# Red words when the ssh program could not be started.
folders-remote-sftp-ssh-failed = Folders не удалось запустить ssh.

# The thing that cannot be done, in "... is not supported yet.": a server's old version of SFTP.
folders-remote-sftp-old = Более старая версия SFTP на этом сервере

# Red words when the connection to the server was lost.
folders-remote-sftp-lost = Соединение с сервером потеряно.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-sftp-that-here = Это действие на этом сервере

# Red words when the server refused something; the variable is the path it was about.
folders-remote-sftp-could-not = Сервер не смог сделать это с { $path }.

# Red words when the server refused something and gave a reason; the variables are the path and the server's own words.
folders-remote-sftp-could-not-said = Сервер не смог сделать это с { $path }: { $said }.

# Red words when the key file picked is missing; the variable is its path.
folders-remote-sftp-no-key = Folders не находит файл ключа { $key }.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-sftp-no-server = нет сервера с именем { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-sftp-turned-away = соединение с { $host } отклонено

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-sftp-no-answer = { $host } не ответил вовремя

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-sftp-no-way = не удаётся добраться до { $host } по сети

# The thing that cannot be done, in "... is not supported yet.": SFTP on a server that does not have it.
folders-remote-sftp-none-here = Этот сервер не предоставляет SFTP.

# Red words passing on what the ssh program said; the variable is its last line.
folders-remote-sftp-ssh-said = Сообщение от ssh: { $said }
