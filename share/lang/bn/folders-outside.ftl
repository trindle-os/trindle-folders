# machine translated (Bengali), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = লুকানো, অথবা লুকানো ফোল্ডারের ভেতরে

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = এর ভেতরে লুকানো আইটেম আছে

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = এতে এত বেশি আইটেম আছে যে যাচাই করা যায় না, তাই Folders এর লুকানো আইটেম বাদ দেবে

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = আপনার হোম ফোল্ডার ও ড্রাইভের বাইরে

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = লুকানো কিছুর, অথবা আপনার হোম ফোল্ডার ও ড্রাইভের বাইরের কিছুর লিংক

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = এর ভেতরে লুকানো কিছুর, অথবা আপনার হোম ফোল্ডার ও ড্রাইভের বাইরের কিছুর লিংক আছে

# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [one] { $name } এখানে সরাবেন?
       *[other] এই { $n }টি জিনিস এখানে সরাবেন?
    }

# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [one] { $name } এখানে আনবেন?
       *[other] এই { $n }টি জিনিস এখানে আনবেন?
    }

# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [one] { $name } এখানে কপি করবেন?
       *[other] এই { $n }টি জিনিস এখানে কপি করবেন?
    }

# Words on the question card when the drop would be moved: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-move =
    { $n ->
        [one] অন্য একটি প্রোগ্রাম এটি Folders-এ ফেলেছে। যেকোনো প্রোগ্রাম যেকোনো ফাইল ফেলতে পারে, আর এটি আপনার সাধারণ ফাইলগুলোর একটি নয়। আপনি না বলা পর্যন্ত Folders এটি সরাবে না।
       *[other] অন্য একটি প্রোগ্রাম এগুলো Folders-এ ফেলেছে। যেকোনো প্রোগ্রাম যেকোনো ফাইল ফেলতে পারে, আর এগুলোর কিছু আপনার সাধারণ ফাইল নয়। আপনি না বলা পর্যন্ত Folders এগুলো সরাবে না।
    }

# Words on the question card when the drop would be brought: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-bring =
    { $n ->
        [one] অন্য একটি প্রোগ্রাম এটি Folders-এ ফেলেছে। যেকোনো প্রোগ্রাম যেকোনো ফাইল ফেলতে পারে, আর এটি আপনার সাধারণ ফাইলগুলোর একটি নয়। আপনি না বলা পর্যন্ত Folders এটি আনবে না।
       *[other] অন্য একটি প্রোগ্রাম এগুলো Folders-এ ফেলেছে। যেকোনো প্রোগ্রাম যেকোনো ফাইল ফেলতে পারে, আর এগুলোর কিছু আপনার সাধারণ ফাইল নয়। আপনি না বলা পর্যন্ত Folders এগুলো আনবে না।
    }

# Words on the question card when the drop would be copied: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-copy =
    { $n ->
        [one] অন্য একটি প্রোগ্রাম এটি Folders-এ ফেলেছে। যেকোনো প্রোগ্রাম যেকোনো ফাইল ফেলতে পারে, আর এটি আপনার সাধারণ ফাইলগুলোর একটি নয়। আপনি না বলা পর্যন্ত Folders এটি কপি করবে না।
       *[other] অন্য একটি প্রোগ্রাম এগুলো Folders-এ ফেলেছে। যেকোনো প্রোগ্রাম যেকোনো ফাইল ফেলতে পারে, আর এগুলোর কিছু আপনার সাধারণ ফাইল নয়। আপনি না বলা পর্যন্ত Folders এগুলো কপি করবে না।
    }

# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
# SAFETY: needs a human review before it ships
folders-outside-leave =
    { $n ->
        [one] থাক
       *[other] থাক
    }

# Button on the question card that moves what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-move =
    { $n ->
        [one] সরান
       *[other] সরান
    }

# Button on the question card that brings what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-bring =
    { $n ->
        [one] আনুন
       *[other] আনুন
    }

# Button on the question card that copies what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-copy =
    { $n ->
        [one] কপি করুন
       *[other] কপি করুন
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, { $from } থেকে: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, { $from } থেকে

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = আরও { $n }টি।

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = গন্তব্য: { $into }।

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = আপনি যা ফেলেছেন তার প্রশ্নের উত্তর আগে দিন।

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = আপনি সর্বশেষ যা ফেলেছেন Folders এখনও তা যাচাই করছে।

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = এখানে শুধু ফাইল ও ফোল্ডার ফেলা যায়।

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = সাইডবারে শুধু ফোল্ডার পিন করা যায়।

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = জিনিস শুধু ফোল্ডারের ওপর ফেলা যায়।

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } আর সেখানে নেই, তাই কিছু কপি বা সরানো হয়নি।

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = ফোল্ডারকে তার নিজের ভেতরে রাখা যায় না।

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = সেগুলো আগে থেকেই ওই ফোল্ডারে আছে।

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = { $name } যে ড্রাইভে আছে Folders সময়মতো সেটির সঙ্গে যোগাযোগ করতে পারেনি, তাই কিছু কপি বা সরানো হয়নি।

# Words when the question card is answered with no.
folders-outside-nothing-brought = কিছু কপি বা সরানো হয়নি।

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] ১টি জিনিস
       *[other] { $n }টি জিনিস
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = এবং আরও { $n }টি

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }।

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] আপনি টানা শুরু করার পর { $n }টি আইটেম বদলে গেছে, তাই Folders সেটি আগের জায়গাতেই রেখে দিয়েছে: { $names }।
       *[other] আপনি টানা শুরু করার পর { $n }টি আইটেম বদলে গেছে, তাই Folders সেগুলো আগের জায়গাতেই রেখে দিয়েছে: { $names }।
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] { $n }টি আইটেম আগের জায়গাতেই আছে: { $each }।
       *[other] { $n }টি আইটেম আগের জায়গাতেই আছে: { $each }।
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = আপনার টেনে আনা সবকিছু অন্য প্রোগ্রামটি পেয়েছে কি না Folders বুঝতে পারছে না, তাই মূল জিনিসগুলো এখানেই রেখেছে। অন্য প্রোগ্রামের কাছে একটি কপি আছে।

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = অন্য প্রোগ্রামটি আপনার টেনে আনা সবকিছু নেয়নি, তাই Folders কিছু সরায়নি।

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = অন্য প্রোগ্রামটি আপনার টেনে আনা সবকিছু নেয়নি, তাই তার কপিটি সম্পূর্ণ নাও হতে পারে।
