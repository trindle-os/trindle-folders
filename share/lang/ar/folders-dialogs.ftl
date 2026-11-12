
# machine translated (Arabic), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = إغلاق

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = المجلد الرئيسي

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = فتح { $name } باستخدام

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = التطبيق الحالي

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = لا يوجد تطبيق على هذا الحاسوب مُعدّ لفتح هذا النوع من الملفات.

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = استخدم هذا التطبيق دائماً

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = استخدمه لكل ملف من نوع { $kind }.

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = اجعله التطبيق الافتراضي

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = أظهر التطبيقات المقترحة فقط

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = أظهر كل التطبيقات

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = فتح

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = اختر تطبيقاً أولاً.

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = تعذّر على Folders جعل { $name } التطبيق الافتراضي.

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = تعذّر على Folders تشغيل { $name }.

# Heading of the Properties window.
folders-dialogs-properties = الخصائص

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = صفحات الخصائص

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = عام

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = التفاصيل

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = المشاركة



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = الاسم

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = المكان

# Label in Properties of the kind of file.
folders-dialogs-row-kind = النوع

# Label in Properties of where a link points.
folders-dialogs-row-link-to = الرابط يشير إلى

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = تاريخ التغيير

# Label in Properties of who owns the item.
folders-dialogs-row-owner = المالك

# Label in Properties of who may open and change the item.
# SAFETY: needs a human review before it ships
folders-dialogs-row-rights = الأذونات

# Label in Properties of the size of the item.
folders-dialogs-row-size = الحجم

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = غير معروف

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
# SAFETY: needs a human review before it ships
folders-dialogs-rights-open-change = { $letters }، يمكنك فتحه وتغييره

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
# SAFETY: needs a human review before it ships
folders-dialogs-rights-open-only = { $letters }، يمكنك فتحه لكن لا يمكنك تغييره

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
# SAFETY: needs a human review before it ships
folders-dialogs-rights-change-only = { $letters }، يمكنك تغييره لكن لا يمكنك فتحه

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
# SAFETY: needs a human review before it ships
folders-dialogs-rights-none = { $letters }، لا يمكنك فتحه

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] ، { $items }
       *[other]
    }{ $unread ->
        [yes] ، وتعذّرت قراءة بعضها
       *[other]
    }{ $adding ->
        [yes] ، ولا يزال الجمع جارياً
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = جارٍ حساب الحجم

# Label in Properties of the row about checksums.
folders-dialogs-checksums = المجاميع الاختبارية

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = احسب المجاميع الاختبارية

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = جارٍ حساب المجاميع الاختبارية

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = تعذّر على Folders قراءة الملف.

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = الصق مجموعاً اختبارياً للمقارنة

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } مطابق

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = غير مطابق

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = تحقق


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [zero] إرسال لا شيء إلى
        [one] إرسال { $name } إلى
        [two] إرسال عنصرين إلى
        [few] إرسال { $n } عناصر إلى
        [many] إرسال { $n } عنصراً إلى
       *[other] إرسال { $n } عنصر إلى
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = ينشئ هذا نسخة هناك. وتبقى الأصول حيث هي.

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = اللوح الآخر، { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = أعمدة عرض القائمة

# Note in the Columns window.
folders-dialogs-columns-note = الاسم يظهر دائماً. الأعمدة التي لا تتسع تُخفى حتى يصبح اللوح أعرض.

# Heading over the columns that suit every file.
folders-dialogs-columns-files = الملفات

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = الموسيقى والصور وملفات PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = أظهر { $name }

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = تعذّر على Folders حفظ الأعمدة.

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [zero] لون لا مجلدات
        [one] لون { $name }
        [two] لون مجلدين
        [few] لون { $n } مجلدات
        [many] لون { $n } مجلداً
       *[other] لون { $n } مجلد
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = بلا لون

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = تعذّر على Folders حفظ لون المجلد.


folders-dialogs-no-match = لا يوجد تطبيق يطابق هذه الكلمات.


folders-dialogs-find-app = ابحث عن تطبيق


folders-dialogs-find-place = ابحث عن مكان


folders-dialogs-no-match-place = لا يوجد مكان يطابق هذه الكلمات.
