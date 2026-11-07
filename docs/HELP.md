# Getting started with Folders

Folders is the file manager. It shows your files and folders, and lets you copy, move, rename, delete, search and open them.

## The first minute

- One pane opens on Home. Press F3, or the two panes button in the top bar, to see two panes side by side. Ctrl+T opens a tab; tabs show once there is more than one.
- The top bar is about the pane you are using: back, forward, up and the path, then New (a new folder or document), View (how items are shown, sorted and grouped, and hidden files) and More (undo, redo and settings).
- Under the top bar, Location shows where you are in full; click it, or press Ctrl+L, to type a place. Search, or Ctrl+F, searches there instead.
- The sidebar on the left lists your places, your drives and the network. Click one to show it in the pane you are using. The pane in use has the slightly different edge.
- Right-click anything for its menu. The less used actions sit in sub-menus. Double-click, or press Enter, to open.
- Copy, move and compare buttons for the two panes sit in the strip between them.
- Deleting sends a file to the Trash. Delete permanently and Wipe securely are separate, and always ask first.
- Nothing is saved with a Save button. A change is kept as soon as you make it.

## Settings

Press Ctrl and comma, or choose Folders settings in the top bar's More menu. The settings open as a card over the window:

- where Folders opens (Home and Recent, where each pane was left, or places you choose);
- which file endings and tick boxes show;
- opening a drive when you plug it in (this needs Trindle's ring, and is greyed without it);
- kept logins: the computers Folders remembers a login for, how many days are left, and a red Forget button.

If a change cannot be kept, the words "It could not be saved." appear in red under it.

## Drives

Drives appear in the sidebar. Click one to open it, or use its eject button. A locked drive asks for its password in a small window. To hide a drive you never use, use its menu; hidden drives can be brought back. Folders needs UDisks for this. Without it, the drives part is left out and the rest works.

## Shares on the network

- Network lists the computers at home that share folders, such as a NAS, a Mac, a Linux computer or a Windows computer. Open one to see its shares, and open a share to use it like a folder.
- You can also type an address, for example `smb://nas/photos` or `sftp://name@server/`.
- A login is only sent to the address it was first used at. If a computer answers at a new address, Folders says so and asks you to type the login again.
- Share on the network (right-click, Pack, send and share) shares one of your own folders. It needs Trindle's helper and asks for the administrator's password.

## Share with AI programs

Right-click a folder and choose Share with AI programs to let Trindle's AI programs see that folder and nothing else. Folders says "[name] is shared with AI programs." when it is done. This needs Trindle's own program for AI sharing; if it is missing, Folders says so. Only you can do this: a made-up key press or click from another program is refused.

## Safety with files from other people

Archives, SVG drawings, films and PDFs are opened in a closed box, with no network and no access to your home folder. Pictures and fonts are read by Folders itself, not in the box: a font is size-limited and read by a parser that forbids unsafe code. Folders never follows a name that came from inside a file, never writes over a file without asking, and never deletes without checking first.

## Anything missing

Folders runs on its own. When another Trindle part is not on the computer (the ring, the helper, Apps, Help, wallpaper), the matching menu row is left out or greyed with a sentence saying why.
