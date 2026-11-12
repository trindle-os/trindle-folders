
# machine translated (Arabic), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done } من { $of } ملفاً، { $rate } في الثانية، { $left }
       *[no] { $rate } في الثانية، { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } من { $of } ملفاً، جارٍ تقدير المدة…
       *[no] جارٍ تقدير المدة…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } بايت

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } كيلوبايت

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } ميغابايت

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } غيغابايت

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = بقيت ثوانٍ قليلة

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [zero] لم يبقَ شيء
        [one] بقيت نحو ثانية واحدة
        [two] بقيت نحو ثانيتين
        [few] بقيت نحو { $n } ثوانٍ
        [many] بقيت نحو { $n } ثانية
       *[other] بقيت نحو { $n } ثانية
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = بقيت نحو دقيقة

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [zero] لم يبقَ شيء
        [one] بقيت نحو دقيقة واحدة
        [two] بقيت نحو دقيقتين
        [few] بقيت نحو { $n } دقائق
        [many] بقيت نحو { $n } دقيقة
       *[other] بقيت نحو { $n } دقيقة
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [zero] لم يبقَ شيء
        [one] بقيت نحو ساعة واحدة
        [two] بقيت نحو ساعتين
        [few] بقيت نحو { $n } ساعات
        [many] بقيت نحو { $n } ساعة
       *[other] بقيت نحو { $n } ساعة
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [zero] لا توجد أعمال أخرى جارية
        [one] عمل آخر واحد جارٍ
        [two] عملان آخران جاريان
        [few] { $n } أعمال أخرى جارية
        [many] { $n } عملاً آخر جارٍ
       *[other] { $n } عمل آخر جارٍ
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
# SAFETY: needs a human review before it ships
folders-progress-heavy =
    { $verb ->
        [moving] النقل يُبقي القرص مشغولاً، لذلك قد تكون البرامج الأخرى أبطأ حتى ينتهي.
       *[copying] النسخ يُبقي القرص مشغولاً، لذلك قد تكون البرامج الأخرى أبطأ حتى ينتهي.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = بانتظار جواب

# End of a piece of work's line while it is paused.
folders-progress-paused = متوقف مؤقتاً

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = استئناف

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = إيقاف مؤقت

# Tip of the button on a piece of work's row that stops it.
# SAFETY: needs a human review before it ships
folders-progress-stop = إيقاف

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = أحدث وأكبر

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = أحدث

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = أكبر

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = عُدّل { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
# SAFETY: needs a human review before it ships
folders-progress-taken =
    { $kind ->
        [folder] يوجد هناك مجلد باسم { $name } بالفعل
        [file] يوجد هناك ملف باسم { $name } بالفعل
       *[unknown] { $name } موجود هناك بالفعل
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = الجديد

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = الموجود بالفعل

# Tip of the button in the name taken question that skips this one.
# SAFETY: needs a human review before it ships
folders-progress-skip = تخطّي

# Tip of the button in the name taken question that keeps both.
# SAFETY: needs a human review before it ships
folders-progress-keep-both = إبقاء الاثنين

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = استبدال

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = استبدال، ونقل المجلد الموجود بالفعل إلى سلة المحذوفات

# Main button of the name taken question for a folder: join the two folders.
# SAFETY: needs a human review before it ships
folders-progress-merge = دمج

# Line over the switch in the name taken question for folders.
# SAFETY: needs a human review before it ships
folders-progress-all-folders = استخدم هذا الجواب لكل مجلد موجود بالفعل.

# Line over the switch in the name taken question for files.
# SAFETY: needs a human review before it ships
folders-progress-all-files = استخدم هذا الجواب لكل ملف موجود بالفعل.

# Name and tip of the switch in the name taken question that uses the answer for the rest.
# SAFETY: needs a human review before it ships
folders-progress-same-for-rest = افعل الشيء نفسه مع الباقي

