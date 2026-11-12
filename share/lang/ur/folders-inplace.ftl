
# machine translated (Urdu), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders { $name } کا نام تبدیل نہیں کر سکا: { $why }۔

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] { $name } نام کا فولڈر یہاں پہلے سے موجود ہے۔
       *[other] { $name } نام کی فائل یہاں پہلے سے موجود ہے۔
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = آپ یہاں کسی چیز کا نام تبدیل نہیں کر سکتے۔

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = پہلے نام بدلنے کے لیے کچھ منتخب کریں۔

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = { $name } بنانا واپس کریں

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = { $old } کا نام { $name } رکھنا واپس کریں

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = { $old } کا نام { $name } رکھ دیا گیا۔

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = { $old } کا نام { $name } رکھا جا رہا ہے…
