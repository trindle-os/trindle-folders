

folders-sharing-cannot-read = Ce dossier ne peut pas être lu.

folders-sharing-only-folder = Seul un dossier lui-même peut être partagé, pas un lien vers un dossier.

folders-sharing-only-yours = Seuls les dossiers qui vous appartiennent peuvent être partagés. Celui-ci appartient à quelqu’un d’autre.

folders-sharing-your-home = votre dossier personnel

folders-sharing-this-folder = ce dossier

folders-sharing-and = { $first } et { $second }

folders-sharing-share = Partager ce dossier

folders-sharing-shared = Partagé sur le réseau domestique.

folders-sharing-not-shared = Non partagé.

folders-sharing-share-tip = Partager ce dossier avec les autres ordinateurs du réseau domestique

folders-sharing-name = Nom

folders-sharing-name-tip = Le nom que voient les autres ordinateurs

folders-sharing-write = Les autres peuvent modifier les fichiers

folders-sharing-write-sub = Ils peuvent y ajouter, modifier et supprimer des fichiers.

folders-sharing-guests = Laisser entrer tout le monde

folders-sharing-guests-sub = Aucun mot de passe n’est demandé.

folders-sharing-guests-tip = Laisser entrer tout le monde, sans mot de passe

folders-sharing-explain =
    { $fetch ->
        [yes] Une fois partagé, il apparaît sous Réseau sur les autres ordinateurs de la maison, y compris ceux sous Windows. La première fois, Trindle récupère Samba, le programme qui partage les dossiers, ce qui prend une minute.
       *[no] Une fois partagé, il apparaît sous Réseau sur les autres ordinateurs de la maison, y compris ceux sous Windows.
    }

folders-sharing-find-it =
    { $host_known ->
        [yes] Les autres ordinateurs le trouvent sous Réseau, sous le nom { $name } sur { $host }.
       *[no] Les autres ordinateurs le trouvent sous Réseau, sous le nom { $name }.
    }

folders-sharing-cannot-reach = Les autres ordinateurs ne peuvent pas encore l’atteindre : Laisser les autres ordinateurs se connecter est désactivé dans Réglages, Réseau.

folders-sharing-let-connect = Laisser les autres ordinateurs se connecter

folders-sharing-visitors =
    { $n ->
        [one] Les visiteurs sans mot de passe ne peuvent pas encore l’ouvrir : { $names } est fermé aux autres comptes. Les laisser passer n’ouvre que le chemin vers ce dossier ; ils ne peuvent pas voir le reste.
        [many] Les visiteurs sans mot de passe ne peuvent pas encore l’ouvrir : { $names } sont fermés aux autres comptes. Les laisser passer n’ouvre que le chemin vers ce dossier ; ils ne peuvent pas voir le reste.
       *[other] Les visiteurs sans mot de passe ne peuvent pas encore l’ouvrir : { $names } sont fermés aux autres comptes. Les laisser passer n’ouvre que le chemin vers ce dossier ; ils ne peuvent pas voir le reste.
    }

folders-sharing-let-through = Laisser les visiteurs accéder à ce dossier

folders-sharing-sign-in-set = Les autres se connectent en tant que { $user }, avec le mot de passe défini pour le partage.

folders-sharing-change-password = Changer le mot de passe utilisé par les autres ordinateurs

folders-sharing-set-password = Les autres se connectent en tant que { $user }. Définissez le mot de passe que les autres ordinateurs utilisent pour ouvrir vos dossiers partagés, d’au moins { $n } caractères. Il est distinct du mot de passe de votre session.

folders-sharing-password = Mot de passe

folders-sharing-password-tip = Mot de passe de partage

folders-sharing-keep-password = Conserver ce mot de passe

folders-sharing-card-title = Partager { $name } sur le réseau

folders-sharing-name-taken = Un autre de vos dossiers partagés porte déjà ce nom. Choisissez-en un autre.

folders-sharing-stopping = Arrêt du partage…

folders-sharing-password-short = Le mot de passe doit contenir au moins { $n } caractères.

folders-sharing-keeping-password = Enregistrement du mot de passe ; le mot de passe de l’administrateur est demandé d’abord.

folders-sharing-letting-connect = Autorisation de connexion des autres ordinateurs ; le mot de passe de l’administrateur est demandé d’abord.

folders-sharing-fetching = Récupération de Samba, le programme qui partage les dossiers, et mise en place. Cela n’arrive que la première fois, et le mot de passe de l’administrateur est demandé d’abord.

folders-sharing-setting-up = Mise en place du partage. Cela n’arrive que la première fois, et le mot de passe de l’administrateur est demandé d’abord.

folders-sharing-changing = Modification du partage…

folders-sharing-sharing-it = Partage en cours…
