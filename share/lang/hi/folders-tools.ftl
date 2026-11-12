
# machine translated (Hindi), not yet reviewed by a person.

# Choice in the Wipe window: write over each file one time.
folders-tools-pass-once = एक बार

# Choice in the Wipe window: write over each file three times.
folders-tools-pass-three = 3 बार

# Choice in the Wipe window: write over each file seven times.
folders-tools-pass-seven = 7 बार

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = जैसे हैं

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = सब छोटे अक्षर

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = सब बड़े अक्षर

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = हर शब्द बड़े अक्षर से

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = पहला शब्द बड़े अक्षर से

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = जैसा है

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = सब छोटे अक्षर

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = सब बड़े अक्षर

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = नया अंत

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = कोई अंत नहीं

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = संग्रह

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = कोई नाम नहीं बचेगा

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = यह नाम के रूप में मान्य नहीं है

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = वह नाम बहुत लंबा है

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = दो चीज़ों को यही नाम मिलेगा

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = किसी और चीज़ का यही नाम पहले से है

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = पुराने नाम में ऐसे अक्षर हैं जिन्हें Folders पढ़ नहीं सकता

# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
folders-tools-caveat-flash = यह फ़्लैश ड्राइव या SSD है, इसलिए पुरानी प्रतियाँ इसके उन हिस्सों में बची रह सकती हैं जहाँ Folders नहीं पहुँच सकता।

# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
folders-tools-caveat-copy-on-write = { $fs } फ़ाइल सिस्टम बदलाव नई जगह सहेजता है, इसलिए पुराना डेटा अब भी वहाँ हो सकता है।

# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
folders-tools-caveat-network = यह किसी दूसरे कंप्यूटर पर { $fs } शेयर है, जो अपनी प्रतियाँ रख सकता है।

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = सिर्फ़ बाईं ओर

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = सिर्फ़ दाईं ओर

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = एक जैसे

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
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

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = यहाँ

# Red notice when Wipe is chosen on things that are not files.
# SAFETY: needs a human review before it ships
folders-tools-only-files-wiped = सिर्फ़ फ़ाइलें पूरी तरह मिटाई जा सकती हैं।

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = तुलना करने के लिए हर पैन में एक फ़ोल्डर खोलें।

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
        [one] 1 चीज़ का नाम बदलना वापस लें
       *[other] { $n } चीज़ों का नाम बदलना वापस लें
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [one] 1 चीज़ का नाम बदला गया।
       *[other] { $n } चीज़ों के नाम बदले गए।
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = नाम बदलना बीच में ही रुक गया: { $error }।

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = आर्काइव के लिए एक नाम लिखें।

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = { $done } जाँचे गए

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = { $total } में से { $done } पढ़े गए

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = फ़ाइलों की सूची बन रही है

# Beside the ring while two files are compared.
folders-tools-comparing = तुलना हो रही है

# Beside the ring while two folders are compared.
folders-tools-looking = फ़ोल्डरों की तुलना हो रही है

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [one] { $name } को संकुचित करें
       *[other] { $n } चीज़ों को संकुचित करें
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = संग्रह का नाम

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = संग्रह का प्रकार

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = आर्काइव { $place } में सहेजा जाएगा

# Main button of the Compress window.
folders-tools-compress = संकुचित करें

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = { $name } को यहाँ निकालें

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = फ़ाइलें { $name } नाम के नए फ़ोल्डर में रखी जाएँगी।

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [one] { $name } का आकार बदलें
       *[other] { $n } चित्रों का आकार बदलें
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = नया आकार

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = आकार का मतलब लंबी तरफ़ की लंबाई है, पिक्सल में। Folders चित्रों को केवल छोटा करता है। हर बदली हुई कॉपी मूल चित्र के पास सहेजी जाती है, और उसके नाम में नया आकार होता है।

# Main button of the Resize window.
folders-tools-resize = आकार बदलें

# Title of the Wipe window; { $name } is the one file, { $n } how many.
# SAFETY: needs a human review before it ships
folders-tools-wipe-title =
    { $n ->
        [one] { $name } को पूरी तरह मिटाएँ?
       *[other] { $n } फ़ाइलों को पूरी तरह मिटाएँ?
    }

# Warning at the top of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-warning = Folders हर फ़ाइल को मिटाने से पहले उस पर दोबारा लिखता है, इसलिए उसे वापस नहीं पाया जा सकता।

# Name beside the choice of how many times to write over, in the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-write-over = ऊपर से लिखें

# Choice tip in the Wipe window: how many times each file is written over.
# SAFETY: needs a human review before it ships
folders-tools-wipe-passes-tip = हर फ़ाइल पर कितनी बार लिखना है

# Red main button of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-button = सुरक्षित रूप से मिटाएँ

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = उन्हें संख्या दें

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = शुरू में

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = अंत में

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = संख्या कहाँ जाए

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = पहली संख्या

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = अंत का क्या करना है

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = नया अंत, जैसे txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders इसका नाम नहीं बदल सकता: { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = पहले { $old } था

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = जैसा है वैसा रहता है

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [one] 1 चीज़ का नाम बदलें
       *[other] { $n } चीज़ों के नाम बदलें
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = बदलें

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = खोजें

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = इससे बदलें

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = वाइल्डकार्ड के रूप में * और ? इस्तेमाल करें

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = वाइल्डकार्ड * और ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = बड़े अक्षर ठीक वैसे ही मिलाएँ

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = अक्षर-रूप मिलाएँ

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = जोड़ें

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = नाम से पहले

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = नाम के बाद

# Name of the numbering row in the rename window.
folders-tools-rename-number = संख्या

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = बड़े अक्षर

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = नामों में बड़े अक्षर

# Name over the ending choice in the rename window.
folders-tools-rename-ending = अंत (एक्सटेंशन)

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] { $place } में कुछ नाम इस्तेमाल नहीं हो सकते। चिह्नित पंक्तियाँ देखें।
       *[no] { $place } में { $total } में से { $changes } नाम बदलेंगे।
    }

# Main button of the rename window.
folders-tools-rename-button = नाम बदलें

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = होम

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [one] { $folder } में डुप्लिकेट
       *[other] { $n } फ़ोल्डरों में डुप्लिकेट
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = खोज रुक गई: { $error }।

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = कोई डुप्लिकेट नहीं मिला।

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [one] 1 फ़ाइल पढ़ी नहीं जा सकी।
       *[other] { $n } फ़ाइलें पढ़ी नहीं जा सकीं।
    }

# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
        [one] प्रतियों का 1 समूह। अतिरिक्त प्रतियाँ चुनी गई हैं, और उन्हें ट्रैश में डालने से { $size } खाली होगा।
       *[other] प्रतियों के { $n } समूह। अतिरिक्त प्रतियाँ चुनी गई हैं, और उन्हें ट्रैश में डालने से { $size } खाली होगा।
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = समूह { $set }, { $size }, { $at } में

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 फ़ाइल पढ़ी नहीं जा सकी और छोड़ दी गई।
       *[other] { $n } फ़ाइलें पढ़ी नहीं जा सकीं और छोड़ दी गईं।
    }

# Note in the duplicates window when every copy of a set is picked.
folders-tools-dupes-keep-one = एक समूह की हर प्रति चुनी गई है। कम से कम एक को बिना चुना छोड़ें।

# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = अतिरिक्त प्रतियाँ चुनें

# Main button of the duplicates window: move the picked files to the Trash.
folders-tools-dupes-trash = ट्रैश में डालें

# Title of the window that compares two files.
folders-tools-diff-title = दो फ़ाइलों की तुलना करें

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders उनकी तुलना नहीं कर सका: { $error }।

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = दोनों फ़ाइलें बिल्कुल एक जैसी हैं।

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = छोटी फ़ाइल के अंत तक दोनों एक जैसी हैं। एक { $one } की है और दूसरी { $other } की।

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = वे बाइट { $n } से आगे अलग हैं।

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] दोनों फ़ाइलें एक जैसी हैं।
        [one] 1 पंक्ति अलग है।
       *[other] { $n } पंक्तियाँ अलग हैं।
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = वे पंक्ति दर पंक्ति तुलना के लिए बहुत अलग हैं। पहले और आखिरी अंतर के बीच की हर चीज़ बदली हुई दिखती है।

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = हर फ़ाइल के सिर्फ़ पहले 4 MB की तुलना होती है।

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = { $left } की { $right } से तुलना करें

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = सामग्री की तुलना करें (धीमा)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = हर फ़ाइल के अंदर क्या है उसकी तुलना करें

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = एक जैसे नाम दिखाएँ

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = जो नाम एक जैसे हैं उन्हें भी दिखाएँ

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = तुलना रुक गई: { $error }।

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = { $left } सिर्फ़ बाईं ओर, { $right } सिर्फ़ दाईं ओर, { $different } अलग, { $same } एक जैसे।

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = दोनों फ़ोल्डरों की सामग्री एक जैसी है।

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = फिर तुलना करें

# Button in the pane compare window; { $n } is how many names are copied to the left folder.
folders-tools-compare-copy-left = गुम या नई { $n } चीज़ें बाईं ओर कॉपी करें

# Button in the pane compare window; { $n } is how many names are copied to the right folder.
folders-tools-compare-copy-right = गुम या नई { $n } चीज़ें दाईं ओर कॉपी करें

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = केवल फ़ाइलें मिटाई जाती हैं, इसलिए ये फ़ोल्डर जैसे हैं वैसे ही छोड़े जाएँगे: { $names }।
