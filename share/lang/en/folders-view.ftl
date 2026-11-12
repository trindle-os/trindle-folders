# Folders window: toolbar, side bar, strip between the panes, pane bars, empty pages, status line and the questions laid over the window.

## The window (src/files/view.rs)

# Title of the question window asked before the Trash is emptied.
folders-view-empty-trash-title = Empty the Trash?
# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
folders-view-empty-trash-body =
    { $n ->
        [one] The item in it will be deleted permanently. You can't undo this.
       *[other] All { $n } items in it will be deleted permanently. You can't undo this.
    }
# Red button of the question asked before the Trash is emptied: empties it at once.
folders-view-empty-trash = Empty the Trash
# Title of the question window asked before items are deleted for good; n is how many.
folders-view-delete-title =
    { $n ->
        [one] Delete this item permanently?
       *[other] Delete these { $n } items permanently?
    }
# Text of the question asked before items are deleted for good.
folders-view-delete-body = You can't undo this.
# Red button of the question asked before items are deleted for good: deletes them at once.
folders-view-delete-permanently = Delete permanently
# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
folders-view-keep-them = Keep them
# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = Where should they go?
# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Leave them where they are
# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = Two panes
# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = Show two panes side by side (F3)
# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Show one pane (F3)
# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = View
# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = New
# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = More options
# Row of the More menu that undoes the last change.
folders-view-more-undo = Undo
# Row of the More menu that does again what was undone.
folders-view-more-redo = Redo
# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Sort the other way
# Row of the View menu that shows a folder as icons.
folders-view-icons = Icons
# Row of the View menu that shows a folder as a list.
folders-view-list = List
# Row of the View menu that shows a folder as a compact list.
folders-view-compact = Compact
# Row of the View menu that opens the choice of columns of the list view.
folders-view-columns = Choose the columns
# First row of the View menu's Sort by sub-menu: sort by name.
folders-view-sort-name = Name
# Row of the View menu that opens the Sort by sub-menu.
folders-view-sort-by = Sort by
# Row of the View menu that opens the Group by sub-menu.
folders-view-group-by = Group by
# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Show hidden files (Ctrl+H)
# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Hide hidden files (Ctrl+H)
# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Hidden files
# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Hide the preview panel (Alt+P)
# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Show the preview panel (Alt+P)
# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Preview panel
# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Tabs
# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Left pane tabs
# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Right pane tabs
# Tip of the button that opens a new tab.
folders-view-tab-new = New tab (Ctrl+T)
# Tip of the button that closes the tab showing.
folders-view-tab-close = Close tab (Ctrl+W)
# Side bar row that opens the Home folder.
folders-view-home = Home
# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Nothing pinned yet. Right-click a folder to pin it.
# Small heading in the side bar over the pinned folders.
folders-view-pinned = Pinned
# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Places
# Small heading in the side bar over Storage and the drives plugged in.
folders-view-drives = Devices
# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Shares
# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = Close { $name }
# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Nearby computers
# Last row under the Network heading of the side bar, which opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Connect to a server
# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = Link the two panes. { $note }
# Tip of the strip button that links the two panes so they move together.
folders-view-link = Link the two panes
# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] Copy to the left pane
       *[other] Copy to the right pane
    }
# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] Move to the left pane
       *[other] Move to the right pane
    }
# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Show this folder in the left pane
       *[other] Show this folder in the right pane
    }
# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Swap the panes
# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Select what differs from the left pane
       *[other] Select what differs from the right pane
    }
# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] Compare with the left pane
       *[other] Compare with the right pane
    }
# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Type a folder, file or server address
# Name read out for the path above a pane's list.
folders-view-where = Location
# Tip of the pane's Back button.
folders-view-back = Back (Alt+Left)
# Tip of the pane's Forward button.
folders-view-forward = Forward (Alt+Right)
# Tip of the pane's button that goes up one folder.
folders-view-up = Up one folder (Alt+Up)
# Tip of the Search button in the top row while the path shows.
folders-view-search = Search (Ctrl+F)
# Tip of the Search button while the search box shows.
folders-view-search-close = Close search (Escape)
# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = You can't search on this page
# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Search
# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Filter, or press Enter to search this folder and its subfolders
# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Search names, including subfolders
# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = Search inside files, including subfolders
# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Type
# Name read out for the list that picks which size to show.
folders-view-find-size = Size
# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Last changed
# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = New name for { $was }
# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = Sidebar name for { $was }
# Far end of a pane's foot: the room left on its disk.
folders-view-status-free = { $size } free
# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } selected, working out the size…
# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } selected, { $size }
# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }
# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Show the files this is about
# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Try again
# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
folders-view-cancel = Cancel
# Line on a pane whose filter matches nothing.
folders-view-none-matches = Nothing here matches the filter
# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Show everything
# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = You haven't opened anything recently
# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Go to Home
# Line on the Trash page when it is empty.
folders-view-none-trash = The Trash is empty
# Line on the Computer page when there are no drives to show.
folders-view-none-drives = No drives to show
# Line on a search page that found nothing.
folders-view-none-found = Nothing found
# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Clear the search
# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = This archive is empty
# Button on an empty archive page: goes up one folder.
folders-view-go-up = Go up one folder
# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = This folder is empty
# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = New folder
# Button on an empty folder's page that makes a new document.
folders-view-new-document = New document
# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] 1 item
       *[other] { $n } items
    }
# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Not connected
# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = Reconnect { $name }
# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } is not connected.
# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Reconnect all
# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] One share is not connected.
       *[other] { $n } shares are not connected.
    }
# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = Forget { $name }
