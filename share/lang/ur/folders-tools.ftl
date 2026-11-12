
# machine translated (Urdu), not yet reviewed by a person.

# Choice in the Wipe window: write over each file one time.
# SAFETY: needs a human review before it ships
folders-tools-pass-once = ایک بار

# Choice in the Wipe window: write over each file three times.
# SAFETY: needs a human review before it ships
folders-tools-pass-three = 3 بار

# Choice in the Wipe window: write over each file seven times.
# SAFETY: needs a human review before it ships
folders-tools-pass-seven = 7 بار

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = جیسے ہیں

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = سب چھوٹے حروف

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = سب بڑے

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = ہر لفظ بڑے حرف سے شروع کریں

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = پہلا لفظ بڑے حرف سے شروع کریں

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = جیسا ہے

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = سب چھوٹے حروف

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = سب بڑے

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = نیا اختتام

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = کوئی اختتام نہیں

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = آرکائیو

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = کوئی نام نہیں بچے گا

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = یہ نام کے طور پر جائز نہیں

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = یہ نام بہت لمبا ہے

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = دو چیزوں کو یہی نام ملے گا

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = کسی اور کا پہلے سے یہی نام ہے

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = پرانے نام میں ایسے حروف ہیں جنہیں Folders نہیں پڑھ سکتا

# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
# SAFETY: needs a human review before it ships
folders-tools-caveat-flash = یہ فلیش ڈرائیو یا SSD ہے، اس لیے پرانی نقول اس کے ان حصوں میں رہ سکتی ہیں جن تک Folders نہیں پہنچ سکتا۔

# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
# SAFETY: needs a human review before it ships
folders-tools-caveat-copy-on-write = { $fs } فائل سسٹم تبدیلیاں نئی جگہ پر محفوظ کرتا ہے، اس لیے پرانا ڈیٹا شاید وہیں موجود ہو۔

# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
# SAFETY: needs a human review before it ships
folders-tools-caveat-network = یہ دوسرے کمپیوٹر پر { $fs } شیئر ہے، جو اپنی نقول رکھ سکتا ہے۔

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = صرف بائیں طرف

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = صرف دائیں طرف

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = ایک جیسا

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
folders-tools-row-different =
    فرق: { $why ->
        [kind] ایک فولڈر یا لنک ہے
        [size] سائز مختلف ہیں
        [date] تاریخیں مختلف ہیں
        [content] مواد مختلف ہے
        [target] لنک کہیں اور اشارہ کرتے ہیں
        [inside] اندر کوئی چیز مختلف ہے
       *[unread] ایک طرف پڑھی نہیں جا سکی
    }{ $newer ->
        [left] ، بائیں طرف نیا
        [right] ، دائیں طرف نیا
       *[none] { "" }
    }

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = یہاں

# Red notice when Wipe is chosen on things that are not files.
# SAFETY: needs a human review before it ships
folders-tools-only-files-wiped = صرف فائلیں مٹائی جا سکتی ہیں۔

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = موازنے کے لیے ہر پین میں ایک فولڈر کھولیں۔

# Undo entry after renaming many files; { $n } is how many were renamed.
# SAFETY: needs a human review before it ships
folders-tools-rename-undo =
    { $n ->
        [one] 1 شے کا نام بدلنا واپس کریں
       *[other] { $n } اشیا کے نام بدلنا واپس کریں
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [one] 1 شے کا نام بدل دیا گیا۔
       *[other] { $n } اشیا کے نام بدل دیے گئے۔
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = نام بدلنا درمیان میں رک گیا: { $error }۔

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = آرکائیو کے لیے نام لکھیں۔

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = { $done } جانچی گئیں

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = { $total } میں سے { $done } پڑھا گیا

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = فائلوں کی فہرست بن رہی ہے

# Beside the ring while two files are compared.
folders-tools-comparing = موازنہ ہو رہا ہے

# Beside the ring while two folders are compared.
folders-tools-looking = فولڈروں کا موازنہ ہو رہا ہے

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [one] { $name } کمپریس کریں
       *[other] { $n } اشیا کمپریس کریں
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = آرکائیو کا نام

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = آرکائیو کی قسم

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = آرکائیو { $place } میں محفوظ کیا جائے گا

# Main button of the Compress window.
folders-tools-compress = کمپریس کریں

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = { $name } کو یہاں نکالیں

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = فائلیں { $name } نام کے نئے فولڈر میں رکھی جائیں گی۔

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [one] { $name } کا سائز بدلیں
       *[other] { $n } تصویروں کا سائز بدلیں
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = نیا سائز

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = سائز لمبی طرف کی لمبائی ہے، پکسل میں۔ Folders تصویروں کو صرف چھوٹا کرتا ہے۔ یہ ہر تبدیل شدہ سائز والی کاپی اصل کے ساتھ محفوظ کرتا ہے، اور نام میں نیا سائز ہوتا ہے۔

# Main button of the Resize window.
folders-tools-resize = سائز بدلیں

# Title of the Wipe window; { $name } is the one file, { $n } how many.
# SAFETY: needs a human review before it ships
folders-tools-wipe-title =
    { $n ->
        [one] کیا { $name } کو محفوظ طریقے سے مٹائیں؟
       *[other] کیا { $n } فائلیں محفوظ طریقے سے مٹائیں؟
    }

# Warning at the top of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-warning = Folders ہر فائل کو مٹانے سے پہلے اس پر اوپر سے لکھ دیتا ہے، اس لیے آپ اسے واپس نہیں لا سکتے۔

# Name beside the choice of how many times to write over, in the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-write-over = اوپر سے لکھیں

# Choice tip in the Wipe window: how many times each file is written over.
# SAFETY: needs a human review before it ships
folders-tools-wipe-passes-tip = ہر فائل پر کتنی بار لکھنا ہے

# Red main button of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-button = محفوظ طریقے سے مٹائیں

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = انہیں نمبر دیں

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = شروع میں

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = آخر میں

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = نمبر کہاں جائے

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = پہلا نمبر

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = اختتام کے ساتھ کیا کرنا ہے

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = نیا اختتام، جیسے txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders اس کا نام نہیں بدل سکتا: { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = پہلے { $old } تھا

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = جیسا ہے ویسا رہتا ہے

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [one] 1 شے کا نام بدلیں
       *[other] { $n } اشیا کے نام بدلیں
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = بدلیں

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = تلاش کریں

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = اس سے بدلیں

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = * اور ? کو وائلڈ کارڈ کے طور پر استعمال کریں

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = وائلڈ کارڈ * اور ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = بڑے چھوٹے حروف بالکل ملائیں

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = حروف کی حالت ملائیں

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = شامل کریں

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = نام سے پہلے

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = نام کے بعد

# Name of the numbering row in the rename window.
folders-tools-rename-number = نمبر

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = بڑے حروف

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = ناموں میں بڑے حروف

# Name over the ending choice in the rename window.
folders-tools-rename-ending = اختتام

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] { $place } میں کچھ نام استعمال نہیں ہو سکتے۔ نشان زدہ قطاریں دیکھیں۔
       *[no] { $place } میں { $total } میں سے { $changes } نام بدلیں گے۔
    }

# Main button of the rename window.
# SAFETY: needs a human review before it ships
folders-tools-rename-button = نام بدلیں

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = ہوم

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [one] { $folder } میں نقول
       *[other] { $n } فولڈرز میں نقول
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = تلاش رک گئی: { $error }۔

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = کوئی نقل نہیں ملی۔

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [one] 1 فائل پڑھی نہیں جا سکی۔
       *[other] { $n } فائلیں پڑھی نہیں جا سکیں۔
    }

# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
# SAFETY: needs a human review before it ships
folders-tools-dupes-sets =
    { $n ->
        [one] نقول کا 1 سیٹ۔ اضافی نقول منتخب ہیں، اور انہیں ردی دان میں ڈالنے سے { $size } خالی ہوتا ہے۔
       *[other] نقول کے { $n } سیٹ۔ اضافی نقول منتخب ہیں، اور انہیں ردی دان میں ڈالنے سے { $size } خالی ہوتا ہے۔
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = سیٹ { $set }، { $size }، { $at } میں

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 فائل پڑھی نہیں جا سکی اور چھوڑ دی گئی۔
       *[other] { $n } فائلیں پڑھی نہیں جا سکیں اور چھوڑ دی گئیں۔
    }

# Note in the duplicates window when every copy of a set is picked.
# SAFETY: needs a human review before it ships
folders-tools-dupes-keep-one = سیٹ کی ہر نقل منتخب ہے۔ کم از کم ایک کو غیر منتخب چھوڑیں۔

# Tip of the button in the duplicates window that picks the extra copies.
# SAFETY: needs a human review before it ships
folders-tools-dupes-pick-extras = اضافی نقول منتخب کریں

# Main button of the duplicates window: move the picked files to the Trash.
# SAFETY: needs a human review before it ships
folders-tools-dupes-trash = ٹریش میں ڈالیں

# Title of the window that compares two files.
folders-tools-diff-title = دو فائلوں کا موازنہ کریں

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders ان کا موازنہ نہیں کر سکا: { $error }۔

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = دونوں فائلیں بالکل ایک جیسی ہیں۔

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = چھوٹی فائل کے ختم ہونے تک یہ ایک جیسی ہیں۔ ایک { $one } ہے اور دوسری { $other }۔

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = یہ بائٹ { $n } سے آگے مختلف ہیں۔

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] دونوں فائلیں ایک جیسی ہیں۔
        [one] 1 سطر مختلف ہے۔
       *[other] { $n } سطریں مختلف ہیں۔
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = یہ اتنی مختلف ہیں کہ سطر بہ سطر موازنہ نہیں ہو سکتا۔ پہلے اور آخری فرق کے درمیان ہر چیز بدلی ہوئی دکھائی جاتی ہے۔

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = ہر فائل کا صرف پہلا 4 MB موازنے میں آتا ہے۔

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = { $left } کا { $right } سے موازنہ کریں

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = مواد کا موازنہ کریں (سست)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = ہر فائل کے اندر جو ہے اس کا موازنہ کریں

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = ایک جیسے نام دکھائیں

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = جو نام ایک جیسے ہیں وہ بھی فہرست میں دکھائیں

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = موازنہ رک گیا: { $error }۔

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = { $left } صرف بائیں طرف، { $right } صرف دائیں طرف، { $different } مختلف، { $same } ایک جیسے۔

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = دونوں فولڈروں کا مواد ایک جیسا ہے۔

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = دوبارہ موازنہ کریں

# Button in the pane compare window; { $n } is how many names are copied to the left folder.
# SAFETY: needs a human review before it ships
folders-tools-compare-copy-left = { $n } غائب یا نئی چیزیں بائیں طرف کاپی کریں

# Button in the pane compare window; { $n } is how many names are copied to the right folder.
# SAFETY: needs a human review before it ships
folders-tools-compare-copy-right = { $n } غائب یا نئی چیزیں دائیں طرف کاپی کریں

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = صرف فائلیں مٹائی جاتی ہیں، اس لیے یہ فولڈر جیسے ہیں ویسے ہی رہیں گے: { $names }۔
