

folders-view-empty-trash-title = Vider la corbeille ?

folders-view-empty-trash-body =
    { $n ->
        [one] L’unique élément qu’elle contient est supprimé définitivement. Cette action est irréversible.
        [many] Les { $n } éléments qu’elle contient sont tous supprimés définitivement. Cette action est irréversible.
       *[other] Les { $n } éléments qu’elle contient sont tous supprimés définitivement. Cette action est irréversible.
    }

folders-view-empty-trash = Vider la corbeille

folders-view-delete-title =
    { $n ->
        [one] Supprimer définitivement cet élément ?
        [many] Supprimer définitivement ces { $n } éléments ?
       *[other] Supprimer définitivement ces { $n } éléments ?
    }

folders-view-delete-body = Cette action est irréversible.

folders-view-delete-permanently = Supprimer définitivement

folders-view-keep-them = Les garder

folders-view-drop-title = Où doivent-ils aller ?

folders-view-drop-leave = Les laisser où ils sont

folders-view-single = Simple

folders-view-dual = Double

folders-view-panes = Panneaux, simple ou double (F3)

folders-view-icons = Icônes

folders-view-list = Liste

folders-view-compact = Compact

folders-view-columns = Choisir les colonnes

folders-view-sort-name = Nom

folders-view-sort-by = Trier par

folders-view-group-by = Grouper par

folders-view-sorted-up = Tri de A à Z, du plus petit ou du plus ancien

folders-view-sorted-down = Tri de Z à A, du plus grand ou du plus récent

folders-view-hide-hidden = Masquer les fichiers cachés

folders-view-show-hidden = Afficher les fichiers cachés

folders-view-panel-hide = Masquer le panneau d’aperçu (Alt+P)

folders-view-panel-show = Afficher le panneau d’aperçu (Alt+P)

folders-view-panel-spoken = Panneau d’aperçu

folders-view-tabs = Onglets

folders-view-tabs-left = Onglets du panneau de gauche

folders-view-tabs-right = Onglets du panneau de droite

folders-view-tab-new = Nouvel onglet (Ctrl+T)

folders-view-tab-close = Fermer l’onglet (Ctrl+W, ou clic du milieu dessus)

folders-view-undo = { $label } (Ctrl+Z)

folders-view-undo-none = Rien à annuler

folders-view-redo = { $label } (Ctrl+Y)

folders-view-redo-none = Rien à rétablir

folders-view-new-folder-tip = Nouveau dossier (Ctrl+Maj+N)

folders-view-home = Dossier personnel

folders-view-pinned-none = Rien d’épinglé pour l’instant. Faites un clic droit sur un dossier pour l’épingler.

folders-view-pinned = Épinglés

folders-view-places = Emplacements

folders-view-drives = Disques

folders-view-shares = Partages

folders-view-share-close = Fermer { $name }

folders-view-connect = Se connecter à un serveur

folders-view-link-noted = Lier les deux panneaux. { $note }

folders-view-link = Lier les deux panneaux

folders-view-copy-to =
    { $way ->
        [left] Copier vers le panneau de gauche
       *[other] Copier vers le panneau de droite
    }

folders-view-move-to =
    { $way ->
        [left] Déplacer vers le panneau de gauche
       *[other] Déplacer vers le panneau de droite
    }

folders-view-same-in =
    { $way ->
        [left] Afficher ce dossier dans le panneau de gauche
       *[other] Afficher ce dossier dans le panneau de droite
    }

folders-view-swap = Échanger les panneaux

folders-view-pick-differing =
    { $way ->
        [left] Choisir ce qui diffère du panneau de gauche
       *[other] Choisir ce qui diffère du panneau de droite
    }

folders-view-compare =
    { $way ->
        [left] Comparer avec le panneau de gauche
       *[other] Comparer avec le panneau de droite
    }

folders-view-locate-hint = Saisissez un dossier, un fichier ou une adresse de serveur

folders-view-where = Emplacement de ce panneau

folders-view-back = Précédent (Alt+Gauche)

folders-view-forward = Suivant (Alt+Droite)

folders-view-up = Dossier parent (Alt+Haut)

folders-view-locate = Saisir un emplacement (Ctrl+L ou Alt+D)

folders-view-filter = Filtrer ou chercher ici (Ctrl+F)

folders-view-filter-hint = Filtrer, ou appuyez sur Entrée pour chercher dans ce dossier et en dessous

folders-view-find-name = Chercher par nom ici et en dessous

folders-view-find-inside = Chercher des mots dans les fichiers ici et en dessous

folders-view-find-close = Fermer et tout afficher

folders-view-find-kind = Type d’élément à afficher

folders-view-find-size = Taille à afficher

folders-view-find-changed = Date de dernière modification

folders-view-rename-hint = Nouveau nom pour { $was }

folders-view-status-free = { $items }, { $size } libres

folders-view-status-selected-working = { $count } sélectionnés, calcul de la taille…

folders-view-status-selected = { $count } sélectionnés, { $size }

folders-view-status-then = { $first }. { $then }

folders-view-listed = Quels fichiers, et pourquoi

folders-view-try-again = Réessayer

folders-view-cancel = Annuler

folders-view-none-matches = Rien ici ne correspond au filtre

folders-view-show-everything = Tout afficher

folders-view-none-recent = Rien d’ouvert récemment

folders-view-go-home = Aller au dossier personnel

folders-view-none-trash = La corbeille est vide

folders-view-none-drives = Aucun disque à afficher

folders-view-none-found = Rien trouvé

folders-view-clear-search = Effacer la recherche

folders-view-none-archive = Cette archive est vide

folders-view-go-up = Remonter d’un dossier

folders-view-none-folder = Ce dossier est vide

folders-view-new-folder = Nouveau dossier

folders-view-new-document = Nouveau document

folders-view-items =
    { $n ->
        [one] 1 élément
        [many] { $n } éléments
       *[other] { $n } éléments
    }

folders-view-not-connected = Non connecté

folders-view-reconnect = Reconnecter { $name }

folders-view-share-down = { $name } n'est pas connecté.

folders-view-reconnect-all = Tout reconnecter

folders-view-shares-down =
    { $n ->
        [one] Un partage n'est pas connecté.
       *[other] { $n } partages ne sont pas connectés.
    }

folders-view-forget-share = Oublier { $name }

