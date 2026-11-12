
# machine translated (French), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = il n’y a aucun serveur nommé { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } n’accepte pas les connexions

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } n’a pas répondu à temps

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = il n’y a aucun moyen d’atteindre { $host } sur le réseau

# SAFETY: needs a human review before it ships
# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
folders-remote-ftp-no-protect = Ce serveur ne peut pas sécuriser les transferts de fichiers, donc Folders n’a rien envoyé : { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Le serveur n’a pas répondu comme prévu quand Folders s’est connecté : { $said }

# SAFETY: needs a human review before it ships
# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = Ce serveur ne propose pas le FTP sécurisé. Essayez l’adresse avec ftp:// au début.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = une connexion sécurisée

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = la connexion

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = une connexion de données

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = le mode de transfert binaire

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = le dossier de départ

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = La connexion avec un compte FTP

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders n’a pas pu ouvrir de connexion de données avec le serveur.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = Le FTP sécurisé

# Red words when the server stopped answering.
folders-remote-ftp-stopped = Le serveur a cessé de répondre.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = La connexion au serveur a été perdue.

# Red words when the server closed the connection.
folders-remote-ftp-closed = Le serveur a fermé la connexion.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = Faire cela sur ce serveur

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Un problème est survenu avec { $what } sur le serveur : { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = le serveur a envoyé quelque chose qui n’est pas une réponse FTP
