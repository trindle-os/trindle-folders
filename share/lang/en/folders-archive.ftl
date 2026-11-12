# Folders' reading and making of archives (zip, 7z and more): the reasons given when one will not open, unpack or be made (src/files/archive.rs).

## Why an archive would not open or unpack (src/files/archive.rs)

# Reason an archive would not open: it is locked with a password and none was given.
folders-archive-trouble-locked = it is locked with a password
# Reason an archive would not open: the password given is not the one.
folders-archive-trouble-wrong-key = that password does not open it
# Reason an archive would not open: what came out with the password makes no sense, so the password is wrong or the archive is damaged.
folders-archive-trouble-wrong-or-bad = that password does not open it, or the archive is damaged
# Reason an archive would not open: it is a RAR locked with a password.
folders-archive-trouble-locked-rar = it's a RAR file with a password Folders can't unlock, but the file itself isn't damaged
# Reason an archive would not open: it is locked and packed a way Folders does not know.
folders-archive-trouble-locked-foreign = it uses a kind of password protection Folders can't unlock
# Reason a file would not open as an archive: it is not a kind Folders reads.
folders-archive-trouble-unknown = it isn't a kind of archive Folders can open
# Reason an archive would not unpack: it holds too many things.
folders-archive-trouble-too-many = it has more than a million items, which is too many for Folders to unpack
# Reason an archive would not unpack: the place it was to go has too little room.
folders-archive-trouble-no-room = there isn't enough space here for what's in it
# Reason an archive would not open: reading it takes more memory or time than is allowed.
folders-archive-trouble-gave-up = it needs more memory or time to open than Folders allows
# Reason an archive would not open: the program that reads this kind is not on the computer.
folders-archive-trouble-no-reader = the part of Trindle that opens this kind of archive is missing, so install updates to put it back
# Reason an archive would not open: it is packed a way Folders does not know.
folders-archive-trouble-foreign = it uses a kind of compression Folders can't read
# Reason a split archive would not open: one of its parts is not there.
folders-archive-trouble-parts = part of it is missing, and all its parts need to be in the same folder
# Reason given for an archive that cannot be read when the file is damaged or ends too soon.
folders-archive-why-damaged = it's damaged or incomplete
# Reason given for an archive that cannot be written when the drive is full.
folders-archive-why-no-room = there's no space left
# Reason given for an archive that cannot be unpacked when the folder cannot be written in.
folders-archive-why-read-only = you don't have permission, or the drive is read-only
# Reason given for an archive that cannot be read when the file has gone.
folders-archive-why-gone = it isn't there any more
# Reason given for an archive that cannot be read: it needs more memory than is allowed.
folders-archive-why-memory = it needs more memory to read than Folders allows
# Reason given for an archive that cannot be read when no better reason is known.
folders-archive-why-other = something unexpected went wrong
# Reason a file inside an archive cannot be opened: it is too big.
folders-archive-inside-too-big = it's too big to open from inside the archive
# Reason a file cannot be taken from an archive: it is not in it.
folders-archive-inside-missing = it isn't in the archive
# Reason a 7z archive would not unpack: it needs more memory than is allowed.
folders-archive-7z-memory = this 7z archive needs more memory to unpack than Folders allows
# Reason making an archive failed: a file grew while it was being packed.
folders-archive-pack-grew = a file grew too big while Folders was packing it
# Reason making an archive failed: something with the archive's name is already there.
folders-archive-pack-exists = there's already something with that name
# Reason making an archive failed: an item has no name to put in it.
folders-archive-pack-no-name = an item has no name Folders can put in the archive
# Reason making an archive failed: Folders reads this kind but cannot write it.
folders-archive-pack-cannot-make = Folders can open this kind of archive but can't make one

## Damaged 7z archives (src/files/archive.rs)

# Reason a 7z archive cannot be read; the cause follows, already written out.
folders-archive-damaged-7z = the 7z archive is damaged: { $why }
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a lock too slow to open.
folders-archive-7z-a-lock-too-slow-to-open = unlocking it would take far too long
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a lock with no settings.
folders-archive-7z-a-lock-with-no-settings = the settings for its password protection are missing
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a lock with short settings.
folders-archive-7z-a-lock-with-short-settings = the settings for its password protection are incomplete
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a lock with no key.
folders-archive-7z-a-lock-with-no-key = its password protection has no usable key
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a locked block is not whole.
folders-archive-7z-a-locked-block-is-not-whole = a locked part of it is incomplete
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: the header ends too soon.
folders-archive-7z-the-header-ends-too-soon = its list of contents ends too soon
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a count is far too big.
folders-archive-7z-a-count-is-far-too-big = a number in it is far too big
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a block has no packed bytes.
folders-archive-7z-a-block-has-no-packed-bytes = a part of it has no packed data
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a block is out of reach.
folders-archive-7z-a-block-is-out-of-reach = a part of it points past the end of the file
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: too short.
folders-archive-7z-too-short = the file is too short
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: no 7z mark.
folders-archive-7z-no-7z-mark = it doesn't start like a 7z file
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: the start header's check does not match.
folders-archive-7z-the-start-header-s-check-does-not-match = the start of the file fails its check
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: the header is out of reach.
folders-archive-7z-the-header-is-out-of-reach = its list of contents points past the end of the file
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: the header's check does not match.
folders-archive-7z-the-header-s-check-does-not-match = its list of contents fails its check
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a packed header with no block.
folders-archive-7z-a-packed-header-with-no-block = its packed list of contents has no data
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: the header is far too big.
folders-archive-7z-the-header-is-far-too-big = its list of contents is far too big
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: the packed header's check does not match.
folders-archive-7z-the-packed-header-s-check-does-not-match = its packed list of contents fails its check
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: an unknown header.
folders-archive-7z-an-unknown-header = its list of contents is in a format Folders doesn't recognise
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: headers packed too many times over.
folders-archive-7z-headers-packed-too-many-times-over = its list of contents is packed too many times over
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: an unknown part in the header.
folders-archive-7z-an-unknown-part-in-the-header = there's something unknown in its list of contents
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: an unknown part in the streams.
folders-archive-7z-an-unknown-part-in-the-streams = there's something unknown in its data
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: an unknown part in the packed streams.
folders-archive-7z-an-unknown-part-in-the-packed-streams = there's something unknown in its packed data
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: packed streams with no sizes.
folders-archive-7z-packed-streams-with-no-sizes = the sizes of its packed data are missing
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: no list of blocks.
folders-archive-7z-no-list-of-blocks = the list of its parts is missing
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: blocks kept elsewhere.
folders-archive-7z-blocks-kept-elsewhere = the list of its parts is stored somewhere Folders can't read
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: blocks with no sizes.
folders-archive-7z-blocks-with-no-sizes = the sizes of its parts are missing
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: an unknown part in the blocks.
folders-archive-7z-an-unknown-part-in-the-blocks = there's something unknown in the list of its parts
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a block with no sensible coders.
folders-archive-7z-a-block-with-no-sensible-coders = a part of it has no sensible packing steps
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a coder with too many ends.
folders-archive-7z-a-coder-with-too-many-ends = one of its packing steps has too many inputs or outputs
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a block's coders do not join up.
folders-archive-7z-a-block-s-coders-do-not-join-up = the packing steps for a part of it don't fit together
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: far too many files.
folders-archive-7z-far-too-many-files = it lists far too many files
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: files in a block with no sizes.
folders-archive-7z-files-in-a-block-with-no-sizes = some files in a part of it have no recorded size
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: files bigger than their block.
folders-archive-7z-files-bigger-than-their-block = some files are bigger than the part of it that holds them
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: an unknown part in the files' sizes.
folders-archive-7z-an-unknown-part-in-the-files-sizes = there's something unknown in its file sizes
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a part is far too big.
folders-archive-7z-a-part-is-far-too-big = a part of it is far too big
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: names kept elsewhere.
folders-archive-7z-names-kept-elsewhere = its file names are stored somewhere Folders can't read
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: too few names.
folders-archive-7z-too-few-names = some file names are missing
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: times kept elsewhere.
folders-archive-7z-times-kept-elsewhere = its file dates are stored somewhere Folders can't read
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: attributes kept elsewhere.
folders-archive-7z-attributes-kept-elsewhere = its file details are stored somewhere Folders can't read
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a block runs past the end.
folders-archive-7z-a-block-runs-past-the-end = a part of it runs past the end of the file
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a block's coders go round in a loop.
folders-archive-7z-a-block-s-coders-go-round-in-a-loop = the packing steps for a part of it go round in a loop
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: LZMA with no settings.
folders-archive-7z-lzma-with-no-settings = its LZMA packing has no settings
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: LZMA2 with no settings.
folders-archive-7z-lzma2-with-no-settings = its LZMA2 packing has no settings
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: LZMA2 with impossible settings.
folders-archive-7z-lzma2-with-impossible-settings = its LZMA2 packing has invalid settings
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a block would not open.
folders-archive-7z-a-block-would-not-open = Folders couldn't open a part of it
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a block ends too soon.
folders-archive-7z-a-block-ends-too-soon = a part of it ends too soon
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a file ends too soon.
folders-archive-7z-a-file-ends-too-soon = a file in it ends too soon
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: a file's check does not match.
folders-archive-7z-a-file-s-check-does-not-match = a file in it fails its check
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: files' sizes and checks do not match up.
folders-archive-7z-files-sizes-and-checks-do-not-match-up = its file sizes and checks don't match up
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: more files than blocks hold.
folders-archive-7z-more-files-than-blocks-hold = it lists more files than its parts hold
# Cause given after "the 7z archive is damaged:" when a 7z archive cannot be read: more files than sizes.
folders-archive-7z-more-files-than-sizes = some of its files have no size recorded

## Damaged zip archives (src/files/archive.rs)

# Reason a zip archive cannot be read; the cause follows, already written out.
folders-archive-damaged-zip = the zip archive is damaged: { $why }
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: no end record.
folders-archive-zip-no-end-record = the record at the end of it is missing
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: no ZIP64 end record.
folders-archive-zip-no-zip64-end-record = the ZIP64 record at the end of it is missing
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: directory out of place.
folders-archive-zip-directory-out-of-place = its list of contents is in the wrong place
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: directory too big.
folders-archive-zip-directory-too-big = its list of contents is too big
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: record runs past the directory.
folders-archive-zip-record-runs-past-the-directory = an entry runs past the end of its list of contents
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: no local header.
folders-archive-zip-no-local-header = the details in front of a file in it are missing
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: entry runs past the end.
folders-archive-zip-entry-runs-past-the-end = a file in it runs past the end of the archive
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: an entry's check does not match.
folders-archive-zip-an-entry-s-check-does-not-match = one of its files fails its check
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: an entry is longer than it says.
folders-archive-zip-an-entry-is-longer-than-it-says = a file in it is longer than its recorded size
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: a locked entry is cut short.
folders-archive-zip-a-locked-entry-is-cut-short = a password-protected file in it is incomplete
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: a lock with a key of no known length.
folders-archive-zip-a-lock-with-a-key-of-no-known-length = its password protection uses a key length Folders doesn't recognise
# Cause given after "the zip archive is damaged:" when a zip archive cannot be read: a locked entry's check does not match.
folders-archive-zip-a-locked-entry-s-check-does-not-match = a password-protected file in it fails its check
