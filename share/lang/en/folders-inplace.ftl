# Folders' rename in place: the words said under the typing box and at the foot of the pane when a name is changed (src/files/inplace.rs).

folders-inplace-not-renamed = { $name } could not be renamed: { $why }.
folders-inplace-taken =
    { $kind ->
        [folder] A folder called { $name } is already here.
       *[other] A file called { $name } is already here.
    }
folders-inplace-only-folder = Only things in a folder can be renamed.
folders-inplace-pick-first = Pick something to rename first.
folders-inplace-undo-making = Undo making { $name }
folders-inplace-undo-renaming = Undo renaming { $old } to { $name }
folders-inplace-renamed = Renamed { $old } to { $name }.
folders-inplace-renaming = Renaming { $old } to { $name }.
