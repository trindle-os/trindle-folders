
# machine translated (Urdu), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = بند کریں

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = ہوم

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = { $name } کو اس سے کھولیں

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = موجودہ ایپ

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = اس کمپیوٹر پر کوئی ایپ اس قسم کی فائل کھولنے کے لیے مقرر نہیں ہے۔

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = ہمیشہ یہی ایپ استعمال کریں

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = ہر { $kind } فائل کے لیے اسے استعمال کریں۔

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = معمول کی ایپ کے طور پر مقرر کریں

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = صرف تجویز کردہ ایپس دکھائیں

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = تمام ایپس دکھائیں

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = کھولیں

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = پہلے کوئی ایپ چنیں۔

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders { $name } کو معمول کی ایپ نہیں بنا سکا۔

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders { $name } شروع نہیں کر سکا۔

# Heading of the Properties window.
folders-dialogs-properties = خصوصیات

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = خصوصیات کے صفحات

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = عمومی

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = تفصیلات

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = اشتراک



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = نام

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = مقام

# Label in Properties of the kind of file.
folders-dialogs-row-kind = قسم

# Label in Properties of where a link points.
folders-dialogs-row-link-to = لنک کی منزل

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = تبدیلی

# Label in Properties of who owns the item.
folders-dialogs-row-owner = مالک

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = اجازتیں

# Label in Properties of the size of the item.
folders-dialogs-row-size = سائز

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = نامعلوم

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }، آپ اسے کھول اور بدل سکتے ہیں

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }، آپ اسے کھول سکتے ہیں مگر بدل نہیں سکتے

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }، آپ اسے بدل سکتے ہیں مگر کھول نہیں سکتے

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }، آپ اسے نہیں کھول سکتے

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] ، { $items }
       *[other]
    }{ $unread ->
        [yes] ، کچھ پڑھی نہیں جا سکیں
       *[other]
    }{ $adding ->
        [yes] ، گنتی جاری ہے
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = سائز معلوم کیا جا رہا ہے

# Label in Properties of the row about checksums.
folders-dialogs-checksums = چیک سم

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = چیک سم نکالیں

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = چیک سم نکالے جا رہے ہیں

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders فائل نہیں پڑھ سکا۔

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = موازنے کے لیے چیک سم چسپاں کریں

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } میل کھاتا ہے

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = میل نہیں کھاتا

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = جانچ


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [one] { $name } کو بھیجیں
       *[other] { $n } اشیا بھیجیں
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = یہاں ایک نقل بنتی ہے۔ اصل چیزیں اپنی جگہ پر رہتی ہیں۔

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = دوسرا پین، { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = فہرست منظر کے کالم

# Note in the Columns window.
folders-dialogs-columns-note = نام ہمیشہ دکھایا جاتا ہے۔ جو کالم نہ سمائیں وہ پین کے چوڑا ہونے تک چھپا دیے جاتے ہیں۔

# Heading over the columns that suit every file.
folders-dialogs-columns-files = فائلیں

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = موسیقی، تصاویر، PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = { $name } دکھائیں

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders کالم محفوظ نہیں کر سکا۔

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [one] { $name } کا رنگ
       *[other] { $n } فولڈروں کا رنگ
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = کوئی رنگ نہیں

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders فولڈر کا رنگ محفوظ نہیں کر سکا۔


folders-dialogs-no-match = کوئی ایپ ان الفاظ سے میل نہیں کھاتی۔


folders-dialogs-find-app = ایپ تلاش کریں


folders-dialogs-find-place = جگہ تلاش کریں


folders-dialogs-no-match-place = کوئی جگہ ان الفاظ سے میل نہیں کھاتی۔
