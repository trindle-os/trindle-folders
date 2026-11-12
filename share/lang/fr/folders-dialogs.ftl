
# machine translated (French), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = Fermer

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = Dossier personnel

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = Ouvrir { $name } avec

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = Application actuelle

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = Aucune application de cet ordinateur n'est prévue pour ouvrir ce type de fichier.

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = Toujours utiliser cette application

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = L'utiliser pour tout fichier { $kind }.

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = Définir comme application par défaut

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = Afficher seulement les applications suggérées

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = Afficher toutes les applications

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = Ouvrir

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = Choisissez d'abord une application.

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders n'a pas pu définir { $name } comme application par défaut.

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders n'a pas pu démarrer { $name }.

# Heading of the Properties window.
folders-dialogs-properties = Propriétés

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = Pages des propriétés

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = Général

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = Détails

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = Partage



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = Nom

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = Emplacement

# Label in Properties of the kind of file.
folders-dialogs-row-kind = Type

# Label in Properties of where a link points.
folders-dialogs-row-link-to = Lien vers

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = Modifié

# Label in Properties of who owns the item.
folders-dialogs-row-owner = Propriétaire

# Label in Properties of who may open and change the item.
# SAFETY: needs a human review before it ships
folders-dialogs-row-rights = Permissions

# Label in Properties of the size of the item.
folders-dialogs-row-size = Taille

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = Inconnu

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
# SAFETY: needs a human review before it ships
folders-dialogs-rights-open-change = { $letters }, vous pouvez l'ouvrir et le modifier

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
# SAFETY: needs a human review before it ships
folders-dialogs-rights-open-only = { $letters }, vous pouvez l'ouvrir mais pas le modifier

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
# SAFETY: needs a human review before it ships
folders-dialogs-rights-change-only = { $letters }, vous pouvez le modifier mais pas l'ouvrir

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
# SAFETY: needs a human review before it ships
folders-dialogs-rights-none = { $letters }, vous ne pouvez pas l'ouvrir

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , certains n'ont pas pu être lus
       *[other]
    }{ $adding ->
        [yes] , calcul en cours
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = Calcul de la taille

# Label in Properties of the row about checksums.
folders-dialogs-checksums = Sommes de contrôle

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = Calculer les sommes de contrôle

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = Calcul des sommes de contrôle

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders n'a pas pu lire le fichier.

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = Collez une somme de contrôle à comparer

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } correspond

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = Aucune correspondance

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = Vérifier


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [one] Envoyer { $name } vers
        [many] Envoyer { $n } d'éléments vers
       *[other] Envoyer { $n } éléments vers
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = Cela crée une copie à cet endroit. Les originaux restent où ils sont.

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = L'autre volet, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = Colonnes de l'affichage en liste

# Note in the Columns window.
folders-dialogs-columns-note = Le nom est toujours affiché. Les colonnes qui ne tiennent pas sont masquées tant que le volet n'est pas plus large.

# Heading over the columns that suit every file.
folders-dialogs-columns-files = Fichiers

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = Musique, photos, PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = Afficher { $name }

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders n'a pas pu enregistrer les colonnes.

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [one] Couleur de { $name }
        [many] Couleur de { $n } de dossiers
       *[other] Couleur de { $n } dossiers
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = Aucune couleur

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders n'a pas pu enregistrer la couleur du dossier.


folders-dialogs-no-match = Aucune application ne correspond à ces mots.


folders-dialogs-find-app = Trouver une application


folders-dialogs-find-place = Trouver un emplacement


folders-dialogs-no-match-place = Aucun emplacement ne correspond à ces mots.
