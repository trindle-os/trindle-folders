# machine translated (Bengali), not yet reviewed by a person.

# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-card = Folders স্থানীয় নেটওয়ার্কে { $at }, পোর্ট { $port }-এ { $host } খুঁজে পেয়েছে। সেখানে যেকোনো ডিভাইস ওই নাম ব্যবহার করতে পারে, তাই Folders আপনার পাসওয়ার্ড পাঠায়নি। এটি সঠিক ঠিকানা হলে সংযুক্ত করুন চাপুন।

# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-stopped = Folders স্থানীয় নেটওয়ার্কে { $at } ঠিকানায় { $host } খুঁজে পেয়েছে। সেখানে যেকোনো ডিভাইস ওই নাম ব্যবহার করতে পারে, তাই Folders আপনার পাসওয়ার্ড পাঠায়নি। ঠিকানা যাচাই করতে { $host } আবার খুলুন।

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders ঠিকানাটি পড়তে পারেনি: { $why }।

# Red words when a server needs a password or key and none could be asked for.
# SAFETY: needs a human review before it ships
folders-remote-mod-needs-password = এই সার্ভারের জন্য পাসওয়ার্ড বা কী লাগবে।

# Red words when a server turned down the login.
# SAFETY: needs a human review before it ships
folders-remote-mod-login-refused = সার্ভার এই লগইন গ্রহণ করেনি।

# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-unknown = আপনি আগে কখনো { $host }-এ সংযোগ করেননি, তাই Folders এখনও এর পরিচয় জানে না।

# Red words when a server's identity differs from the one remembered; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-changed = { $host }-এর পরিচয় আগের বারের থেকে আলাদা। সেটি নতুন করে বানানো হয়ে থাকতে পারে, অথবা কেউ সংযোগটি মাঝপথে আটকাচ্ছে।

# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-unknown = Folders { $host }-এর সার্টিফিকেট যাচাই করতে পারেনি: { $why }।

# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-changed = আপনি বিশ্বাস করার পর থেকে { $host }-এর সার্টিফিকেট বদলে গেছে।

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders সংযোগ করতে পারেনি: { $why }।

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders { $what } খুঁজে পাচ্ছে না।

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } আগে থেকেই আছে।

# Red words when a server or phone refused permission; the variable is what it was refused for.
# SAFETY: needs a human review before it ships
folders-remote-mod-denied = { $what } ব্যবহারের অনুমতি আপনার নেই।

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } অন্য একটি প্রোগ্রামে ব্যবহার হচ্ছে।

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } সমর্থিত নয়।

# Red words when a copy or other work with a server was cancelled.
# SAFETY: needs a human review before it ships
folders-remote-mod-cancelled = বাতিল করা হয়েছে।

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = লিংক অনুসরণ করা

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = খোলার আগে উইন্ডোজ শেয়ারে বা শেয়ার থেকে কপি করা

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = খোলার আগে NFS শেয়ারে বা শেয়ার থেকে কপি করা

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = এটি sftp://, ftp:// বা mtp:// দিয়ে শুরু হয়নি

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = এটি অবশ্যই sftp://, ftp://, ftps://, ftpes://, dav://, davs://, smb://, nfs:// বা mtp:// দিয়ে শুরু হতে হবে

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = একটি [ আছে কিন্তু ] নেই

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = কোনো সার্ভারের নাম নেই

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = সার্ভারের নামটি সঠিক নয়

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = পথে একটি লাইন ব্রেক আছে

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = সার্ভারের নামের পরে বাড়তি কিছু আছে

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } সঠিক পোর্ট নম্বর নয়

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = সার্ভার একটি অনিরাপদ নাম পাঠিয়েছে বলে বাদ দেওয়া হয়েছে

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = নামহীন কিছু

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = উত্তরটি অনেক বেশি বড় ছিল, তাই Folders সেটি থামিয়ে দিয়েছে।

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } এখন একটি ফোল্ডার, তাই Folders এটি সরায়নি।
