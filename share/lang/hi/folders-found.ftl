# machine translated (Hindi), not yet reviewed by a person.

folders-found-need-inside-words = पहले वे शब्द लिखें जो फ़ाइलों के अंदर खोजने हैं।

folders-found-need-something = जो खोजना है वह लिखें, या कोई प्रकार, आकार या समय चुनें।

folders-found-only-here = केवल इस कंप्यूटर के फ़ोल्डरों में खोजा जा सकता है।

folders-found-shown = { $all } में से { $shown } दिखाए गए

folders-found-in-folder = { $folder } में

folders-found-in-root = खोजे गए फ़ोल्डर में

folders-found-in-folder-line =
    { $folder } में। पंक्ति { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(और { $more } और)
    }

folders-found-in-root-line =
    खोजे गए फ़ोल्डर में। पंक्ति { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(और { $more } और)
    }

folders-found-status =
    { $n ->
        [one] 1 मिला
       *[other] { $n } मिले
    }{ $filtered ->
        [yes] { " " }(कुल { $all } में से)
       *[other]
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
