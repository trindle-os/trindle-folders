# Folders' dealings with files dragged to and from other programs: the question asked about things another program names, and the words said about drops that are refused or fall short (src/files/outside.rs).

## Reasons a dropped thing is asked about (src/files/outside.rs)

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = hidden, or in a hidden folder
# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = has hidden items in it
# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = has too many items to check, so Folders will leave out its hidden items
# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = outside your home folder and your drives
# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = a link to something hidden, or outside your home folder and your drives
# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = has a link in it to something hidden, or outside your home folder and your drives

## The question card (src/files/outside.rs)

# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [one] Move { $name } here?
       *[other] Move these { $n } items here?
    }
# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [one] Bring { $name } here?
       *[other] Bring these { $n } items here?
    }
# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [one] Copy { $name } here?
       *[other] Copy these { $n } items here?
    }
# Words on the question card when the drop would be moved: why it is asked.
folders-outside-body-move =
    { $n ->
        [one] Another program dropped this on Folders. Any program can drop any file, and this isn't one of your ordinary files. Folders won't move it until you say so.
       *[other] Another program dropped these on Folders. Any program can drop any file, and some of these aren't your ordinary files. Folders won't move them until you say so.
    }
# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
        [one] Another program dropped this on Folders. Any program can drop any file, and this isn't one of your ordinary files. Folders won't bring it until you say so.
       *[other] Another program dropped these on Folders. Any program can drop any file, and some of these aren't your ordinary files. Folders won't bring them until you say so.
    }
# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
        [one] Another program dropped this on Folders. Any program can drop any file, and this isn't one of your ordinary files. Folders won't copy it until you say so.
       *[other] Another program dropped these on Folders. Any program can drop any file, and some of these aren't your ordinary files. Folders won't copy them until you say so.
    }
# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
        [one] Leave it
       *[other] Leave them
    }
# Button on the question card that moves what was dropped.
folders-outside-do-move =
    { $n ->
        [one] Move it
       *[other] Move them
    }
# Button on the question card that brings what was dropped.
folders-outside-do-bring =
    { $n ->
        [one] Bring it
       *[other] Bring them
    }
# Button on the question card that copies what was dropped.
folders-outside-do-copy =
    { $n ->
        [one] Copy it
       *[other] Copy them
    }
# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, from { $from }: { $why }
# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, from { $from }
# Line on the question card saying how many more things there are than are listed.
folders-outside-more = And { $n } more.
# Line on the question card saying which folder or place the things would go into.
folders-outside-into = Into { $into }.

## Refusals and notices on a drop (src/files/outside.rs)

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Answer the question about what you dropped first.
# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders is still checking the last thing you dropped.
# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Only files and folders can be dropped here.
# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = Only folders can be pinned to the sidebar.
# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = You can only drop things on a folder.
# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } isn't there any more, so nothing was copied or moved.
# Red words when a folder is dropped into itself.
folders-outside-inside-itself = You can't put a folder inside itself.
# Words when things are dragged into the folder they are already in.
folders-outside-already-there = They're already in that folder.
# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders couldn't reach the drive that { $name } is on in time, so nothing was copied or moved.
# Words when the question card is answered with no.
folders-outside-nothing-brought = Nothing was copied or moved.
# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] 1 item
       *[other] { $n } items
    }

## Things dragged out to another program (src/files/outside.rs)

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = and { $n } more
# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.
# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] { $n } item changed after you started dragging it, so Folders left it where it was: { $names }.
       *[other] { $n } items changed after you started dragging them, so Folders left them where they were: { $names }.
    }
# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] { $n } item is still where it was: { $each }.
       *[other] { $n } items are still where they were: { $each }.
    }
# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })
# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }
# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders can't tell whether the other program got everything you dragged, so it left the originals here. The other program has a copy.
# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = The other program didn't take everything you dragged, so Folders didn't move anything.
# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = The other program didn't take everything you dragged, so its copy may not be complete.
