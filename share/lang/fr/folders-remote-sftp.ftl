
# machine translated (French), not yet reviewed by a person.

# Red words when the server sent a message Folders could not read.
folders-remote-sftp-nonsense = Le serveur a envoyé un message que Folders n’a pas pu lire.

# Red words when the ssh program is not on this computer.
folders-remote-sftp-no-ssh = Le programme ssh n’est pas installé.

# Red words when the ssh program could not be started.
folders-remote-sftp-ssh-failed = Folders n’a pas pu lancer ssh.

# The thing that cannot be done, in "... is not supported yet.": a server's old version of SFTP.
folders-remote-sftp-old = La version plus ancienne de SFTP de ce serveur

# Red words when the connection to the server was lost.
folders-remote-sftp-lost = La connexion au serveur a été perdue.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-sftp-that-here = Faire cela sur ce serveur

# Red words when the server refused something; the variable is the path it was about.
folders-remote-sftp-could-not = Le serveur n’a pas pu faire cela avec { $path }.

# Red words when the server refused something and gave a reason; the variables are the path and the server's own words.
folders-remote-sftp-could-not-said = Le serveur n’a pas pu faire cela avec { $path } : { $said }.

# Red words when the key file picked is missing; the variable is its path.
folders-remote-sftp-no-key = Folders ne trouve pas le fichier de clé { $key }.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-sftp-no-server = il n’y a aucun serveur nommé { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-sftp-turned-away = la connexion à { $host } a été refusée

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-sftp-no-answer = { $host } n’a pas répondu à temps

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-sftp-no-way = il n’y a aucun moyen d’atteindre { $host } sur le réseau

# The thing that cannot be done, in "... is not supported yet.": SFTP on a server that does not have it.
folders-remote-sftp-none-here = Ce serveur ne propose pas SFTP.

# Red words passing on what the ssh program said; the variable is its last line.
folders-remote-sftp-ssh-said = Message de ssh : { $said }
