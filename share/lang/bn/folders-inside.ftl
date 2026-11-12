
# machine translated (Bengali), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = আগে আর্কাইভের ভেতর থেকে কিছু বেছে নিন।

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = আপনি আর্কাইভ থেকে জিনিস শুধু এই কম্পিউটারের ফোল্ডারে বের করে রাখতে পারেন।

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = Folders { $name } পড়তে পারেনি: { $why }।

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
        [one] Folders ১টি আইটেম বের করতে পারেনি, কারণ এর নাম ফোল্ডারের বাইরের দিকে যায় অথবা পথে কিছু আছে।
       *[other] Folders { $n }টি আইটেম বের করতে পারেনি, কারণ সেগুলোর নাম ফোল্ডারের বাইরের দিকে যায় অথবা পথে কিছু আছে।
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = Folders আর্কাইভ থেকে এটি বের করতে পারেনি: { $why }।
