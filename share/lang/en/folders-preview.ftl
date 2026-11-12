# The quick look in Folders: the window that shows a picture, text, PDF, font, sound or video when Space is pressed (src/files/preview.rs).

## Quick look (src/files/preview.rs)

# Words in the quick look when a page of a PDF could not be drawn; the page number.
folders-preview-page-failed = Folders couldn't show page { $page }.
# Red notice at the foot of Folders when the rest of a long text could not be read; the reason.
folders-preview-rest-failed = Folders couldn't read the rest of it: { $why }
# Words in the quick look when a file on a server could not be fetched; the reason.
folders-preview-fetch-failed = Folders couldn't download it: { $why }
# Words in the quick look when a file's name says it is one kind but inside it is not; the kind, such as PDF or picture.
folders-preview-broken = Its name says it's { $what }, but its contents don't match. It may be damaged or incomplete.
# Words in the quick look when a picture is too large to show; the size in millions of pixels, a number.
folders-preview-too-large-pixels = This picture is too big to show here. It has { $millions } million pixels.
# Words in the quick look when a picture is too large to show; its size in bytes, as words such as 12.0 MB.
folders-preview-too-large-bytes = This picture is too big to show here. It's { $size }.
# Words in the quick look for a kind of file that cannot be shown.
folders-preview-other = Folders can't show this kind of file. Open it in its app instead.
# Words in the quick look when a file could not be read; the reason.
folders-preview-unread = Folders couldn't read it: { $why }
# Tip of the round button under a long text in the quick look that reads the next part.
folders-preview-read-more = Show more
# Line under a text in the quick look, for a name with an ending: how it is encoded, such as UTF-8.
folders-preview-text = { $encoding }
# Line under a long text in the quick look when only the first part is read: the encoding, then how much is read, as words such as 64 KB.
folders-preview-text-so-far = { $encoding }, first { $size } shown
# Line under a picture in the quick look: its width and height in pixels.
folders-preview-picture-size = { $w } by { $h } pixels
# Between the page buttons under a PDF in the quick look when the number of pages is not known; the page number.
folders-preview-page = Page { $page }
# Between the page buttons under a PDF in the quick look: the page number, then how many pages there are.
folders-preview-page-of = Page { $page } of { $pages }
# Tip of the round button under a PDF in the quick look that goes to the page before.
folders-preview-page-before = Previous page (Page Up)
# Tip of the round button under a PDF in the quick look that goes to the page after.
folders-preview-page-after = Next page (Page Down)
# Line under a PDF in the quick look: what kind of file it is.
folders-preview-pdf = PDF document
# Words in the quick look while a page of a PDF is being drawn.
folders-preview-drawing = Loading the page…
# Words under a font in the quick look once it is installed.
folders-preview-font-installed = Installed for you. Apps you open from now on can use it.
# Button under a font in the quick look that installs it.
folders-preview-font-install = Install font
# Words in the quick look when a sound or video cannot be played.
folders-preview-unplayable = Folders can't play this. It may be damaged, or in a format ffmpeg can't play.
# Line under a video in the quick look: width and height in pixels.
folders-preview-video = { $w } by { $h }
# Line under a video in the quick look: width and height in pixels, then its length such as 1:05.
folders-preview-video-length = { $w } by { $h }, { $length }
# Line under a sound in the quick look.
folders-preview-sound = Sound
# Line under a sound in the quick look: its length such as 1:05.
folders-preview-sound-length = { $length }
# Tip of the close button in the heading of the quick look.
folders-preview-close = Close (Space)
# Tip of the round button under the quick look that goes to the file before.
folders-preview-file-before = Previous file (Left)
# Tip of the round button under the quick look that goes to the file after.
folders-preview-file-after = Next file (Right)
# Name of the main button under the quick look that opens the file in its usual app.
folders-preview-open = Open in its app
# Screen reader name of a picture drawn in the quick look.
folders-preview-picture = Picture
# Screen reader name of a video drawn in the quick look.
folders-preview-video-name = Video
# Screen reader name of the sound icon drawn in the quick look.
folders-preview-sound-name = Sound
# Tip of the round button in the quick look that pauses a sound or video.
folders-preview-pause = Pause (Space)
# Tip of the round button in the quick look that plays a sound or video.
folders-preview-play = Play (Space)
# Screen reader name and tip of the slider that shows how far a sound or video has played and moves it.
folders-preview-position = Position
# Tip of the round button in the quick look that turns the sound back on.
folders-preview-unmute = Unmute
# Tip of the round button in the quick look that mutes the sound.
folders-preview-mute = Mute
# Screen reader name and tip of the volume slider in the quick look.
folders-preview-volume = Volume

## The kind of file is said only when the name has no ending to say it (20 October 2026).

# Under a text file's quick look, for a name with no ending: its kind, then how it is written.
folders-preview-kind-text = Text, { $encoding }
# The same, while only the first part is read.
folders-preview-kind-text-so-far = Text, { $encoding }, first { $size } shown
# Under a video's quick look, for a name with no ending.
folders-preview-kind-video = Video, { $w } by { $h }
# The same, with how long it runs.
folders-preview-kind-video-length = Video, { $w } by { $h }, { $length }
# Under a sound's quick look, for a name with no ending, with how long it runs.
folders-preview-kind-sound-length = Sound, { $length }
