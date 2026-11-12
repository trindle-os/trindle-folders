
# machine translated (French), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Title of the question window asked before the Trash is emptied.
folders-view-empty-trash-title = Vider la corbeille ?

# SAFETY: needs a human review before it ships
# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
folders-view-empty-trash-body =
    { $n ->
        [one] L’élément qu’elle contient sera supprimé définitivement. Cette action est irréversible.
       *[other] Les { $n } éléments qu’elle contient seront tous supprimés définitivement. Cette action est irréversible.
    }

# SAFETY: needs a human review before it ships
# Red button of the question asked before the Trash is emptied: empties it at once.
folders-view-empty-trash = Vider la corbeille

# SAFETY: needs a human review before it ships
# Title of the question window asked before items are deleted for good; n is how many.
folders-view-delete-title =
    { $n ->
        [one] Supprimer définitivement cet élément ?
        [many] Supprimer définitivement ces { $n } éléments ?
       *[other] Supprimer définitivement ces { $n } éléments ?
    }

# SAFETY: needs a human review before it ships
# Text of the question asked before items are deleted for good.
folders-view-delete-body = Cette action est irréversible.

# SAFETY: needs a human review before it ships
# Red button of the question asked before items are deleted for good: deletes them at once.
folders-view-delete-permanently = Supprimer définitivement

# SAFETY: needs a human review before it ships
# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
folders-view-keep-them = Les garder

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = Où doivent-ils aller ?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Les laisser où ils sont




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = Icônes

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = Liste

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = Compact

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = Choisir les colonnes

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = Nom

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = Trier par

# Name read out for the toolbar's Group by strip.
folders-view-group-by = Grouper par





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Masquer le panneau d’aperçu (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Afficher le panneau d’aperçu (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Panneau d’aperçu

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Onglets

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Onglets du panneau de gauche

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Onglets du panneau de droite

# Tip of the button that opens a new tab.
folders-view-tab-new = Nouvel onglet (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = Fermer l’onglet (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = Dossier personnel

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Rien d’épinglé pour l’instant. Faites un clic droit sur un dossier pour l’épingler.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = Épinglés

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Emplacements

# Small heading in the side bar over the drives plugged in.
folders-view-drives = Disques

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Partages

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = Fermer { $name }



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Se connecter à un serveur

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = Lier les deux panneaux. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = Lier les deux panneaux

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] Copier vers le panneau de gauche
       *[other] Copier vers le panneau de droite
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] Déplacer vers le panneau de gauche
       *[other] Déplacer vers le panneau de droite
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Afficher ce dossier dans le panneau de gauche
       *[other] Afficher ce dossier dans le panneau de droite
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Échanger les panneaux

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Sélectionner ce qui diffère du panneau de gauche
       *[other] Sélectionner ce qui diffère du panneau de droite
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] Comparer avec le panneau de gauche
       *[other] Comparer avec le panneau de droite
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Saisissez un dossier, un fichier ou une adresse de serveur

# Name read out for the path above a pane's list.
folders-view-where = Emplacement

# Tip of the pane's Back button.
folders-view-back = Précédent (Alt+Gauche)

# Tip of the pane's Forward button.
folders-view-forward = Suivant (Alt+Droite)

# Tip of the pane's button that goes up one folder.
folders-view-up = Dossier parent (Alt+Haut)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Filtrer, ou appuyez sur Entrée pour chercher dans ce dossier et ses sous-dossiers

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Chercher dans les noms, sous-dossiers compris

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = Chercher dans les fichiers, sous-dossiers compris


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Type

# Name read out for the list that picks which size to show.
folders-view-find-size = Taille

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Dernière modification

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = Nouveau nom pour { $was }

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } libres

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } sélectionnés, calcul de la taille…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } sélectionnés, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Afficher les fichiers concernés

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Réessayer

# SAFETY: needs a human review before it ships
# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
folders-view-cancel = Annuler

# Line on a pane whose filter matches nothing.
folders-view-none-matches = Rien ici ne correspond au filtre

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Tout afficher

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = Vous n’avez rien ouvert récemment

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Aller au dossier personnel

# Line on the Trash page when it is empty.
folders-view-none-trash = La corbeille est vide

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = Aucun disque à afficher

# Line on a search page that found nothing.
folders-view-none-found = Rien trouvé

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Effacer la recherche

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = Cette archive est vide

# Button on an empty archive page: goes up one folder.
folders-view-go-up = Remonter d’un dossier

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = Ce dossier est vide

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = Nouveau dossier

# Button on an empty folder's page that makes a new document.
folders-view-new-document = Nouveau document

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] 1 élément
        [many] { $n } éléments
       *[other] { $n } éléments
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Non connecté

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = Reconnecter { $name }

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } n'est pas connecté.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Tout reconnecter

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] Un partage n'est pas connecté.
       *[other] { $n } partages ne sont pas connectés.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = Oublier { $name }

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = Deux volets

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = Afficher deux volets côte à côte (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Afficher un seul volet (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = Affichage

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = Nouveau

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = Plus d’options

# Row of the More menu that undoes the last change.
folders-view-more-undo = Annuler

# Row of the More menu that does again what was undone.
folders-view-more-redo = Rétablir

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Trier dans l'autre sens

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Afficher les fichiers cachés (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Masquer les fichiers cachés (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Fichiers cachés

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Ordinateurs à proximité

# Tip of the Search button in the top row while the path shows.
folders-view-search = Rechercher (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = Fermer la recherche (Échap)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = Vous ne pouvez pas rechercher sur cette page

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Rechercher

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = Nom dans la barre latérale pour { $was }
