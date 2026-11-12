
# machine translated (Hindi), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = सर्वर की फ़ाइलों के साथ काटना, कॉपी करना और चिपकाना काम नहीं करता। इसके बजाय पैनलों के बीच कॉपी और ले जाने वाले बटन इस्तेमाल करें।

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = संग्रह के अंदर कुछ भी बदला नहीं जा सकता। पहले जो चाहिए उसे निकाल लें।

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = इन्हें दूसरे ऐप में काटा गया था, इसलिए Folders ने इन्हें कॉपी कर लिया।

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = यह पहले से कॉपी या ले जाया जा रहा है।

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = यह अभी यहाँ कॉपी हो रहा है।

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10,000 या अधिक चीज़ें

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] 1 चीज़
       *[other] { $n } चीज़ें
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] एक चीज़ के लिए { $verb } संभव नहीं हुआ: { $names }।
       *[other] { $n } चीज़ों के लिए { $verb } संभव नहीं हुआ: { $names }।
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }। { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } और { $more } और

# Words on a job card while things are copied: what, and where to.
folders-act-copying = { $what } को { $place } में कॉपी किया जा रहा है

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = { $what } को { $place } में कॉपी किया गया।

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = कॉपी करना

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = { $what } की कॉपी वापस लें

# Words on a job card while things are moved: what, and where to.
folders-act-moving = { $what } को { $place } में ले जाया जा रहा है

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = { $what } को { $place } में ले जाया गया।

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = ले जाना

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = { $what } को ले जाना वापस लें

# Words on a job card while things are put in the Trash.
folders-act-trashing = { $what } को ट्रैश में डाला जा रहा है

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = { $what } को ट्रैश में डाल दिया गया।

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = ट्रैश में डालना

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = { $what } को ट्रैश में डालना वापस लें

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = { $what } को मिटाया जा रहा है

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = { $what } को हमेशा के लिए मिटा दिया गया।

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = मिटाना

# Words on a job card while things are put back from the Trash.
folders-act-restoring = { $what } को वापस रखा जा रहा है

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = { $what } को वापस रख दिया गया।

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = { $what } को वापस रखना वापस लें

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = वापस रखना

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = वापस लिया जा रहा है

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = वापस ले लिया गया।

# Words on a job card while things are packed into an archive.
folders-act-compressing = { $what } को संकुचित किया जा रहा है

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = { $place } में { $file } बनाई गई।

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = संकुचित करना

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = { $what } का संकुचन वापस लें

# Words on a job card while an archive is unpacked.
folders-act-extracting = { $what } को निकाला जा रहा है

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = { $what } को { $place } में निकाला गया।

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = निकालना

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = { $what } को निकालना वापस लें

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = { $what } को पूरी तरह मिटाया जा रहा है

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = { $what } को पूरी तरह मिटा दिया गया।

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = पूरी तरह मिटाना

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = { $what } को बाहर निकाला जा रहा है

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = { $what } को संग्रह से निकालकर { $place } में रखा गया।

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = बाहर निकालना

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = { $what } को बाहर निकालना वापस लें

# Words on a job card while pictures are turned.
folders-act-turning = { $what } को घुमाया जा रहा है

# Words under the file lines when pictures have been turned.
folders-act-turned = { $what } को घुमाया गया।

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = घुमाना

# Words on a job card while pictures are resized.
folders-act-resizing = { $what } का आकार बदला जा रहा है

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = { $what } की आकार बदली हुई कॉपी बनाई गई।

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = आकार बदलना

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = { $what } का आकार बदलना वापस लें

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = नया फ़ोल्डर

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = नया दस्तावेज़

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders { $base } नहीं बना सका: { $why }।

# Red words under a name being typed when it is empty.
folders-act-name-empty = कोई नाम लिखें।

# Red words under a name being typed when it has a slash.
folders-act-name-slash = नाम में स्लैश नहीं हो सकता।

# Red words under a name being typed when it is . or ..
folders-act-name-kept = यह नाम सिस्टम इस्तेमाल करता है। कोई दूसरा नाम चुनें।

# Red words under a name being typed when it is too long.
folders-act-name-long = यह नाम बहुत लंबा है।

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders यह काम शुरू नहीं कर सका।

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = { $what } फिर से करें

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = { $what } वापस लें

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = काम पूरा होने से पहले रोक दिया गया।

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [one] Folders ने एक चीज़ छोड़ दी।
       *[other] Folders ने { $n } चीज़ें छोड़ दीं।
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = पहले कुछ चुनें।

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = वह पहले से ट्रैश में है।

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = सर्वर की फ़ाइलें ट्रैश में नहीं जा सकतीं। इसके बजाय "हमेशा के लिए मिटाएँ" इस्तेमाल करें।

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = पहले कॉपी या ले जाने के लिए कुछ चुनें।

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = चीज़ें संग्रह से सिर्फ़ किसी फ़ोल्डर में निकाली जा सकती हैं।

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = चीज़ों को ट्रैश में कॉपी नहीं किया जा सकता।

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = दूसरा पैन कोई फ़ोल्डर नहीं दिखा रहा।

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = पहले कॉपी करने के लिए कुछ चुनें।

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = { $what } काटा गया।

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = { $what } कॉपी किया गया।

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = चीज़ें सिर्फ़ किसी फ़ोल्डर में चिपकाई जा सकती हैं।

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = क्लिपबोर्ड पर कोई फ़ाइल नहीं है।

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = कुछ काटा या कॉपी नहीं किया गया है।

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = नेटवर्क की जगहें यहाँ से मिटाई नहीं जा सकतीं।

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = ट्रैश पहले से खाली है।

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = पहले ट्रैश में कुछ चुनें।

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = { $name } बनाया जा रहा है

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = नया फ़ोल्डर या दस्तावेज़ सिर्फ़ किसी फ़ोल्डर के अंदर बनाया जा सकता है।

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = { $name } बनाना वापस लें

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders ले जाई गई चीज़ों के फ़ोल्डर रंग बरकरार नहीं रख सका।

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = फिर से करने के लिए कुछ नहीं है।

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = वापस लेने के लिए कुछ नहीं है।

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = { $what } फिर से किया जा रहा है

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = { $what } फिर से किया गया।

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = फिर से करना

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = { $what } वापस लिया जा रहा है

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = { $what } वापस ले लिया गया।

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = होम

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = ट्रैश

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = चीज़ें सिर्फ़ किसी फ़ोल्डर या ट्रैश पर छोड़ी जा सकती हैं।

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] 1 कड़ी
       *[other] { $n } कड़ियाँ
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = { $what } बनाई गई।

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] एक कड़ी नहीं बनाई जा सकी: { $names }।
       *[other] { $n } कड़ियाँ नहीं बनाई जा सकीं: { $names }।
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = एक सवाल आपके जवाब का इंतज़ार कर रहा है। Folders बंद करने से पहले जवाब दें, या काम रोकें।

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = कोई काम चल रहा हो तो Folders बंद नहीं हो सकता। उसके पूरा होने का इंतज़ार करें, या उसे रोकें, फिर Folders बंद करें।
