
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = 早期副本中的内容无法更改。请把内容复制出来再保留。

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] 今天 { $time }
        [yesterday] 昨天 { $time }
       *[other] { $date } { $time }
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] 今天 { $time }
        [yesterday] 昨天 { $time }
       *[other] { $date } { $time }
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = 这是 { $when } 的早期副本。你无法更改它，但可以把内容复制出来。

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = 这是早期副本。你无法更改它，但可以把内容复制出来。

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = 早期：{ $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = 撤销还原 { $name }

# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = Folders 已将它还原。现有的文件在它旁边，名称已更改。

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = Folders 已将它还原到原处。

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = 它与现在的相同。

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = 它与现在的不同。

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = Folders 无法比较：{ $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = 早期版本

# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = 此处已删除的内容

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = { $name } 的早期版本

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = 没有与现在不同的早期版本。

# Note at the foot of the earlier versions card about what putting one back does.
folders-earlier-versions-foot = 还原时会把现有的文件保留在旁边。你可以撤销这一步。

# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = 从 { $name } 中删除的内容

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = 早期副本中的内容都没有从此文件夹中删除。

# Note at the foot of the deleted things card about what putting one back does.
folders-earlier-deleted-foot = 还原会把它放回此文件夹。你可以撤销这一步。

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = 当时的文件夹

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }，来自 { $date } 创建的副本

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = 文件夹，最后见于 { $when }

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }，最后见于 { $when }

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = 处理中…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = 在新标签页中打开当天的文件夹

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = 打开

# Tip and name of the round button on a row of the earlier versions card that puts it back.
folders-earlier-put-back-tip = 还原

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = 与现有的比较

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = 这不是以前的版本。

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = 以前的版本只能放回你的主文件夹内。

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = Folders 无法把文件放回那里。

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = Folders 无法重新创建它原来所在的文件夹：{ $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = Folders 无法复制以前的版本：{ $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = Folders 无法把现有的文件挪开：{ $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = Folders 无法放回以前的版本：{ $why }。现有的文件现在名为 { $name }。

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = Folders 无法放回以前的版本：{ $why }
