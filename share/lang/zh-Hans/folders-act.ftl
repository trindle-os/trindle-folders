
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = 服务器上的文件不能使用剪切、复制和粘贴。请改用窗格之间的复制和移动按钮。

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = 压缩包里的内容无法更改。请先把需要的内容解压出来。

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = 这些内容是在另一个应用中剪切的，所以 Folders 改为复制了它们。

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = 所选内容已在复制或移动中。

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = 所选内容仍在复制到这里。

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10,000 个项目或更多

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
       *[other] { $n } 个项目
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
       *[other] 有 { $n } 个项目无法{ $verb }：{ $names }。
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }：{ $why }。{ $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } 以及另外 { $more } 个

# Words on a job card while things are copied: what, and where to.
folders-act-copying = 正在将 { $what } 复制到 { $place }

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = 已将 { $what } 复制到 { $place }。

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = 复制

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = 撤销复制 { $what }

# Words on a job card while things are moved: what, and where to.
folders-act-moving = 正在将 { $what } 移动到 { $place }

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = 已将 { $what } 移动到 { $place }。

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = 移动

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = 撤销移动 { $what }

# SAFETY: needs a human review before it ships
# Words on a job card while things are put in the Trash.
folders-act-trashing = 正在将 { $what } 移到回收站

# SAFETY: needs a human review before it ships
# Words under the file lines when things have been put in the Trash.
folders-act-trashed = 已将 { $what } 移到回收站。

# SAFETY: needs a human review before it ships
# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = 移到回收站

# SAFETY: needs a human review before it ships
# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = 撤销将 { $what } 移到回收站

# SAFETY: needs a human review before it ships
# Words on a job card while things are deleted for good.
folders-act-deleting = 正在删除 { $what }

# SAFETY: needs a human review before it ships
# Words under the file lines when things have been deleted for good.
folders-act-deleted = 已永久删除 { $what }。

# SAFETY: needs a human review before it ships
# The word for what was tried, put in 'could not be ...' when deleting failed.
folders-act-verb-deleted = 删除

# Words on a job card while things are put back from the Trash.
folders-act-restoring = 正在还原 { $what }

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = 已还原 { $what }。

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = 撤销还原 { $what }

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = 还原

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = 正在撤销

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = 已撤销。

# Words on a job card while things are packed into an archive.
folders-act-compressing = 正在压缩 { $what }

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = 已在 { $place } 中生成 { $file }。

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = 压缩

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = 撤销压缩 { $what }

# Words on a job card while an archive is unpacked.
folders-act-extracting = 正在解压 { $what }

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = 已将 { $what } 解压到 { $place }。

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = 解压

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = 撤销解压 { $what }

# SAFETY: needs a human review before it ships
# Words on a job card while files are wiped securely.
folders-act-wiping = 正在彻底擦除 { $what }

# SAFETY: needs a human review before it ships
# Words under the file lines when files have been wiped securely.
folders-act-wiped = 已彻底擦除 { $what }。

# SAFETY: needs a human review before it ships
# The word for what was tried, put in 'could not be ...' when wiping failed.
folders-act-verb-wiped = 彻底擦除

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = 正在取出 { $what }

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = 已将 { $what } 从压缩包取出到 { $place }。

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = 取出

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = 撤销取出 { $what }

# Words on a job card while pictures are turned.
folders-act-turning = 正在旋转 { $what }

# Words under the file lines when pictures have been turned.
folders-act-turned = 已旋转 { $what }。

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = 旋转

# Words on a job card while pictures are resized.
folders-act-resizing = 正在调整 { $what } 的大小

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = 已生成 { $what } 调整大小后的副本。

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = 调整大小

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = 撤销调整 { $what } 的大小

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = 新建文件夹

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = 新建文档

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders 无法创建 { $base }：{ $why }。

# Red words under a name being typed when it is empty.
folders-act-name-empty = 请输入名称。

# Red words under a name being typed when it has a slash.
folders-act-name-slash = 名称中不能包含斜杠。

# Red words under a name being typed when it is . or ..
folders-act-name-kept = 系统已使用这个名称。请换一个。

# Red words under a name being typed when it is too long.
folders-act-name-long = 这个名称太长了。

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders 无法开始这项任务。

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = 重做{ $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = 撤销{ $what }

# SAFETY: needs a human review before it ships
# Words under the file lines when work was stopped before it was done.
folders-act-stopped = 任务在完成前已停止。

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done }Folders 跳过了 { $n } 项。

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = 请先选择一些内容。

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = 它已经在回收站里了。

# SAFETY: needs a human review before it ships
# Words under the file lines when a server's files are put in the Trash.
folders-act-far-trash = 服务器上的文件不能放进回收站。请改用“永久删除”。

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = 请先选择要复制或移动的内容。

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = 压缩包里的内容只能取出到文件夹中。

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = 不能把内容复制到回收站。

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = 另一个窗格没有显示文件夹。

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = 请先选择要复制的内容。

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = 已剪切 { $what }。

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = 已复制 { $what }。

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = 内容只能粘贴到文件夹中。

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = 剪贴板上没有文件。

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = 没有剪切或复制任何内容。

# SAFETY: needs a human review before it ships
# Words under the file lines when Delete is used on the Network list.
folders-act-network-delete = 无法在这里删除网络位置。

# SAFETY: needs a human review before it ships
# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = 回收站已经是空的。

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = 请先在回收站中选择一些内容。

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = 正在创建 { $name }

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = 只能在文件夹中新建文件夹或文档。

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = 撤销创建 { $name }

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders 无法为已移动的项目保留文件夹颜色。

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = 没有可重做的内容。

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = 没有可撤销的内容。

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = 正在重做{ $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = 已重做{ $what }。

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = 重做

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = 正在撤销{ $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = 已撤销{ $what }。

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = 主文件夹

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = 回收站

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = 内容只能放到文件夹或回收站上。

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
       *[other] { $n } 个链接
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = 已创建 { $what }。

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
       *[other] 有 { $n } 个链接无法创建：{ $names }。
    }

# SAFETY: needs a human review before it ships
# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = 有个问题正在等你回答。请先回答，或停止该任务，再关闭 Folders。

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = 任务运行期间，Folders 无法关闭。请等它完成，或先停止它，然后再关闭 Folders。
