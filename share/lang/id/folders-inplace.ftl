
# machine translated (Indonesian), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders tidak dapat mengganti nama { $name }: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] Folder bernama { $name } sudah ada di sini.
       *[other] File bernama { $name } sudah ada di sini.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = Anda tidak dapat mengganti nama apa pun di sini.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Pilih sesuatu untuk diganti namanya terlebih dahulu.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = Urungkan pembuatan { $name }

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = Urungkan penggantian nama { $old } menjadi { $name }

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = Nama { $old } diganti menjadi { $name }.

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = Mengganti nama { $old } menjadi { $name }…
