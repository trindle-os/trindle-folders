# Folders: the windows the right-click menu opens for the bigger jobs: Compress, Extract to, renaming many at once, Wipe, finding duplicates, comparing the two panes and two files, and resizing pictures.

folders-tools-pass-once = Once
folders-tools-pass-three = 3 times
folders-tools-pass-seven = 7 times
folders-tools-case-keep = As they are
folders-tools-case-lower = All small
folders-tools-case-upper = All capitals
folders-tools-case-words = Each word
folders-tools-case-first = First word
folders-tools-ending-keep = As it is
folders-tools-ending-lower = All small
folders-tools-ending-upper = All capitals
folders-tools-ending-new = A new ending
folders-tools-ending-none = No ending
folders-tools-archive-name = Archive
folders-tools-clash-empty = no name would be left
folders-tools-clash-bad = that is not allowed as a name
folders-tools-clash-long = that name is too long
folders-tools-clash-twice = two would get this name
folders-tools-clash-taken = something else has this name
folders-tools-clash-not-text = the old name is not plain text
folders-tools-caveat-flash = This disk is flash, not spinning, so old copies may live on in parts of it Trindle cannot reach.
folders-tools-caveat-copy-on-write = The { $fs } filesystem writes changes to a new place, so the old data may stay where it was.
folders-tools-caveat-network = This is a { $fs } share on another computer, which may keep copies of its own.
folders-tools-row-only-left = Only on the left
folders-tools-row-only-right = Only on the right
folders-tools-row-same = The same
folders-tools-row-different =
    Different: { $why ->
        [kind] one is a folder or link
        [size] sizes differ
        [date] dates differ
        [content] contents differ
        [target] links point elsewhere
        [inside] something inside differs
       *[unread] one side could not be read
    }{ $newer ->
        [left] , newer on the left
        [right] , newer on the right
       *[none] { "" }
    }
folders-tools-place-here = Here
folders-tools-only-files-wiped = Only files can be wiped.
folders-tools-compare-need-folders = Both panes need to show a folder to compare them.
folders-tools-rename-undo =
    { $n ->
        [one] Undo renaming 1 item
       *[other] Undo renaming { $n } items
    }
folders-tools-rename-done =
    { $n ->
        [one] Renamed 1 item.
       *[other] Renamed { $n } items.
    }
folders-tools-rename-stopped = Renaming stopped part way: { $error }.
folders-tools-name-empty = A name cannot be empty.
folders-tools-looked-at = Looked at { $done }
folders-tools-read-of = Read { $done } of { $total }
folders-tools-listing-files = Listing files
folders-tools-comparing = Comparing
folders-tools-looking = Looking
folders-tools-compress-title =
    { $n ->
        [one] Compress { $name }
       *[other] Compress { $n } items
    }
folders-tools-pack-name-tip = Name of the archive
folders-tools-pack-kind-tip = Kind of archive
folders-tools-pack-made-in = Made in { $place }
folders-tools-compress = Compress
folders-tools-extract-to = Extract { $name } to
folders-tools-extract-new-folder = A new folder named { $name } is made there.
folders-tools-resize-title =
    { $n ->
        [one] Resize { $name }
       *[other] Resize { $n } pictures
    }
folders-tools-resize-size-tip = New size
folders-tools-resize-note = A share of its size, or how many pixels along its longer side; a picture is never made bigger. A copy is made beside each, named with its new size; the pictures themselves stay as they are.
folders-tools-resize = Resize
folders-tools-wipe-title =
    { $n ->
        [one] Wipe { $name }?
       *[other] Wipe { $n } files?
    }
folders-tools-wipe-warning = Each file is written over before it goes, so it cannot be brought back. This cannot be undone.
folders-tools-wipe-write-over = Write over
folders-tools-wipe-passes-tip = How many times each file is written over
folders-tools-wipe-button = Wipe securely
folders-tools-rename-number-tip = Number them
folders-tools-rename-at-start = At the start
folders-tools-rename-at-end = At the end
folders-tools-rename-where-tip = Where the number goes
folders-tools-rename-first-number-tip = First number
folders-tools-rename-ending-tip = What happens to the ending
folders-tools-rename-new-ending-tip = New ending, such as txt
folders-tools-rename-cannot = Cannot: { $why }
folders-tools-rename-was = was { $old }
folders-tools-rename-stays = stays as it is
folders-tools-rename-title =
    { $n ->
        [one] Rename 1 item
       *[other] Rename { $n } items
    }
folders-tools-rename-replace = Replace
folders-tools-rename-find-tip = Find
folders-tools-rename-with-tip = Replace with
folders-tools-rename-wildcard-tip = Use * and ? as wildcards
folders-tools-rename-wildcard = Wildcards * and ?
folders-tools-rename-match-case-tip = Match capitals exactly
folders-tools-rename-match-case = Match case
folders-tools-rename-add = Add
folders-tools-rename-before-tip = Before the name
folders-tools-rename-after-tip = After the name
folders-tools-rename-number = Number
folders-tools-rename-letters = Letters
folders-tools-rename-case-tip = Capitals in the names
folders-tools-rename-ending = Ending
folders-tools-rename-summary =
    { $clash ->
        [yes] Some names cannot be used; see the marked rows. In { $place }.
       *[no] { $changes } of { $total } names change. In { $place }.
    }
folders-tools-rename-button = Rename them
folders-tools-home = Home
folders-tools-dupes-title =
    { $n ->
        [one] Duplicates in { $folder }
       *[other] Duplicates in { $n } folders
    }
folders-tools-dupes-stopped = The search stopped: { $error }.
folders-tools-dupes-none = No duplicates were found.
folders-tools-dupes-unread =
    { $n ->
        [one] 1 file could not be read.
       *[other] { $n } files could not be read.
    }
folders-tools-dupes-sets =
    { $n ->
        [one] 1 set of copies; putting the extras in the Trash frees { $size }. The extra copies are picked; Ctrl and a click picks or drops one.
       *[other] { $n } sets of copies; putting the extras in the Trash frees { $size }. The extra copies are picked; Ctrl and a click picks or drops one.
    }
folders-tools-dupes-row = Set { $set }, { $size }, in { $at }
folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 file could not be read and was left out.
       *[other] { $n } files could not be read and were left out.
    }
folders-tools-dupes-keep-one = Every copy of a set is picked; keep at least one.
folders-tools-dupes-pick-extras = Pick the extra copies
folders-tools-dupes-trash = Move to Trash
folders-tools-diff-title = Compare the two
folders-tools-diff-failed = They could not be compared: { $error }.
folders-tools-diff-same-bytes = The two are the same, byte for byte.
folders-tools-diff-same-as-shorter = They are the same as far as the shorter goes; one is { $one } and the other { $other }.
folders-tools-diff-from-byte = They differ from byte { $n } on.
folders-tools-diff-lines =
    { $n ->
        [0] The two are the same.
        [one] 1 line differs.
       *[other] { $n } lines differ.
    }
folders-tools-diff-too-different = They are too different to match line by line, so every line between their first and last difference shows as changed.
folders-tools-diff-cut = Only the first 4 MB of each is compared.
folders-tools-compare-title = Compare { $left } with { $right }
folders-tools-compare-bytes = Read every byte (slower)
folders-tools-compare-bytes-tip = Compare what is inside each file
folders-tools-compare-show-same = Show what is the same
folders-tools-compare-show-same-tip = Show the names that are the same too
folders-tools-compare-stopped = The compare stopped: { $error }.
folders-tools-compare-counts = { $left } only on the left, { $right } only on the right, { $different } different, { $same } the same.
folders-tools-compare-all-same = Both folders hold the same.
folders-tools-compare-again = Compare again
folders-tools-compare-copy-left = Copy { $n } missing or newer to the left
folders-tools-compare-copy-right = Copy { $n } missing or newer to the right
