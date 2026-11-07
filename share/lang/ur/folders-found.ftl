

folders-found-need-inside-words = پہلے وہ الفاظ لکھیں جو فائلوں کے اندر تلاش کرنے ہیں۔

folders-found-need-something = جو تلاش کرنا ہے وہ لکھیں، یا کوئی قسم، سائز یا وقت چنیں۔

folders-found-only-here = صرف اس کمپیوٹر کے فولڈروں میں تلاش کی جا سکتی ہے۔

folders-found-shown = { $all } میں سے { $shown } دکھائے گئے

folders-found-in-folder = { $folder } میں

folders-found-in-root = تلاش کیے گئے فولڈر میں

folders-found-in-folder-line =
    { $folder } میں۔ سطر { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(اور { $more } مزید)
    }

folders-found-in-root-line =
    تلاش کیے گئے فولڈر میں۔ سطر { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(اور { $more } مزید)
    }

folders-found-status =
    { $n ->
        [one] 1 ملا
       *[other] { $n } ملے
    }{ $filtered ->
        [yes] { " " }(کل { $all } میں سے)
       *[other]
    }{ $looking ->
        [yes] ، تلاش جاری ہے…
       *[other]
    }{ $full ->
        [yes] ۔ صرف پہلے { $most } دکھائے گئے
       *[other]
    }{ $unread ->
        [0]
       *[other] ۔ { $unread } پڑھے نہیں جا سکے
    }{ $skipped ->
        [0]
       *[other] ۔ { $skipped } بہت بڑے یا متن نہ ہونے کی وجہ سے چھوڑ دیے گئے
    }
