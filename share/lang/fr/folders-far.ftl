# machine translated (French), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = il n'y a aucun serveur nommé { $host }

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = il n'est pas arrivé en entier, donc l'original n'a pas bougé

# Reason a link was not sent to a server.
folders-far-not-followed = c'est un lien, donc Folders ne l'a pas envoyé

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = ce n'est ni un fichier ni un dossier, donc Folders ne l'a pas envoyé

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders n'a pas pu savoir de quel type d'élément il s'agit, il l'a donc laissé sur place

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = Sur ce type de serveur, un fichier peut en remplacer un autre du même nom.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = Vous ne pouvez pas faire cela sur les fichiers d'un serveur

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders n'a pas pu le lire

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders n'a pas pu l'ouvrir.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (un dossier ne peut pas se placer dans lui-même)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (un dossier de ce nom existe déjà)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (un fichier de ce nom existe déjà)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders ne connaît pas l'adresse du serveur.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = la copie envoyée avait une taille incorrecte

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = Cela ne fonctionne qu'avec les fichiers d'un serveur.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders n'a pas pu mettre le nouveau fichier en place, donc l'ancien est toujours là, sous le nom { $old }, et le nouveau s'appelle { $new } : { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders n'a pas pu supprimer l'ancien fichier. Il est toujours là, sous le nom { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders n'a pas pu supprimer l'ancien fichier, qui est donc toujours là, sous le nom { $old } : { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } n'est pas vide.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
# SAFETY: needs a human review before it ships
folders-far-unsafe-name = Folders a ignoré un fichier car son nom n'est pas sûr.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
# SAFETY: needs a human review before it ships
folders-far-unsafe-names =
    { $n ->
        [one] Folders a ignoré un élément car son nom n'est pas sûr : { $names }.
        [many] Folders a ignoré { $n } d'éléments car leurs noms ne sont pas sûrs : { $names }.
       *[other] Folders a ignoré { $n } éléments car leurs noms ne sont pas sûrs : { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = Téléchargement de { $name }…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders n'a pas pu télécharger { $name } : { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = Copie de { $name } ouverte. Quand vous l'enregistrez, vos modifications sont renvoyées au serveur.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = { $name } enregistré sur le serveur.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders n'a pas pu enregistrer { $name } sur le serveur : { $why }. Vos modifications sont toujours dans la copie sur cet ordinateur.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } renommé en { $now }.

# Heading of the Connect to a server card.
folders-far-connect-title = Se connecter à un serveur

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Fermer

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Adresse, par exemple sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
# SAFETY: needs a human review before it ships
folders-far-connect-password = Mot de passe, si nécessaire

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = Fichier de clé, si vous en utilisez un

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = Choisir le fichier de clé


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders utilise vos réglages ssh, votre agent ssh, vos clés et votre liste de serveurs connus.

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = Vous ne vous êtes jamais connecté à { $host }, donc Folders ne peut pas vérifier que c'est le bon. Faites-lui confiance seulement si vous comptiez vous y connecter.

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = { $host } a une identité différente de la dernière fois. Quelqu'un tente peut-être d'intercepter votre connexion, donc Folders ne s'est pas connecté. Si son propriétaire l'a modifiée, supprimez d'abord son ancienne entrée de vos hôtes connus.

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-password = Ce serveur demande un mot de passe. Saisissez-le ci-dessus, puis réessayez.

# Note on the Connect to a server card when a server asks for a key or password.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-key = Ce serveur a besoin d'une clé ou d'un mot de passe. Choisissez votre fichier de clé ci-dessus, ou ajoutez-la à votre agent ssh, puis réessayez.

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = Folders n'a pas pu vérifier le certificat de { $host } : { $why }. Si possible, comparez son empreinte avec celle que vous donne le propriétaire. Ne lui faites confiance que si elles correspondent.

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = Le certificat de { $host } a changé depuis que vous lui avez fait confiance. Son propriétaire l'a peut-être renouvelé, ou quelqu'un tente peut-être d'intercepter votre connexion. Ne lui faites confiance que si le propriétaire confirme cette empreinte.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Connexion

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = Lui faire confiance et se connecter

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Se connecter

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Les téléphones apparaissent dans la barre latérale quand vous les branchez et les déverrouillez.
