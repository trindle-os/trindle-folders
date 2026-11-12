
# machine translated (Arabic), not yet reviewed by a person.

# Choice in the Wipe window: write over each file one time.
folders-tools-pass-once = مرة واحدة

# Choice in the Wipe window: write over each file three times.
folders-tools-pass-three = 3 مرات

# Choice in the Wipe window: write over each file seven times.
folders-tools-pass-seven = 7 مرات

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = كما هي

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = كل الأحرف صغيرة

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = كلها كبيرة

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = كبّر أول حرف من كل كلمة

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = كبّر أول حرف من الكلمة الأولى

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = كما هو

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = كل الأحرف صغيرة

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = كلها كبيرة

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = امتداد جديد

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = بلا امتداد

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = أرشيف

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = لن يبقى اسم

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = هذا غير مسموح به كاسم

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = هذا الاسم طويل جدًا

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = عنصران سيأخذان هذا الاسم

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = هناك شيء آخر يحمل هذا الاسم بالفعل

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = في الاسم القديم أحرف لا يستطيع Folders قراءتها

# SAFETY: needs a human review before it ships
# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
folders-tools-caveat-flash = هذا محرك فلاش أو SSD، لذلك قد تبقى نسخ قديمة في أجزاء منه لا يصل إليها Folders.

# SAFETY: needs a human review before it ships
# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
folders-tools-caveat-copy-on-write = يحفظ نظام الملفات { $fs } التغييرات في مكان جديد، لذلك قد تبقى البيانات القديمة موجودة.

# SAFETY: needs a human review before it ships
# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
folders-tools-caveat-network = هذه مشاركة { $fs } على حاسوب آخر، وقد يحتفظ بنسخ خاصة به.

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = على اليسار فقط

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = على اليمين فقط

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = متطابق

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
folders-tools-row-different =
    مختلف: { $why ->
        [kind] أحدهما مجلد أو رابط
        [size] الأحجام مختلفة
        [date] التواريخ مختلفة
        [content] المحتويات مختلفة
        [target] الروابط تشير إلى مكانين مختلفين
        [inside] شيء في الداخل مختلف
       *[unread] تعذّرت قراءة أحد الجانبين
    }{ $newer ->
        [left] ، الأحدث على اليسار
        [right] ، الأحدث على اليمين
       *[none] { "" }
    }

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = هنا

# SAFETY: needs a human review before it ships
# Red notice when Wipe is chosen on things that are not files.
folders-tools-only-files-wiped = يمكن مسح الملفات فقط.

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = افتح مجلدًا في كل لوح لمقارنتهما.

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
        [zero] تراجع عن إعادة تسمية لا شيء
        [one] تراجع عن إعادة تسمية عنصر واحد
        [two] تراجع عن إعادة تسمية عنصرين
        [few] تراجع عن إعادة تسمية { $n } عناصر
        [many] تراجع عن إعادة تسمية { $n } عنصرًا
       *[other] تراجع عن إعادة تسمية { $n } عنصر
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [zero] لم تُعد تسمية أي عنصر.
        [one] أُعيدت تسمية عنصر واحد.
        [two] أُعيدت تسمية عنصرين.
        [few] أُعيدت تسمية { $n } عناصر.
        [many] أُعيدت تسمية { $n } عنصرًا.
       *[other] أُعيدت تسمية { $n } عنصر.
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = توقفت إعادة التسمية في منتصفها: { $error }.

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = اكتب اسمًا للأرشيف.

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = جرى فحص { $done }

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = قُرئ { $done } من { $total }

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = جارٍ سرد الملفات

# Beside the ring while two files are compared.
folders-tools-comparing = جارٍ المقارنة

# Beside the ring while two folders are compared.
folders-tools-looking = جارٍ مقارنة المجلدات

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [zero] ضغط لا شيء
        [one] ضغط { $name }
        [two] ضغط عنصرين
        [few] ضغط { $n } عناصر
        [many] ضغط { $n } عنصرًا
       *[other] ضغط { $n } عنصر
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = اسم الأرشيف

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = نوع الأرشيف

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = سيُحفظ الأرشيف في { $place }

# Main button of the Compress window.
folders-tools-compress = ضغط

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = استخراج { $name } إلى

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = ستوضع الملفات في مجلد جديد باسم { $name }.

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [zero] تغيير حجم لا شيء
        [one] تغيير حجم { $name }
        [two] تغيير حجم صورتين
        [few] تغيير حجم { $n } صور
        [many] تغيير حجم { $n } صورة
       *[other] تغيير حجم { $n } صورة
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = الحجم الجديد

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = الحجم هو طول الضلع الأطول بالبكسل. يصغّر Folders الصور فقط. ويحفظ كل نسخة معدّلة الحجم بجانب الأصل، ويضع الحجم الجديد في اسمها.

# SAFETY: needs a human review before it ships
# Main button of the Resize window.
folders-tools-resize = تغيير الحجم

# SAFETY: needs a human review before it ships
# Title of the Wipe window; { $name } is the one file, { $n } how many.
folders-tools-wipe-title =
    { $n ->
        [zero] مسح لا شيء؟
        [one] مسح { $name }؟
        [two] مسح ملفين؟
        [few] مسح { $n } ملفات؟
        [many] مسح { $n } ملفًا؟
       *[other] مسح { $n } ملف؟
    }

# SAFETY: needs a human review before it ships
# Warning at the top of the Wipe window.
folders-tools-wipe-warning = يكتب Folders فوق كل ملف قبل حذفه، فلا يمكنك استعادته.

# SAFETY: needs a human review before it ships
# Name beside the choice of how many times to write over, in the Wipe window.
folders-tools-wipe-write-over = الكتابة فوقه

# SAFETY: needs a human review before it ships
# Choice tip in the Wipe window: how many times each file is written over.
folders-tools-wipe-passes-tip = عدد مرات الكتابة فوق كل ملف

# SAFETY: needs a human review before it ships
# Red main button of the Wipe window.
folders-tools-wipe-button = مسح آمن

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = ترقيمها

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = في البداية

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = في النهاية

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = مكان الرقم

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = الرقم الأول

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = ما يُفعل بالامتداد

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = امتداد جديد، مثل txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = لا يستطيع Folders إعادة تسمية هذا: { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = كان { $old }

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = يبقى كما هو

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [zero] إعادة تسمية لا شيء
        [one] إعادة تسمية عنصر واحد
        [two] إعادة تسمية عنصرين
        [few] إعادة تسمية { $n } عناصر
        [many] إعادة تسمية { $n } عنصرًا
       *[other] إعادة تسمية { $n } عنصر
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = استبدال

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = بحث عن

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = استبدال بـ

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = استخدام * و? كحروف بدل

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = حروف البدل * و?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = مطابقة الحروف الكبيرة تمامًا

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = مطابقة حالة الأحرف

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = إضافة

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = قبل الاسم

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = بعد الاسم

# Name of the numbering row in the rename window.
folders-tools-rename-number = ترقيم

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = الأحرف الكبيرة

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = الأحرف الكبيرة في الأسماء

# Name over the ending choice in the rename window.
folders-tools-rename-ending = الامتداد

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] في { $place }، لا يمكن استخدام بعض الأسماء. انظر الصفوف المعلّمة.
       *[no] في { $place }، سيتغير { $changes } من { $total } من الأسماء.
    }

# Main button of the rename window.
folders-tools-rename-button = إعادة التسمية

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = المجلد الرئيسي

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [zero] المكررات في لا مجلد
        [one] المكررات في { $folder }
        [two] المكررات في مجلدين
        [few] المكررات في { $n } مجلدات
        [many] المكررات في { $n } مجلدًا
       *[other] المكررات في { $n } مجلد
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = توقف البحث: { $error }.

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = لم يُعثر على مكررات.

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [zero] قُرئت كل الملفات.
        [one] تعذّرت قراءة ملف واحد.
        [two] تعذّرت قراءة ملفين.
        [few] تعذّرت قراءة { $n } ملفات.
        [many] تعذّرت قراءة { $n } ملفًا.
       *[other] تعذّرت قراءة { $n } ملف.
    }

# SAFETY: needs a human review before it ships
# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
        [zero] لا مجموعات نسخ.
        [one] مجموعة نسخ واحدة. النسخ الزائدة محددة، ونقلها إلى سلة المحذوفات يوفّر { $size }.
        [two] مجموعتا نسخ. النسخ الزائدة محددة، ونقلها إلى سلة المحذوفات يوفّر { $size }.
        [few] { $n } مجموعات نسخ. النسخ الزائدة محددة، ونقلها إلى سلة المحذوفات يوفّر { $size }.
        [many] { $n } مجموعة نسخ. النسخ الزائدة محددة، ونقلها إلى سلة المحذوفات يوفّر { $size }.
       *[other] { $n } مجموعة نسخ. النسخ الزائدة محددة، ونقلها إلى سلة المحذوفات يوفّر { $size }.
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = المجموعة { $set }، { $size }، في { $at }

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [zero] قُرئت كل الملفات.
        [one] تعذّرت قراءة ملف واحد فاستُبعد.
        [two] تعذّرت قراءة ملفين فاستُبعدا.
        [few] تعذّرت قراءة { $n } ملفات فاستُبعدت.
        [many] تعذّرت قراءة { $n } ملفًا فاستُبعدت.
       *[other] تعذّرت قراءة { $n } ملف فاستُبعدت.
    }

# SAFETY: needs a human review before it ships
# Note in the duplicates window when every copy of a set is picked.
folders-tools-dupes-keep-one = كل النسخ في مجموعة محددة. اترك واحدة على الأقل غير محددة.

# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = تحديد النسخ الزائدة

# SAFETY: needs a human review before it ships
# Main button of the duplicates window: move the picked files to the Trash.
folders-tools-dupes-trash = نقل إلى سلة المهملات

# Title of the window that compares two files.
folders-tools-diff-title = مقارنة ملفين

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = تعذّر على Folders المقارنة بينهما: { $error }.

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = الملفان متطابقان تمامًا.

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = هما متطابقان حتى نهاية الأقصر. أحدهما { $one } والآخر { $other }.

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = يختلفان ابتداءً من البايت { $n }.

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] الملفان متطابقان.
        [one] يختلف سطر واحد.
        [two] يختلف سطران.
        [few] تختلف { $n } أسطر.
        [many] يختلف { $n } سطرًا.
       *[other] يختلف { $n } سطر.
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = هما مختلفان جدًا لمقارنتهما سطرًا بسطر. يظهر كل ما بين أول اختلاف وآخره على أنه تغيّر.

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = تُقارن فقط أول 4 MB من كل ملف.

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = مقارنة { $left } بـ { $right }

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = مقارنة المحتويات (أبطأ)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = مقارنة ما بداخل كل ملف

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = إظهار الأسماء المتطابقة

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = إظهار الأسماء المتطابقة أيضًا

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = توقفت المقارنة: { $error }.

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = { $left } على اليسار فقط، و{ $right } على اليمين فقط، و{ $different } مختلف، و{ $same } متطابق.

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = يحتوي المجلدان على المحتويات نفسها.

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = قارن مرة أخرى

# SAFETY: needs a human review before it ships
# Button in the pane compare window; { $n } is how many names are copied to the left folder.
folders-tools-compare-copy-left = نسخ { $n } من العناصر المفقودة أو الأحدث إلى اليسار

# SAFETY: needs a human review before it ships
# Button in the pane compare window; { $n } is how many names are copied to the right folder.
folders-tools-compare-copy-right = نسخ { $n } من العناصر المفقودة أو الأحدث إلى اليمين

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = تُمسح الملفات فقط، لذلك ستبقى هذه المجلدات كما هي: { $names }.
