
# machine translated (French), not yet reviewed by a person.

# Red words when a thing on a server changed just before it was to be removed and was left alone; the variable is its name.
folders-remote-dav-changed-at-removal = { $name } (non supprimé car il venait de changer)

# Reason in red words when the server's reply is cut into pieces and one has no size.
folders-remote-dav-chunk = une partie de la réponse du serveur n’avait pas de taille

# Reason in red words ("Could not reach it: ...") when a server's name has no address; the variables are the name and the system's reason.
folders-remote-dav-not-found = { $host } est introuvable ({ $why })

# Reason in red words ("Could not reach it: ...") when a server's name gives no address; the variable is the name.
folders-remote-dav-no-address = il n’y a pas d’adresse pour { $host } sur le réseau

# Red words when the address opened names a file, not a folder; the variable is the path.
folders-remote-dav-is-file = { $path } est un fichier, pas un dossier.

# Red words when a file being sent ended before all of it was sent.
folders-remote-dav-file-ended = Le fichier s’est terminé avant d’avoir été envoyé en entier.

# SAFETY: needs a human review before it ships
# Red words when the server wants a kind of login Folders cannot do.
folders-remote-dav-unknown-login = Le serveur utilise un type de connexion que Folders ne prend pas en charge.

# Red words when the server does not list folders over WebDAV.
folders-remote-dav-no-listing = Ce serveur ne permet pas à Folders de lister les dossiers par WebDAV.

# Red words when the server did not answer like a web server.
folders-remote-dav-not-web = Le serveur n’a pas répondu comme un serveur web devrait le faire.

# Red words when the server has no room left.
folders-remote-dav-full = Le serveur est plein.

# Red words when the server answered with a number Folders has no words for; the variables are the number and the name of the thing.
folders-remote-dav-status = Le serveur a répondu avec le code { $status } pour { $name }.

# Red words when the server's folder listing could not be read; the variable is the reason.
folders-remote-dav-listing-unreadable = Folders n’a pas pu lire la liste des dossiers : { $why }

# Red words when a file being fetched ended before all of it came.
folders-remote-dav-cut-short = Seule une partie du fichier est arrivée.

# Red words when a folder to be removed still holds something; the variable is its name.
folders-remote-dav-not-empty = { $name } n’est pas vide.
