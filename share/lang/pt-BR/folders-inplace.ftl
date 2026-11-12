
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = O Folders não conseguiu renomear { $name }: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] Já existe aqui uma pasta chamada { $name }.
       *[other] Já existe aqui um arquivo chamado { $name }.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = Você não pode renomear nada aqui.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Selecione algo para renomear primeiro.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = Desfazer a criação de { $name }

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = Desfazer a renomeação de { $old } para { $name }

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = { $old } renomeado para { $name }.

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = Renomeando { $old } para { $name }…

