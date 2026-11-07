# machine translated (Bengali), not yet reviewed by a person.

folders-tools-pass-once = একবার

folders-tools-pass-three = ৩ বার

folders-tools-pass-seven = ৭ বার

folders-tools-case-keep = যেমন আছে

folders-tools-case-lower = সব ছোট হাতের

folders-tools-case-upper = সব বড় হাতের

folders-tools-case-words = প্রতিটি শব্দ

folders-tools-case-first = প্রথম শব্দ

folders-tools-ending-keep = যেমন আছে

folders-tools-ending-lower = সব ছোট হাতের

folders-tools-ending-upper = সব বড় হাতের

folders-tools-ending-new = নতুন শেষাংশ

folders-tools-ending-none = শেষাংশ ছাড়া

folders-tools-archive-name = আর্কাইভ

folders-tools-clash-empty = কোনো নাম থাকবে না

folders-tools-clash-bad = এটি নাম হিসেবে চলে না

folders-tools-clash-long = নামটি অনেক লম্বা

folders-tools-clash-twice = দুটি ফাইল এই নাম পাবে

folders-tools-clash-taken = অন্য কিছুর এই নাম আছে

folders-tools-clash-not-text = পুরনো নামটি সাধারণ লেখা নয়

folders-tools-caveat-flash = এটি ফ্ল্যাশ ডিস্ক, ঘূর্ণায়মান ডিস্ক নয়, তাই পুরনো কপি এমন জায়গায় থেকে যেতে পারে যেখানে Trindle পৌঁছাতে পারে না।

folders-tools-caveat-copy-on-write = { $fs } ফাইলসিস্টেম বদলগুলো নতুন জায়গায় লেখে, তাই পুরনো তথ্য আগের জায়গাতেই থেকে যেতে পারে।

folders-tools-caveat-network = এটি অন্য কম্পিউটারের একটি { $fs } শেয়ার, যা নিজের মতো কপি রেখে দিতে পারে।

folders-tools-row-only-left = শুধু বাঁয়ে

folders-tools-row-only-right = শুধু ডানে

folders-tools-row-same = একই

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

folders-tools-place-here = এখানে

folders-tools-only-files-wiped = শুধু ফাইল মুছে ফেলা (ওয়াইপ) যায়।

folders-tools-compare-need-folders = তুলনা করতে দুটি প্যানেই ফোল্ডার দেখাতে হবে।

folders-tools-rename-undo =
    { $n ->
        [one] ১টি আইটেমের নাম বদল ফিরিয়ে নিন
       *[other] { $n }টি আইটেমের নাম বদল ফিরিয়ে নিন
    }

folders-tools-rename-done =
    { $n ->
        [one] ১টি আইটেমের নাম বদলানো হয়েছে।
       *[other] { $n }টি আইটেমের নাম বদলানো হয়েছে।
    }

folders-tools-rename-stopped = নাম বদল মাঝপথে থেমে গেছে: { $error }।

folders-tools-name-empty = নাম খালি রাখা যায় না।

folders-tools-looked-at = { $done }টি দেখা হয়েছে

folders-tools-read-of = { $total }-এর মধ্যে { $done } পড়া হয়েছে

folders-tools-listing-files = ফাইলের তালিকা করা হচ্ছে

folders-tools-comparing = তুলনা করা হচ্ছে

folders-tools-looking = দেখা হচ্ছে

folders-tools-compress-title =
    { $n ->
        [one] { $name } কম্প্রেস করুন
       *[other] { $n }টি আইটেম কম্প্রেস করুন
    }

folders-tools-pack-name-tip = আর্কাইভের নাম

folders-tools-pack-kind-tip = আর্কাইভের ধরন

folders-tools-pack-made-in = { $place }-এ তৈরি হবে

folders-tools-compress = কম্প্রেস করুন

folders-tools-extract-to = { $name } এখানে খুলুন

folders-tools-extract-new-folder = সেখানে { $name } নামে একটি নতুন ফোল্ডার তৈরি হবে।

folders-tools-resize-title =
    { $n ->
        [one] { $name }-এর মাপ বদলান
       *[other] { $n }টি ছবির মাপ বদলান
    }

folders-tools-resize-size-tip = নতুন মাপ

folders-tools-resize-note = ছবির মাপের একটি অংশ, বা লম্বা দিকে কত পিক্সেল; ছবি কখনও বড় করা হয় না। প্রতিটির পাশে নতুন মাপসহ নামে একটি কপি তৈরি হয়; আসল ছবি যেমন আছে তেমনই থাকে।

folders-tools-resize = মাপ বদলান

folders-tools-wipe-title =
    { $n ->
        [one] { $name } ওয়াইপ করবেন?
       *[other] { $n }টি ফাইল ওয়াইপ করবেন?
    }

folders-tools-wipe-warning = প্রতিটি ফাইল সরানোর আগে এর ওপর অন্য তথ্য লেখা হয়, তাই আর ফেরত আনা যায় না। এটি ফেরানো যাবে না।

folders-tools-wipe-write-over = ওপরে লিখুন

folders-tools-wipe-passes-tip = প্রতিটি ফাইলের ওপর কতবার লেখা হবে

folders-tools-wipe-button = নিরাপদে ওয়াইপ করুন

folders-tools-rename-number-tip = নম্বর দিন

folders-tools-rename-at-start = শুরুতে

folders-tools-rename-at-end = শেষে

folders-tools-rename-where-tip = নম্বর কোথায় বসবে

folders-tools-rename-first-number-tip = প্রথম নম্বর

folders-tools-rename-ending-tip = শেষাংশের কী হবে

folders-tools-rename-new-ending-tip = নতুন শেষাংশ, যেমন txt

folders-tools-rename-cannot = করা যাবে না: { $why }

folders-tools-rename-was = আগে ছিল { $old }

folders-tools-rename-stays = যেমন আছে তেমনই থাকবে

folders-tools-rename-title =
    { $n ->
        [one] ১টি আইটেমের নাম বদলান
       *[other] { $n }টি আইটেমের নাম বদলান
    }

folders-tools-rename-replace = প্রতিস্থাপন

folders-tools-rename-find-tip = খুঁজুন

folders-tools-rename-with-tip = এটি দিয়ে বদলান

folders-tools-rename-wildcard-tip = * ও ? ওয়াইল্ডকার্ড হিসেবে ব্যবহার করুন

folders-tools-rename-wildcard = ওয়াইল্ডকার্ড * ও ?

folders-tools-rename-match-case-tip = বড় হাতের অক্ষর হুবহু মেলান

folders-tools-rename-match-case = অক্ষরের ধরন মেলান

folders-tools-rename-add = যোগ করুন

folders-tools-rename-before-tip = নামের আগে

folders-tools-rename-after-tip = নামের পরে

folders-tools-rename-number = নম্বর

folders-tools-rename-letters = অক্ষর

folders-tools-rename-case-tip = নামের বড় হাতের অক্ষর

folders-tools-rename-ending = শেষাংশ

folders-tools-rename-summary =
    { $clash ->
        [yes] কিছু নাম ব্যবহার করা যাবে না; চিহ্নিত সারিগুলো দেখুন। { $place }-এ।
       *[no] { $total }টির মধ্যে { $changes }টি নাম বদলাবে। { $place }-এ।
    }

folders-tools-rename-button = নাম বদলান

folders-tools-home = হোম

folders-tools-dupes-title =
    { $n ->
        [one] { $folder }-এর ডুপ্লিকেট
       *[other] { $n }টি ফোল্ডারের ডুপ্লিকেট
    }

folders-tools-dupes-stopped = খোঁজা থেমে গেছে: { $error }।

folders-tools-dupes-none = কোনো ডুপ্লিকেট পাওয়া যায়নি।

folders-tools-dupes-unread =
    { $n ->
        [one] ১টি ফাইল পড়া যায়নি।
       *[other] { $n }টি ফাইল পড়া যায়নি।
    }

folders-tools-dupes-sets =
    { $n ->
        [one] ১ সেট কপি; বাড়তি কপিগুলো ট্র্যাশে দিলে { $size } খালি হবে। বাড়তি কপিগুলো বাছা আছে; Ctrl চেপে ক্লিক করলে একটি বাছা বা বাদ দেওয়া যায়।
       *[other] { $n } সেট কপি; বাড়তি কপিগুলো ট্র্যাশে দিলে { $size } খালি হবে। বাড়তি কপিগুলো বাছা আছে; Ctrl চেপে ক্লিক করলে একটি বাছা বা বাদ দেওয়া যায়।
    }

folders-tools-dupes-row = সেট { $set }, { $size }, { $at }-এ

folders-tools-dupes-unread-left-out =
    { $n ->
        [one] ১টি ফাইল পড়া যায়নি, তাই বাদ দেওয়া হয়েছে।
       *[other] { $n }টি ফাইল পড়া যায়নি, তাই বাদ দেওয়া হয়েছে।
    }

folders-tools-dupes-keep-one = একটি সেটের সব কপি বাছা আছে; অন্তত একটি রাখুন।

folders-tools-dupes-pick-extras = বাড়তি কপিগুলো বাছুন

folders-tools-dupes-trash = ট্র্যাশে পাঠান

folders-tools-diff-title = দুটি তুলনা করুন

folders-tools-diff-failed = তুলনা করা যায়নি: { $error }।

folders-tools-diff-same-bytes = দুটি বাইটে বাইটে একই।

folders-tools-diff-same-as-shorter = ছোটটি যতদূর যায় ততদূর দুটি একই; একটি { $one } আর অন্যটি { $other }।

folders-tools-diff-from-byte = বাইট { $n } থেকে দুটি আলাদা।

folders-tools-diff-lines =
    { $n ->
        [0] দুটি একই।
        [one] ১টি লাইন আলাদা।
       *[other] { $n }টি লাইন আলাদা।
    }

folders-tools-diff-too-different = দুটি এত আলাদা যে লাইন ধরে মেলানো যায় না, তাই প্রথম ও শেষ পার্থক্যের মধ্যে সব লাইন বদলানো হিসেবে দেখায়।

folders-tools-diff-cut = প্রতিটির শুধু প্রথম ৪ MB তুলনা করা হয়।

folders-tools-compare-title = { $left } ও { $right } তুলনা করুন

folders-tools-compare-bytes = প্রতিটি বাইট পড়ুন (ধীর)

folders-tools-compare-bytes-tip = প্রতিটি ফাইলের ভিতরে কী আছে তুলনা করুন

folders-tools-compare-show-same = যেগুলো একই তাও দেখান

folders-tools-compare-show-same-tip = যে নামগুলো একই সেগুলোও দেখান

folders-tools-compare-stopped = তুলনা থেমে গেছে: { $error }।

folders-tools-compare-counts = শুধু বাঁয়ে { $left }টি, শুধু ডানে { $right }টি, আলাদা { $different }টি, একই { $same }টি।

folders-tools-compare-all-same = দুটি ফোল্ডারে একই জিনিস আছে।

folders-tools-compare-again = আবার তুলনা করুন

folders-tools-compare-copy-left = নেই বা নতুন { $n }টি বাঁয়ে কপি করুন

folders-tools-compare-copy-right = নেই বা নতুন { $n }টি ডানে কপি করুন
