

folders-tools-pass-once = एक बार

folders-tools-pass-three = 3 बार

folders-tools-pass-seven = 7 बार

folders-tools-case-keep = जैसे हैं

folders-tools-case-lower = सब छोटे

folders-tools-case-upper = सब बड़े अक्षर

folders-tools-case-words = हर शब्द

folders-tools-case-first = पहला शब्द

folders-tools-ending-keep = जैसा है

folders-tools-ending-lower = सब छोटे

folders-tools-ending-upper = सब बड़े अक्षर

folders-tools-ending-new = नया अंत

folders-tools-ending-none = कोई अंत नहीं

folders-tools-archive-name = संग्रह

folders-tools-clash-empty = कोई नाम नहीं बचेगा

folders-tools-clash-bad = यह नाम के रूप में मान्य नहीं है

folders-tools-clash-long = वह नाम बहुत लंबा है

folders-tools-clash-twice = दो को यही नाम मिलेगा

folders-tools-clash-taken = किसी और चीज़ का यही नाम है

folders-tools-clash-not-text = पुराना नाम सादा टेक्स्ट नहीं है

folders-tools-caveat-flash = यह डिस्क फ़्लैश है, घूमने वाली नहीं, इसलिए पुरानी प्रतियाँ इसके उन हिस्सों में बची रह सकती हैं जहाँ Trindle नहीं पहुँच सकता।

folders-tools-caveat-copy-on-write = { $fs } फ़ाइल सिस्टम बदलाव नई जगह लिखता है, इसलिए पुराना डेटा जहाँ था वहीं बचा रह सकता है।

folders-tools-caveat-network = यह किसी दूसरे कंप्यूटर पर { $fs } शेयर है, जो अपनी प्रतियाँ रख सकता है।

folders-tools-row-only-left = सिर्फ़ बाईं ओर

folders-tools-row-only-right = सिर्फ़ दाईं ओर

folders-tools-row-same = एक जैसे

folders-tools-row-different =
    अलग: { $why ->
        [kind] एक फ़ोल्डर या कड़ी है
        [size] आकार अलग हैं
        [date] तारीखें अलग हैं
        [content] सामग्री अलग है
        [target] कड़ियाँ कहीं और जाती हैं
        [inside] अंदर कुछ अलग है
       *[unread] एक तरफ़ पढ़ी नहीं जा सकी
    }{ $newer ->
        [left] , बाईं ओर नया
        [right] , दाईं ओर नया
       *[none] { "" }
    }

folders-tools-place-here = यहाँ

folders-tools-only-files-wiped = सिर्फ़ फ़ाइलें पूरी तरह मिटाई जा सकती हैं।

folders-tools-compare-need-folders = तुलना करने के लिए दोनों पैन में फ़ोल्डर दिखना चाहिए।

folders-tools-rename-undo =
    { $n ->
        [one] 1 चीज़ का नाम बदलना वापस लें
       *[other] { $n } चीज़ों का नाम बदलना वापस लें
    }

folders-tools-rename-done =
    { $n ->
        [one] 1 चीज़ का नाम बदला गया।
       *[other] { $n } चीज़ों के नाम बदले गए।
    }

folders-tools-rename-stopped = नाम बदलना बीच में रुक गया: { $error }।

folders-tools-name-empty = नाम खाली नहीं हो सकता।

folders-tools-looked-at = { $done } देखे गए

folders-tools-read-of = { $total } में से { $done } पढ़े गए

folders-tools-listing-files = फ़ाइलों की सूची बन रही है

folders-tools-comparing = तुलना हो रही है

folders-tools-looking = खोज रहा है

folders-tools-compress-title =
    { $n ->
        [one] { $name } को संकुचित करें
       *[other] { $n } चीज़ों को संकुचित करें
    }

folders-tools-pack-name-tip = संग्रह का नाम

folders-tools-pack-kind-tip = संग्रह का प्रकार

folders-tools-pack-made-in = { $place } में बनाया गया

folders-tools-compress = संकुचित करें

folders-tools-extract-to = { $name } को यहाँ निकालें

folders-tools-extract-new-folder = वहाँ { $name } नाम का नया फ़ोल्डर बनाया जाता है।

folders-tools-resize-title =
    { $n ->
        [one] { $name } का आकार बदलें
       *[other] { $n } चित्रों का आकार बदलें
    }

folders-tools-resize-size-tip = नया आकार

folders-tools-resize-note = उसके आकार का एक हिस्सा, या उसकी लंबी तरफ़ कितने पिक्सल; चित्र कभी बड़ा नहीं किया जाता। हर एक के पास एक कॉपी बनती है, जिसका नाम उसके नए आकार से रखा जाता है; चित्र खुद जैसे हैं वैसे रहते हैं।

folders-tools-resize = आकार बदलें

folders-tools-wipe-title =
    { $n ->
        [one] { $name } को पूरी तरह मिटाएँ?
       *[other] { $n } फ़ाइलों को पूरी तरह मिटाएँ?
    }

folders-tools-wipe-warning = हर फ़ाइल को हटाने से पहले उस पर दोबारा लिखा जाता है, इसलिए उसे वापस नहीं लाया जा सकता। इसे वापस नहीं लिया जा सकता।

folders-tools-wipe-write-over = ऊपर से लिखें

folders-tools-wipe-passes-tip = हर फ़ाइल पर कितनी बार ऊपर से लिखा जाए

folders-tools-wipe-button = सुरक्षित रूप से मिटाएँ

folders-tools-rename-number-tip = उन्हें संख्या दें

folders-tools-rename-at-start = शुरू में

folders-tools-rename-at-end = अंत में

folders-tools-rename-where-tip = संख्या कहाँ जाए

folders-tools-rename-first-number-tip = पहली संख्या

folders-tools-rename-ending-tip = अंत (एक्सटेंशन) का क्या हो

folders-tools-rename-new-ending-tip = नया अंत, जैसे txt

folders-tools-rename-cannot = नहीं हो सकता: { $why }

folders-tools-rename-was = पहले { $old } था

folders-tools-rename-stays = जैसा है वैसा रहता है

folders-tools-rename-title =
    { $n ->
        [one] 1 चीज़ का नाम बदलें
       *[other] { $n } चीज़ों के नाम बदलें
    }

folders-tools-rename-replace = बदलें

folders-tools-rename-find-tip = खोजें

folders-tools-rename-with-tip = इससे बदलें

folders-tools-rename-wildcard-tip = वाइल्डकार्ड के रूप में * और ? इस्तेमाल करें

folders-tools-rename-wildcard = वाइल्डकार्ड * और ?

folders-tools-rename-match-case-tip = बड़े अक्षर ठीक वैसे ही मिलाएँ

folders-tools-rename-match-case = अक्षर-रूप मिलाएँ

folders-tools-rename-add = जोड़ें

folders-tools-rename-before-tip = नाम से पहले

folders-tools-rename-after-tip = नाम के बाद

folders-tools-rename-number = संख्या

folders-tools-rename-letters = अक्षर

folders-tools-rename-case-tip = नामों में बड़े अक्षर

folders-tools-rename-ending = अंत (एक्सटेंशन)

folders-tools-rename-summary =
    { $clash ->
        [yes] कुछ नाम इस्तेमाल नहीं हो सकते; चिह्नित पंक्तियाँ देखें। { $place } में।
       *[no] { $total } में से { $changes } नाम बदलते हैं। { $place } में।
    }

folders-tools-rename-button = इनके नाम बदलें

folders-tools-home = होम

folders-tools-dupes-title =
    { $n ->
        [one] { $folder } में डुप्लिकेट
       *[other] { $n } फ़ोल्डरों में डुप्लिकेट
    }

folders-tools-dupes-stopped = खोज रुक गई: { $error }।

folders-tools-dupes-none = कोई डुप्लिकेट नहीं मिला।

folders-tools-dupes-unread =
    { $n ->
        [one] 1 फ़ाइल पढ़ी नहीं जा सकी।
       *[other] { $n } फ़ाइलें पढ़ी नहीं जा सकीं।
    }

folders-tools-dupes-sets =
    { $n ->
        [one] प्रतियों का 1 समूह; अतिरिक्त प्रतियों को ट्रैश में डालने से { $size } खाली होता है। अतिरिक्त प्रतियाँ चुनी गई हैं; Ctrl और क्लिक से एक को चुनें या छोड़ें।
       *[other] प्रतियों के { $n } समूह; अतिरिक्त प्रतियों को ट्रैश में डालने से { $size } खाली होता है। अतिरिक्त प्रतियाँ चुनी गई हैं; Ctrl और क्लिक से एक को चुनें या छोड़ें।
    }

folders-tools-dupes-row = समूह { $set }, { $size }, { $at } में

folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 फ़ाइल पढ़ी नहीं जा सकी और छोड़ दी गई।
       *[other] { $n } फ़ाइलें पढ़ी नहीं जा सकीं और छोड़ दी गईं।
    }

folders-tools-dupes-keep-one = एक समूह की हर प्रति चुनी गई है; कम से कम एक रखें।

folders-tools-dupes-pick-extras = अतिरिक्त प्रतियाँ चुनें

folders-tools-dupes-trash = ट्रैश में डालें

folders-tools-diff-title = दोनों की तुलना करें

folders-tools-diff-failed = उनकी तुलना नहीं हो सकी: { $error }।

folders-tools-diff-same-bytes = दोनों एक जैसे हैं, बाइट दर बाइट।

folders-tools-diff-same-as-shorter = छोटे वाले की लंबाई तक वे एक जैसे हैं; एक { $one } का है और दूसरा { $other } का।

folders-tools-diff-from-byte = वे बाइट { $n } से आगे अलग हैं।

folders-tools-diff-lines =
    { $n ->
        [0] दोनों एक जैसे हैं।
        [one] 1 पंक्ति अलग है।
       *[other] { $n } पंक्तियाँ अलग हैं।
    }

folders-tools-diff-too-different = वे पंक्ति दर पंक्ति मिलाने के लिए बहुत अलग हैं, इसलिए उनके पहले और आखिरी अंतर के बीच की हर पंक्ति बदली हुई दिखती है।

folders-tools-diff-cut = हर एक के सिर्फ़ पहले 4 MB की तुलना होती है।

folders-tools-compare-title = { $left } की { $right } से तुलना करें

folders-tools-compare-bytes = हर बाइट पढ़ें (धीमा)

folders-tools-compare-bytes-tip = हर फ़ाइल के अंदर क्या है उसकी तुलना करें

folders-tools-compare-show-same = जो एक जैसा है उसे दिखाएँ

folders-tools-compare-show-same-tip = जो नाम एक जैसे हैं उन्हें भी दिखाएँ

folders-tools-compare-stopped = तुलना रुक गई: { $error }।

folders-tools-compare-counts = { $left } सिर्फ़ बाईं ओर, { $right } सिर्फ़ दाईं ओर, { $different } अलग, { $same } एक जैसे।

folders-tools-compare-all-same = दोनों फ़ोल्डरों में एक जैसा है।

folders-tools-compare-again = फिर तुलना करें

folders-tools-compare-copy-left = गुम या नए { $n } को बाईं ओर कॉपी करें

folders-tools-compare-copy-right = गुम या नए { $n } को दाईं ओर कॉपी करें

