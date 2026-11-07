

folders-progress-pace-known =
    { $files ->
        [yes] { $of }টির মধ্যে { $done }টি ফাইল, প্রতি সেকেন্ডে { $rate }, { $left }
       *[no] প্রতি সেকেন্ডে { $rate }, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $of }টির মধ্যে { $done }টি ফাইল, কতক্ষণ লাগবে হিসাব চলছে…
       *[no] কতক্ষণ লাগবে হিসাব চলছে…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } বাইট

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = আর কয়েক সেকেন্ড বাকি

folders-progress-left-seconds =
    { $n ->
        [one] আর প্রায় { $n } সেকেন্ড বাকি
       *[other] আর প্রায় { $n } সেকেন্ড বাকি
    }

folders-progress-left-minute = আর প্রায় এক মিনিট বাকি

folders-progress-left-minutes =
    { $n ->
        [one] আর প্রায় { $n } মিনিট বাকি
       *[other] আর প্রায় { $n } মিনিট বাকি
    }

folders-progress-left-hours =
    { $n ->
        [one] আর প্রায় { $n } ঘণ্টা বাকি
       *[other] আর প্রায় { $n } ঘণ্টা বাকি
    }

folders-progress-more =
    { $n ->
        [one] আরও { $n }টি কাজ চলছে
       *[other] আরও { $n }টি কাজ চলছে
    }

folders-progress-heavy =
    { $verb ->
        [moving] সরানোয় ডিস্কে অনেক চাপ পড়ে, তাই শেষ না হওয়া পর্যন্ত অন্য প্রোগ্রাম ধীর মনে হতে পারে।
       *[copying] কপি করায় ডিস্কে অনেক চাপ পড়ে, তাই শেষ না হওয়া পর্যন্ত অন্য প্রোগ্রাম ধীর মনে হতে পারে।
    }

folders-progress-waiting = উত্তরের অপেক্ষায়

folders-progress-paused = থামানো আছে

folders-progress-carry-on = চালিয়ে যান

folders-progress-pause = থামান

folders-progress-stop = বন্ধ করুন

folders-progress-mark-newer-larger = নতুন, বড়

folders-progress-mark-newer = নতুন

folders-progress-mark-larger = বড়

folders-progress-changed = বদলেছে { $when }

folders-progress-taken =
    { $kind ->
        [folder] { $name } নামের একটি ফোল্ডার আগে থেকেই আছে
        [file] { $name } নামের একটি ফাইল আগে থেকেই আছে
       *[unknown] { $name } আগে থেকেই আছে
    }

folders-progress-coming = যেটি আনা হচ্ছে

folders-progress-there = যেটি আগে থেকে আছে

folders-progress-skip = বাদ দিন

folders-progress-keep-both = দুটোই রাখুন

folders-progress-replace = বদলে দিন

folders-progress-replace-folder-spoken = বদলে দিন: আগে থেকে থাকা ফোল্ডারটি ট্র্যাশে যাবে

folders-progress-merge = মিলিয়ে দিন

folders-progress-all-folders = আগে থেকে থাকা প্রতিটি ফোল্ডারের জন্য একই উত্তর।

folders-progress-all-files = আগে থেকে থাকা প্রতিটি ফাইলের জন্য একই উত্তর।

folders-progress-same-for-rest = বাকিগুলোর জন্য একই করুন

