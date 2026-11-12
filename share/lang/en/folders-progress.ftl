# Folders: work under way, in the line at the foot of a pane in place of the words about the items (how fast, how long, pause and stop) and the question when a name is already taken.

## Progress (src/files/progress.rs)

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done } of { $of } files, { $rate } a second, { $left }
       *[no] { $rate } a second, { $left }
    }
# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } of { $of } files, working out how long…
       *[no] working out how long…
    }
# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }
# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }
# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } bytes
# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB
# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB
# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB
# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = a few seconds left
# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] about 1 second left
       *[other] about { $n } seconds left
    }
# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = about a minute left
# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] about 1 minute left
       *[other] about { $n } minutes left
    }
# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] about 1 hour left
       *[other] about { $n } hours left
    }
# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] 1 more job is running
       *[other] { $n } more jobs are running
    }
# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Moving keeps the disk busy, so other programs may be slower until it's done.
       *[copying] Copying keeps the disk busy, so other programs may be slower until it's done.
    }
# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = waiting for an answer
# End of a piece of work's line while it is paused.
folders-progress-paused = paused
# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Resume
# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Pause
# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = Stop
# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = newer and larger
# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = newer
# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = larger
# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = changed { $when }
# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] A folder called { $name } is already there
        [file] A file called { $name } is already there
       *[unknown] { $name } is already there
    }
# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = The new one
# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = The one already there
# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Skip
# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = Keep both
# Word on the button in the name taken question that replaces the one already there.
folders-progress-replace = Replace
# Screen reader name of the Replace button for a folder in the name taken question.
folders-progress-replace-folder-spoken = Replace, and move the folder already there to the Trash
# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = Merge
# Line over the switch in the name taken question for folders.
folders-progress-all-folders = Use this answer for every folder that's already there.
# Line over the switch in the name taken question for files.
folders-progress-all-files = Use this answer for every file that's already there.
# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = Do the same for the rest
