# Words of renaming many items at once in Folders: the reasons given when the renames cannot be carried out.

## Renaming (src/files/rename.rs)

# Reason shown when renames cannot start because some of the new names clash.
folders-rename-clash = some of the new names can't be used
# Reason shown when a rename stops because a new name is already taken; the full path of the name follows.
folders-rename-already-there = { $path } already exists
# Why a rename of names that swap could not start: no place to note them first.
folders-rename-no-journal = Folders couldn't save a list of the old names first
