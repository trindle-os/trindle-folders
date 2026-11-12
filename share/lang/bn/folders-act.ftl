# machine translated (Bengali), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = সার্ভারের ফাইলে কাট, কপি ও পেস্ট কাজ করে না। তার বদলে প্যানের মাঝের কপি ও সরানোর বোতাম ব্যবহার করুন।

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = আর্কাইভের ভেতরে কিছু বদলানো যায় না। আগে যা দরকার তা বের করে নিন।

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = এগুলো অন্য অ্যাপে কাটা হয়েছিল, তাই Folders সেগুলো কপি করেছে।

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = এটি ইতিমধ্যে কপি বা সরানো হচ্ছে।

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = এটি এখনও এখানে কপি হচ্ছে।

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = ১০,০০০ বা তার বেশি আইটেম

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] ১টি আইটেম
       *[other] { $n }টি আইটেম
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] একটি আইটেম { $verb } যায়নি: { $names }।
       *[other] { $n }টি আইটেম { $verb } যায়নি: { $names }।
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }। { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } এবং আরও { $more }টি

# Words on a job card while things are copied: what, and where to.
folders-act-copying = { $what } কপি করা হচ্ছে { $place }-এ

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = { $what } কপি করা হয়েছে { $place }-এ।

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = কপি করা

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = { $what } কপি করা ফিরিয়ে নিন

# Words on a job card while things are moved: what, and where to.
folders-act-moving = { $what } সরানো হচ্ছে { $place }-এ

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = { $what } সরানো হয়েছে { $place }-এ।

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = সরানো

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = { $what } সরানো ফিরিয়ে নিন

# Words on a job card while things are put in the Trash.
folders-act-trashing = { $what } ট্র্যাশে ফেলা হচ্ছে

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = { $what } ট্র্যাশে ফেলা হয়েছে।

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = ট্র্যাশে ফেলা

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = { $what } ট্র্যাশে ফেলা ফিরিয়ে নিন

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = { $what } মুছে ফেলা হচ্ছে

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = { $what } চিরতরে মুছে ফেলা হয়েছে।

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = মুছে ফেলা

# Words on a job card while things are put back from the Trash.
folders-act-restoring = { $what } ফিরিয়ে রাখা হচ্ছে

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = { $what } ফিরিয়ে রাখা হয়েছে।

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = { $what } ফিরিয়ে রাখা ফিরিয়ে নিন

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = ফিরিয়ে রাখা

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = ফিরিয়ে নেওয়া হচ্ছে

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = ফিরিয়ে নেওয়া হয়েছে।

# Words on a job card while things are packed into an archive.
folders-act-compressing = { $what } সংকুচিত করা হচ্ছে

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = { $place }-এ { $file } তৈরি হয়েছে।

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = কম্প্রেস করা

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = { $what } সংকুচিত করা ফিরিয়ে নিন

# Words on a job card while an archive is unpacked.
folders-act-extracting = { $what } খোলা হচ্ছে

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = { $what } খুলে { $place }-এ রাখা হয়েছে।

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = খোলা

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = { $what } খোলা ফিরিয়ে নিন

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = { $what } নিরাপদে মুছে ফেলা হচ্ছে

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = { $what } নিরাপদে মুছে ফেলা হয়েছে।

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = নিরাপদে মুছে ফেলা

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = { $what } বের করা হচ্ছে

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = আর্কাইভ থেকে { $what } বের করে { $place }-এ রাখা হয়েছে।

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = বের করা

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = { $what } বের করা ফিরিয়ে নিন

# Words on a job card while pictures are turned.
folders-act-turning = { $what } ঘোরানো হচ্ছে

# Words under the file lines when pictures have been turned.
folders-act-turned = { $what } ঘোরানো হয়েছে।

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = ঘোরানো

# Words on a job card while pictures are resized.
folders-act-resizing = { $what } এর আকার বদলানো হচ্ছে

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = { $what } এর আকার বদলানো একটি কপি তৈরি হয়েছে।

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = আকার বদলানো

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = { $what } এর আকার বদলানো ফিরিয়ে নিন

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = নতুন ফোল্ডার

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = নতুন নথি

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders { $base } তৈরি করতে পারেনি: { $why }।

# Red words under a name being typed when it is empty.
folders-act-name-empty = একটি নাম লিখুন।

# Red words under a name being typed when it has a slash.
folders-act-name-slash = নামে স্ল্যাশ থাকতে পারবে না।

# Red words under a name being typed when it is . or ..
folders-act-name-kept = নামটি সিস্টেম ব্যবহার করে। অন্য একটি নাম দিন।

# Red words under a name being typed when it is too long.
folders-act-name-long = নামটি অনেক লম্বা।

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders কাজটি শুরু করতে পারেনি।

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = { $what } আবার করুন

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = { $what } ফিরিয়ে নিন

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = কাজটি শেষ হওয়ার আগেই থামানো হয়েছে।

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [one] Folders একটি আইটেম বাদ দিয়েছে।
       *[other] Folders { $n }টি আইটেম বাদ দিয়েছে।
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = আগে কিছু বেছে নিন।

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = এটি ইতিমধ্যে ট্র্যাশে আছে।

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = সার্ভারের ফাইল ট্র্যাশে যায় না। তার বদলে "চিরতরে মুছুন" ব্যবহার করুন।

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = আগে কপি বা সরানোর জন্য কিছু বেছে নিন।

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = আর্কাইভ থেকে জিনিস শুধু ফোল্ডারে বের করা যায়।

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = কপি ট্র্যাশে ফেলা যায় না।

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = অন্য প্যানে কোনো ফোল্ডার দেখানো হচ্ছে না।

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = আগে কপি করার জন্য কিছু বেছে নিন।

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = { $what } কাটা হয়েছে। একটি ফোল্ডারে পেস্ট করুন।

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = { $what } কপি করা হয়েছে। একটি ফোল্ডারে পেস্ট করুন।

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = জিনিস শুধু ফোল্ডারে পেস্ট করা যায়।

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = ক্লিপবোর্ডে কোনো ফাইল নেই।

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = কিছু কাটা বা কপি করা হয়নি।

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = নেটওয়ার্কের জায়গা এখান থেকে মোছা যায় না।

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = ট্র্যাশ ইতিমধ্যে খালি।

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = আগে ট্র্যাশে কিছু বেছে নিন।

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = { $name } তৈরি হচ্ছে

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = নতুন ফোল্ডার বা নথি শুধু একটি ফোল্ডারের ভেতরে তৈরি করা যায়।

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = { $name } তৈরি করা ফিরিয়ে নিন

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders সরানো আইটেমগুলোর ফোল্ডারের রং ধরে রাখতে পারেনি।

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = আবার করার মতো কিছু নেই।

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = ফিরিয়ে নেওয়ার মতো কিছু নেই।

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = { $what } আবার করা হচ্ছে

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = { $what } আবার করা হয়েছে।

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = আবার করা

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = { $what } ফিরিয়ে নেওয়া হচ্ছে

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = { $what } ফিরিয়ে নেওয়া হয়েছে।

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = হোম

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = ট্র্যাশ

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = জিনিস শুধু ফোল্ডার বা ট্র্যাশে ছাড়া যায়।

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] ১টি লিংক
       *[other] { $n }টি লিংক
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = { $what } তৈরি হয়েছে।

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] একটি লিংক তৈরি করা যায়নি: { $names }।
       *[other] { $n }টি লিংক তৈরি করা যায়নি: { $names }।
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = একটি প্রশ্ন আপনার উত্তরের অপেক্ষায় আছে। Folders বন্ধ করার আগে উত্তর দিন, বা কাজটি থামান।

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = কোনো কাজ চলার সময় Folders বন্ধ করা যায় না। কাজটি শেষ হওয়া পর্যন্ত অপেক্ষা করুন, বা থামিয়ে দিন, তারপর Folders বন্ধ করুন।
