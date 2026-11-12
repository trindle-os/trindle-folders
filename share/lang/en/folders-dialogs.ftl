# Folders' small windows opened from the right-click menu: Open with, Properties, Send to, Columns in the List view and Folder colour (src/files/dialogs.rs).

## Shared window words (src/files/dialogs.rs)

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = Close
# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = Home

## Open with (src/files/dialogs.rs)

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = Open { $name } with
# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = Current app
# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = No app on this computer is set up to open this kind of file.
folders-dialogs-no-match = No app matches those words.
folders-dialogs-find-app = Find an app
folders-dialogs-find-place = Find a place
folders-dialogs-no-match-place = No place matches those words.
# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = Always use this app
# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = Use it for every { $kind } file.
# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = Set as the default app
# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = Show only suggested apps
# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = Show all apps
# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = Open
# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = Choose an app first.
# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders couldn't make { $name } the default app.
# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders couldn't start { $name }.

## Properties (src/files/dialogs.rs)

# Heading of the Properties window.
folders-dialogs-properties = Properties
# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = Properties pages
# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = General
# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = Details
# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = Sharing
# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = Name
# Label in Properties of the folder the item is in.
folders-dialogs-row-where = Where
# Label in Properties of the kind of file.
folders-dialogs-row-kind = Kind
# Label in Properties of where a link points.
folders-dialogs-row-link-to = Link to
# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = Changed
# Label in Properties of who owns the item.
folders-dialogs-row-owner = Owner
# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = Permissions
# Label in Properties of the size of the item.
folders-dialogs-row-size = Size
# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = Unknown
# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })
# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, you can open and change it
# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, you can open it but not change it
# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, you can change it but not open it
# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, you can't open it
# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , some couldn't be read
       *[other]
    }{ $adding ->
        [yes] , still adding up
       *[other]
    }
# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = Working out the size
# Label in Properties of the row about checksums.
folders-dialogs-checksums = Checksums
# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = Work out checksums
# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = Working out the checksums
# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders couldn't read the file.
# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = Paste a checksum to compare
# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } matches
# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = No match
# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = Check

## Send to (src/files/dialogs.rs)

# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [one] Send { $name } to
       *[other] Send { $n } items to
    }
# Note in the Send to window saying what sending does.
folders-dialogs-send-note = This makes a copy there. The originals stay where they are.
# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = The other pane, { $name }

## Columns (src/files/dialogs.rs)

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = List view columns
# Note in the Columns window.
folders-dialogs-columns-note = Name is always shown. Columns that don't fit are hidden until the pane is wider.
# Heading over the columns that suit every file.
folders-dialogs-columns-files = Files
# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = Music, photos, PDFs
# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = Show { $name }
# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders couldn't save the columns.

## Folder colour (src/files/dialogs.rs)

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [one] Colour for { $name }
       *[other] Colour for { $n } folders
    }
# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = No colour
# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders couldn't save the folder colour.
