# Folders' dealings with files dragged to and from other programs: the question asked about things another program names, and the words said about drops that are refused or fall short (src/files/outside.rs).

folders-outside-wary-hidden = hidden, or in a hidden folder
folders-outside-wary-holds = has hidden things in it
folders-outside-wary-deep = has too much in it to look through, so its hidden things are not brought
folders-outside-wary-system = outside your home folder and your drives
folders-outside-wary-leads = a link to something hidden, or outside your home folder and your drives
folders-outside-wary-holds-link = has a link in it to something hidden, or outside your home folder and your drives

folders-outside-title-move =
    { $n ->
        [one] Move { $name } here?
       *[other] Move these { $n } items here?
    }
folders-outside-title-bring =
    { $n ->
        [one] Bring { $name } here?
       *[other] Bring these { $n } items here?
    }
folders-outside-title-copy =
    { $n ->
        [one] Copy { $name } here?
       *[other] Copy these { $n } items here?
    }
folders-outside-body-move =
    { $n ->
        [one] Another program dropped it on Folders and named it itself. Any program can name any file, and what is named here is not all ordinary files of yours, so nothing is moved until you say.
       *[other] Another program dropped them on Folders and named them itself. Any program can name any file, and what is named here is not all ordinary files of yours, so nothing is moved until you say.
    }
folders-outside-body-bring =
    { $n ->
        [one] Another program dropped it on Folders and named it itself. Any program can name any file, and what is named here is not all ordinary files of yours, so nothing is brought until you say.
       *[other] Another program dropped them on Folders and named them itself. Any program can name any file, and what is named here is not all ordinary files of yours, so nothing is brought until you say.
    }
folders-outside-body-copy =
    { $n ->
        [one] Another program dropped it on Folders and named it itself. Any program can name any file, and what is named here is not all ordinary files of yours, so nothing is copied until you say.
       *[other] Another program dropped them on Folders and named them itself. Any program can name any file, and what is named here is not all ordinary files of yours, so nothing is copied until you say.
    }
folders-outside-leave =
    { $n ->
        [one] Leave it
       *[other] Leave them
    }
folders-outside-do-move =
    { $n ->
        [one] Move it
       *[other] Move them
    }
folders-outside-do-bring =
    { $n ->
        [one] Bring it
       *[other] Bring them
    }
folders-outside-do-copy =
    { $n ->
        [one] Copy it
       *[other] Copy them
    }
folders-outside-named-why = { $name }, from { $from }: { $why }
folders-outside-named = { $name }, from { $from }
folders-outside-more = And { $n } more.
folders-outside-into = Into { $into }.

folders-outside-answer-first = Answer the question about what was dropped first.
folders-outside-still-looking = The last thing dropped is still being looked at.
folders-outside-only-files = Only files and folders can be dropped here.
folders-outside-only-folders-pin = Only folders can be pinned to the sidebar.
folders-outside-only-on-folder = Things can only be dropped on a folder.
folders-outside-gone = { $name } is not there, so nothing was brought.
folders-outside-inside-itself = A folder cannot be put inside itself.
folders-outside-already-there = They are in that folder already.
folders-outside-not-answering = The place { $name } is in is not answering, so nothing was brought.
folders-outside-nothing-brought = Nothing was brought.
folders-outside-items-dragged =
    { $n ->
        [one] 1 item
       *[other] { $n } items
    }

folders-outside-lines-more = and { $n } more
folders-outside-stayed-one = { $name } { $why }.
folders-outside-stayed-changed =
    { $n ->
        [one] { $n } thing changed after it was picked up, so it is still where it was: { $names }.
       *[other] { $n } things changed after they were picked up, so they are still where they were: { $names }.
    }
folders-outside-stayed-many =
    { $n ->
        [one] { $n } thing is still where it was: { $each }.
       *[other] { $n } things are still where they were: { $each }.
    }
folders-outside-stayed-each = { $name } ({ $why })
folders-outside-both = { $first } { $second }
folders-outside-unshown = The other program took what was dragged in a way that cannot show it read all of it, so nothing here was moved: what it has is a copy.
folders-outside-short-move = The other program did not take all of what was dragged, so nothing here was moved.
folders-outside-short-copy = The other program did not take all of what was dragged, so it may not have all of it.
