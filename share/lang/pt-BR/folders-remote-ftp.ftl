
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = não existe servidor chamado { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } não está aceitando conexões

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } não respondeu a tempo

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = não há como alcançar { $host } na rede

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
# SAFETY: needs a human review before it ships
folders-remote-ftp-no-protect = Este servidor não consegue manter as transferências de arquivos seguras, então o Folders não enviou nada: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = O servidor não respondeu como esperado quando o Folders se conectou: { $said }

# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = Este servidor não oferece FTP seguro. Tente o endereço com ftp:// no início.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = uma conexão segura

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = o login

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = uma conexão de dados

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = o modo de transferência binário

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = a pasta em que você começa

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = Entrar com uma conta FTP

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = O Folders não conseguiu abrir uma conexão de dados com o servidor.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = FTP seguro

# Red words when the server stopped answering.
folders-remote-ftp-stopped = O servidor parou de responder.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = A conexão com o servidor foi perdida.

# Red words when the server closed the connection.
folders-remote-ftp-closed = O servidor fechou a conexão.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = Fazer isso neste servidor

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Algo deu errado com { $what } no servidor: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = o servidor enviou algo que não é uma resposta FTP

