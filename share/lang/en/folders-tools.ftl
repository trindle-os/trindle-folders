# Folders: the windows the right-click menu opens for the bigger jobs: Compress, Extract to, renaming many at once, Wipe, finding duplicates, comparing the two panes and two files, and resizing pictures.

## Tools (src/files/tools.rs)

# Choice in the Wipe window: write over each file one time.
folders-tools-pass-once = Once
# Choice in the Wipe window: write over each file three times.
folders-tools-pass-three = 3 times
# Choice in the Wipe window: write over each file seven times.
folders-tools-pass-seven = 7 times
# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = As they are
# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = All small letters
# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = All capitals
# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = Capitalise each word
# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = Capitalise the first word
# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = As it is
# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = All small letters
# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = All capitals
# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = New ending
# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = No ending
# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = Archive
# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = no name would be left
# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = that is not allowed as a name
# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = that name is too long
# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = two items would get this name
# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = something else already has this name
# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = the old name has characters Folders can't read
# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
folders-tools-caveat-flash = This is a flash drive or SSD, so old copies may stay on parts of it that Folders can't reach.
# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
folders-tools-caveat-copy-on-write = The { $fs } file system saves changes in a new place, so the old data may still be there.
# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
folders-tools-caveat-network = This is a { $fs } share on another computer, which may keep its own copies.
# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = Only on the left
# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = Only on the right
# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = The same
# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
folders-tools-row-different =
    Different: { $why ->
        [kind] one is a folder or link
        [size] sizes differ
        [date] dates differ
        [content] contents differ
        [target] links point elsewhere
        [inside] something inside differs
       *[unread] one side couldn't be read
    }{ $newer ->
        [left] , newer on the left
        [right] , newer on the right
       *[none] { "" }
    }
# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = Here
# Red notice when Wipe is chosen on things that are not files.
folders-tools-only-files-wiped = Only files can be wiped.
# Line in the Wipe window naming the folders that were picked too, which are not wiped.
folders-tools-wipe-left-out = Only files are wiped, so these folders will be left as they are: { $names }.
# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = Open a folder in each pane to compare them.
# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
        [one] Undo renaming 1 item
       *[other] Undo renaming { $n } items
    }
# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [one] Renamed 1 item.
       *[other] Renamed { $n } items.
    }
# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = Renaming stopped part way through: { $error }.
# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = Type a name for the archive.
# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = Checked { $done }
# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = Read { $done } of { $total }
# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = Listing files
# Beside the ring while two files are compared.
folders-tools-comparing = Comparing
# Beside the ring while two folders are compared.
folders-tools-looking = Comparing the folders
# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [one] Compress { $name }
       *[other] Compress { $n } items
    }
# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = Name of the archive
# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = Kind of archive
# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = The archive will be saved in { $place }
# Main button of the Compress window.
folders-tools-compress = Compress
# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = Extract { $name } to
# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = The files will be put in a new folder called { $name }.
# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [one] Resize { $name }
       *[other] Resize { $n } pictures
    }
# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = New size
# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = The size is the length of the longer side in pixels. Folders only makes pictures smaller. It saves each resized copy beside the original, with the new size in its name.
# Main button of the Resize window.
folders-tools-resize = Resize
# Title of the Wipe window; { $name } is the one file, { $n } how many.
folders-tools-wipe-title =
    { $n ->
        [one] Wipe { $name }?
       *[other] Wipe { $n } files?
    }
# Warning at the top of the Wipe window.
folders-tools-wipe-warning = Folders writes over each file before deleting it, so you can't get it back.
# Name beside the choice of how many times to write over, in the Wipe window.
folders-tools-wipe-write-over = Write over
# Choice tip in the Wipe window: how many times each file is written over.
folders-tools-wipe-passes-tip = How many times to write over each file
# Red main button of the Wipe window.
folders-tools-wipe-button = Wipe securely
# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = Number them
# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = At the start
# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = At the end
# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = Where the number goes
# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = First number
# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = What to do with the ending
# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = New ending, such as txt
# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders can't rename this: { $why }
# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = was { $old }
# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = stays as it is
# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [one] Rename 1 item
       *[other] Rename { $n } items
    }
# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = Replace
# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = Find
# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = Replace with
# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = Use * and ? as wildcards
# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = Wildcards * and ?
# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = Match capital letters exactly
# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = Match case
# Name of the before and after boxes in the rename window.
folders-tools-rename-add = Add
# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = Before the name
# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = After the name
# Name of the numbering row in the rename window.
folders-tools-rename-number = Number
# Name over the capitals choice in the rename window.
folders-tools-rename-letters = Capitals
# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = Capital letters in the names
# Name over the ending choice in the rename window.
folders-tools-rename-ending = Ending
# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] In { $place }, some names can't be used. See the marked rows.
       *[no] In { $place }, { $changes } of { $total } names will change.
    }
# Main button of the rename window.
folders-tools-rename-button = Rename
# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = Home
# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [one] Duplicates in { $folder }
       *[other] Duplicates in { $n } folders
    }
# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = The search stopped: { $error }.
# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = No duplicates were found.
# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [one] 1 file could not be read.
       *[other] { $n } files could not be read.
    }
# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
        [one] 1 set of copies. The extra copies are selected, and moving them to the Trash frees { $size }.
       *[other] { $n } sets of copies. The extra copies are selected, and moving them to the Trash frees { $size }.
    }
# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = Set { $set }, { $size }, in { $at }
# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 file could not be read and was left out.
       *[other] { $n } files could not be read and were left out.
    }
# Note in the duplicates window when every copy of a set is picked.
folders-tools-dupes-keep-one = Every copy in a set is selected. Leave at least one unselected.
# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = Select the extra copies
# Main button of the duplicates window: move the picked files to the Trash.
folders-tools-dupes-trash = Move to Trash
# Title of the window that compares two files.
folders-tools-diff-title = Compare two files
# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders couldn't compare them: { $error }.
# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = The two files are exactly the same.
# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = They're the same up to the end of the shorter one. One is { $one } and the other is { $other }.
# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = They're different from byte { $n } onwards.
# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] The two files are the same.
        [one] 1 line differs.
       *[other] { $n } lines differ.
    }
# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = They're too different to compare line by line. Everything between the first and last difference shows as changed.
# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = Only the first 4 MB of each file is compared.
# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = Compare { $left } with { $right }
# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = Compare contents (slower)
# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = Compare what's inside each file
# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = Show matching names
# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = List matching names too
# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = The comparison stopped: { $error }.
# Note in the pane compare window with the four counts.
folders-tools-compare-counts = { $left } only on the left, { $right } only on the right, { $different } different, { $same } the same.
# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = Both folders have the same contents.
# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = Compare again
# Button in the pane compare window; { $n } is how many names are copied to the left folder.
folders-tools-compare-copy-left = Copy { $n } missing or newer items to the left
# Button in the pane compare window; { $n } is how many names are copied to the right folder.
folders-tools-compare-copy-right = Copy { $n } missing or newer items to the right
