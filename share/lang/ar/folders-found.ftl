

folders-found-need-inside-words = اكتب الكلمات المراد البحث عنها داخل الملفات أولاً.

folders-found-need-something = اكتب ما تبحث عنه، أو اختر نوعاً أو حجماً أو وقتاً.

folders-found-only-here = لا يمكن البحث إلا في المجلدات الموجودة على هذا الحاسوب.

folders-found-shown = معروض { $shown } من { $all }

folders-found-in-folder = في { $folder }

folders-found-in-root = في المجلد الذي جرى البحث فيه

folders-found-in-folder-line =
    في { $folder }. السطر { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(و{ $more } أخرى)
    }

folders-found-in-root-line =
    في المجلد الذي جرى البحث فيه. السطر { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(و{ $more } أخرى)
    }

folders-found-status =
    { $n ->
        [zero] لا نتائج
        [one] عُثر على نتيجة واحدة
        [two] عُثر على نتيجتين
        [few] عُثر على { $n } نتائج
        [many] عُثر على { $n } نتيجة
       *[other] عُثر على { $n } نتيجة
    }{ $filtered ->
        [yes] { " " }من { $all }
       *[other]
    }{ $looking ->
        [yes] ، ولا يزال البحث جارياً…
       *[other]
    }{ $full ->
        [yes] . معروض أول { $most } فقط
       *[other]
    }{ $unread ->
        [0]
       *[other] . تعذّرت قراءة { $unread }
    }{ $skipped ->
        [0]
       *[other] . تُخطّي { $skipped } لأنها كبيرة جداً أو ليست نصاً
    }

