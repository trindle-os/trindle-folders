
# machine translated (Spanish), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders no pudo cambiar el nombre de { $name }: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] Ya hay una carpeta llamada { $name }.
       *[other] Ya hay un archivo llamado { $name }.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = No puedes cambiar el nombre de nada aquí.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Selecciona primero algo a lo que cambiar el nombre.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = Deshacer la creación de { $name }

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = Deshacer el cambio de nombre de { $old } a { $name }

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = Se cambió el nombre de { $old } a { $name }.

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = Cambiando el nombre de { $old } a { $name }…

