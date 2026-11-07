# Folders: work under way, in the line at the foot of a pane in place of the words about the items (how fast, how long, pause and stop) and the question when a name is already taken.

folders-progress-pace-known =
    { $files ->
        [yes] { $done } of { $of } files, { $rate } a second, { $left }
       *[no] { $rate } a second, { $left }
    }
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } of { $of } files, working out how long…
       *[no] working out how long…
    }
folders-progress-line = { $doing }: { $how }
folders-progress-line-heavy = { $line } { $heavy }
folders-progress-speed-bytes = { $n } bytes
folders-progress-speed-kb = { $n } KB
folders-progress-speed-mb = { $n } MB
folders-progress-speed-gb = { $n } GB
folders-progress-left-few = a few seconds left
folders-progress-left-seconds =
    { $n ->
        [one] about 1 second left
       *[other] about { $n } seconds left
    }
folders-progress-left-minute = about a minute left
folders-progress-left-minutes =
    { $n ->
        [one] about 1 minute left
       *[other] about { $n } minutes left
    }
folders-progress-left-hours =
    { $n ->
        [one] about 1 hour left
       *[other] about { $n } hours left
    }
folders-progress-more =
    { $n ->
        [one] 1 more piece of work is under way
       *[other] { $n } more pieces of work are under way
    }
folders-progress-heavy =
    { $verb ->
        [moving] Moving uses the disk heavily, so other programs may feel slower until it ends.
       *[copying] Copying uses the disk heavily, so other programs may feel slower until it ends.
    }
folders-progress-waiting = waiting for an answer
folders-progress-paused = paused
folders-progress-carry-on = Carry on
folders-progress-pause = Pause
folders-progress-stop = Stop
folders-progress-mark-newer-larger = newer, larger
folders-progress-mark-newer = newer
folders-progress-mark-larger = larger
folders-progress-changed = changed { $when }
folders-progress-taken =
    { $kind ->
        [folder] A folder called { $name } is already there
        [file] A file called { $name } is already there
       *[unknown] { $name } is already there
    }
folders-progress-coming = The one being brought
folders-progress-there = The one already there
folders-progress-skip = Skip
folders-progress-keep-both = Keep both
folders-progress-replace = Replace
folders-progress-replace-folder-spoken = Replace: the folder already there goes to the Trash
folders-progress-merge = Merge
folders-progress-all-folders = The same answer for every folder already there.
folders-progress-all-files = The same answer for every file already there.
folders-progress-same-for-rest = Do the same for the rest
