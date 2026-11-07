# machine translated (Bengali), not yet reviewed by a person.

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = { $host } নামের কোনো সার্ভার পাওয়া যায়নি

folders-far-not-all-arrived = সবটা পৌঁছায়নি, তাই এটি যেখানে ছিল সেখানেই আছে

folders-far-not-followed = এটি একটি লিংক, লিংক অনুসরণ করা হয় না, তাই পাঠানো হয়নি

folders-far-not-sent = এটি ফাইল বা ফোল্ডার নয়, তাই পাঠানো হয়নি

folders-far-not-told = সার্ভার বলছে না এটি কী, তাই বাদ দেওয়া হয়েছে

folders-far-ftp-cannot-refuse = এই সার্ভার দুটি ফাইলকে একই নাম দিয়ে ফেলতে পারে।

folders-far-cannot-do = সার্ভারের ফাইলে এটি করা যায় না

folders-far-not-looked-at = এটি দেখা যায়নি

folders-far-not-opened = এটি খোলা যায়নি।

folders-far-inside-itself = { $name } (ফোল্ডার নিজের ভিতরে যেতে পারে না)

folders-far-taken-by-folder = { $name } (এই নামে একটি ফোল্ডার আছে)

folders-far-taken-by-file = { $name } (এই নামে একটি ফাইল আছে)

folders-far-address-unknown = সার্ভারের ঠিকানা জানা নেই।

folders-far-copy-wrong-length = পাঠানো কপিটির মাপ যা হওয়ার কথা তা নয়

folders-far-only-server-file = সার্ভারের ফাইলেরই শুধু এভাবে নাম বদলানো যায়।

folders-far-aside-both = পুরনোটি অক্ষত অবস্থায় { $old } নামে আছে, আর নতুনটি { $new } নামে: { $why }

folders-far-aside-kept = পুরনোটি সরানো যায়নি, সেটি অক্ষত অবস্থায় { $old } নামে আছে

folders-far-aside-kept-why = পুরনোটি সরানো যায়নি, সেটি অক্ষত অবস্থায় { $old } নামে আছে: { $why }

folders-far-not-empty = { $name } খালি নয়।

folders-far-unsafe-name = বাদ দেওয়া হয়েছে: সার্ভার একটি অনিরাপদ নাম পাঠিয়েছে।

folders-far-unsafe-names =
    { $n ->
        [one] বাদ দেওয়া হয়েছে: সার্ভার একটি অনিরাপদ নাম পাঠিয়েছে: { $names }।
       *[other] { $n }টি বাদ দেওয়া হয়েছে: সার্ভার অনিরাপদ নাম পাঠিয়েছে: { $names }।
    }

folders-far-fetching = { $name } আনা হচ্ছে…

folders-far-cannot-fetch = { $name } আনা যায়নি: { $why }।

folders-far-opened-copy = { $name }-এর একটি কপি খোলা হয়েছে। সংরক্ষণ করলে বদলগুলো সার্ভারে ফেরত যাবে।

folders-far-saved = { $name } সার্ভারে সংরক্ষণ করা হয়েছে।

folders-far-not-saved = { $name } সার্ভারে সংরক্ষণ করা যায়নি: { $why }। এখানের কপিতে বদলগুলো রয়ে গেছে।

folders-far-renamed = { $was }-এর নাম বদলে { $now } করা হয়েছে।

folders-far-connect-title = সার্ভারের সাথে যুক্ত হোন

folders-far-connect-close = বন্ধ করুন

folders-far-connect-address = ঠিকানা, যেমন sftp://jo@example.com

folders-far-connect-password = পাসওয়ার্ড, দরকার হলে

folders-far-connect-key = কী ফাইল, বেছে নিলে

folders-far-connect-pick-key = কী ফাইল বেছে নিন

folders-far-connect-choose = বেছে নিন

folders-far-connect-ssh-setup = আপনার নিজের ssh সেট-আপ ব্যবহার হচ্ছে: এর কনফিগ, এজেন্ট, কী ও চেনা সার্ভার।

folders-far-connect-new-host = { $host }-এ আগে কখনও যাওয়া হয়নি, তাই Trindle বলতে পারে না এটি ঠিক সার্ভার কি না। আপনি এটিকেই আশা করে থাকলে বিশ্বাস করুন।

folders-far-connect-host-changed = { $host }-এর পরিচয় আগের মতো আর নেই। এর মানে মাঝখানে কেউ থাকতে পারে, তাই এটি খোলা হয়নি। এর মালিক বদলে থাকলে আগে আপনার চেনা হোস্টের তালিকা থেকে পুরনোটি সরিয়ে দিন।

folders-far-connect-needs-password = এই সার্ভার পাসওয়ার্ড চাইছে। ওপরে লিখে আবার চেষ্টা করুন।

folders-far-connect-needs-key = এই সার্ভার কী বা পাসওয়ার্ড চাইছে। ওপরে আপনার কী ফাইলের নাম দিন, বা এটি আপনার ssh এজেন্টে যোগ করে আবার চেষ্টা করুন।

folders-far-connect-cert-new = { $host }-এর সার্টিফিকেট যাচাই করা যায়নি: { $why }। পারলে এর ফিঙ্গারপ্রিন্ট সার্ভারের মালিকের দেওয়া ফিঙ্গারপ্রিন্টের সাথে মিলিয়ে দেখুন, মিললে তবেই বিশ্বাস করুন।

folders-far-connect-cert-changed = { $host }-এর সার্টিফিকেট আপনার আগে বিশ্বাস করা সার্টিফিকেটটি নয়। মালিক হয়তো নবায়ন করেছেন, অথবা মাঝখানে কেউ থাকতে পারে। মালিক এই ফিঙ্গারপ্রিন্ট নিশ্চিত করলে তবেই নতুনটি বিশ্বাস করুন।

folders-far-connect-connecting = যুক্ত হচ্ছে

folders-far-connect-trust = বিশ্বাস করে যুক্ত হোন

folders-far-connect-go = যুক্ত হোন

folders-far-phones = ফোন প্লাগ করে আনলক করলে সেগুলো নিজে থেকেই সাইডবারে দেখা যায়।
