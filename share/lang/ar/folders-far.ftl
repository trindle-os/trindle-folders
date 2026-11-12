
# machine translated (Arabic), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = لا يوجد خادم باسم { $host }

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = لم يصل كله، لذلك لم يُنقل الأصل

# Reason a link was not sent to a server.
folders-far-not-followed = هو رابط، لذلك لم يرسله Folders

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = ليس ملفاً ولا مجلداً، لذلك لم يرسله Folders

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = تعذّر على Folders معرفة نوع هذا العنصر، لذلك تركه هناك

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = على هذا النوع من الخوادم، قد يحل ملف محل ملف آخر بالاسم نفسه.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = لا يمكنك فعل ذلك بالملفات الموجودة على خادم

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = تعذّر على Folders قراءته

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = تعذّر على Folders فتحه.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (لا يمكن وضع مجلد داخل نفسه)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (يوجد بالفعل مجلد بهذا الاسم)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (يوجد بالفعل ملف بهذا الاسم)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = لا يعرف Folders عنوان الخادم.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = النسخة التي أُرسلت حجمها غير صحيح

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = هذا يعمل مع الملفات الموجودة على خادم فقط.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
# SAFETY: needs a human review before it ships
folders-far-aside-both = تعذّر على Folders وضع الملف الجديد في مكانه، لذلك لا يزال القديم موجوداً، وصار اسمه { $old }، واسم الجديد { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = تعذّر على Folders إزالة الملف القديم. وهو لا يزال موجوداً، وصار اسمه { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = تعذّر على Folders إزالة الملف القديم، لذلك لا يزال موجوداً، وصار اسمه { $old }: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } ليس فارغاً.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
# SAFETY: needs a human review before it ships
folders-far-unsafe-name = تخطى Folders ملفاً لأن اسمه غير آمن للاستخدام.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
# SAFETY: needs a human review before it ships
folders-far-unsafe-names =
    { $n ->
        [zero] لم يتخطَّ Folders أي عنصر.
        [one] تخطى Folders عنصراً واحداً لأن اسمه غير آمن للاستخدام: { $names }.
        [two] تخطى Folders عنصرين لأن اسميهما غير آمنين للاستخدام: { $names }.
        [few] تخطى Folders { $n } عناصر لأن أسماءها غير آمنة للاستخدام: { $names }.
        [many] تخطى Folders { $n } عنصراً لأن أسماءها غير آمنة للاستخدام: { $names }.
       *[other] تخطى Folders { $n } عنصر لأن أسماءها غير آمنة للاستخدام: { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = جارٍ تنزيل { $name }…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = تعذّر على Folders تنزيل { $name }: { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = فُتحت نسخة من { $name }. عندما تحفظها، تعود تغييراتك إلى الخادم.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = حُفظ { $name } على الخادم.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = تعذّر على Folders حفظ { $name } على الخادم: { $why }. تغييراتك لا تزال في النسخة الموجودة على هذا الحاسوب.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = أُعيدت تسمية { $was } إلى { $now }.

# Heading of the Connect to a server card.
folders-far-connect-title = الاتصال بخادم

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = إغلاق

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = العنوان، مثل sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
# SAFETY: needs a human review before it ships
folders-far-connect-password = كلمة المرور، إن لزمت

# Placeholder of the box on the Connect to a server card where the key file is named.
# SAFETY: needs a human review before it ships
folders-far-connect-key = ملف المفتاح، إن كنت تستخدم واحداً

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = اختر ملف المفتاح


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = يستخدم Folders إعدادات ssh لديك ووكيل ssh ومفاتيحك وقائمة الخوادم المعروفة.

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = لم تتصل بـ { $host } من قبل، لذلك لا يستطيع Folders التحقق من أنه الخادم الصحيح. ثق به فقط إذا كنت تتوقع الاتصال به.

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = لدى { $host } هوية مختلفة عن المرة السابقة. قد يحاول أحد اعتراض اتصالك، لذلك لم يتصل Folders. إذا غيّر مالكه ذلك، فأزل إدخاله القديم من الخوادم المعروفة لديك أولاً.

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-password = هذا الخادم يحتاج إلى كلمة مرور. اكتبها أعلاه وحاول مرة أخرى.

# Note on the Connect to a server card when a server asks for a key or password.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-key = هذا الخادم يحتاج إلى مفتاح أو كلمة مرور. اختر ملف مفتاحك أعلاه، أو أضفه إلى وكيل ssh لديك، ثم حاول مرة أخرى.

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = تعذّر على Folders فحص شهادة { $host }: { $why }. إن استطعت، فقارن بصمتها بالبصمة التي يعطيك إياها المالك. ثق بها فقط إذا تطابقتا.

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = تغيّرت شهادة { $host } منذ أن وثقت بها. قد يكون المالك جددها، وقد يحاول أحد اعتراض اتصالك. ثق بها فقط إذا أكد المالك هذه البصمة.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = جارٍ الاتصال

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = ثق به واتصل

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = اتصال

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = تظهر الهواتف في الشريط الجانبي عندما توصّلها وتفتح قفلها.

