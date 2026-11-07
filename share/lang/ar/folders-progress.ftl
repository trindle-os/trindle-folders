

folders-progress-pace-known =
    { $files ->
        [yes] { $done } من { $of } ملفاً، { $rate } في الثانية، { $left }
       *[no] { $rate } في الثانية، { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $done } من { $of } ملفاً، جارٍ تقدير المدة…
       *[no] جارٍ تقدير المدة…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } بايت

folders-progress-speed-kb = { $n } كيلوبايت

folders-progress-speed-mb = { $n } ميغابايت

folders-progress-speed-gb = { $n } غيغابايت

folders-progress-left-few = بقيت ثوانٍ قليلة

folders-progress-left-seconds =
    { $n ->
        [zero] لم يبقَ شيء
        [one] بقيت نحو ثانية واحدة
        [two] بقيت نحو ثانيتين
        [few] بقيت نحو { $n } ثوانٍ
        [many] بقيت نحو { $n } ثانية
       *[other] بقيت نحو { $n } ثانية
    }

folders-progress-left-minute = بقيت نحو دقيقة

folders-progress-left-minutes =
    { $n ->
        [zero] لم يبقَ شيء
        [one] بقيت نحو دقيقة واحدة
        [two] بقيت نحو دقيقتين
        [few] بقيت نحو { $n } دقائق
        [many] بقيت نحو { $n } دقيقة
       *[other] بقيت نحو { $n } دقيقة
    }

folders-progress-left-hours =
    { $n ->
        [zero] لم يبقَ شيء
        [one] بقيت نحو ساعة واحدة
        [two] بقيت نحو ساعتين
        [few] بقيت نحو { $n } ساعات
        [many] بقيت نحو { $n } ساعة
       *[other] بقيت نحو { $n } ساعة
    }

folders-progress-more =
    { $n ->
        [zero] لا توجد أعمال أخرى جارية
        [one] عمل آخر واحد جارٍ
        [two] عملان آخران جاريان
        [few] { $n } أعمال أخرى جارية
        [many] { $n } عملاً آخر جارٍ
       *[other] { $n } عمل آخر جارٍ
    }

folders-progress-heavy =
    { $verb ->
        [moving] النقل يستخدم القرص بكثافة، لذلك قد تبدو البرامج الأخرى أبطأ حتى ينتهي.
       *[copying] النسخ يستخدم القرص بكثافة، لذلك قد تبدو البرامج الأخرى أبطأ حتى ينتهي.
    }

folders-progress-waiting = بانتظار جواب

folders-progress-paused = متوقف مؤقتاً

folders-progress-carry-on = متابعة

folders-progress-pause = إيقاف مؤقت

folders-progress-stop = إيقاف

folders-progress-mark-newer-larger = أحدث، أكبر

folders-progress-mark-newer = أحدث

folders-progress-mark-larger = أكبر

folders-progress-changed = عُدّل { $when }

folders-progress-taken =
    { $kind ->
        [folder] يوجد هناك مجلد باسم { $name } بالفعل
        [file] يوجد هناك ملف باسم { $name } بالفعل
       *[unknown] { $name } موجود هناك بالفعل
    }

folders-progress-coming = الذي يُجلب

folders-progress-there = الموجود بالفعل

folders-progress-skip = تخطّي

folders-progress-keep-both = إبقاء الاثنين

folders-progress-replace = استبدال

folders-progress-replace-folder-spoken = استبدال: المجلد الموجود بالفعل يذهب إلى سلة المحذوفات

folders-progress-merge = دمج

folders-progress-all-folders = الجواب نفسه لكل مجلد موجود بالفعل.

folders-progress-all-files = الجواب نفسه لكل ملف موجود بالفعل.

folders-progress-same-for-rest = افعل الشيء نفسه مع الباقي

