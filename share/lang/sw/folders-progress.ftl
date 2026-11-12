# machine translated (Swahili), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] faili { $done } kati ya { $of }, { $rate } kwa sekunde, { $left }
       *[no] { $rate } kwa sekunde, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] faili { $done } kati ya { $of }, inakadiria itachukua muda gani…
       *[no] inakadiria itachukua muda gani…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = baiti { $n }

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = KB { $n }

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = MB { $n }

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = GB { $n }

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = zimebaki sekunde chache

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] imebaki takriban sekunde 1
       *[other] zimebaki takriban sekunde { $n }
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = imebaki takriban dakika moja

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] imebaki takriban dakika 1
       *[other] zimebaki takriban dakika { $n }
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] imebaki takriban saa 1
       *[other] zimebaki takriban saa { $n }
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] Kazi nyingine 1 inaendelea
       *[other] Kazi nyingine { $n } zinaendelea
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Kuhamisha kunaifanya diski iwe na shughuli nyingi, kwa hiyo programu nyingine zinaweza kuwa polepole hadi kazi iishe.
       *[copying] Kunakili kunaifanya diski iwe na shughuli nyingi, kwa hiyo programu nyingine zinaweza kuwa polepole hadi kazi iishe.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = inasubiri jibu

# End of a piece of work's line while it is paused.
folders-progress-paused = imesimamishwa kwa muda

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Endelea

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Simamisha kwa muda

# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = Acha

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = mpya zaidi na kubwa zaidi

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = mpya zaidi

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = kubwa zaidi

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = ilibadilishwa { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] Folda iitwayo { $name } ipo tayari
        [file] Faili iitwayo { $name } ipo tayari
       *[unknown] { $name } ipo tayari
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = Mpya

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = Iliyopo tayari

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Ruka

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = Weka zote mbili

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = Badilisha

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = Badilisha, na uhamishe folda iliyopo tayari hadi Tupioni

# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = Unganisha

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = Tumia jibu hili kwa kila folda iliyopo tayari.

# Line over the switch in the name taken question for files.
folders-progress-all-files = Tumia jibu hili kwa kila faili iliyopo tayari.

# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = Fanya vivyo hivyo kwa zilizobaki
