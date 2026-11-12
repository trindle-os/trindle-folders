
# machine translated (French), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
folders-remote-mod-heard-card = Folders a trouvé { $host } sur le réseau local à l’adresse { $at }, port { $port }. N’importe quel appareil peut utiliser ce nom sur un réseau local, donc Folders n’a pas envoyé votre mot de passe. Si c’est la bonne adresse, appuyez sur Se connecter.

# SAFETY: needs a human review before it ships
# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
folders-remote-mod-heard-stopped = Folders a trouvé { $host } sur le réseau local à l’adresse { $at }. N’importe quel appareil peut utiliser ce nom sur un réseau local, donc Folders n’a pas envoyé votre mot de passe. Rouvrez { $host } pour vérifier son adresse.

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders n’a pas pu lire cette adresse : { $why }.

# SAFETY: needs a human review before it ships
# Red words when a server needs a password or key and none could be asked for.
folders-remote-mod-needs-password = Ce serveur demande un mot de passe ou une clé.

# SAFETY: needs a human review before it ships
# Red words when a server turned down the login.
folders-remote-mod-login-refused = Le serveur n’a pas accepté cette connexion.

# SAFETY: needs a human review before it ships
# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
folders-remote-mod-host-key-unknown = Vous ne vous êtes encore jamais connecté à { $host }, donc Folders ne connaît pas encore son identité.

# SAFETY: needs a human review before it ships
# Red words when a server's identity differs from the one remembered; the variable is the server's name.
folders-remote-mod-host-key-changed = { $host } a une identité différente de la dernière fois. Il a peut-être été reconstruit, ou quelqu’un intercepte peut-être la connexion.

# SAFETY: needs a human review before it ships
# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
folders-remote-mod-cert-unknown = Folders n’a pas pu vérifier le certificat de { $host } : { $why }.

# SAFETY: needs a human review before it ships
# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
folders-remote-mod-cert-changed = Le certificat de { $host } a changé depuis que vous lui avez fait confiance.

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders n’a pas pu se connecter : { $why }.

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders ne trouve pas { $what }.

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } existe déjà.

# SAFETY: needs a human review before it ships
# Red words when a server or phone refused permission; the variable is what it was refused for.
folders-remote-mod-denied = Vous n’avez pas l’autorisation d’utiliser { $what }.

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } est utilisé par un autre programme.

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } n’est pas pris en charge.

# Red words when a copy or other work with a server was cancelled.
folders-remote-mod-cancelled = Annulé.

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = Le suivi des liens

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = La copie vers ou depuis un partage Windows avant son ouverture

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = La copie vers ou depuis un partage NFS avant son ouverture

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = elle ne commence pas par sftp://, ftp:// ou mtp://

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = elle doit commencer par sftp://, ftp://, ftps://, ftpes://, dav://, davs://, smb://, nfs:// ou mtp://

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = un [ n’a pas son ]

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = elle ne nomme pas de serveur

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = le nom du serveur n’est pas valide

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = le chemin contient un saut de ligne

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = il y a quelque chose en trop après le nom du serveur

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } n’est pas un numéro de port valide

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = ignoré car le serveur a envoyé un nom dangereux

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = un élément sans nom

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = La réponse était beaucoup trop volumineuse, donc Folders l’a arrêtée.

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } est maintenant un dossier, donc Folders ne l'a pas supprimé.
