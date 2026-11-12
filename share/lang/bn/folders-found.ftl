# machine translated (Bengali), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = আগে ফাইলের ভিতরে খোঁজার শব্দগুলো লিখুন।

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = কী খুঁজবেন তা লিখুন, অথবা ধরন, আকার বা সময় বেছে নিন।

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = শুধু এই কম্পিউটারের ফোল্ডারে খোঁজা যায়।

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $all }টির মধ্যে { $shown }টি দেখানো হচ্ছে

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = { $folder }-এ

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = আপনি যে ফোল্ডারে খুঁজেছেন তাতে

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    { $folder }-এ। লাইন { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(আরও { $more }টি)
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    আপনি যে ফোল্ডারে খুঁজেছেন তাতে। লাইন { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(আরও { $more }টি)
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] মোট { $all }টির মধ্যে { $n }টি পাওয়া গেছে
       *[other] { $n ->
            [one] ১টি পাওয়া গেছে
           *[other] { $n }টি পাওয়া গেছে
        }
    }{ $looking ->
        [yes] , এখনও খোঁজা হচ্ছে…
       *[other]
    }{ $full ->
        [yes] । শুধু প্রথম { $most }টি দেখানো হচ্ছে
       *[other]
    }{ $unread ->
        [0]
       *[other] । { $unread }টি পড়া যায়নি
    }{ $skipped ->
        [0]
       *[other] । { $skipped }টি বেশি বড় বা লেখা নয় বলে বাদ দেওয়া হয়েছে
    }
