

folders-shares-nfs-denied = { $server } ne laisse pas cet ordinateur ouvrir { $export }.

folders-shares-nfs-no-share = { $server } ne partage rien qui s’appelle { $export }.

folders-shares-left-none = Sera redemandée à la prochaine ouverture

folders-shares-left-under-day = Moins d’un jour restant

folders-shares-left-days =
    { $n ->
        [one] { $n } jour restant
        [many] { $n } jours restants
       *[other] { $n } jours restants
    }

folders-shares-held =
    { $past ->
        [yes] { $server } répond maintenant à l’adresse { $now }, différente de celle où votre connexion pour cet ordinateur a été utilisée avant ({ $before }).
       *[no] { $server } répond à l’adresse { $now }, une adresse où votre connexion pour cet ordinateur n’a jamais été utilisée.
    }{ $full ->
        [yes] {" "}Seules les { $most } dernières adresses où elle a été utilisée sont retenues.
       *[no] {""}
    } N’importe quel appareil d’un réseau peut répondre à un nom, donc la connexion mémorisée pour cet ordinateur n’a pas été envoyée. Si c’est bien votre { $server }, saisissez son nom d’utilisateur et son mot de passe pour l’utiliser ici.

folders-shares-nowhere = L’adresse à laquelle { $server } répond n’a pas pu être trouvée à l’avance, donc rien ne lui a été envoyé et ce qui est saisi ici n’a aucun endroit sûr où aller. Ouvrez-le plutôt par son adresse, comme smb://192.168.1.20/.

folders-shares-lapsed = { $server } répond à l’adresse { $now }, où votre connexion pour cet ordinateur a été utilisée pour la dernière fois il y a plus de { $days } jours. Un autre appareil a pu recevoir cette adresse depuis, donc la connexion mémorisée n’a pas été envoyée. Si c’est bien votre { $server }, saisissez son nom d’utilisateur et son mot de passe pour l’utiliser ici.

folders-shares-unknown = { $server } répond à l’adresse { $now }, où une connexion pour cet ordinateur a été utilisée avant, mais quand, ou laquelle, n’a pas été noté. Un autre appareil a pu recevoir cette adresse depuis, donc la connexion mémorisée n’a pas été envoyée. Si c’est bien votre { $server }, saisissez son nom d’utilisateur et son mot de passe pour l’utiliser ici.

folders-shares-unshown = { $server } répond à l’adresse { $now }, celle à laquelle votre connexion pour cet ordinateur est allée plus tôt. Cette fois, cette adresse a seulement été entendue sur le réseau, où n’importe quel appareil peut répondre à un nom, et elle ne vous a pas été montrée ici avant, donc la connexion n’a pas été envoyée. Si c’est bien votre { $server }, saisissez son nom d’utilisateur et son mot de passe pour l’utiliser ici.

folders-shares-asking =
    { $server } demande un nom d’utilisateur et un mot de passe. Ce sont ceux d’un compte sur cet ordinateur.{ $heard ->
        [yes] {" "}Il répond à l’adresse { $at }. N’importe quel appareil d’un réseau peut répondre à un nom, vérifiez donc que c’est bien son adresse avant de saisir.
       *[no] {""}
    }

folders-shares-where =
    Ce que vous saisissez va à { $address }{ $heard ->
        [yes] {" "}et nulle part ailleurs. Avec Mémoriser activé, votre connexion est conservée pour cette adresse seulement.
       *[no] {"."}
    }

folders-shares-again = { $server } répond à l’adresse { $at }, port { $port }. Cette adresse a seulement été entendue sur le réseau, où n’importe quel appareil peut répondre à un nom, donc votre connexion pour cet ordinateur n’a pas été envoyée. Si c’est bien son adresse, appuyez sur Ouvrir.

folders-shares-keyring-unreachable = Le trousseau n’a pas pu être atteint : { $why }.

folders-shares-unkept = Il est ouvert, mais la connexion n’a pas été mémorisée. { $why }

folders-shares-nfs-version = { $server } ne propose pas NFS version 4, la seule que Trindle ouvre.

folders-shares-nfs-opened-unknown = Il s’est ouvert, mais son emplacement est inconnu.

folders-shares-opened-unknown = Le partage s’est ouvert, mais son emplacement est inconnu.

folders-shares-not-address = Ce n’est pas une adresse.

folders-shares-seen-label = Adresses où smb://{ $server } a été ouvert

folders-shares-password-label = Mot de passe de { $user } sur { $what }

folders-shares-remember = Mémoriser pendant 90 jours

folders-shares-remember-line = Repart de zéro à chaque utilisation.

folders-shares-share-on = { $share } sur { $server }

folders-shares-card-title = Ouvrir { $it }

folders-shares-close = Fermer

folders-shares-user-placeholder = Nom d’utilisateur, comme jo, ou WORKGROUP\jo

folders-shares-password = Mot de passe

folders-shares-forget = Oublier les adresses précédentes

folders-shares-forget-spoken = Oublier les adresses où cette connexion a été utilisée avant

folders-shares-opening = Ouverture

folders-shares-open = Ouvrir

folders-shares-bad-computer = Ce n’est pas le nom d’un ordinateur.

folders-shares-bad-path = Ce n’est pas le chemin d’un dossier partagé.

folders-shares-opening-export = Ouverture de { $export } sur { $host }…

folders-shares-opening-share = Ouverture de { $share } sur { $server }…

folders-shares-answer-card = Répondez d’abord à la carte de { $it }.

folders-shares-open-waiting = Il est ouvert. { $words }

folders-shares-none = { $server } ne partage aucun dossier avec vous.

folders-shares-closing = Fermeture de { $name }…

folders-shares-close-failed = { $name } n’a pas pu être fermé : { $why }.

folders-shares-reconnecting = Reconnexion à { $name }…

folders-shares-not-remembered = Les partages n'ont pas pu être mémorisés pour la prochaine fois.
