# machine translated (Bengali), not yet reviewed by a person.

folders-found-need-inside-words = আগে ফাইলের ভিতরে খোঁজার শব্দগুলো লিখুন।

folders-found-need-something = কী খুঁজবেন তা লিখুন, অথবা ধরন, আকার বা সময় বেছে নিন।

folders-found-only-here = শুধু এই কম্পিউটারের ফোল্ডারে খোঁজা যায়।

folders-found-shown = { $all }টির মধ্যে { $shown }টি দেখানো হচ্ছে

folders-found-in-folder = { $folder }-এ

folders-found-in-root = যে ফোল্ডারে খোঁজা হয়েছে তাতে

folders-found-in-folder-line =
    { $folder }-এ। লাইন { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(আরও { $more }টি)
    }

folders-found-in-root-line =
    যে ফোল্ডারে খোঁজা হয়েছে তাতে। লাইন { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(আরও { $more }টি)
    }

folders-found-status =
    { $n ->
        [one] ১টি পাওয়া গেছে
       *[other] { $n }টি পাওয়া গেছে
    }{ $filtered ->
        [yes] { " " }(মোট { $all }টির মধ্যে)
       *[other]
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
