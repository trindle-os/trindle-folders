

folders-tools-pass-once = مرة واحدة

folders-tools-pass-three = 3 مرات

folders-tools-pass-seven = 7 مرات

folders-tools-case-keep = كما هي

folders-tools-case-lower = كلها صغيرة

folders-tools-case-upper = كلها كبيرة

folders-tools-case-words = كل كلمة

folders-tools-case-first = الكلمة الأولى

folders-tools-ending-keep = كما هو

folders-tools-ending-lower = كلها صغيرة

folders-tools-ending-upper = كلها كبيرة

folders-tools-ending-new = امتداد جديد

folders-tools-ending-none = بلا امتداد

folders-tools-archive-name = أرشيف

folders-tools-clash-empty = لن يبقى اسم

folders-tools-clash-bad = هذا غير مسموح به كاسم

folders-tools-clash-long = هذا الاسم طويل جدًا

folders-tools-clash-twice = ملفان سيأخذان هذا الاسم

folders-tools-clash-taken = شيء آخر يحمل هذا الاسم

folders-tools-clash-not-text = الاسم القديم ليس نصًا عاديًا

folders-tools-caveat-flash = هذا القرص من نوع فلاش وليس دوّارًا، لذلك قد تبقى نسخ قديمة في أجزاء منه لا يصل إليها Trindle.

folders-tools-caveat-copy-on-write = يكتب نظام الملفات { $fs } التغييرات في مكان جديد، لذلك قد تبقى البيانات القديمة حيث كانت.

folders-tools-caveat-network = هذه مشاركة { $fs } على حاسوب آخر، وقد يحتفظ بنسخ خاصة به.

folders-tools-row-only-left = على اليسار فقط

folders-tools-row-only-right = على اليمين فقط

folders-tools-row-same = متطابق

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

folders-tools-place-here = هنا

folders-tools-only-files-wiped = يمكن مسح الملفات فقط.

folders-tools-compare-need-folders = يجب أن يعرض كلا اللوحين مجلدًا لمقارنتهما.

folders-tools-rename-undo =
    { $n ->
        [zero] تراجع عن إعادة تسمية لا شيء
        [one] تراجع عن إعادة تسمية عنصر واحد
        [two] تراجع عن إعادة تسمية عنصرين
        [few] تراجع عن إعادة تسمية { $n } عناصر
        [many] تراجع عن إعادة تسمية { $n } عنصرًا
       *[other] تراجع عن إعادة تسمية { $n } عنصر
    }

folders-tools-rename-done =
    { $n ->
        [zero] لم تُعد تسمية أي عنصر.
        [one] أُعيدت تسمية عنصر واحد.
        [two] أُعيدت تسمية عنصرين.
        [few] أُعيدت تسمية { $n } عناصر.
        [many] أُعيدت تسمية { $n } عنصرًا.
       *[other] أُعيدت تسمية { $n } عنصر.
    }

folders-tools-rename-stopped = توقفت إعادة التسمية في منتصفها: { $error }.

folders-tools-name-empty = لا يمكن أن يكون الاسم فارغًا.

folders-tools-looked-at = جرى فحص { $done }

folders-tools-read-of = قُرئ { $done } من { $total }

folders-tools-listing-files = جارٍ سرد الملفات

folders-tools-comparing = جارٍ المقارنة

folders-tools-looking = جارٍ الفحص

folders-tools-compress-title =
    { $n ->
        [zero] ضغط لا شيء
        [one] ضغط { $name }
        [two] ضغط عنصرين
        [few] ضغط { $n } عناصر
        [many] ضغط { $n } عنصرًا
       *[other] ضغط { $n } عنصر
    }

folders-tools-pack-name-tip = اسم الأرشيف

folders-tools-pack-kind-tip = نوع الأرشيف

folders-tools-pack-made-in = يُنشأ في { $place }

folders-tools-compress = ضغط

folders-tools-extract-to = استخراج { $name } إلى

folders-tools-extract-new-folder = يُنشأ هناك مجلد جديد باسم { $name }.

folders-tools-resize-title =
    { $n ->
        [zero] تغيير حجم لا شيء
        [one] تغيير حجم { $name }
        [two] تغيير حجم صورتين
        [few] تغيير حجم { $n } صور
        [many] تغيير حجم { $n } صورة
       *[other] تغيير حجم { $n } صورة
    }

folders-tools-resize-size-tip = الحجم الجديد

folders-tools-resize-note = نسبة من حجمها، أو عدد البكسلات على طولها الأكبر؛ ولا تُكبَّر الصورة أبدًا. تُنشأ نسخة بجانب كل صورة، باسم يحمل حجمها الجديد؛ أما الصور نفسها فتبقى كما هي.

folders-tools-resize = تغيير الحجم

folders-tools-wipe-title =
    { $n ->
        [zero] مسح لا شيء؟
        [one] مسح { $name }؟
        [two] مسح ملفين؟
        [few] مسح { $n } ملفات؟
        [many] مسح { $n } ملفًا؟
       *[other] مسح { $n } ملف؟
    }

folders-tools-wipe-warning = يُكتب فوق كل ملف قبل أن يُزال، فلا يمكن استرجاعه. لا يمكن التراجع عن هذا.

folders-tools-wipe-write-over = الكتابة فوقه

folders-tools-wipe-passes-tip = عدد مرات الكتابة فوق كل ملف

folders-tools-wipe-button = مسح آمن

folders-tools-rename-number-tip = ترقيمها

folders-tools-rename-at-start = في البداية

folders-tools-rename-at-end = في النهاية

folders-tools-rename-where-tip = مكان الرقم

folders-tools-rename-first-number-tip = الرقم الأول

folders-tools-rename-ending-tip = ما يحدث للامتداد

folders-tools-rename-new-ending-tip = امتداد جديد، مثل txt

folders-tools-rename-cannot = غير ممكن: { $why }

folders-tools-rename-was = كان { $old }

folders-tools-rename-stays = يبقى كما هو

folders-tools-rename-title =
    { $n ->
        [zero] إعادة تسمية لا شيء
        [one] إعادة تسمية عنصر واحد
        [two] إعادة تسمية عنصرين
        [few] إعادة تسمية { $n } عناصر
        [many] إعادة تسمية { $n } عنصرًا
       *[other] إعادة تسمية { $n } عنصر
    }

folders-tools-rename-replace = استبدال

folders-tools-rename-find-tip = بحث عن

folders-tools-rename-with-tip = استبدال بـ

folders-tools-rename-wildcard-tip = استخدام * و? كحروف بدل

folders-tools-rename-wildcard = حروف البدل * و?

folders-tools-rename-match-case-tip = مطابقة الحروف الكبيرة تمامًا

folders-tools-rename-match-case = مطابقة حالة الأحرف

folders-tools-rename-add = إضافة

folders-tools-rename-before-tip = قبل الاسم

folders-tools-rename-after-tip = بعد الاسم

folders-tools-rename-number = ترقيم

folders-tools-rename-letters = الأحرف

folders-tools-rename-case-tip = الأحرف الكبيرة في الأسماء

folders-tools-rename-ending = الامتداد

folders-tools-rename-summary =
    { $clash ->
        [yes] لا يمكن استخدام بعض الأسماء؛ انظر الصفوف المعلّمة. في { $place }.
       *[no] يتغير { $changes } من { $total } من الأسماء. في { $place }.
    }

folders-tools-rename-button = إعادة تسميتها

folders-tools-home = المجلد الرئيسي

folders-tools-dupes-title =
    { $n ->
        [zero] المكررات في لا مجلد
        [one] المكررات في { $folder }
        [two] المكررات في مجلدين
        [few] المكررات في { $n } مجلدات
        [many] المكررات في { $n } مجلدًا
       *[other] المكررات في { $n } مجلد
    }

folders-tools-dupes-stopped = توقف البحث: { $error }.

folders-tools-dupes-none = لم يُعثر على مكررات.

folders-tools-dupes-unread =
    { $n ->
        [zero] قُرئت كل الملفات.
        [one] تعذّرت قراءة ملف واحد.
        [two] تعذّرت قراءة ملفين.
        [few] تعذّرت قراءة { $n } ملفات.
        [many] تعذّرت قراءة { $n } ملفًا.
       *[other] تعذّرت قراءة { $n } ملف.
    }

folders-tools-dupes-sets =
    { $n ->
        [zero] لا مجموعات نسخ.
        [one] مجموعة نسخ واحدة؛ ونقل النسخ الزائدة إلى سلة المهملات يوفّر { $size }. النسخ الزائدة محددة؛ والنقر مع Ctrl يحدد نسخة أو يلغي تحديدها.
        [two] مجموعتا نسخ؛ ونقل النسخ الزائدة إلى سلة المهملات يوفّر { $size }. النسخ الزائدة محددة؛ والنقر مع Ctrl يحدد نسخة أو يلغي تحديدها.
        [few] { $n } مجموعات نسخ؛ ونقل النسخ الزائدة إلى سلة المهملات يوفّر { $size }. النسخ الزائدة محددة؛ والنقر مع Ctrl يحدد نسخة أو يلغي تحديدها.
        [many] { $n } مجموعة نسخ؛ ونقل النسخ الزائدة إلى سلة المهملات يوفّر { $size }. النسخ الزائدة محددة؛ والنقر مع Ctrl يحدد نسخة أو يلغي تحديدها.
       *[other] { $n } مجموعة نسخ؛ ونقل النسخ الزائدة إلى سلة المهملات يوفّر { $size }. النسخ الزائدة محددة؛ والنقر مع Ctrl يحدد نسخة أو يلغي تحديدها.
    }

folders-tools-dupes-row = المجموعة { $set }، { $size }، في { $at }

folders-tools-dupes-unread-left-out =
    { $n ->
        [zero] قُرئت كل الملفات.
        [one] تعذّرت قراءة ملف واحد فاستُبعد.
        [two] تعذّرت قراءة ملفين فاستُبعدا.
        [few] تعذّرت قراءة { $n } ملفات فاستُبعدت.
        [many] تعذّرت قراءة { $n } ملفًا فاستُبعدت.
       *[other] تعذّرت قراءة { $n } ملف فاستُبعدت.
    }

folders-tools-dupes-keep-one = كل نسخ المجموعة محددة؛ أبقِ واحدة على الأقل.

folders-tools-dupes-pick-extras = تحديد النسخ الزائدة

folders-tools-dupes-trash = نقل إلى سلة المهملات

folders-tools-diff-title = مقارنة الاثنين

folders-tools-diff-failed = تعذّرت المقارنة بينهما: { $error }.

folders-tools-diff-same-bytes = الاثنان متطابقان، بايتًا ببايت.

folders-tools-diff-same-as-shorter = هما متطابقان بقدر طول الأقصر؛ أحدهما { $one } والآخر { $other }.

folders-tools-diff-from-byte = يختلفان ابتداءً من البايت { $n }.

folders-tools-diff-lines =
    { $n ->
        [0] الاثنان متطابقان.
        [one] يختلف سطر واحد.
        [two] يختلف سطران.
        [few] تختلف { $n } أسطر.
        [many] يختلف { $n } سطرًا.
       *[other] يختلف { $n } سطر.
    }

folders-tools-diff-too-different = هما مختلفان جدًا بحيث لا يمكن المطابقة سطرًا بسطر، لذلك يظهر كل سطر بين أول اختلاف وآخره على أنه تغيّر.

folders-tools-diff-cut = تُقارن فقط أول 4 MB من كل منهما.

folders-tools-compare-title = مقارنة { $left } بـ { $right }

folders-tools-compare-bytes = قراءة كل بايت (أبطأ)

folders-tools-compare-bytes-tip = مقارنة ما بداخل كل ملف

folders-tools-compare-show-same = إظهار المتطابق

folders-tools-compare-show-same-tip = إظهار الأسماء المتطابقة أيضًا

folders-tools-compare-stopped = توقفت المقارنة: { $error }.

folders-tools-compare-counts = { $left } على اليسار فقط، و{ $right } على اليمين فقط، و{ $different } مختلف، و{ $same } متطابق.

folders-tools-compare-all-same = يحوي المجلدان الشيء نفسه.

folders-tools-compare-again = قارن مرة أخرى

folders-tools-compare-copy-left = نسخ { $n } مفقودًا أو أحدث إلى اليسار

folders-tools-compare-copy-right = نسخ { $n } مفقودًا أو أحدث إلى اليمين
