# machine translated (Bengali), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = আগের কপির ভেতরে কিছু বদলানো যায় না। রাখতে চাইলে জিনিসগুলো সেখান থেকে কপি করে নিন।

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] আজ { $time }-এ
        [yesterday] গতকাল { $time }-এ
       *[other] { $date } { $time }-এ
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] আজ { $time }-এ
        [yesterday] গতকাল { $time }-এ
       *[other] { $date } { $time }-এ
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = এটি { $when } তারিখের আগের একটি কপি। এটি বদলানো যায় না, তবে এর ভেতর থেকে জিনিস কপি করা যায়।

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = এটি আগের একটি কপি। এটি বদলানো যায় না, তবে এর ভেতর থেকে জিনিস কপি করা যায়।

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = আগের: { $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = { $name } ফিরিয়ে রাখা ফিরিয়ে নিন

# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = Folders এটি ফিরিয়ে রেখেছে। বর্তমানটি নতুন নামে এর পাশেই আছে।

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = Folders এটি আগের জায়গায় ফিরিয়ে রেখেছে।

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = এটি বর্তমানটির মতোই।

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = এটি বর্তমানটির থেকে আলাদা।

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = Folders সেগুলো তুলনা করতে পারেনি: { $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = আগের সংস্করণ

# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = এখান থেকে মোছা জিনিস

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = { $name }-এর আগের সংস্করণ

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = বর্তমানটির থেকে আলাদা এমন কোনো আগের সংস্করণ নেই।

# Note at the foot of the earlier versions card about what putting one back does.
folders-earlier-versions-foot = একটি ফিরিয়ে রাখলে বর্তমানটি তার পাশেই থেকে যায়। আপনি এটি ফিরিয়ে নিতে পারেন।

# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = { $name } থেকে মোছা জিনিস

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = আগের কপিগুলোর কিছুই এই ফোল্ডার থেকে মুছে ফেলা হয়নি।

# Note at the foot of the deleted things card about what putting one back does.
folders-earlier-deleted-foot = একটি ফিরিয়ে রাখলে সেটি এই ফোল্ডারে ফিরে আসে। আপনি এটি ফিরিয়ে নিতে পারেন।

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = ফোল্ডারটি তখন যেমন ছিল

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }, { $date } তারিখে তৈরি কপি থেকে

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = একটি ফোল্ডার, শেষবার দেখা গেছে { $when }

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }, শেষবার দেখা গেছে { $when }

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = কাজ চলছে…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = সেই দিনের ফোল্ডার নতুন ট্যাবে খুলুন

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = খুলুন

# Tip and name of the round button on a row of the earlier versions card that puts it back.
folders-earlier-put-back-tip = ফিরিয়ে রাখুন

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = বর্তমানটির সঙ্গে তুলনা করুন

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = এটি আগের কোনো সংস্করণ নয়।

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = আপনি আগের সংস্করণ শুধু আপনার হোম ফোল্ডারের ভেতরে ফিরিয়ে রাখতে পারেন।

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = Folders সেখানে ফাইল ফিরিয়ে রাখতে পারে না।

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = Folders যে ফোল্ডারে এটি ছিল তা আবার তৈরি করতে পারেনি: { $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = Folders আগের সংস্করণ কপি করতে পারেনি: { $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = Folders বর্তমানটিকে অন্য জায়গায় সরিয়ে রাখতে পারেনি: { $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = Folders আগের সংস্করণটি ফিরিয়ে রাখতে পারেনি: { $why }। বর্তমানটির নাম এখন { $name }।

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = Folders আগের সংস্করণটি ফিরিয়ে রাখতে পারেনি: { $why }
