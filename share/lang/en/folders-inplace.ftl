# Folders' rename in place: the words said under the typing box and at the foot of the pane when a name is changed (src/files/inplace.rs).

## Renaming in place (src/files/inplace.rs)

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders couldn't rename { $name }: { $why }.
# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] A folder called { $name } is already here.
       *[other] A file called { $name } is already here.
    }
# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = You can't rename anything here.
# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Select something to rename first.
# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = Undo making { $name }
# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = Undo renaming { $old } to { $name }
# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = Renamed { $old } to { $name }.
# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = Renaming { $old } to { $name }…
