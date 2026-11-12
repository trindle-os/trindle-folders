# machine translated (Bengali), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders { $name }-এর নাম বদলাতে পারেনি: { $why }।

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] { $name } নামে একটি ফোল্ডার এখানে আগে থেকেই আছে।
       *[other] { $name } নামে একটি ফাইল এখানে আগে থেকেই আছে।
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = এখানে কিছুর নাম বদলানো যায় না।

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = আগে নাম বদলানোর জন্য কিছু বেছে নিন।

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = { $name } তৈরি পূর্বাবস্থায় ফেরান

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = { $old } থেকে { $name } নাম বদল পূর্বাবস্থায় ফেরান

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = { $old } এর নাম বদলে { $name } করা হয়েছে।

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = { $old }-এর নাম বদলে { $name } করা হচ্ছে…
