
# machine translated (French), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders n’a pas pu renommer { $name } : { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] Un dossier nommé { $name } existe déjà ici.
       *[other] Un fichier nommé { $name } existe déjà ici.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = Vous ne pouvez rien renommer ici.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Sélectionnez d’abord un élément à renommer.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = Annuler la création de { $name }

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = Annuler le renommage de { $old } en { $name }

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = { $old } renommé en { $name }.

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = Renommage de { $old } en { $name }…
