# machine translated (Bengali), not yet reviewed by a person.

# Choice in the Wipe window: write over each file one time.
# SAFETY: needs a human review before it ships
folders-tools-pass-once = একবার

# Choice in the Wipe window: write over each file three times.
# SAFETY: needs a human review before it ships
folders-tools-pass-three = ৩ বার

# Choice in the Wipe window: write over each file seven times.
# SAFETY: needs a human review before it ships
folders-tools-pass-seven = ৭ বার

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = যেমন আছে

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = সব ছোট হাতের অক্ষর

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = সব বড় হাতের

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = প্রতিটি শব্দ বড় হাতের করুন

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = প্রথম শব্দ বড় হাতের করুন

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = যেমন আছে

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = সব ছোট হাতের অক্ষর

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = সব বড় হাতের

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = নতুন শেষাংশ

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = শেষাংশ ছাড়া

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = আর্কাইভ

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = কোনো নাম থাকবে না

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = এটি নাম হিসেবে চলে না

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = নামটি অনেক লম্বা

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = দুটি জিনিস এই নাম পাবে

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = অন্য কিছুর আগে থেকেই এই নাম আছে

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = পুরনো নামে এমন অক্ষর আছে যা Folders পড়তে পারে না

# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
# SAFETY: needs a human review before it ships
folders-tools-caveat-flash = এটি একটি ফ্ল্যাশ ড্রাইভ বা SSD, তাই পুরনো কপি এমন জায়গায় থেকে যেতে পারে যেখানে Folders পৌঁছাতে পারে না।

# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
# SAFETY: needs a human review before it ships
folders-tools-caveat-copy-on-write = { $fs } ফাইল সিস্টেম বদলগুলো নতুন জায়গায় সংরক্ষণ করে, তাই পুরনো তথ্য এখনও থেকে যেতে পারে।

# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
# SAFETY: needs a human review before it ships
folders-tools-caveat-network = এটি অন্য কম্পিউটারের একটি { $fs } শেয়ার, যা নিজের কপি রেখে দিতে পারে।

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = শুধু বাঁয়ে

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = শুধু ডানে

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = একই

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
folders-tools-row-different =
    আলাদা: { $why ->
        [kind] একটি ফোল্ডার বা লিংক
        [size] মাপ আলাদা
        [date] তারিখ আলাদা
        [content] বিষয়বস্তু আলাদা
        [target] লিংক অন্য জায়গায় যায়
        [inside] ভিতরের কিছু আলাদা
       *[unread] একদিক পড়া যায়নি
    }{ $newer ->
        [left] , বাঁয়েরটি নতুন
        [right] , ডানেরটি নতুন
       *[none] { "" }
    }

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = এখানে

# Red notice when Wipe is chosen on things that are not files.
folders-tools-only-files-wiped = শুধু ফাইল মুছে ফেলা (ওয়াইপ) যায়।

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = তুলনা করতে প্রতিটি প্যানে একটি ফোল্ডার খুলুন।

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
        [one] ১টি আইটেমের নাম বদল ফিরিয়ে নিন
       *[other] { $n }টি আইটেমের নাম বদল ফিরিয়ে নিন
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [one] ১টি আইটেমের নাম বদলানো হয়েছে।
       *[other] { $n }টি আইটেমের নাম বদলানো হয়েছে।
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = নাম বদল মাঝপথে থেমে গেছে: { $error }।

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = আর্কাইভের একটি নাম লিখুন।

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = { $done }টি যাচাই করা হয়েছে

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = { $total }-এর মধ্যে { $done } পড়া হয়েছে

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = ফাইলের তালিকা করা হচ্ছে

# Beside the ring while two files are compared.
folders-tools-comparing = তুলনা করা হচ্ছে

# Beside the ring while two folders are compared.
folders-tools-looking = ফোল্ডার তুলনা করা হচ্ছে

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [one] { $name } কম্প্রেস করুন
       *[other] { $n }টি আইটেম কম্প্রেস করুন
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = আর্কাইভের নাম

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = আর্কাইভের ধরন

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = আর্কাইভ { $place }-এ সংরক্ষণ করা হবে

# Main button of the Compress window.
folders-tools-compress = কম্প্রেস করুন

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = { $name } এখানে খুলুন

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = ফাইলগুলো { $name } নামের একটি নতুন ফোল্ডারে রাখা হবে।

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [one] { $name }-এর মাপ বদলান
       *[other] { $n }টি ছবির মাপ বদলান
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = নতুন মাপ

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = মাপ বলতে পিক্সেলে লম্বা দিকের দৈর্ঘ্য বোঝায়। Folders শুধু ছবি ছোট করে। প্রতিটি মাপ বদলানো কপি মূল ছবির পাশে সংরক্ষিত হয়, নামে নতুন মাপ থাকে।

# Main button of the Resize window.
folders-tools-resize = মাপ বদলান

# Title of the Wipe window; { $name } is the one file, { $n } how many.
# SAFETY: needs a human review before it ships
folders-tools-wipe-title =
    { $n ->
        [one] { $name } ওয়াইপ করবেন?
       *[other] { $n }টি ফাইল ওয়াইপ করবেন?
    }

# Warning at the top of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-warning = Folders প্রতিটি ফাইল মুছে ফেলার আগে তার ওপর অন্য তথ্য লিখে দেয়, তাই আর ফেরত পাওয়া যাবে না।

# Name beside the choice of how many times to write over, in the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-write-over = ওপরে লিখুন

# Choice tip in the Wipe window: how many times each file is written over.
# SAFETY: needs a human review before it ships
folders-tools-wipe-passes-tip = প্রতিটি ফাইলের ওপর কতবার লিখতে হবে

# Red main button of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-button = নিরাপদে ওয়াইপ করুন

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = নম্বর দিন

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = শুরুতে

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = শেষে

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = নম্বর কোথায় বসবে

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = প্রথম নম্বর

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = শেষাংশের সঙ্গে কী করবেন

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = নতুন শেষাংশ, যেমন txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders এটির নাম বদলাতে পারছে না: { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = আগে ছিল { $old }

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = যেমন আছে তেমনই থাকবে

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [one] ১টি আইটেমের নাম বদলান
       *[other] { $n }টি আইটেমের নাম বদলান
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = প্রতিস্থাপন

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = খুঁজুন

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = এটি দিয়ে বদলান

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = * ও ? ওয়াইল্ডকার্ড হিসেবে ব্যবহার করুন

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = ওয়াইল্ডকার্ড * ও ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = বড় হাতের অক্ষর হুবহু মেলান

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = অক্ষরের ধরন মেলান

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = যোগ করুন

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = নামের আগে

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = নামের পরে

# Name of the numbering row in the rename window.
folders-tools-rename-number = নম্বর

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = বড় হাতের অক্ষর

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = নামের বড় হাতের অক্ষর

# Name over the ending choice in the rename window.
folders-tools-rename-ending = শেষাংশ

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] { $place }-এ কিছু নাম ব্যবহার করা যাবে না। চিহ্নিত সারিগুলো দেখুন।
       *[no] { $place }-এ { $total }টির মধ্যে { $changes }টি নাম বদলাবে।
    }

# Main button of the rename window.
folders-tools-rename-button = নাম বদলান

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = হোম

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [one] { $folder }-এর ডুপ্লিকেট
       *[other] { $n }টি ফোল্ডারের ডুপ্লিকেট
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = খোঁজা থেমে গেছে: { $error }।

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = কোনো ডুপ্লিকেট পাওয়া যায়নি।

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [one] ১টি ফাইল পড়া যায়নি।
       *[other] { $n }টি ফাইল পড়া যায়নি।
    }

# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
        [one] ১ সেট কপি। বাড়তি কপিগুলো বাছা আছে, সেগুলো ট্র্যাশে সরালে { $size } খালি হবে।
       *[other] { $n } সেট কপি। বাড়তি কপিগুলো বাছা আছে, সেগুলো ট্র্যাশে সরালে { $size } খালি হবে।
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = সেট { $set }, { $size }, { $at }-এ

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [one] ১টি ফাইল পড়া যায়নি, তাই বাদ দেওয়া হয়েছে।
       *[other] { $n }টি ফাইল পড়া যায়নি, তাই বাদ দেওয়া হয়েছে।
    }

# Note in the duplicates window when every copy of a set is picked.
folders-tools-dupes-keep-one = একটি সেটের সব কপি বাছা আছে। অন্তত একটি বাছাই ছাড়া রাখুন।

# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = বাড়তি কপিগুলো বাছুন

# Main button of the duplicates window: move the picked files to the Trash.
folders-tools-dupes-trash = ট্র্যাশে পাঠান

# Title of the window that compares two files.
folders-tools-diff-title = দুটি ফাইল তুলনা করুন

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders তুলনা করতে পারেনি: { $error }।

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = দুটি ফাইল হুবহু একই।

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = ছোটটির শেষ পর্যন্ত দুটি একই। একটি { $one } আর অন্যটি { $other }।

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = বাইট { $n } থেকে দুটি আলাদা।

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] দুটি ফাইল একই।
        [one] ১টি লাইন আলাদা।
       *[other] { $n }টি লাইন আলাদা।
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = দুটি এত আলাদা যে লাইন ধরে তুলনা করা যায় না। প্রথম ও শেষ পার্থক্যের মাঝের সবকিছু বদলানো হিসেবে দেখায়।

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = প্রতিটি ফাইলের শুধু প্রথম ৪ MB তুলনা করা হয়।

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = { $left } ও { $right } তুলনা করুন

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = বিষয়বস্তু তুলনা করুন (ধীর)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = প্রতিটি ফাইলের ভিতরে কী আছে তুলনা করুন

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = মিলে যাওয়া নাম দেখান

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = মিলে যাওয়া নামগুলোও তালিকায় দেখান

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = তুলনা থেমে গেছে: { $error }।

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = শুধু বাঁয়ে { $left }টি, শুধু ডানে { $right }টি, আলাদা { $different }টি, একই { $same }টি।

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = দুটি ফোল্ডারের বিষয়বস্তু একই।

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = আবার তুলনা করুন

# Button in the pane compare window; { $n } is how many names are copied to the left folder.
folders-tools-compare-copy-left = নেই বা নতুন { $n }টি জিনিস বাঁয়ে কপি করুন

# Button in the pane compare window; { $n } is how many names are copied to the right folder.
folders-tools-compare-copy-right = নেই বা নতুন { $n }টি জিনিস ডানে কপি করুন

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = শুধু ফাইল ওয়াইপ করা হয়, তাই এই ফোল্ডারগুলো যেমন আছে তেমনই থাকবে: { $names }।
