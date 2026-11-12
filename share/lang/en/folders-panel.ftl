# Folders: the preview panel at the right of the window, with the facts it lists and the quiet words it says when a file cannot be shown.

## Preview panel (src/files/panel.rs)

# What a fact of the preview panel says until the figure has been worked out.
folders-panel-working = working out…
# Tip of the greyed toolbar button for the preview panel: why it cannot be switched on.
folders-panel-too-narrow = The window is too narrow for the preview panel
# Said at the window's foot when the preview panel is asked for in a window too narrow for it.
folders-panel-too-narrow-said = The window is too narrow for the preview panel.
# The Where fact of the preview panel for a thing straight in the home folder: the home folder by name.
folders-panel-home = Home
# The Where fact of the preview panel for a thing inside a folder of the home folder; rest is the path from Home.
folders-panel-home-under = Home/{ $rest }
# Said at the window's foot in red when the preview panel's switch or width could not be saved.
folders-panel-keep-failed = Folders couldn't save the preview panel's settings for next time.
# Name of a fact in the preview panel: how many things a folder, archive or place holds.
folders-panel-fact-holds = Contains
# Name of a fact in the preview panel: how big the thing is.
folders-panel-fact-size = Size
# Name of a fact in the preview panel: when the thing was last changed.
folders-panel-fact-changed = Changed
# Name of a fact in the preview panel: the folder the thing is in.
folders-panel-fact-where = Where
# Name of a fact in the preview panel: what kind of file it is.
folders-panel-fact-kind = Type
# Name of a fact in the preview panel: how big an archive's contents are unpacked.
folders-panel-fact-unpacked = Unpacked
# Name of a fact in the preview panel: the width and height of a picture or video in pixels.
folders-panel-fact-pixels = Pixels
# Name of a fact in the preview panel: the camera a picture was taken with.
folders-panel-fact-camera = Camera
# Name of a fact in the preview panel: how many pages a PDF has.
folders-panel-fact-pages = Pages
# Name of a fact in the preview panel: how long a video or sound plays.
folders-panel-fact-length = Length
# Name of a fact in the preview panel: the title of a song.
folders-panel-fact-title = Title
# Name of a fact in the preview panel: the artist of a song.
folders-panel-fact-artist = Artist
# Name of a fact in the preview panel: the album a song is on.
folders-panel-fact-album = Album
# Name of a fact in the preview panel: the name of a font.
folders-panel-fact-font = Font
# The Holds fact of the preview panel for a folder or archive that could not be read.
folders-panel-unread = could not be read
# The Holds fact of the preview panel for an archive that is locked with a password.
folders-panel-locked = locked with a password
# The Holds fact of the preview panel for a folder: how many things it holds, counting what is inside its folders; items is such as 3 items.
folders-panel-in-all = { $items } in all
# The Pixels fact of the preview panel: width by height of a picture or video.
folders-panel-by = { $w } by { $h }
# Quiet words under the icon in the preview panel for a file whose name says one kind but whose bytes are not; what is the kind, such as PDF document.
folders-panel-broken = Its name says it's { $what }, but its contents don't match.
# Quiet words under the icon in the preview panel for a picture too large to show.
folders-panel-too-large = This picture is too large to show here.
# Quiet words under the icon in the preview panel for a file that could not be read; why is the reason.
folders-panel-read-failed = Folders couldn't read it: { $why }
# Name at the top of the preview panel when several things are picked; count is how many.
folders-panel-selected = { $count } selected
# Name read out for the edge between the panes and the preview panel.
folders-panel-edge = Preview panel width
# Name read out for the picture in the preview panel; name is the file's.
folders-panel-picture-name = Preview of { $name }
# Name read out for the start of a text file in the preview panel; name is the file's.
folders-panel-text-name = Start of { $name }
