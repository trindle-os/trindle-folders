# Words of the Folders program itself: what it says at the foot of its window, in the panes and while items are dragged.

## Folders (src/files/app.rs)

# Note over the panes when the panes are linked and the other pane has no folder that matches the one opened here.
folders-app-link-no-folder = There's no folder with that name in the other pane
# Note over the panes when the panes are linked and the other pane has no folder with the name of the one opened here; name is that folder's name.
folders-app-link-no-folder-named = There's no folder called { $name } in the other pane
# Said at the foot of the window after a folder is opened in a new tab behind the current one; name is the folder's name.
folders-app-opened-tab = Opened { $name } in a new tab.
# Red words at the foot of the window when a new Folders window could not be started.
folders-app-no-window = Folders couldn't open a new window.
# Red words at the foot of the window when the choice of one pane or two could not be saved.
folders-app-panes-not-kept = Folders couldn't save how many panes to show next time.
# Red words at the foot of the window when the path typed in the path bar leads nowhere; text is what was typed.
folders-app-nothing-at = There's nothing at { $text }.
# Red words at the foot of the window when the program chosen to open a file could not be started; name is the program's name.
folders-app-not-started = Folders couldn't start { $name }.
# Said in a window run as administrator when a file, an archive or a preview is asked for.
folders-app-root-folders-only = In an administrator window, you can only work with folders. You can't open, look inside or preview files as administrator.
# Red words at the foot of the window when the list of pinned folders could not be saved.
folders-app-pins-not-saved = Folders couldn't save your pinned folders.
# Said at the foot of the window when a folder is dropped on the sidebar and is already pinned.
folders-app-pinned-already = That's already pinned to the sidebar.
# Said at the foot of the window after one folder is pinned to the sidebar; name is the folder's name.
folders-app-pinned-one = Pinned { $name } to the sidebar.
# Said at the foot of the window after several folders are pinned to the sidebar; n is how many.
folders-app-pinned-many =
    { $n ->
        [one] Pinned { $n } folder to the sidebar.
       *[other] Pinned { $n } folders to the sidebar.
    }
# What several dragged items are called in the words at the foot of the window; n is how many.
folders-app-items =
    { $n ->
        [one] { $n } item
       *[other] { $n } items
    }
# Said at the foot of the window while items are being dragged; what is the item's name or how many there are.
folders-app-dragging = Moving { $what }. Hold Ctrl to copy.
# Said at the foot of the window while items are held after a right-button drag; what is the item's name or how many there are.
folders-app-holding = Dragging { $what }. Let go to choose what to do.
# Said at the foot of the window when the last tab of a pane is closed and the other pane still has tabs.
folders-app-only-tab = This is the only tab in this pane, so it stays open while the other pane has tabs.
# Red words at the foot of the window when the choice to link the panes could not be saved.
folders-app-link-not-kept = Folders couldn't save whether the panes are linked.
# Red words at the foot of the window when the view chosen for a folder could not be saved.
folders-app-view-not-kept = Folders couldn't save the view for this folder.
# Red words at the foot of the window when the choice of grouping could not be saved.
folders-app-grouping-not-kept = Folders couldn't save the grouping for next time.
# Red words when the list of what opens what cannot be read, so the choice is not written; why it cannot be read.
folders-app-opener-unreadable = Folders couldn't read which app opens each kind of file: { $why }. Nothing was changed.
