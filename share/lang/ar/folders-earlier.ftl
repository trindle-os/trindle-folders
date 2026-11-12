
# machine translated (Arabic), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = لا يمكنك تغيير أي شيء في نسخة سابقة. انسخ منها ما تريد الاحتفاظ به.

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] اليوم في { $time }
        [yesterday] أمس في { $time }
       *[other] { $date } في { $time }
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] اليوم في { $time }
        [yesterday] أمس في { $time }
       *[other] { $date } في { $time }
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = هذه نسخة سابقة من { $when }. لا يمكنك تغييرها، لكن يمكنك نسخ ما تريد منها.

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = هذه نسخة سابقة. لا يمكنك تغييرها، لكن يمكنك نسخ ما تريد منها.

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = سابق: { $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = التراجع عن إعادة { $name }

# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = أعاده Folders. والنسخة الحالية بجانبه باسم جديد.

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = أعاده Folders إلى مكانه.

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = هو مطابق للنسخة الحالية.

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = هو مختلف عن النسخة الحالية.

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = تعذّر على Folders المقارنة بينهما: { $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = الإصدارات السابقة

# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = ما حُذف من هنا

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = الإصدارات السابقة من { $name }

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = لا توجد نسخ سابقة تختلف عن النسخة الحالية.

# Note at the foot of the earlier versions card about what putting one back does.
folders-earlier-versions-foot = إعادة إحداها تُبقي النسخة الحالية بجانبها. ويمكنك التراجع عن ذلك.

# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = ما حُذف من { $name }

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = لم يُحذف من هذا المجلد شيء من النسخ السابقة.

# Note at the foot of the deleted things card about what putting one back does.
folders-earlier-deleted-foot = إعادة أحدها تعيده إلى هذا المجلد. ويمكنك التراجع عن ذلك.

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = المجلد كما كان حينها

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }، من النسخة المأخوذة في { $date }

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = مجلد، شوهد آخر مرة في { $when }

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }، شوهد آخر مرة في { $when }

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = جارٍ العمل…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = افتح مجلد ذلك اليوم في تبويب جديد

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = افتح

# Tip and name of the round button on a row of the earlier versions card that puts it back.
folders-earlier-put-back-tip = أعده

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = قارن بالنسخة الحالية

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = هذه ليست نسخة سابقة.

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = لا يمكنك إعادة نسخة سابقة إلا داخل مجلدك الرئيسي.

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = لا يستطيع Folders إعادة ملف إلى هناك.

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = تعذّر على Folders إعادة إنشاء المجلد الذي كان فيه: { $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = تعذّر على Folders نسخ النسخة السابقة: { $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = تعذّر على Folders إزاحة النسخة الحالية جانباً: { $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = تعذّر على Folders إعادة النسخة السابقة: { $why }. صار اسم النسخة الحالية { $name }.

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = تعذّر على Folders إعادة النسخة السابقة: { $why }
