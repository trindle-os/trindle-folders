# machine translated (Bengali), not yet reviewed by a person.

folders-shares-nfs-denied = { $server } এই কম্পিউটারকে { $export } খুলতে দেয় না।

folders-shares-nfs-no-share = { $server } এ { $export } নামে কিছু শেয়ার করা নেই।

folders-shares-left-none = পরের বার খুলতে আবার জিজ্ঞাসা করবে

folders-shares-left-under-day = এক দিনেরও কম বাকি

folders-shares-left-days =
    { $n ->
        [one] { $n } দিন বাকি
       *[other] { $n } দিন বাকি
    }

folders-shares-held =
    { $past ->
        [yes] { $server } এখন { $now } ঠিকানায় সাড়া দিচ্ছে, যা আগে এর জন্য আপনার লগইন যে ঠিকানায় ব্যবহার হয়েছিল তার থেকে আলাদা ({ $before }).
       *[no] { $server } { $now } ঠিকানায় সাড়া দিচ্ছে, যে ঠিকানায় এর জন্য আপনার লগইন আগে ব্যবহার হয়নি।
    }{ $full ->
        [yes] {" "}যে ঠিকানাগুলোতে এটি সর্বশেষ ব্যবহার হয়েছে শুধু সেই { $most }টি মনে রাখা হয়।
       *[no] {""}
    } নেটওয়ার্কের যেকোনো ডিভাইস একটি নামে সাড়া দিতে পারে, তাই এর জন্য রাখা লগইন পাঠানো হয়নি। এটি যদি আপনার নিজের { $server } হয়, তবে এখানে ব্যবহার করতে এর ব্যবহারকারীর নাম ও পাসওয়ার্ড লিখুন।

folders-shares-nowhere = { $server } যে ঠিকানায় সাড়া দেয় তা আগে থেকে পাওয়া যায়নি, তাই এতে কিছু পাঠানো হয়নি এবং এখানে যা লেখা হবে তা নিরাপদে যাওয়ার মতো কোথাও নেই। বদলে এর ঠিকানা দিয়ে খুলুন, যেমন smb://192.168.1.20/।

folders-shares-lapsed = { $server } { $now } ঠিকানায় সাড়া দিচ্ছে, যেখানে এর জন্য আপনার লগইন সর্বশেষ { $days } দিনের বেশি আগে ব্যবহার হয়েছিল। এর মধ্যে অন্য কোনো ডিভাইস ওই ঠিকানা পেয়ে থাকতে পারে, তাই এর জন্য রাখা লগইন পাঠানো হয়নি। এটি যদি আপনার নিজের { $server } হয়, তবে এখানে ব্যবহার করতে এর ব্যবহারকারীর নাম ও পাসওয়ার্ড লিখুন।

folders-shares-unknown = { $server } { $now } ঠিকানায় সাড়া দিচ্ছে, যেখানে এর জন্য একটি লগইন আগে ব্যবহার হয়েছিল, কিন্তু কখন বা কোন লগইন তা লিখে রাখা হয়নি। এর মধ্যে অন্য কোনো ডিভাইস ওই ঠিকানা পেয়ে থাকতে পারে, তাই এর জন্য রাখা লগইন পাঠানো হয়নি। এটি যদি আপনার নিজের { $server } হয়, তবে এখানে ব্যবহার করতে এর ব্যবহারকারীর নাম ও পাসওয়ার্ড লিখুন।

folders-shares-unshown = { $server } { $now } ঠিকানায় সাড়া দিচ্ছে, যে ঠিকানায় এর জন্য আপনার লগইন আগে গিয়েছিল। এবার ওই ঠিকানা শুধু নেটওয়ার্কে শোনা গেছে, যেখানে যেকোনো ডিভাইস একটি নামে সাড়া দিতে পারে, আর এটি আগে আপনাকে এখানে দেখানো হয়নি, তাই লগইন পাঠানো হয়নি। এটি যদি আপনার নিজের { $server } হয়, তবে এখানে ব্যবহার করতে এর ব্যবহারকারীর নাম ও পাসওয়ার্ড লিখুন।

folders-shares-asking =
    { $server } ব্যবহারকারীর নাম ও পাসওয়ার্ড চাইছে। এগুলো ওই কম্পিউটারের একটি অ্যাকাউন্টের।{ $heard ->
        [yes] {" "}এটি { $at } ঠিকানায় সাড়া দিচ্ছে। নেটওয়ার্কের যেকোনো ডিভাইস একটি নামে সাড়া দিতে পারে, তাই লেখার আগে দেখে নিন এটিই ওর ঠিকানা কি না।
       *[no] {""}
    }

folders-shares-where =
    আপনি যা লিখবেন তা যাবে { $address } ঠিকানায়{ $heard ->
        [yes] {" "}এবং আর কোথাও নয়। মনে রাখা চালু থাকলে আপনার লগইন শুধু ওই ঠিকানার জন্য রাখা হবে।
       *[no] {"।"}
    }

folders-shares-again = { $server } { $at } ঠিকানায়, পোর্ট { $port }-এ সাড়া দিচ্ছে। ওই ঠিকানা শুধু নেটওয়ার্কে শোনা গেছে, যেখানে যেকোনো ডিভাইস একটি নামে সাড়া দিতে পারে, তাই এর জন্য আপনার লগইন পাঠানো হয়নি। ওটিই যদি এর ঠিকানা হয়, তবে খুলুন চাপুন।

folders-shares-keyring-unreachable = কিরিং পাওয়া যায়নি: { $why }।

folders-shares-unkept = এটি খোলা আছে, কিন্তু লগইন রাখা হয়নি। { $why }

folders-shares-nfs-version = { $server } NFS সংস্করণ ৪ দেয় না, যা Trindle-এর খোলার একমাত্র ধরন।

folders-shares-nfs-opened-unknown = এটি খুলেছে, কিন্তু কোথায় আছে তা জানা নেই।

folders-shares-opened-unknown = শেয়ারটি খুলেছে, কিন্তু কোথায় আছে তা জানা নেই।

folders-shares-not-address = এটি কোনো ঠিকানা নয়।

folders-shares-seen-label = smb://{ $server } যে ঠিকানাগুলোতে খোলা হয়েছিল

folders-shares-password-label = { $what }-এ { $user }-এর পাসওয়ার্ড

folders-shares-remember = ৯০ দিন মনে রাখুন

folders-shares-remember-line = প্রতিবার ব্যবহারে আবার নতুন করে শুরু হয়।

folders-shares-share-on = { $server }-এ { $share }

folders-shares-card-title = { $it } খুলুন

folders-shares-close = বন্ধ করুন

folders-shares-user-placeholder = ব্যবহারকারীর নাম, যেমন jo, অথবা WORKGROUP\jo

folders-shares-password = পাসওয়ার্ড

folders-shares-forget = আগের ঠিকানাগুলো ভুলে যান

folders-shares-forget-spoken = এই লগইন আগে যে ঠিকানাগুলোতে ব্যবহার হয়েছিল সেগুলো ভুলে যান

folders-shares-opening = খোলা হচ্ছে

folders-shares-open = খুলুন

folders-shares-bad-computer = এটি কোনো কম্পিউটারের নাম নয়।

folders-shares-bad-path = এটি কোনো শেয়ার করা ফোল্ডারের পথ নয়।

folders-shares-opening-export = { $host }-এ { $export } খোলা হচ্ছে…

folders-shares-opening-share = { $server }-এ { $share } খোলা হচ্ছে…

folders-shares-answer-card = আগে { $it }-এর কার্ডটির উত্তর দিন।

folders-shares-open-waiting = এটি খোলা আছে। { $words }

folders-shares-none = { $server } আপনার সঙ্গে কোনো ফোল্ডার শেয়ার করে না।

folders-shares-closing = { $name } বন্ধ করা হচ্ছে…

folders-shares-close-failed = { $name } বন্ধ করা যায়নি: { $why }।

folders-shares-reconnecting = { $name } আবার যুক্ত হচ্ছে…

folders-shares-not-remembered = শেয়ারগুলো পরের বারের জন্য মনে রাখা যায়নি।
