# machine translated (Bengali), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = { $host } নামের কোনো সার্ভার নেই

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = সবটা পৌঁছায়নি, তাই মূলটি সরানো হয়নি

# Reason a link was not sent to a server.
folders-far-not-followed = এটি একটি লিংক, তাই Folders এটি পাঠায়নি

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = এটি ফাইল বা ফোল্ডার নয়, তাই Folders এটি পাঠায়নি

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = এটি কী ধরনের আইটেম তা Folders বুঝতে পারেনি, তাই সেটি সেখানেই রেখে দিয়েছে

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = এই ধরনের সার্ভারে একটি ফাইল একই নামের অন্য ফাইলকে বদলে দিতে পারে।

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = সার্ভারের ফাইলে এটি করা যায় না

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders এটি পড়তে পারেনি

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders এটি খুলতে পারেনি।

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (ফোল্ডার নিজের ভিতরে যেতে পারে না)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (এই নামে আগে থেকেই একটি ফোল্ডার আছে)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (এই নামে আগে থেকেই একটি ফাইল আছে)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders সার্ভারের ঠিকানা জানে না।

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = পাঠানো কপিটির আকার ঠিক ছিল না

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = এটি শুধু সার্ভারের ফাইলের ক্ষেত্রে কাজ করে।

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders নতুন ফাইলটি জায়গামতো রাখতে পারেনি, তাই পুরনোটি এখনও আছে, তার নাম এখন { $old }, আর নতুনটির নাম { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders পুরনো ফাইলটি সরাতে পারেনি। সেটি এখনও আছে, তার নাম এখন { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders পুরনো ফাইলটি সরাতে পারেনি, তাই সেটি এখনও আছে, তার নাম এখন { $old }: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } খালি নয়।

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders একটি ফাইল বাদ দিয়েছে, কারণ তার নামটি ব্যবহার করা নিরাপদ নয়।

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
        [one] Folders একটি আইটেম বাদ দিয়েছে, কারণ তার নামটি ব্যবহার করা নিরাপদ নয়: { $names }।
       *[other] Folders { $n }টি আইটেম বাদ দিয়েছে, কারণ সেগুলোর নাম ব্যবহার করা নিরাপদ নয়: { $names }।
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = { $name } ডাউনলোড করা হচ্ছে…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders { $name } ডাউনলোড করতে পারেনি: { $why }।

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = { $name }-এর একটি কপি খোলা হয়েছে। আপনি সংরক্ষণ করলে আপনার বদলগুলো সার্ভারে চলে যাবে।

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = { $name } সার্ভারে সংরক্ষণ করা হয়েছে।

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders { $name } সার্ভারে সংরক্ষণ করতে পারেনি: { $why }। আপনার বদলগুলো এই কম্পিউটারের কপিতে রয়ে গেছে।

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was }-এর নাম বদলে { $now } করা হয়েছে।

# Heading of the Connect to a server card.
folders-far-connect-title = সার্ভারের সাথে যুক্ত হোন

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = বন্ধ করুন

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = ঠিকানা, যেমন sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
# SAFETY: needs a human review before it ships
folders-far-connect-password = পাসওয়ার্ড, দরকার হলে

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = কী ফাইল, ব্যবহার করলে

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = কী ফাইল বেছে নিন


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders আপনার ssh সেটিংস, ssh এজেন্ট, কী ও চেনা সার্ভারের তালিকা ব্যবহার করে।

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = আপনি আগে কখনও { $host }-এ সংযোগ করেননি, তাই এটি ঠিক সার্ভার কি না Folders যাচাই করতে পারে না। এতে সংযোগ করার কথা থাকলে তবেই বিশ্বাস করুন।

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = { $host }-এর পরিচয় আগের বার থেকে আলাদা। কেউ আপনার সংযোগ মাঝপথে আটকানোর চেষ্টা করতে পারে, তাই Folders সংযোগ করেনি। মালিক বদলে থাকলে আগে আপনার চেনা হোস্টের তালিকা থেকে এর পুরনো এন্ট্রি সরিয়ে দিন।

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-password = এই সার্ভারে পাসওয়ার্ড লাগে। ওপরে লিখে আবার চেষ্টা করুন।

# Note on the Connect to a server card when a server asks for a key or password.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-key = এই সার্ভারে কী বা পাসওয়ার্ড লাগে। ওপরে আপনার কী ফাইল বেছে নিন, বা এটি আপনার ssh এজেন্টে যোগ করুন, তারপর আবার চেষ্টা করুন।

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = Folders { $host }-এর সার্টিফিকেট যাচাই করতে পারেনি: { $why }। পারলে এর ফিঙ্গারপ্রিন্ট মালিকের দেওয়া ফিঙ্গারপ্রিন্টের সঙ্গে মিলিয়ে দেখুন। মিললে তবেই বিশ্বাস করুন।

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = আপনি বিশ্বাস করার পর থেকে { $host }-এর সার্টিফিকেট বদলে গেছে। মালিক হয়তো এটি নবায়ন করেছেন, অথবা কেউ আপনার সংযোগ মাঝপথে আটকানোর চেষ্টা করছে। মালিক এই ফিঙ্গারপ্রিন্ট নিশ্চিত করলে তবেই বিশ্বাস করুন।

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = যুক্ত হচ্ছে

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = বিশ্বাস করে যুক্ত হোন

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = যুক্ত হোন

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = ফোন প্লাগ করে আনলক করলে সেগুলো সাইডবারে দেখা যায়।
