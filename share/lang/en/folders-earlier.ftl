# Words of Folders' earlier copies: looking at older versions of a file or folder, things deleted from a folder, and putting them back.

## Earlier copies (src/files/earlier.rs)

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = You can't change anything in an earlier copy. Copy things out of it to keep them.
# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] today at { $time }
        [yesterday] yesterday at { $time }
       *[other] { $date } at { $time }
    }
# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] Today at { $time }
        [yesterday] Yesterday at { $time }
       *[other] { $date } at { $time }
    }
# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = This is an earlier copy from { $when }. You can't change it, but you can copy things out of it.
# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = This is an earlier copy. You can't change it, but you can copy things out of it.
# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = Earlier: { $name }
# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = Undo putting back { $name }
# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = Folders put it back. The current one is next to it under a new name.
# Said under a version's row after it is put back where it was.
folders-earlier-put-back = Folders put it back where it was.
# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = It's the same as the current one.
# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = It's different from the current one.
# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = Folders couldn't compare them: { $why }
# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = Earlier versions
# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = Things deleted from here
# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = Earlier versions of { $name }
# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = There are no earlier versions that differ from the current one.
# Note at the foot of the earlier versions card about what putting one back does.
folders-earlier-versions-foot = Putting one back keeps the current one next to it. You can undo this.
# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = Things deleted from { $name }
# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = Nothing from the earlier copies has been deleted from this folder.
# Note at the foot of the deleted things card about what putting one back does.
folders-earlier-deleted-foot = Putting one back returns it to this folder. You can undo this.
# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = The folder as it was then
# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }, from the copy made { $date }
# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = A folder, last seen { $when }
# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }, last seen { $when }
# Second line of a version's row while it is being put back or compared.
folders-earlier-working = Working…
# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = Open that day's folder in a new tab
# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = Open
# Tip and name of the round button on a row of the earlier versions card that puts it back.
folders-earlier-put-back-tip = Put it back
# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = Compare with the current one
# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = That isn't an earlier version.
# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = You can only put back an earlier version inside your home folder.
# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = Folders can't put a file back there.
# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = Folders couldn't recreate the folder it was in: { $why }
# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = Folders couldn't copy the earlier version: { $why }
# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
folders-earlier-put-back-aside-failed = Folders couldn't move the current one out of the way: { $why }
# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
folders-earlier-put-back-in-place-named = Folders couldn't put the earlier version back: { $why }. The current one is now called { $name }.
# Red words when the earlier version could not be put in place; why it failed.
folders-earlier-put-back-in-place = Folders couldn't put the earlier version back: { $why }
