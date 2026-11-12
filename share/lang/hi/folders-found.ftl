# machine translated (Hindi), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = पहले वे शब्द लिखें जो फ़ाइलों के अंदर खोजने हैं।

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = खोजने के लिए कुछ लिखें, या कोई प्रकार, आकार या समय चुनें।

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = केवल इस कंप्यूटर के फ़ोल्डरों में खोजा जा सकता है।

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $all } में से { $shown } दिखाए गए

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = { $folder } में

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = आपके खोजे गए फ़ोल्डर में

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    { $folder } में। पंक्ति { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(और { $more } और)
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    आपके खोजे गए फ़ोल्डर में। पंक्ति { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(साथ में { $more } और)
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] { $all } में से { $n } मिले
       *[other] { $n ->
            [one] 1 मिला
           *[other] { $n } मिले
        }
    }{ $looking ->
        [yes] , अभी खोज जारी है…
       *[other]
    }{ $full ->
        [yes] । केवल पहले { $most } दिखाए गए हैं
       *[other]
    }{ $unread ->
        [0]
       *[other] । { $unread } पढ़े नहीं जा सके
    }{ $skipped ->
        [0]
       *[other] । { $skipped } बहुत बड़े होने या टेक्स्ट न होने से छोड़े गए
    }
