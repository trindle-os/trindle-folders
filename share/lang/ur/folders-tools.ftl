

folders-tools-pass-once = ایک بار

folders-tools-pass-three = 3 بار

folders-tools-pass-seven = 7 بار

folders-tools-case-keep = جیسے ہیں

folders-tools-case-lower = سب چھوٹے

folders-tools-case-upper = سب بڑے

folders-tools-case-words = ہر لفظ

folders-tools-case-first = پہلا لفظ

folders-tools-ending-keep = جیسا ہے

folders-tools-ending-lower = سب چھوٹے

folders-tools-ending-upper = سب بڑے

folders-tools-ending-new = نیا اختتام

folders-tools-ending-none = کوئی اختتام نہیں

folders-tools-archive-name = آرکائیو

folders-tools-clash-empty = کوئی نام نہیں بچے گا

folders-tools-clash-bad = یہ نام کے طور پر جائز نہیں

folders-tools-clash-long = یہ نام بہت لمبا ہے

folders-tools-clash-twice = دو کو یہی نام ملے گا

folders-tools-clash-taken = کسی اور کا یہی نام ہے

folders-tools-clash-not-text = پرانا نام سادہ متن نہیں ہے

folders-tools-caveat-flash = یہ ڈسک فلیش ہے، گھومنے والی نہیں، اس لیے پرانی نقول اس کے ان حصوں میں رہ سکتی ہیں جن تک Trindle نہیں پہنچ سکتا۔

folders-tools-caveat-copy-on-write = { $fs } فائل سسٹم تبدیلیاں نئی جگہ لکھتا ہے، اس لیے پرانا ڈیٹا اپنی جگہ پر رہ سکتا ہے۔

folders-tools-caveat-network = یہ دوسرے کمپیوٹر پر { $fs } شیئر ہے، جو اپنی نقول رکھ سکتا ہے۔

folders-tools-row-only-left = صرف بائیں طرف

folders-tools-row-only-right = صرف دائیں طرف

folders-tools-row-same = ایک جیسا

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

folders-tools-place-here = یہاں

folders-tools-only-files-wiped = صرف فائلیں مٹائی جا سکتی ہیں۔

folders-tools-compare-need-folders = موازنے کے لیے دونوں پین میں فولڈر دکھانا ضروری ہے۔

folders-tools-rename-undo =
    { $n ->
        [one] 1 شے کا نام بدلنا واپس کریں
       *[other] { $n } اشیا کے نام بدلنا واپس کریں
    }

folders-tools-rename-done =
    { $n ->
        [one] 1 شے کا نام بدل دیا گیا۔
       *[other] { $n } اشیا کے نام بدل دیے گئے۔
    }

folders-tools-rename-stopped = نام بدلنا درمیان میں رک گیا: { $error }۔

folders-tools-name-empty = نام خالی نہیں ہو سکتا۔

folders-tools-looked-at = { $done } دیکھی گئیں

folders-tools-read-of = { $total } میں سے { $done } پڑھا گیا

folders-tools-listing-files = فائلوں کی فہرست بن رہی ہے

folders-tools-comparing = موازنہ ہو رہا ہے

folders-tools-looking = دیکھا جا رہا ہے

folders-tools-compress-title =
    { $n ->
        [one] { $name } کمپریس کریں
       *[other] { $n } اشیا کمپریس کریں
    }

folders-tools-pack-name-tip = آرکائیو کا نام

folders-tools-pack-kind-tip = آرکائیو کی قسم

folders-tools-pack-made-in = { $place } میں بنایا گیا

folders-tools-compress = کمپریس کریں

folders-tools-extract-to = { $name } کو یہاں نکالیں

folders-tools-extract-new-folder = وہاں { $name } نام کا نیا فولڈر بنایا جاتا ہے۔

folders-tools-resize-title =
    { $n ->
        [one] { $name } کا سائز بدلیں
       *[other] { $n } تصویروں کا سائز بدلیں
    }

folders-tools-resize-size-tip = نیا سائز

folders-tools-resize-note = اس کے سائز کا ایک حصہ، یا اس کی لمبی طرف کتنے پکسل؛ تصویر کو کبھی بڑا نہیں کیا جاتا۔ ہر ایک کے ساتھ ایک کاپی بنتی ہے جس کے نام میں نیا سائز ہوتا ہے؛ اصل تصویریں جیسی ہیں ویسی رہتی ہیں۔

folders-tools-resize = سائز بدلیں

folders-tools-wipe-title =
    { $n ->
        [one] کیا { $name } کو محفوظ طریقے سے مٹائیں؟
       *[other] کیا { $n } فائلیں محفوظ طریقے سے مٹائیں؟
    }

folders-tools-wipe-warning = ہر فائل کو جانے سے پہلے اوپر سے لکھ دیا جاتا ہے، اس لیے اسے واپس نہیں لایا جا سکتا۔ اسے واپس نہیں کیا جا سکتا۔

folders-tools-wipe-write-over = اوپر سے لکھیں

folders-tools-wipe-passes-tip = ہر فائل پر کتنی بار اوپر سے لکھا جائے

folders-tools-wipe-button = محفوظ طریقے سے مٹائیں

folders-tools-rename-number-tip = انہیں نمبر دیں

folders-tools-rename-at-start = شروع میں

folders-tools-rename-at-end = آخر میں

folders-tools-rename-where-tip = نمبر کہاں جائے

folders-tools-rename-first-number-tip = پہلا نمبر

folders-tools-rename-ending-tip = اختتام کے ساتھ کیا ہو

folders-tools-rename-new-ending-tip = نیا اختتام، جیسے txt

folders-tools-rename-cannot = نہیں ہو سکتا: { $why }

folders-tools-rename-was = پہلے { $old } تھا

folders-tools-rename-stays = جیسا ہے ویسا رہتا ہے

folders-tools-rename-title =
    { $n ->
        [one] 1 شے کا نام بدلیں
       *[other] { $n } اشیا کے نام بدلیں
    }

folders-tools-rename-replace = بدلیں

folders-tools-rename-find-tip = تلاش کریں

folders-tools-rename-with-tip = اس سے بدلیں

folders-tools-rename-wildcard-tip = * اور ? کو وائلڈ کارڈ کے طور پر استعمال کریں

folders-tools-rename-wildcard = وائلڈ کارڈ * اور ?

folders-tools-rename-match-case-tip = بڑے چھوٹے حروف بالکل ملائیں

folders-tools-rename-match-case = حروف کی حالت ملائیں

folders-tools-rename-add = شامل کریں

folders-tools-rename-before-tip = نام سے پہلے

folders-tools-rename-after-tip = نام کے بعد

folders-tools-rename-number = نمبر

folders-tools-rename-letters = حروف

folders-tools-rename-case-tip = ناموں میں بڑے حروف

folders-tools-rename-ending = اختتام

folders-tools-rename-summary =
    { $clash ->
        [yes] کچھ نام استعمال نہیں ہو سکتے؛ نشان زدہ قطاریں دیکھیں۔ { $place } میں۔
       *[no] { $total } میں سے { $changes } نام بدلتے ہیں۔ { $place } میں۔
    }

folders-tools-rename-button = ان کے نام بدلیں

folders-tools-home = ہوم

folders-tools-dupes-title =
    { $n ->
        [one] { $folder } میں نقول
       *[other] { $n } فولڈرز میں نقول
    }

folders-tools-dupes-stopped = تلاش رک گئی: { $error }۔

folders-tools-dupes-none = کوئی نقل نہیں ملی۔

folders-tools-dupes-unread =
    { $n ->
        [one] 1 فائل پڑھی نہیں جا سکی۔
       *[other] { $n } فائلیں پڑھی نہیں جا سکیں۔
    }

folders-tools-dupes-sets =
    { $n ->
        [one] نقول کا 1 سیٹ؛ اضافی نقول کو ٹریش میں ڈالنے سے { $size } خالی ہوتا ہے۔ اضافی نقول منتخب ہیں؛ Ctrl اور کلک سے کسی کو منتخب یا غیر منتخب کریں۔
       *[other] نقول کے { $n } سیٹ؛ اضافی نقول کو ٹریش میں ڈالنے سے { $size } خالی ہوتا ہے۔ اضافی نقول منتخب ہیں؛ Ctrl اور کلک سے کسی کو منتخب یا غیر منتخب کریں۔
    }

folders-tools-dupes-row = سیٹ { $set }، { $size }، { $at } میں

folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 فائل پڑھی نہیں جا سکی اور چھوڑ دی گئی۔
       *[other] { $n } فائلیں پڑھی نہیں جا سکیں اور چھوڑ دی گئیں۔
    }

folders-tools-dupes-keep-one = سیٹ کی ہر نقل منتخب ہے؛ کم از کم ایک رکھیں۔

folders-tools-dupes-pick-extras = اضافی نقول منتخب کریں

folders-tools-dupes-trash = ٹریش میں ڈالیں

folders-tools-diff-title = دونوں کا موازنہ کریں

folders-tools-diff-failed = ان کا موازنہ نہیں ہو سکا: { $error }۔

folders-tools-diff-same-bytes = دونوں بائٹ بہ بائٹ ایک جیسی ہیں۔

folders-tools-diff-same-as-shorter = چھوٹی فائل کے ختم ہونے تک یہ ایک جیسی ہیں؛ ایک { $one } ہے اور دوسری { $other }۔

folders-tools-diff-from-byte = یہ بائٹ { $n } سے مختلف ہیں۔

folders-tools-diff-lines =
    { $n ->
        [0] دونوں ایک جیسی ہیں۔
        [one] 1 سطر مختلف ہے۔
       *[other] { $n } سطریں مختلف ہیں۔
    }

folders-tools-diff-too-different = یہ اتنی مختلف ہیں کہ سطر بہ سطر نہیں ملائی جا سکتیں، اس لیے ان کے پہلے اور آخری فرق کے درمیان ہر سطر بدلی ہوئی دکھائی جاتی ہے۔

folders-tools-diff-cut = ہر ایک کا صرف پہلا 4 MB موازنے میں آتا ہے۔

folders-tools-compare-title = { $left } کا { $right } سے موازنہ کریں

folders-tools-compare-bytes = ہر بائٹ پڑھیں (سست)

folders-tools-compare-bytes-tip = ہر فائل کے اندر جو ہے اس کا موازنہ کریں

folders-tools-compare-show-same = جو ایک جیسا ہے وہ دکھائیں

folders-tools-compare-show-same-tip = جو نام ایک جیسے ہیں وہ بھی دکھائیں

folders-tools-compare-stopped = موازنہ رک گیا: { $error }۔

folders-tools-compare-counts = { $left } صرف بائیں طرف، { $right } صرف دائیں طرف، { $different } مختلف، { $same } ایک جیسے۔

folders-tools-compare-all-same = دونوں فولڈروں میں ایک جیسا ہے۔

folders-tools-compare-again = دوبارہ موازنہ کریں

folders-tools-compare-copy-left = { $n } غائب یا نئے بائیں طرف کاپی کریں

folders-tools-compare-copy-right = { $n } غائب یا نئے دائیں طرف کاپی کریں
