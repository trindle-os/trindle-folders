
# machine translated (Urdu), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $of } میں سے { $done } فائلیں، { $rate } فی سیکنڈ، { $left }
       *[no] { $rate } فی سیکنڈ، { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $of } میں سے { $done } فائلیں، وقت کا اندازہ لگایا جا رہا ہے…
       *[no] وقت کا اندازہ لگایا جا رہا ہے…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }۔ { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } بائٹ

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = چند سیکنڈ باقی

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] تقریباً 1 سیکنڈ باقی
       *[other] تقریباً { $n } سیکنڈ باقی
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = تقریباً ایک منٹ باقی

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] تقریباً 1 منٹ باقی
       *[other] تقریباً { $n } منٹ باقی
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] تقریباً 1 گھنٹہ باقی
       *[other] تقریباً { $n } گھنٹے باقی
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] 1 اور کام جاری ہے
       *[other] { $n } اور کام جاری ہیں
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] منتقلی کے دوران ڈسک مصروف رہتی ہے، اس لیے مکمل ہونے تک دوسرے پروگرام سست ہو سکتے ہیں۔
       *[copying] نقل کے دوران ڈسک مصروف رہتی ہے، اس لیے مکمل ہونے تک دوسرے پروگرام سست ہو سکتے ہیں۔
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = جواب کا انتظار ہے

# End of a piece of work's line while it is paused.
folders-progress-paused = موقوف

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = جاری رکھیں

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = موقوف کریں

# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = روکیں

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = نئی اور بڑی

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = نئی

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = بڑی

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = بدلی گئی { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] { $name } نام کا فولڈر پہلے سے موجود ہے
        [file] { $name } نام کی فائل پہلے سے موجود ہے
       *[unknown] { $name } پہلے سے موجود ہے
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = نئی والی

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = جو پہلے سے موجود ہے

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = چھوڑ دیں

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = دونوں رکھیں

# SAFETY: needs a human review before it ships
# Word on the button in the name taken question that replaces the one already there.
folders-progress-replace = بدل دیں

# SAFETY: needs a human review before it ships
# Screen reader name of the Replace button for a folder in the name taken question.
folders-progress-replace-folder-spoken = بدل دیں، اور پہلے سے موجود فولڈر کو ردی دان میں منتقل کریں

# SAFETY: needs a human review before it ships
# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = ضم کریں

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = پہلے سے موجود ہر فولڈر کے لیے یہی جواب استعمال کریں۔

# Line over the switch in the name taken question for files.
folders-progress-all-files = پہلے سے موجود ہر فائل کے لیے یہی جواب استعمال کریں۔

# SAFETY: needs a human review before it ships
# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = باقی کے لیے بھی یہی کریں
