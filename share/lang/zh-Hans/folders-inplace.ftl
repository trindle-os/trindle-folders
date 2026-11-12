
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders 无法重命名 { $name }：{ $why }。

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] 这里已有名为 { $name } 的文件夹。
       *[other] 这里已有名为 { $name } 的文件。
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = 这里不能重命名任何项目。

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = 请先选择要重命名的项目。

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = 撤销创建 { $name }

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = 撤销将 { $old } 重命名为 { $name }

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = 已将 { $old } 重命名为 { $name }。

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = 正在将 { $old } 重命名为 { $name }…

