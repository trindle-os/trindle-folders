# Folders' words about doing work: copying, moving, trashing, deleting, packing, undo and redo, and what is said under the file lines about them.

## Work and actions (src/files/act.rs)

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Cut, copy and paste don't work with files on a server. Use the copy and move buttons between the panes instead.
# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = You can't change anything inside an archive. Extract what you need first.
# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = These were cut in another app, so Folders copied them instead.
# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = This is already being copied or moved.
# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = This is still being copied here.
# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10,000 items or more
# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] 1 item
       *[other] { $n } items
    }
# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] One item could not be { $verb }: { $names }.
       *[other] { $n } items could not be { $verb }: { $names }.
    }
# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }
# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } and { $more } more
# Words on a job card while things are copied: what, and where to.
folders-act-copying = Copying { $what } to { $place }
# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = Copied { $what } to { $place }.
# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = copied
# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = Undo copying { $what }
# Words on a job card while things are moved: what, and where to.
folders-act-moving = Moving { $what } to { $place }
# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = Moved { $what } to { $place }.
# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = moved
# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = Undo moving { $what }
# Words on a job card while things are put in the Trash.
folders-act-trashing = Putting { $what } in the Trash
# Words under the file lines when things have been put in the Trash.
folders-act-trashed = Put { $what } in the Trash.
# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = put in the Trash
# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = Undo putting { $what } in the Trash
# Words on a job card while things are deleted for good.
folders-act-deleting = Deleting { $what }
# Words under the file lines when things have been deleted for good.
folders-act-deleted = Deleted { $what } permanently.
# The word for what was tried, put in 'could not be ...' when deleting failed.
folders-act-verb-deleted = deleted
# Words on a job card while things are put back from the Trash.
folders-act-restoring = Putting back { $what }
# Words under the file lines when things have been put back from the Trash.
folders-act-restored = Put back { $what }.
# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = Undo putting back { $what }
# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = put back
# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Undoing
# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Undone.
# Words on a job card while things are packed into an archive.
folders-act-compressing = Compressing { $what }
# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = Made { $file } in { $place }.
# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = compressed
# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = Undo compressing { $what }
# Words on a job card while an archive is unpacked.
folders-act-extracting = Extracting { $what }
# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = Extracted { $what } into { $place }.
# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = extracted
# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = Undo extracting { $what }
# Words on a job card while files are wiped securely.
folders-act-wiping = Wiping { $what }
# Words under the file lines when files have been wiped securely.
folders-act-wiped = Wiped { $what }.
# The word for what was tried, put in 'could not be ...' when wiping failed.
folders-act-verb-wiped = wiped
# Words on a job card while things are taken out of an archive.
folders-act-taking-out = Taking out { $what }
# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = Took { $what } out of the archive into { $place }.
# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = taken out
# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = Undo taking out { $what }
# Words on a job card while pictures are turned.
folders-act-turning = Turning { $what }
# Words under the file lines when pictures have been turned.
folders-act-turned = Turned { $what }.
# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = turned
# Words on a job card while pictures are resized.
folders-act-resizing = Resizing { $what }
# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = Made a resized copy of { $what }.
# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = resized
# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = Undo resizing { $what }
# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = New folder
# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = New document
# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders couldn't make { $base }: { $why }.
# Red words under a name being typed when it is empty.
folders-act-name-empty = Type a name.
# Red words under a name being typed when it has a slash.
folders-act-name-slash = A name can't have a slash in it.
# Red words under a name being typed when it is . or ..
folders-act-name-kept = The system uses that name. Choose another.
# Red words under a name being typed when it is too long.
folders-act-name-long = That name is too long.
# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders couldn't start this job.
# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = Redo { $what }
# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = Undo { $what }
# Words under the file lines when work was stopped before it was done.
folders-act-stopped = The job was stopped before it finished.
# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [one] Folders skipped one item.
       *[other] Folders skipped { $n } items.
    }
# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Select something first.
# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = That's already in the Trash.
# Words under the file lines when a server's files are put in the Trash.
folders-act-far-trash = Files on a server can't be put in the Trash. Use Delete permanently instead.
# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Select something to copy or move first.
# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = You can only take things out of an archive into a folder.
# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = You can't copy things to the Trash.
# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = The other pane isn't showing a folder.
# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Select something to copy first.
# Words under the file lines after Cut: what was cut.
folders-act-cut-said = Cut { $what }.
# Words under the file lines after Copy: what was copied.
folders-act-copied-said = Copied { $what }.
# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = You can only paste into a folder.
# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = There are no files on the clipboard.
# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = Nothing has been cut or copied.
# Words under the file lines when Delete is used on the Network list.
folders-act-network-delete = You can't delete network places from here.
# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = The Trash is already empty.
# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Select something in the Trash first.
# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }
# Words under the file lines while a new folder or document is made on a server.
folders-act-making = Making { $name }
# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = You can only make a new folder or document inside a folder.
# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = Undo making { $name }
# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders couldn't keep the folder colours on the items it moved.
# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = There is nothing to redo.
# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = There is nothing to undo.
# Words on a job card while a step is done again; what the step was.
folders-act-redoing = Redoing { $what }
# Words under the file lines when a step was done again; what the step was.
folders-act-redid = Redid { $what }.
# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = redone
# Words on a job card while a step is undone; what the step was.
folders-act-undoing = Undoing { $what }
# Words under the file lines when a step was undone; what the step was.
folders-act-undid = Undid { $what }.
# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Home
# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Trash
# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = You can only drop things on a folder or the Trash.
# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] 1 link
       *[other] { $n } links
    }
# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = Made { $what }.
# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] One link could not be made: { $names }.
       *[other] { $n } links could not be made: { $names }.
    }
# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = A question is waiting for your answer. Answer it, or stop the job, before you close Folders.
# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = Folders can't close while a job is running. Wait for it to finish, or stop it, then close Folders.
