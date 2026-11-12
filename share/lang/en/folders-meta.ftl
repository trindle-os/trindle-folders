# Folders' reading and writing of tags inside songs and notes inside photos: the reasons given when a change is refused or a file is damaged (src/files/meta.rs).

## Song and photo tags (src/files/meta.rs)

# Reason given when a file to read or change is a link, not the file itself.
folders-meta-link = this is a link, not a file
# Reason given when a photo's notes cannot be read; the cause follows, already written out.
folders-meta-damaged = the photo's camera details are damaged because { $why }
# Cause of damaged photo notes: a block points outside the notes.
folders-meta-bad-out-of-reach = one part points outside them
# Cause of damaged photo notes: a block goes beyond the end.
folders-meta-bad-block-past-end = one part runs past their end
# Cause of damaged photo notes: the notes have no TIFF header.
folders-meta-bad-no-tiff = their TIFF header is missing
# Cause of damaged photo notes: the notes are much too large.
folders-meta-bad-too-big = they are far too big
# Cause of damaged photo notes: the parts of a JPEG photo do not line up.
folders-meta-bad-jpeg-parts = the parts of the JPEG don't fit together
# Cause of damaged photo notes: a part of a JPEG photo goes beyond the end.
folders-meta-bad-jpeg-past-end = a part of the JPEG runs past the end of the file
# Cause of damaged photo notes: a part of a PNG photo goes beyond the end.
folders-meta-bad-png-past-end = a part of the PNG runs past the end of the file
# Cause of damaged photo notes: a PNG file holds no picture.
folders-meta-bad-png-empty = the PNG has no picture in it
# Reason a photo's date taken is refused: it is not written like the example.
folders-meta-refuse-date = the date taken must be written like 2026:01:02 03:04:05
# Reason a photo's note is refused: it holds a nul character.
folders-meta-refuse-nul = a note can't contain a null character
# Reason a photo's way-up setting is refused: it is not from 1 to 8.
folders-meta-refuse-way-up = the setting for which way up it's shown must be a number from 1 to 8
# Reason a photo's notes are refused: they are too long to fit in a JPEG.
folders-meta-refuse-too-long = the camera details are too long to fit in a JPEG
# Reason notes cannot be changed in a file that is not a JPEG or PNG photo.
folders-meta-only-jpeg-png = camera details can only be changed in JPEG and PNG photos
# Reason a change to a song or photo was not begun: the drive has no room for the copy.
folders-meta-no-room = there's no room left on the drive
# Reason a song's tag is refused: the tag is too big.
folders-meta-tag-too-big = the song's details are too big
