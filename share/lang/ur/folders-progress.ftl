

folders-progress-pace-known =
    { $files ->
        [yes] { $of } میں سے { $done } فائلیں، { $rate } فی سیکنڈ، { $left }
       *[no] { $rate } فی سیکنڈ، { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $of } میں سے { $done } فائلیں، وقت کا اندازہ لگایا جا رہا ہے…
       *[no] وقت کا اندازہ لگایا جا رہا ہے…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } بائٹ

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = چند سیکنڈ باقی

folders-progress-left-seconds =
    { $n ->
        [one] تقریباً 1 سیکنڈ باقی
       *[other] تقریباً { $n } سیکنڈ باقی
    }

folders-progress-left-minute = تقریباً ایک منٹ باقی

folders-progress-left-minutes =
    { $n ->
        [one] تقریباً 1 منٹ باقی
       *[other] تقریباً { $n } منٹ باقی
    }

folders-progress-left-hours =
    { $n ->
        [one] تقریباً 1 گھنٹہ باقی
       *[other] تقریباً { $n } گھنٹے باقی
    }

folders-progress-more =
    { $n ->
        [one] 1 اور کام جاری ہے
       *[other] { $n } اور کام جاری ہیں
    }

folders-progress-heavy =
    { $verb ->
        [moving] منتقلی ڈسک کو بہت استعمال کرتی ہے، اس لیے ختم ہونے تک دوسرے پروگرام سست لگ سکتے ہیں۔
       *[copying] نقل ڈسک کو بہت استعمال کرتی ہے، اس لیے ختم ہونے تک دوسرے پروگرام سست لگ سکتے ہیں۔
    }

folders-progress-waiting = جواب کا انتظار ہے

folders-progress-paused = موقوف

folders-progress-carry-on = جاری رکھیں

folders-progress-pause = موقوف کریں

folders-progress-stop = روکیں

folders-progress-mark-newer-larger = نئی، بڑی

folders-progress-mark-newer = نئی

folders-progress-mark-larger = بڑی

folders-progress-changed = بدلی گئی { $when }

folders-progress-taken =
    { $kind ->
        [folder] { $name } نام کا فولڈر پہلے سے موجود ہے
        [file] { $name } نام کی فائل پہلے سے موجود ہے
       *[unknown] { $name } پہلے سے موجود ہے
    }

folders-progress-coming = جو لائی جا رہی ہے

folders-progress-there = جو پہلے سے موجود ہے

folders-progress-skip = چھوڑ دیں

folders-progress-keep-both = دونوں رکھیں

folders-progress-replace = بدل دیں

folders-progress-replace-folder-spoken = بدل دیں: پہلے سے موجود فولڈر ردی دان میں چلا جائے گا

folders-progress-merge = ضم کریں

folders-progress-all-folders = پہلے سے موجود ہر فولڈر کے لیے یہی جواب۔

folders-progress-all-files = پہلے سے موجود ہر فائل کے لیے یہی جواب۔

folders-progress-same-for-rest = باقی کے لیے بھی یہی کریں
