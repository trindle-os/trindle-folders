
# machine translated (Bengali), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $of }টির মধ্যে { $done }টি ফাইল, প্রতি সেকেন্ডে { $rate }, { $left }
       *[no] প্রতি সেকেন্ডে { $rate }, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $of }টির মধ্যে { $done }টি ফাইল, কতক্ষণ লাগবে হিসাব চলছে…
       *[no] কতক্ষণ লাগবে হিসাব চলছে…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }। { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } বাইট

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = আর কয়েক সেকেন্ড বাকি

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] আর প্রায় { $n } সেকেন্ড বাকি
       *[other] আর প্রায় { $n } সেকেন্ড বাকি
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = আর প্রায় এক মিনিট বাকি

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] আর প্রায় { $n } মিনিট বাকি
       *[other] আর প্রায় { $n } মিনিট বাকি
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] আর প্রায় { $n } ঘণ্টা বাকি
       *[other] আর প্রায় { $n } ঘণ্টা বাকি
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] আরও { $n }টি কাজ চলছে
       *[other] আরও { $n }টি কাজ চলছে
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] সরানোর সময় ডিস্ক ব্যস্ত থাকে, তাই শেষ না হওয়া পর্যন্ত অন্য প্রোগ্রাম ধীর হতে পারে।
       *[copying] কপি করার সময় ডিস্ক ব্যস্ত থাকে, তাই শেষ না হওয়া পর্যন্ত অন্য প্রোগ্রাম ধীর হতে পারে।
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = উত্তরের অপেক্ষায়

# End of a piece of work's line while it is paused.
folders-progress-paused = থামানো আছে

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = চালিয়ে যান

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = থামান

# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = বন্ধ করুন

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = নতুন ও বড়

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = নতুন

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = বড়

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = বদলেছে { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] { $name } নামের একটি ফোল্ডার আগে থেকেই আছে
        [file] { $name } নামের একটি ফাইল আগে থেকেই আছে
       *[unknown] { $name } আগে থেকেই আছে
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = নতুনটি

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = যেটি আগে থেকে আছে

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = বাদ দিন

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = দুটোই রাখুন

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = বদলে দিন

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = বদলে দিন, আর আগে থেকে থাকা ফোল্ডারটি ট্র্যাশে সরিয়ে দিন

# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = মিলিয়ে দিন

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = আগে থেকে থাকা প্রতিটি ফোল্ডারে এই উত্তরই ব্যবহার করুন।

# Line over the switch in the name taken question for files.
folders-progress-all-files = আগে থেকে থাকা প্রতিটি ফাইলে এই উত্তরই ব্যবহার করুন।

# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = বাকিগুলোর জন্য একই করুন

