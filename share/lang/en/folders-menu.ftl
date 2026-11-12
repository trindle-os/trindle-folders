# Folders' right-click menu: its rows, the sub-menus they are kept in, and the words said after a row is chosen.

## Menu (src/files/menu.rs)

# Row of Folders' right-click menu (Rename), shown when one thing is picked.
folders-menu-rename = Rename
# Row of Folders' right-click menu (Rename), shown when several things are picked.
folders-menu-rename-them = Rename them
# Row of Folders' right-click menu (Install font), shown when one thing is picked.
folders-menu-install-the-font-for-you = Install font just for you
# Row of Folders' right-click menu (Install font), shown when several things are picked.
folders-menu-install-the-fonts-for-you = Install fonts just for you
# Row of Folders' right-click menu that does: Open.
folders-menu-open = Open
# Row of Folders' right-click menu that does: Properties.
folders-menu-properties = Properties
# Row of Folders' right-click menu that does: Select all.
folders-menu-select-all = Select all
# Row of Folders' right-click menu that does: Select none.
folders-menu-select-none = Select none
# Row of Folders' right-click menu that does: Invert.
folders-menu-invert-selection = Invert selection
# Row of Folders' right-click menu that does: Copy.
folders-menu-copy = Copy
# Row of Folders' right-click menu that does: Copy path.
folders-menu-copy-location = Copy location
# Row of Folders' right-click menu that does: Pin.
folders-menu-unpin = Unpin
# Row of Folders' right-click menu that does: Pin.
folders-menu-pin-to-the-sidebar = Pin to the sidebar
# Row of Folders' right-click menu that does: Colour.
folders-menu-folder-colour = Folder colour
# Row of Folders' right-click menu that does: Terminal.
folders-menu-open-in-a-terminal = Open in a terminal
# Row of Folders' right-click menu that does: As root.
folders-menu-open-as-administrator = Open as administrator
# Row of Folders' right-click menu that does: Delete.
folders-menu-delete-permanently = Delete permanently
# Row of Folders' right-click menu that does: Dupes.
folders-menu-find-duplicates = Find duplicates
# Row of Folders' right-click menu that does: New folder.
folders-menu-new-folder = New folder
# Row of Folders' right-click menu that does: New document.
folders-menu-new-document = New document
# Row of Folders' right-click menu that does: Extract here.
folders-menu-take-out-next-to-the-archive = Extract next to the archive
# Row of Folders' right-click menu that does: Empty trash.
folders-menu-empty-the-trash = Empty the Trash
# Row of Folders' right-click menu that does: Restore.
folders-menu-put-back = Put back
# Row of Folders' right-click menu that does: Open with.
folders-menu-open-with-another-app = Open with another app
# Row of Folders' right-click menu that does: Preview.
folders-menu-preview = Preview
# Row of Folders' right-click menu that does: Show folder.
folders-menu-show-in-its-folder = Show in its folder
# Row of Folders' right-click menu that does: Cut.
folders-menu-cut = Cut
# Row of Folders' right-click menu that does: Trash.
folders-menu-move-to-trash = Move to Trash
# Row of Folders' right-click menu that does: Paste.
folders-menu-paste = Paste
# Row of Folders' right-click menu that does: Open tab.
folders-menu-open-in-a-new-tab = Open in a new tab
# Row of Folders' right-click menu that does: Open window.
folders-menu-open-in-a-new-window = Open in a new window
# Row of Folders' right-click menu that does: Link.
folders-menu-make-a-link = Make a link
# Row of Folders' right-click menu that does: Send to.
folders-menu-send-to = Send to
# Row of Folders' right-click menu that does: Extract here.
folders-menu-extract-here = Extract here
# Row of Folders' right-click menu that does: Extract to.
folders-menu-extract-to-a-folder = Extract to a folder
# Row of Folders' right-click menu that does: Extract to.
folders-menu-extract-each-to-its-own-folder = Extract each to its own folder
# Row of Folders' right-click menu that does: Compress.
folders-menu-compress = Compress
# Row of Folders' right-click menu that does: Wipe.
folders-menu-wipe-securely = Wipe securely
# Row of Folders' right-click menu that does: Share on network.
folders-menu-share-on-the-network = Share on the network
# Row of Folders' right-click menu that does: Share with ai.
folders-menu-share-with-ai-programs = Share with AI programs
# Row of Folders' right-click menu that does: Wallpaper.
folders-menu-set-as-wallpaper = Set as wallpaper
# Row of Folders' right-click menu that does: Rotate left.
folders-menu-turn-left = Rotate left
# Row of Folders' right-click menu that does: Rotate right.
folders-menu-turn-right = Rotate right
# Row of Folders' right-click menu that does: Turn upside down.
folders-menu-turn-upside-down = Turn upside down
# Row of Folders' right-click menu that does: Resize.
folders-menu-resize = Resize
# Row of Folders' right-click menu that does: Compare files.
folders-menu-compare-the-two = Compare the two
# Row of Folders' right-click menu that does: Copy here.
folders-menu-copy-here = Copy here
# Row of Folders' right-click menu that does: Move here.
folders-menu-move-here = Move here
# Row of Folders' right-click menu that does: Link here.
folders-menu-make-a-link-here = Make a link here
# Row of Folders' right-click menu that does: Leave them.
folders-menu-cancel = Cancel
# Row of Folders' right-click menu that does: Pin up.
folders-menu-move-up = Move up
# Row of Folders' right-click menu that does: Pin down.
folders-menu-move-down = Move down
# Row of Folders' right-click menu that does: Ring pin.
folders-menu-unpin-from-the-ring = Unpin from the ring
# Row of Folders' right-click menu that does: Ring pin.
folders-menu-pin-to-the-ring = Pin to the ring
# Row of Folders' right-click menu that opens a sub-menu of select rows; the sub-menu is named for what is in it.
folders-menu-sub-select = Select
# Row of Folders' right-click menu that opens a sub-menu of pack rows; the sub-menu is named for what is in it.
folders-menu-sub-pack = Compress, send and share
# Row of Folders' right-click menu that opens a sub-menu of picture rows; the sub-menu is named for what is in it.
folders-menu-sub-picture = Picture
# Row of Folders' right-click menu that opens a sub-menu of tools rows; the sub-menu is named for what is in it.
folders-menu-sub-tools = Tools
# Red words under the file lines when showing or hiding a column could not be saved.
folders-menu-columns-not-saved = Folders couldn't save the columns.
# Red words under the file lines when Open in a new window failed.
folders-menu-no-new-window = Folders couldn't open a new window.
# Words under the file lines after Copy location: one thing, or the number of things whose locations were copied.
folders-menu-copied-location =
    { $n ->
        [one] Copied its location.
       *[other] Copied { $n } locations.
    }
# Red words under the file lines when Open as administrator failed.
folders-menu-no-admin-window = Folders couldn't open a window as administrator.
# Red words under the file lines when a picture cannot be the wallpaper; the name of the picture, then the reasons.
folders-menu-wallpaper-refused = { $name } can't be used as the wallpaper: { $why }.
# Words under the file lines after a picture was set as the wallpaper.
folders-menu-wallpaper-set = { $name } is now the wallpaper.
# Red words under the file lines when setting the wallpaper failed.
folders-menu-wallpaper-failed = Folders couldn't set the wallpaper.
# Name of the undo step after a link was made; shown where undo is offered.
folders-menu-undo-link = Undo making { $name }
# Words under the file lines after a link was made.
folders-menu-link-made = Made { $name }.
# Red words under the file lines when a link could not be made; the reason follows.
folders-menu-link-failed = Folders couldn't make the link: { $why }.
# Words under the file lines after a folder was shared with AI programs.
folders-menu-ai-shared = { $name } is shared with AI programs.
# Words under the file lines after a folder or file was taken off the ring.
folders-menu-ring-off = Unpinned { $name } from the ring.
# Words under the file lines after a folder or file was pinned to the ring.
folders-menu-ring-pinned = Pinned { $name } to the ring, next to your favourite apps.
# Words under the file lines when the thing to pin is on the ring already.
folders-menu-ring-already = { $name } is already on the ring.
# Red words under the file lines when the ring has no room for another pin; the number is how many it holds.
folders-menu-ring-full = The ring is full. It has room for { $n } pins, so unpin one first.
# Red words under the file lines when a name cannot be written in the ring's list of favourites.
folders-menu-ring-unkept = Folders can't pin { $name } to the ring because its name has a character Folders can't save there.
# Red words under the file lines when the ring's favourites list could not be saved.
folders-menu-ring-not-saved = Folders couldn't save the ring's favourites.
# Name given to a new link to a file or folder; the name of the thing linked to follows.
folders-menu-link-name = Link to { $name }
