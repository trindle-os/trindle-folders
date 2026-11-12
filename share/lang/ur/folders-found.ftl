
# machine translated (Urdu), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = پہلے وہ الفاظ لکھیں جو فائلوں کے اندر تلاش کرنے ہیں۔

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = تلاش کرنے کے لیے کچھ لکھیں، یا کوئی قسم، سائز یا وقت چنیں۔

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = صرف اس کمپیوٹر کے فولڈروں میں تلاش کی جا سکتی ہے۔

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $all } میں سے { $shown } دکھائے گئے

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = { $folder } میں

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = آپ کے تلاش کیے گئے فولڈر میں

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    { $folder } میں۔ سطر { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(اور { $more } مزید)
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    آپ کے تلاش کیے گئے فولڈر میں۔ سطر { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(اور { $more } مزید)
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] { $all } میں سے { $n } ملے
       *[other] { $n ->
            [one] 1 ملا
           *[other] { $n } ملے
        }
    }{ $looking ->
        [yes] ، تلاش جاری ہے…
       *[other]
    }{ $full ->
        [yes] ۔ صرف پہلے { $most } دکھائے جا رہے ہیں
       *[other]
    }{ $unread ->
        [0]
       *[other] ۔ { $unread } پڑھے نہیں جا سکے
    }{ $skipped ->
        [0]
       *[other] ۔ { $skipped } بہت بڑے یا متن نہ ہونے کی وجہ سے چھوڑ دیے گئے
    }
