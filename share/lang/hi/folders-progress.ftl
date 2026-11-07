

folders-progress-pace-known =
    { $files ->
        [yes] { $of } में से { $done } फ़ाइलें, एक सेकंड में { $rate }, { $left }
       *[no] एक सेकंड में { $rate }, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $of } में से { $done } फ़ाइलें, अंदाज़ा लगाया जा रहा है कितना समय लगेगा…
       *[no] अंदाज़ा लगाया जा रहा है कितना समय लगेगा…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } बाइट

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = कुछ सेकंड बाकी

folders-progress-left-seconds =
    { $n ->
        [one] लगभग 1 सेकंड बाकी
       *[other] लगभग { $n } सेकंड बाकी
    }

folders-progress-left-minute = लगभग एक मिनट बाकी

folders-progress-left-minutes =
    { $n ->
        [one] लगभग 1 मिनट बाकी
       *[other] लगभग { $n } मिनट बाकी
    }

folders-progress-left-hours =
    { $n ->
        [one] लगभग 1 घंटा बाकी
       *[other] लगभग { $n } घंटे बाकी
    }

folders-progress-more =
    { $n ->
        [one] 1 और काम चल रहा है
       *[other] { $n } और काम चल रहे हैं
    }

folders-progress-heavy =
    { $verb ->
        [moving] ले जाने में डिस्क पर बहुत ज़ोर पड़ता है, इसलिए खत्म होने तक दूसरे प्रोग्राम धीमे लग सकते हैं।
       *[copying] कॉपी करने में डिस्क पर बहुत ज़ोर पड़ता है, इसलिए खत्म होने तक दूसरे प्रोग्राम धीमे लग सकते हैं।
    }

folders-progress-waiting = जवाब की प्रतीक्षा है

folders-progress-paused = रुका हुआ

folders-progress-carry-on = जारी रखें

folders-progress-pause = रोकें

folders-progress-stop = बंद करें

folders-progress-mark-newer-larger = नया, बड़ा

folders-progress-mark-newer = नया

folders-progress-mark-larger = बड़ा

folders-progress-changed = { $when } बदला गया

folders-progress-taken =
    { $kind ->
        [folder] { $name } नाम का फ़ोल्डर पहले से मौजूद है
        [file] { $name } नाम की फ़ाइल पहले से मौजूद है
       *[unknown] { $name } पहले से मौजूद है
    }

folders-progress-coming = जो लाई जा रही है

folders-progress-there = जो पहले से वहाँ है

folders-progress-skip = छोड़ें

folders-progress-keep-both = दोनों रखें

folders-progress-replace = बदलें

folders-progress-replace-folder-spoken = बदलें: पहले से मौजूद फ़ोल्डर ट्रैश में चला जाएगा

folders-progress-merge = मिलाएँ

folders-progress-all-folders = पहले से मौजूद हर फ़ोल्डर के लिए यही उत्तर।

folders-progress-all-files = पहले से मौजूद हर फ़ाइल के लिए यही उत्तर।

folders-progress-same-for-rest = बाकी के लिए भी यही करें

