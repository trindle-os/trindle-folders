
# machine translated (Vietnamese), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders không thể đổi tên { $name }: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] Đã có một thư mục tên { $name } ở đây.
       *[other] Đã có một tệp tên { $name } ở đây.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = Bạn không thể đổi tên gì ở đây.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Hãy chọn một mục để đổi tên trước.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = Hoàn tác việc tạo { $name }

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = Hoàn tác việc đổi tên { $old } thành { $name }

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = Đã đổi tên { $old } thành { $name }.

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = Đang đổi tên { $old } thành { $name }…
