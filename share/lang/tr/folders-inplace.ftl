
# machine translated (Turkish), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders { $name } öğesini yeniden adlandıramadı: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] Burada { $name } adlı bir klasör zaten var.
       *[other] Burada { $name } adlı bir dosya zaten var.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = Burada hiçbir şeyi yeniden adlandıramazsınız.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Önce yeniden adlandırılacak bir öğe seçin.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = { $name } oluşturmayı geri al

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = { $old } adının { $name } olarak değiştirilmesini geri al

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = { $old } adı { $name } olarak değiştirildi.

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = { $old } adı { $name } olarak değiştiriliyor…
