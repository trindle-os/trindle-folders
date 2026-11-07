# machine translated (French), not yet reviewed by a person.

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = aucun serveur nommé { $host } n'a été trouvé

folders-far-not-all-arrived = il n'est pas arrivé en entier, il est donc resté où il était

folders-far-not-followed = un lien, qui n'est pas suivi, donc non envoyé

folders-far-not-sent = ni un fichier ni un dossier, donc non envoyé

folders-far-not-told = le serveur ne dit pas ce que c'est, il a donc été laissé

folders-far-ftp-cannot-refuse = Ce serveur peut donner le même nom à deux fichiers.

folders-far-cannot-do = Cela ne peut pas être fait sur les fichiers d'un serveur

folders-far-not-looked-at = il n'a pas pu être consulté

folders-far-not-opened = Impossible de l'ouvrir.

folders-far-inside-itself = { $name } (un dossier ne peut pas se placer dans lui-même)

folders-far-taken-by-folder = { $name } (un dossier de ce nom existe déjà)

folders-far-taken-by-file = { $name } (un fichier de ce nom existe déjà)

folders-far-address-unknown = L'adresse du serveur est inconnue.

folders-far-copy-wrong-length = la copie envoyée n'avait pas la longueur attendue

folders-far-only-server-file = Seul un fichier d'un serveur peut être renommé ainsi.

folders-far-aside-both = l'ancien est intact sous le nom { $old }, et le nouveau sous le nom { $new } : { $why }

folders-far-aside-kept = l'ancien n'a pas pu être supprimé, et il est intact sous le nom { $old }

folders-far-aside-kept-why = l'ancien n'a pas pu être supprimé, et il est intact sous le nom { $old } : { $why }

folders-far-not-empty = { $name } n'est pas vide.

folders-far-unsafe-name = Ignoré : le serveur a envoyé un nom dangereux.

folders-far-unsafe-names =
    { $n ->
        [one] Ignoré : le serveur a envoyé un nom dangereux : { $names }.
        [many] { $n } ignorés : le serveur a envoyé des noms dangereux : { $names }.
       *[other] { $n } ignorés : le serveur a envoyé des noms dangereux : { $names }.
    }

folders-far-fetching = Récupération de { $name }…

folders-far-cannot-fetch = Impossible de récupérer { $name } : { $why }.

folders-far-opened-copy = Copie de { $name } ouverte. L'enregistrer renvoie les modifications au serveur.

folders-far-saved = { $name } enregistré sur le serveur.

folders-far-not-saved = Impossible d'enregistrer { $name } sur le serveur : { $why }. La copie locale garde les modifications.

folders-far-renamed = { $was } renommé en { $now }.

folders-far-connect-title = Se connecter à un serveur

folders-far-connect-close = Fermer

folders-far-connect-address = Adresse, par exemple sftp://jo@example.com

folders-far-connect-password = Mot de passe, si nécessaire

folders-far-connect-key = Fichier de clé, si vous en choisissez un

folders-far-connect-pick-key = Choisir le fichier de clé

folders-far-connect-choose = Choisir

folders-far-connect-ssh-setup = Votre propre configuration ssh est utilisée : sa config, son agent, ses clés et ses serveurs connus.

folders-far-connect-new-host = { $host } n'a jamais été visité, Trindle ne peut donc pas savoir si c'est le bon. Faites-lui confiance seulement si vous comptiez vous y connecter.

folders-far-connect-host-changed = { $host } n'a plus l'identité qu'il avait auparavant. Quelqu'un s'est peut-être interposé : il n'est donc pas ouvert. Si son propriétaire l'a modifié, supprimez d'abord l'ancien de vos hôtes connus.

folders-far-connect-needs-password = Ce serveur demande un mot de passe. Saisissez-le ci-dessus, puis réessayez.

folders-far-connect-needs-key = Ce serveur demande une clé ou un mot de passe. Indiquez votre fichier de clé ci-dessus, ou ajoutez-la à votre agent ssh, puis réessayez.

folders-far-connect-cert-new = Le certificat de { $host } n'a pas pu être vérifié : { $why }. Si possible, comparez son empreinte avec celle que donne le propriétaire du serveur, et ne lui faites confiance que si elles correspondent.

folders-far-connect-cert-changed = Le certificat de { $host } n'est pas celui auquel vous avez fait confiance auparavant. Son propriétaire l'a peut-être renouvelé, ou quelqu'un s'est peut-être interposé. Ne faites confiance au nouveau que si son propriétaire confirme cette empreinte.

folders-far-connect-connecting = Connexion

folders-far-connect-trust = Lui faire confiance et se connecter

folders-far-connect-go = Se connecter

folders-far-phones = Les téléphones apparaissent d'eux-mêmes dans la barre latérale une fois branchés et déverrouillés.
