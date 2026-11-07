# Folders' small windows opened from the right-click menu: Open with, Properties, Send to, Columns in the List view and Folder colour (src/files/dialogs.rs).

folders-dialogs-close = Close
folders-dialogs-home = Home

folders-dialogs-open-with-title = Open { $name } with
folders-dialogs-opens-now = Opens it now
folders-dialogs-no-apps = No app on this computer says it opens this kind of file.
folders-dialogs-no-match = No app has those words in its name.
folders-dialogs-find-app = Find an app
folders-dialogs-find-place = Find a place
folders-dialogs-no-match-place = No place has those words in its name.
folders-dialogs-always-use = Always use it
folders-dialogs-always-sub = For every { $kind } file.
folders-dialogs-always-tip = Make it the usual app for this kind
folders-dialogs-show-suggested = Show only the suggested apps
folders-dialogs-show-every = Show every app
folders-dialogs-open = Open
folders-dialogs-choose-app = Choose an app to open it with.
folders-dialogs-usual-failed = { $name } could not be made the usual app.
folders-dialogs-start-failed = { $name } could not be started.

folders-dialogs-properties = Properties
folders-dialogs-properties-pages = Part of Properties
folders-dialogs-page-general = General
folders-dialogs-page-details = Details
folders-dialogs-page-sharing = Sharing
folders-dialogs-row-name = Name
folders-dialogs-row-where = Where
folders-dialogs-row-kind = Kind
folders-dialogs-row-link-to = Link to
folders-dialogs-row-changed = Changed
folders-dialogs-row-owner = Owner
folders-dialogs-row-rights = Rights
folders-dialogs-row-size = Size
folders-dialogs-kind-unknown = Unknown
folders-dialogs-kind-with-name = { $words } ({ $kind })
folders-dialogs-rights-open-change = { $letters }, you can open and change it
folders-dialogs-rights-open-only = { $letters }, you can open it, not change it
folders-dialogs-rights-change-only = { $letters }, you can change it, not open it
folders-dialogs-rights-none = { $letters }, you cannot open it
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , some could not be read
       *[other]
    }{ $adding ->
        [yes] , still adding up
       *[other]
    }
folders-dialogs-size-adding = Adding up
folders-dialogs-checksums = Checksums
folders-dialogs-sums-work-out = Work out the checksums
folders-dialogs-sums-working = Working them out
folders-dialogs-sums-failed = The file could not be read
folders-dialogs-sums-paste = Paste a checksum to check
folders-dialogs-sums-matches = { $kind } matches
folders-dialogs-sums-no-match = No match
folders-dialogs-sums-check = Check

folders-dialogs-send-title =
    { $n ->
        [one] Send { $name } to
       *[other] Send { $n } items to
    }
folders-dialogs-send-note = A copy is made there; the items stay where they are.
folders-dialogs-send-other = The other pane, { $name }

folders-dialogs-columns-title = Columns in the List view
folders-dialogs-columns-note = Name is always shown. Columns that do not fit beside it are left out until the pane is wider.
folders-dialogs-columns-files = Files
folders-dialogs-columns-media = Music, photos, PDFs
folders-dialogs-columns-show = Show { $name }
folders-dialogs-columns-failed = The columns could not be saved.

folders-dialogs-colour-title =
    { $n ->
        [one] Colour for { $name }
       *[other] Colour for { $n } folders
    }
folders-dialogs-colour-none = No colour of its own
folders-dialogs-colour-failed = The folder colour could not be saved.
