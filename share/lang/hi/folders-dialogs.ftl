# machine translated (Hindi), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = बंद करें

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = होम

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = { $name } को इससे खोलें

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = मौजूदा ऐप

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = इस कंप्यूटर पर कोई ऐप इस तरह की फ़ाइल खोलने के लिए सेट नहीं है।

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = हमेशा यही ऐप इस्तेमाल करें

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = हर { $kind } फ़ाइल के लिए इसका इस्तेमाल करें।

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = इसे डिफ़ॉल्ट ऐप बनाएँ

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = केवल सुझाए गए ऐप दिखाएँ

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = सभी ऐप दिखाएँ

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = खोलें

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = पहले कोई ऐप चुनें।

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders { $name } को डिफ़ॉल्ट ऐप नहीं बना सका।

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders { $name } शुरू नहीं कर सका।

# Heading of the Properties window.
folders-dialogs-properties = गुण

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = गुण पृष्ठ

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = सामान्य

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = ब्योरा

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = साझा करना



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = नाम

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = स्थान

# Label in Properties of the kind of file.
folders-dialogs-row-kind = प्रकार

# Label in Properties of where a link points.
folders-dialogs-row-link-to = लिंक का लक्ष्य

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = बदला गया

# Label in Properties of who owns the item.
folders-dialogs-row-owner = स्वामी

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = अनुमतियाँ

# Label in Properties of the size of the item.
folders-dialogs-row-size = आकार

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = अज्ञात

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, आप इसे खोल और बदल सकते हैं

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, आप इसे खोल सकते हैं पर बदल नहीं सकते

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, आप इसे बदल सकते हैं पर खोल नहीं सकते

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, आप इसे नहीं खोल सकते

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , कुछ पढ़े नहीं जा सके
       *[other]
    }{ $adding ->
        [yes] , जोड़ना जारी है
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = आकार निकाला जा रहा है

# Label in Properties of the row about checksums.
folders-dialogs-checksums = चेकसम

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = चेकसम निकालें

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = चेकसम निकाले जा रहे हैं

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders फ़ाइल पढ़ नहीं सका।

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = तुलना करने के लिए चेकसम चिपकाएँ

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } मेल खाता है

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = मेल नहीं खाता

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = जाँचें


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [one] { $name } को यहाँ भेजें
       *[other] { $n } आइटम यहाँ भेजें
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = इससे वहाँ एक प्रति बनती है। मूल चीज़ें अपनी जगह पर ही रहती हैं।

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = दूसरा पैन, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = सूची दृश्य के कॉलम

# Note in the Columns window.
folders-dialogs-columns-note = नाम हमेशा दिखता है। जो कॉलम समा नहीं पाते, वे पैनल चौड़ा होने तक छिपे रहते हैं।

# Heading over the columns that suit every file.
folders-dialogs-columns-files = फ़ाइलें

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = संगीत, फ़ोटो, PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = { $name } दिखाएँ

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders कॉलम सहेज नहीं सका।

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [one] { $name } का रंग
       *[other] { $n } फ़ोल्डरों का रंग
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = कोई रंग नहीं

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders फ़ोल्डर का रंग सहेज नहीं सका।


folders-dialogs-no-match = इन शब्दों से कोई ऐप मेल नहीं खाता।


folders-dialogs-find-app = ऐप खोजें


folders-dialogs-find-place = जगह खोजें


folders-dialogs-no-match-place = इन शब्दों से कोई जगह मेल नहीं खाती।
