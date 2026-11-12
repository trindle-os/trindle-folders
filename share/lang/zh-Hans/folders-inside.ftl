
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = 请先选择压缩包里的项目。

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = 只能把压缩包里的项目提取到这台电脑上的文件夹中。

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = Folders 无法读取 { $name }：{ $why }。

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
       *[other] Folders 无法提取 { $n } 个项目，因为它们的名称指向文件夹之外，或有东西挡住了。
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = Folders 无法从压缩包中提取它：{ $why }。

