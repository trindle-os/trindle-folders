
# machine translated (French), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders ne peut pas lire le contenu de ce dossier.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = Vous ne pouvez partager qu’un dossier lui-même, pas un lien vers un dossier.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = Vous ne pouvez partager que les dossiers qui vous appartiennent. Celui-ci appartient à quelqu’un d’autre.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = votre dossier personnel

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = ce dossier

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } et { $second }

# SAFETY: needs a human review before it ships
# Name of the switch that shares the folder.
folders-sharing-share = Partager ce dossier

# SAFETY: needs a human review before it ships
# One line under the share switch when the folder is shared.
folders-sharing-shared = Partagé sur le réseau domestique.


# SAFETY: needs a human review before it ships
# Tip of the share switch.
folders-sharing-share-tip = Partager sur le réseau domestique

# Label over the box for the name other computers see.
folders-sharing-name = Nom

# Tip of the box for the name other computers see.
folders-sharing-name-tip = Le nom que voient les autres ordinateurs

# SAFETY: needs a human review before it ships
# Name of the switch that lets others change files, and its tip.
folders-sharing-write = Laisser les autres modifier les fichiers

# SAFETY: needs a human review before it ships
# One line under the switch that lets others change files.
folders-sharing-write-sub = Ils peuvent y ajouter, modifier et supprimer des fichiers.

# SAFETY: needs a human review before it ships
# Name of the switch that lets anyone in without a password.
folders-sharing-guests = Laisser entrer tout le monde

# SAFETY: needs a human review before it ships
# One line under the let in anyone switch.
folders-sharing-guests-sub = Aucun mot de passe nécessaire.

# SAFETY: needs a human review before it ships
# Tip of the let in anyone switch.
folders-sharing-guests-tip = Laisser entrer tout le monde sans mot de passe

# SAFETY: needs a human review before it ships
# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Une fois partagé, il apparaît sous Réseau sur vos autres ordinateurs à la maison, y compris ceux sous Windows. La première fois, Trindle télécharge Samba, le programme qui partage les dossiers. Cela prend une minute.
       *[no] Une fois partagé, il apparaît sous Réseau sur vos autres ordinateurs à la maison, y compris ceux sous Windows.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] Sur les autres ordinateurs, cherchez { $name } sur { $host } sous Réseau.
       *[no] Sur les autres ordinateurs, cherchez { $name } sous Réseau.
    }

# SAFETY: needs a human review before it ships
# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = Les autres ordinateurs ne peuvent pas encore atteindre ce dossier. Dans Réglages, ouvrez Réseau et activez « Laisser les autres ordinateurs se connecter ».

# SAFETY: needs a human review before it ships
# Tip of the button that lets other computers connect.
folders-sharing-let-connect = Laisser les autres ordinateurs se connecter

# SAFETY: needs a human review before it ships
# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
folders-sharing-visitors =
    { $n ->
        [one] Les invités sans mot de passe ne peuvent pas encore l’ouvrir, car { $names } est fermé aux autres comptes. Si vous les laissez passer, ils peuvent accéder à ce dossier mais ne voient rien d’autre sur le chemin.
       *[other] Les invités sans mot de passe ne peuvent pas encore l’ouvrir, car { $names } sont fermés aux autres comptes. Si vous les laissez passer, ils peuvent accéder à ce dossier mais ne voient rien d’autre sur le chemin.
    }

# SAFETY: needs a human review before it ships
# Tip of the button that lets visitors through to the folder.
folders-sharing-let-through = Laisser les invités accéder à ce dossier

# SAFETY: needs a human review before it ships
# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
folders-sharing-sign-in-set = Les autres se connectent en tant que { $user } avec le mot de passe de partage.

# SAFETY: needs a human review before it ships
# Tip of the button that changes the sharing password.
folders-sharing-change-password = Changer le mot de passe de partage

# SAFETY: needs a human review before it ships
# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
folders-sharing-set-password = Les autres se connectent en tant que { $user }. Définissez un mot de passe de partage d’au moins { $n } caractères. Il est distinct du mot de passe de votre session.

# SAFETY: needs a human review before it ships
# Label beside the sharing password box.
folders-sharing-password = Mot de passe

# SAFETY: needs a human review before it ships
# Tip of the sharing password box.
folders-sharing-password-tip = Mot de passe de partage

# SAFETY: needs a human review before it ships
# Tip of the button that keeps the sharing password.
folders-sharing-keep-password = Enregistrer le mot de passe

# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = Partager { $name } sur le réseau

# Red words when the share's name is used by another share.
folders-sharing-name-taken = Vous avez déjà un dossier partagé portant ce nom. Choisissez-en un autre.

# Calm words while sharing is being stopped.
folders-sharing-stopping = Arrêt du partage…

# SAFETY: needs a human review before it ships
# Red words when the sharing password is too short; { $n } is the fewest letters.
folders-sharing-password-short = Le mot de passe doit contenir au moins { $n } caractères.

# SAFETY: needs a human review before it ships
# Calm words while the sharing password is being kept.
folders-sharing-keeping-password = Enregistrement du mot de passe. On vous demandera d’abord le mot de passe de l’administrateur.

# SAFETY: needs a human review before it ships
# Calm words while other computers are being let connect.
folders-sharing-letting-connect = Autorisation de connexion des autres ordinateurs. On vous demandera d’abord le mot de passe de l’administrateur.

# SAFETY: needs a human review before it ships
# Calm words while Samba is fetched and set up the first time.
folders-sharing-fetching = Téléchargement et mise en place de Samba, le programme qui partage les dossiers. On vous demandera d’abord le mot de passe de l’administrateur. Cela n’arrive que la première fois.

# SAFETY: needs a human review before it ships
# Calm words while sharing is set up the first time and Samba is already there.
folders-sharing-setting-up = Mise en place du partage. On vous demandera d’abord le mot de passe de l’administrateur. Cela n’arrive que la première fois.

# Calm words while an existing share is changed.
folders-sharing-changing = Modification du partage…

# Calm words while a share is made.
folders-sharing-sharing-it = Partage du dossier…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = Donnez un nom au dossier partagé.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = Le nom d'un dossier partagé ne peut pas dépasser { $most } caractères.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = Le nom d'un dossier partagé ne peut contenir aucun de ces caractères : % < > * ? | / \ + = ; : " ni la virgule.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = Un nom finissant par $ masquerait le partage, vous ne pouvez donc pas l'utiliser.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = Le nom d'un dossier partagé ne peut pas commencer par un tiret.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba utilise déjà le nom { $name }. Choisissez-en un autre.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Partagé

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = Vous ne pouvez partager que les dossiers qui vous appartiennent.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = Ce nom est déjà un nom de compte sur cet ordinateur, vous ne pouvez donc pas l'utiliser pour un partage. Choisissez-en un autre.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = Les réglages Samba de cet ordinateur n'autorisent pas les partages sans mot de passe.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = Cet ordinateur ne peut pas partager d'autres dossiers.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = Ce dossier n'est plus là.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba n'a donné aucune raison.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Message de Samba : { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } appartient à quelqu'un d'autre, vous ne pouvez donc pas donner l'accès aux invités depuis ici.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders n'a pas pu donner aux invités l'accès à { $path } : { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle n'a pas pu télécharger Samba car cet ordinateur semble hors ligne. Connectez-vous à Internet et réessayez.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Trindle n'a pas pu installer Samba pendant qu'une autre installation est en cours. Réessayez dans quelques minutes.
