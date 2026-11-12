
# machine translated (Russian), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Не удалось переименовать «{ $name }»: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] Папка «{ $name }» уже существует.
       *[other] Файл «{ $name }» уже существует.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = Здесь ничего нельзя переименовать.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Сначала выберите, что переименовать.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = Отменить создание «{ $name }»

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = Отменить переименование «{ $old }» в «{ $name }»

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = «{ $old }» переименовано в «{ $name }».

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = Переименование «{ $old }» в «{ $name }»…
