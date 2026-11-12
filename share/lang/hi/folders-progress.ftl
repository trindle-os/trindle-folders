
# machine translated (Hindi), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $of } में से { $done } फ़ाइलें, एक सेकंड में { $rate }, { $left }
       *[no] एक सेकंड में { $rate }, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $of } में से { $done } फ़ाइलें, अंदाज़ा लगाया जा रहा है कितना समय लगेगा…
       *[no] अंदाज़ा लगाया जा रहा है कितना समय लगेगा…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }। { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } बाइट

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = कुछ सेकंड बाकी

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] लगभग 1 सेकंड बाकी
       *[other] लगभग { $n } सेकंड बाकी
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = लगभग एक मिनट बाकी

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] लगभग 1 मिनट बाकी
       *[other] लगभग { $n } मिनट बाकी
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] लगभग 1 घंटा बाकी
       *[other] लगभग { $n } घंटे बाकी
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] 1 और काम चल रहा है
       *[other] { $n } और काम चल रहे हैं
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] ले जाने से डिस्क व्यस्त रहती है, इसलिए खत्म होने तक दूसरे प्रोग्राम धीमे चल सकते हैं।
       *[copying] कॉपी करने से डिस्क व्यस्त रहती है, इसलिए खत्म होने तक दूसरे प्रोग्राम धीमे चल सकते हैं।
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = जवाब की प्रतीक्षा है

# End of a piece of work's line while it is paused.
folders-progress-paused = रुका हुआ

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = जारी रखें

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = रोकें

# Tip of the button on a piece of work's row that stops it.
# SAFETY: needs a human review before it ships
folders-progress-stop = बंद करें

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = नया और बड़ा

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = नया

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = बड़ा

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = { $when } बदला गया

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] { $name } नाम का फ़ोल्डर पहले से मौजूद है
        [file] { $name } नाम की फ़ाइल पहले से मौजूद है
       *[unknown] { $name } पहले से मौजूद है
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = नई वाली

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = जो पहले से वहाँ है

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = छोड़ें

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = दोनों रखें

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = बदलें

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = बदलें, और पहले से मौजूद फ़ोल्डर को ट्रैश में डालें

# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = मिलाएँ

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = पहले से मौजूद हर फ़ोल्डर के लिए यही उत्तर इस्तेमाल करें।

# Line over the switch in the name taken question for files.
folders-progress-all-files = पहले से मौजूद हर फ़ाइल के लिए यही उत्तर इस्तेमाल करें।

# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = बाकी के लिए भी यही करें

