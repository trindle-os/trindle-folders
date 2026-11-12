
# machine translated (Bengali), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = বন্ধ করুন

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = হোম

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = { $name } খুলুন যে অ্যাপে

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = বর্তমান অ্যাপ

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = এই কম্পিউটারের কোনো অ্যাপ এই ধরনের ফাইল খোলার জন্য সেট করা নেই।

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = সবসময় এই অ্যাপ ব্যবহার করুন

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = প্রতিটি { $kind } ফাইলের জন্য এটি ব্যবহার করুন।

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = ডিফল্ট অ্যাপ হিসেবে সেট করুন

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = শুধু প্রস্তাবিত অ্যাপ দেখান

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = সব অ্যাপ দেখান

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = খুলুন

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = আগে একটি অ্যাপ বেছে নিন।

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders { $name }-কে ডিফল্ট অ্যাপ করতে পারেনি।

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders { $name } চালু করতে পারেনি।

# Heading of the Properties window.
folders-dialogs-properties = বৈশিষ্ট্য

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = বৈশিষ্ট্যের পাতা

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = সাধারণ

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = বিবরণ

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = ভাগ করা



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = নাম

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = কোথায়

# Label in Properties of the kind of file.
folders-dialogs-row-kind = ধরন

# Label in Properties of where a link points.
folders-dialogs-row-link-to = লিঙ্ক

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = পরিবর্তিত

# Label in Properties of who owns the item.
folders-dialogs-row-owner = মালিক

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = অনুমতি

# Label in Properties of the size of the item.
folders-dialogs-row-size = আকার

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = অজানা

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, আপনি এটি খুলতে ও বদলাতে পারেন

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, আপনি এটি খুলতে পারেন কিন্তু বদলাতে পারেন না

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, আপনি এটি বদলাতে পারেন কিন্তু খুলতে পারেন না

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, আপনি এটি খুলতে পারবেন না

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , কিছু পড়া যায়নি
       *[other]
    }{ $adding ->
        [yes] , এখনও যোগ হচ্ছে
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = আকার হিসাব করা হচ্ছে

# Label in Properties of the row about checksums.
folders-dialogs-checksums = চেকসাম

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = চেকসাম হিসাব করুন

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = চেকসাম হিসাব করা হচ্ছে

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders ফাইলটি পড়তে পারেনি।

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = মেলাতে একটি চেকসাম পেস্ট করুন

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } মিলেছে

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = মেলেনি

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = যাচাই


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [one] { $name } পাঠান যেখানে
       *[other] { $n }টি আইটেম পাঠান যেখানে
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = এতে সেখানে একটি কপি তৈরি হয়। মূল আইটেমগুলো নিজের জায়গায় থাকে।

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = অন্য প্যান, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = তালিকা দৃশ্যের কলাম

# Note in the Columns window.
folders-dialogs-columns-note = নাম সবসময় দেখানো হয়। যে কলামগুলো জায়গায় ধরে না, প্যান চওড়া না হওয়া পর্যন্ত সেগুলো লুকানো থাকে।

# Heading over the columns that suit every file.
folders-dialogs-columns-files = ফাইল

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = গান, ছবি, PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = { $name } দেখান

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders কলামগুলো সংরক্ষণ করতে পারেনি।

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [one] { $name }-এর রং
       *[other] { $n }টি ফোল্ডারের রং
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = কোনো রং নেই

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders ফোল্ডারের রং সংরক্ষণ করতে পারেনি।


folders-dialogs-no-match = কোনো অ্যাপ এই শব্দগুলোর সঙ্গে মেলে না।


folders-dialogs-find-app = একটি অ্যাপ খুঁজুন


folders-dialogs-find-place = একটি জায়গা খুঁজুন


folders-dialogs-no-match-place = কোনো জায়গা এই শব্দগুলোর সঙ্গে মেলে না।
