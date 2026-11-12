
# machine translated (French), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = Cet ordinateur n’a pas le droit d’ouvrir { $export } sur { $server }.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } ne partage pas de dossier nommé { $export }.

# SAFETY: needs a human review before it ships
# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = On vous le redemandera la prochaine fois

# SAFETY: needs a human review before it ships
# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = Moins d’un jour restant

# SAFETY: needs a human review before it ships
# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] { $n } jour restant
        [many] { $n } jours restants
       *[other] { $n } jours restants
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
folders-shares-held =
    { $past ->
        [yes] { $server } est maintenant à l’adresse { $now }. Votre connexion pour cet ordinateur a été utilisée à une autre adresse avant ({ $before }).
       *[no] { $server } est à l’adresse { $now }. Votre connexion pour cet ordinateur n’a encore jamais été utilisée à cette adresse.
    }{ $full ->
        [yes] {" "}Folders ne retient que les { $most } dernières adresses.
       *[no] {""}
    } N’importe quel appareil d’un réseau peut prétendre avoir un nom, donc Folders n’a pas envoyé votre connexion enregistrée. Si c’est bien votre { $server }, saisissez son nom d’utilisateur et son mot de passe pour l’utiliser ici.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when no address could be found for the computer to show.
folders-shares-nowhere = Folders n’a pas pu trouver l’adresse de { $server }, donc rien ne lui a été envoyé. Ce que vous saisissez ici ne peut pas être envoyé en toute sécurité. Ouvrez-le plutôt par son adresse, comme smb://192.168.1.20/.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
folders-shares-lapsed = { $server } est à l’adresse { $now }. Votre connexion pour cet ordinateur y a été utilisée pour la dernière fois il y a plus de { $days } jours. Un autre appareil a pu recevoir cette adresse depuis, donc Folders n’a pas envoyé votre connexion enregistrée. Si c’est bien votre { $server }, saisissez son nom d’utilisateur et son mot de passe pour l’utiliser ici.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
folders-shares-unknown = { $server } est à l’adresse { $now }. Une connexion pour cet ordinateur y a été utilisée avant, mais Folders n’a pas noté quand, ni laquelle. Un autre appareil a pu recevoir cette adresse depuis, donc Folders n’a pas envoyé votre connexion enregistrée. Si c’est bien votre { $server }, saisissez son nom d’utilisateur et son mot de passe pour l’utiliser ici.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
folders-shares-unshown = { $server } est à l’adresse { $now }, où votre connexion pour cet ordinateur a été utilisée avant. Cette fois, Folders l’a trouvé sur le réseau local, où n’importe quel appareil peut utiliser ce nom. Vous n’avez encore jamais vu cette adresse, donc Folders n’a pas envoyé votre connexion. Si c’est bien votre { $server }, saisissez son nom d’utilisateur et son mot de passe pour l’utiliser ici.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
folders-shares-asking =
    Saisissez le nom d’utilisateur et le mot de passe d’un compte sur { $server }.{ $heard ->
        [yes] {" "}Il est à l’adresse { $at }. N’importe quel appareil d’un réseau peut prétendre avoir un nom, vérifiez donc que c’est bien son adresse avant de saisir.
       *[no] {""}
    }

# SAFETY: needs a human review before it ships
# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
folders-shares-where =
    Ce que vous saisissez va à { $address }{ $heard ->
        [yes] {" "}et nulle part ailleurs. Avec Mémoriser activé, Folders conserve votre connexion pour cette adresse seulement.
       *[no] {"."}
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
folders-shares-again = Folders a trouvé { $server } sur le réseau local à l’adresse { $at }, port { $port }. N’importe quel appareil peut utiliser ce nom sur un réseau local, donc Folders n’a pas envoyé votre connexion. Si c’est la bonne adresse, appuyez sur Ouvrir.

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders n’a pas pu joindre le trousseau : { $why }.

# SAFETY: needs a human review before it ships
# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = Il est ouvert, mais Folders n’a pas pu enregistrer la connexion. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } ne propose pas NFS version 4, la seule version que Folders peut ouvrir.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = Le partage s’est ouvert, mais Folders n’a pas pu trouver où il se trouve sur cet ordinateur.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = Le partage s’est ouvert, mais Folders n’a pas pu trouver où il se trouve sur cet ordinateur.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = Ce n’est pas une adresse.

# SAFETY: needs a human review before it ships
# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = Adresses utilisées pour smb://{ $server }

# SAFETY: needs a human review before it ships
# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
folders-shares-password-label = Mot de passe de { $user } sur { $what }

# SAFETY: needs a human review before it ships
# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
folders-shares-remember = Mémoriser pendant 90 jours

# SAFETY: needs a human review before it ships
# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = Les 90 jours repartent de zéro à chaque utilisation.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } sur { $server }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = Ouvrir { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Fermer

# SAFETY: needs a human review before it ships
# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = Nom d’utilisateur, comme jo ou WORKGROUP\jo

# SAFETY: needs a human review before it ships
# Hint in the empty password box of the sign-in card.
folders-shares-password = Mot de passe

# SAFETY: needs a human review before it ships
# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
folders-shares-forget = Oublier les adresses précédentes

# SAFETY: needs a human review before it ships
# Name a screen reader gives the switch that forgets the earlier addresses of a login.
folders-shares-forget-spoken = Oublier les adresses précédentes pour cette connexion

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Ouverture…

# Name of the main button of the sign-in card.
folders-shares-open = Ouvrir

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = Ce n’est pas un nom d’ordinateur.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = Ce n’est pas un chemin vers un dossier partagé.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = Ouverture de { $export } sur { $host }…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = Ouverture de { $share } sur { $server }…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Terminez d’abord la connexion à { $it }.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = Il est ouvert. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } ne partage aucun dossier avec vous.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = Fermeture de { $name }…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders n’a pas pu fermer { $name } : { $why }.

# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = Reconnexion à { $name }…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders n'a pas pu enregistrer vos partages pour la prochaine fois.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = Ce nom d'utilisateur ne peut pas être utilisé pour un partage.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = Ce domaine ne peut pas être utilisé pour un partage.

# Red words when a share's password holds a line break.
folders-shares-bad-password = Le mot de passe d'un partage ne peut pas contenir de saut de ligne.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } a besoin d'un nom d'utilisateur et d'un mot de passe pour { $it }.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = Le nom d'utilisateur ou le mot de passe pour { $it } est incorrect.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = Votre compte sur { $server } est verrouillé ou désactivé, ou son mot de passe a expiré.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } n'a pas de dossier partagé nommé { $share }.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = Ce compte n'a pas le droit d'ouvrir { $it }.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } n'a pas répondu. Il est peut-être en veille ou éteint. Réveillez-le, puis réessayez.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders n'a pas trouvé d'ordinateur nommé { $server } sur ce réseau.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } n'utilise que SMB1, une ancienne méthode de partage qui n'est plus sûre. Folders ne l'utilise pas.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = Cela nécessite le paquet { $package } de Debian, qui n'est pas installé.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders n'a pas pu ouvrir le partage.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders n'a pas pu ouvrir le dossier partagé.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders n'a pas pu lister ses dossiers partagés.
