# machine translated (Bengali), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders এই ফোল্ডারের ভেতরের জিনিস পড়তে পারছে না।

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = আপনি শুধু ফোল্ডার নিজেই শেয়ার করতে পারেন, ফোল্ডারের লিংক নয়।

# Red words when a folder that belongs to someone else is chosen to share.
# SAFETY: needs a human review before it ships
folders-sharing-only-yours = আপনি শুধু নিজের ফোল্ডার শেয়ার করতে পারেন। এটি অন্য কারও।

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = আপনার হোম ফোল্ডার

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = এই ফোল্ডার

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } ও { $second }

# Name of the switch that shares the folder.
# SAFETY: needs a human review before it ships
folders-sharing-share = এই ফোল্ডার শেয়ার করুন

# One line under the share switch when the folder is shared.
folders-sharing-shared = বাসার নেটওয়ার্কে শেয়ার করা আছে।


# Tip of the share switch.
# SAFETY: needs a human review before it ships
folders-sharing-share-tip = বাসার নেটওয়ার্কে শেয়ার করুন

# Label over the box for the name other computers see.
folders-sharing-name = নাম

# Tip of the box for the name other computers see.
folders-sharing-name-tip = অন্য কম্পিউটারে যে নাম দেখা যায়

# Name of the switch that lets others change files, and its tip.
# SAFETY: needs a human review before it ships
folders-sharing-write = অন্যদের ফাইল বদলাতে দিন

# One line under the switch that lets others change files.
# SAFETY: needs a human review before it ships
folders-sharing-write-sub = তারা এতে ফাইল যোগ করতে, বদলাতে ও মুছতে পারবে।

# Name of the switch that lets anyone in without a password.
# SAFETY: needs a human review before it ships
folders-sharing-guests = যে কাউকে ঢুকতে দিন

# One line under the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-sub = পাসওয়ার্ড লাগবে না।

# Tip of the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-tip = পাসওয়ার্ড ছাড়াই যে কাউকে ঢুকতে দিন

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] শেয়ার করা হয়ে গেলে এটি বাসার আপনার অন্য কম্পিউটারে, উইন্ডোজেও, "নেটওয়ার্ক"-এর নিচে দেখা যাবে। প্রথমবার Trindle ফোল্ডার শেয়ার করার প্রোগ্রাম Samba ডাউনলোড করে। এতে এক মিনিট লাগে।
       *[no] শেয়ার করা হয়ে গেলে এটি বাসার আপনার অন্য কম্পিউটারে, উইন্ডোজেও, "নেটওয়ার্ক"-এর নিচে দেখা যাবে।
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] অন্য কম্পিউটারে "নেটওয়ার্ক"-এর নিচে { $host }-এ { $name } খুঁজুন।
       *[no] অন্য কম্পিউটারে "নেটওয়ার্ক"-এর নিচে { $name } খুঁজুন।
    }

# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = অন্য কম্পিউটার এখনও এই ফোল্ডারে পৌঁছাতে পারছে না। সেটিংসে নেটওয়ার্ক খুলুন এবং "অন্য কম্পিউটারকে যুক্ত হতে দিন" চালু করুন।

# Tip of the button that lets other computers connect.
# SAFETY: needs a human review before it ships
folders-sharing-let-connect = অন্য কম্পিউটারকে যুক্ত হতে দিন

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
# SAFETY: needs a human review before it ships
folders-sharing-visitors =
    { $n ->
        [one] পাসওয়ার্ড ছাড়া অতিথিরা এখনও এটি খুলতে পারবে না, কারণ { $names } অন্য অ্যাকাউন্টের জন্য বন্ধ। তাদের ঢুকতে দিলে তারা এই ফোল্ডারে পৌঁছাতে পারবে, কিন্তু পথের আর কিছু দেখতে পাবে না।
       *[other] পাসওয়ার্ড ছাড়া অতিথিরা এখনও এটি খুলতে পারবে না, কারণ { $names } অন্য অ্যাকাউন্টের জন্য বন্ধ। তাদের ঢুকতে দিলে তারা এই ফোল্ডারে পৌঁছাতে পারবে, কিন্তু পথের আর কিছু দেখতে পাবে না।
    }

# Tip of the button that lets visitors through to the folder.
# SAFETY: needs a human review before it ships
folders-sharing-let-through = অতিথিদের এই ফোল্ডারে পৌঁছাতে দিন

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
# SAFETY: needs a human review before it ships
folders-sharing-sign-in-set = অন্যরা শেয়ারের পাসওয়ার্ড দিয়ে { $user } হিসেবে লগ ইন করবে।

# Tip of the button that changes the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-change-password = শেয়ারের পাসওয়ার্ড বদলান

# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-set-password = অন্যরা { $user } হিসেবে লগ ইন করবে। কমপক্ষে { $n }টি অক্ষরের একটি শেয়ারের পাসওয়ার্ড ঠিক করুন। এটি আপনার লগইন পাসওয়ার্ড থেকে আলাদা।

# Label beside the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password = পাসওয়ার্ড

# Tip of the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password-tip = শেয়ারের পাসওয়ার্ড

# Tip of the button that keeps the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-keep-password = পাসওয়ার্ড সংরক্ষণ করুন

# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = { $name } নেটওয়ার্কে শেয়ার করুন

# Red words when the share's name is used by another share.
folders-sharing-name-taken = এই নামে আপনার একটি শেয়ার করা ফোল্ডার আগে থেকেই আছে। অন্য নাম বেছে নিন।

# Calm words while sharing is being stopped.
folders-sharing-stopping = শেয়ার বন্ধ করা হচ্ছে…

# Red words when the sharing password is too short; { $n } is the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-password-short = পাসওয়ার্ডে কমপক্ষে { $n }টি অক্ষর লাগবে।

# Calm words while the sharing password is being kept.
# SAFETY: needs a human review before it ships
folders-sharing-keeping-password = পাসওয়ার্ড সংরক্ষণ করা হচ্ছে। আগে আপনার কাছে অ্যাডমিনিস্ট্রেটরের পাসওয়ার্ড চাওয়া হবে।

# Calm words while other computers are being let connect.
# SAFETY: needs a human review before it ships
folders-sharing-letting-connect = অন্য কম্পিউটারকে যুক্ত হতে দেওয়া হচ্ছে। আগে আপনার কাছে অ্যাডমিনিস্ট্রেটরের পাসওয়ার্ড চাওয়া হবে।

# Calm words while Samba is fetched and set up the first time.
# SAFETY: needs a human review before it ships
folders-sharing-fetching = ফোল্ডার শেয়ার করার প্রোগ্রাম Samba ডাউনলোড করে সাজানো হচ্ছে। আগে আপনার কাছে অ্যাডমিনিস্ট্রেটরের পাসওয়ার্ড চাওয়া হবে। এটি শুধু প্রথমবার হয়।

# Calm words while sharing is set up the first time and Samba is already there.
# SAFETY: needs a human review before it ships
folders-sharing-setting-up = শেয়ার সাজানো হচ্ছে। আগে আপনার কাছে অ্যাডমিনিস্ট্রেটরের পাসওয়ার্ড চাওয়া হবে। এটি শুধু প্রথমবার হয়।

# Calm words while an existing share is changed.
folders-sharing-changing = শেয়ার বদলানো হচ্ছে…

# Calm words while a share is made.
folders-sharing-sharing-it = ফোল্ডারটি শেয়ার করা হচ্ছে…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = শেয়ার করা ফোল্ডারটির একটি নাম দিন।

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = শেয়ার করা ফোল্ডারের নামে { $most }টির বেশি অক্ষর থাকতে পারবে না।

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = শেয়ার করা ফোল্ডারের নামে % < > * ? | / \ + = ; : " বা কমা থাকতে পারবে না।

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = $ দিয়ে শেষ হওয়া নাম শেয়ারটিকে লুকিয়ে ফেলবে, তাই এটি ব্যবহার করা যাবে না।

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = শেয়ার করা ফোল্ডারের নাম ড্যাশ দিয়ে শুরু হতে পারবে না।

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba আগে থেকেই { $name } নামটি ব্যবহার করে। অন্য একটি বেছে নিন।

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = শেয়ার

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = আপনি শুধু নিজের ফোল্ডার শেয়ার করতে পারেন।

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = এই নামটি এই কম্পিউটারে আগে থেকেই একটি অ্যাকাউন্টের নাম, তাই শেয়ারে এটি ব্যবহার করা যাবে না। অন্য একটি বেছে নিন।

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = এই কম্পিউটারের Samba সেটিংয়ে পাসওয়ার্ড ছাড়া শেয়ার করা যায় না।

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = এই কম্পিউটার আর কোনো ফোল্ডার শেয়ার করতে পারবে না।

# Red words when the folder to share has gone.
folders-sharing-folder-gone = ওই ফোল্ডারটি আর নেই।

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba কোনো কারণ জানায়নি।

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Samba-র বার্তা: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } অন্য কারও, তাই এখান থেকে এটিতে অতিথিদের প্রবেশ দেওয়া যাবে না।

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders { $path }-এ অতিথিদের প্রবেশ দিতে পারেনি: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = এই কম্পিউটার অফলাইনে আছে মনে হচ্ছে, তাই Trindle Samba ডাউনলোড করতে পারেনি। ইন্টারনেটে যুক্ত হয়ে আবার চেষ্টা করুন।

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = অন্য কিছু ইনস্টল হওয়ার সময় Trindle Samba ইনস্টল করতে পারেনি। কয়েক মিনিট পরে আবার চেষ্টা করুন।
