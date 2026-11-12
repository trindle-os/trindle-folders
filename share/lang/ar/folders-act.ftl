
# machine translated (Arabic), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = لا يعمل القص والنسخ واللصق مع ملفات الخادم. استخدم زرَّي النسخ والنقل بين اللوحين بدلاً من ذلك.

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = لا يمكنك تغيير أي شيء داخل الأرشيف. استخرج ما تحتاج إليه أولاً.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = قُصَّت هذه العناصر في تطبيق آخر، لذلك نسخها Folders بدلاً من نقلها.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = يجري نسخ هذا أو نقله بالفعل.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = لا يزال هذا قيد النسخ إلى هنا.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10,000 عنصر أو أكثر

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [zero] لا عناصر
        [one] عنصر واحد
        [two] عنصران
        [few] { $n } عناصر
        [many] { $n } عنصراً
       *[other] { $n } عنصر
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [zero] لم تفشل أي عملية من نوع { $verb }: { $names }.
        [one] فشل { $verb } لعنصر واحد: { $names }.
        [two] فشل { $verb } لعنصرين: { $names }.
        [few] فشل { $verb } لـ{ $n } عناصر: { $names }.
        [many] فشل { $verb } لـ{ $n } عنصراً: { $names }.
       *[other] فشل { $verb } لـ{ $n } عنصر: { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } و{ $more } أخرى

# Words on a job card while things are copied: what, and where to.
folders-act-copying = نسخ { $what } إلى { $place }

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = نُسخ { $what } إلى { $place }.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = النسخ

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = التراجع عن نسخ { $what }

# Words on a job card while things are moved: what, and where to.
folders-act-moving = نقل { $what } إلى { $place }

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = نُقل { $what } إلى { $place }.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = النقل

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = التراجع عن نقل { $what }

# Words on a job card while things are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-trashing = وضع { $what } في سلة المحذوفات

# Words under the file lines when things have been put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-trashed = وُضع { $what } في سلة المحذوفات.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
# SAFETY: needs a human review before it ships
folders-act-verb-trashed = النقل إلى سلة المحذوفات

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = التراجع عن وضع { $what } في سلة المحذوفات

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = حذف { $what }

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = حُذف { $what } نهائياً.

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = الحذف

# Words on a job card while things are put back from the Trash.
folders-act-restoring = إعادة { $what }

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = أُعيد { $what }.

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = التراجع عن إعادة { $what }

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = الإعادة

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = جارٍ التراجع

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = تم التراجع.

# Words on a job card while things are packed into an archive.
folders-act-compressing = ضغط { $what }

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = أُنشئ { $file } في { $place }.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = الضغط

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = التراجع عن ضغط { $what }

# Words on a job card while an archive is unpacked.
folders-act-extracting = استخراج { $what }

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = استُخرج { $what } إلى { $place }.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = الاستخراج

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = التراجع عن استخراج { $what }

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = مسح { $what } مسحاً آمناً

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = مُسح { $what } مسحاً آمناً.

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = المسح الآمن

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = إخراج { $what }

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = أُخرج { $what } من الأرشيف إلى { $place }.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = الإخراج

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = التراجع عن إخراج { $what }

# Words on a job card while pictures are turned.
folders-act-turning = تدوير { $what }

# Words under the file lines when pictures have been turned.
folders-act-turned = دُوِّر { $what }.

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = التدوير

# Words on a job card while pictures are resized.
folders-act-resizing = تغيير حجم { $what }

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = أُنشئت نسخة بحجم مختلف من { $what }.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = تغيير الحجم

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = التراجع عن تغيير حجم { $what }

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = مجلد جديد

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = مستند جديد

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = تعذّر على Folders إنشاء { $base }: { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = اكتب اسماً.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = لا يمكن أن يحتوي الاسم على شرطة مائلة.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = هذا الاسم يستخدمه النظام. اختر اسماً آخر.

# Red words under a name being typed when it is too long.
folders-act-name-long = هذا الاسم طويل جداً.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = تعذّر على Folders بدء هذا العمل.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = إعادة { $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = التراجع عن { $what }

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = أُوقف العمل قبل أن ينتهي.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [zero] لم يتخطَّ Folders أي عنصر.
        [one] تخطى Folders عنصراً واحداً.
        [two] تخطى Folders عنصرين.
        [few] تخطى Folders { $n } عناصر.
        [many] تخطى Folders { $n } عنصراً.
       *[other] تخطى Folders { $n } عنصر.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = حدّد شيئاً أولاً.

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = هذا موجود في سلة المحذوفات بالفعل.

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = لا يمكن وضع الملفات الموجودة على خادم في سلة المحذوفات. استخدم الحذف النهائي بدلاً من ذلك.

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = حدّد شيئاً لنسخه أو نقله أولاً.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = لا يمكنك إخراج الأشياء من الأرشيف إلا إلى مجلد.

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = لا يمكنك نسخ الأشياء إلى سلة المحذوفات.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = اللوح الآخر لا يعرض مجلداً.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = حدّد شيئاً لنسخه أولاً.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = قُصَّ { $what }.

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = نُسخ { $what }.

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = لا يمكنك اللصق إلا في مجلد.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = لا توجد ملفات في الحافظة.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = لم يُقصّ أو يُنسخ شيء.

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = لا يمكنك حذف الأماكن الشبكية من هنا.

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = سلة المحذوفات فارغة بالفعل.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = حدّد شيئاً في سلة المحذوفات أولاً.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = إنشاء { $name }

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = لا يمكنك إنشاء مجلد أو مستند جديد إلا داخل مجلد.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = التراجع عن إنشاء { $name }

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = تعذّر على Folders الاحتفاظ بألوان المجلدات على العناصر التي نقلها.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = لا يوجد ما يمكن إعادته.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = لا يوجد ما يمكن التراجع عنه.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = إعادة { $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = أُعيد { $what }.

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = الإعادة

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = التراجع عن { $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = تم التراجع عن { $what }.

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = المجلد الرئيسي

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = سلة المحذوفات

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = لا يمكنك إفلات الأشياء إلا على مجلد أو على سلة المحذوفات.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [zero] لا روابط
        [one] رابط واحد
        [two] رابطان
        [few] { $n } روابط
        [many] { $n } رابطاً
       *[other] { $n } رابط
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = أُنشئ { $what }.

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [zero] لم يفشل إنشاء أي رابط: { $names }.
        [one] تعذّر إنشاء رابط واحد: { $names }.
        [two] تعذّر إنشاء رابطين: { $names }.
        [few] تعذّر إنشاء { $n } روابط: { $names }.
        [many] تعذّر إنشاء { $n } رابطاً: { $names }.
       *[other] تعذّر إنشاء { $n } رابط: { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = هناك سؤال ينتظر إجابتك. أجب عنه، أو أوقف العمل، قبل أن تغلق Folders.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = لا يمكن لـ Folders أن يُغلق أثناء تشغيل عمل. انتظر حتى ينتهي أو أوقفه، ثم أغلق Folders.
