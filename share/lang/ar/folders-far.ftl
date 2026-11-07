

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = تعذّر العثور على خادم باسم { $host }

folders-far-not-all-arrived = لم يصل كله، لذلك بقي حيث كان

folders-far-not-followed = رابط، لا يُتّبع، لذلك لم يُرسَل

folders-far-not-sent = ليس ملفاً ولا مجلداً، لذلك لم يُرسَل

folders-far-not-told = لا يقول الخادم ما هو، لذلك تُرك

folders-far-ftp-cannot-refuse = قد يعطي هذا الخادم ملفين الاسم نفسه.

folders-far-cannot-do = لا يمكن فعل ذلك بملفات الخادم

folders-far-not-looked-at = تعذّر النظر فيه

folders-far-not-opened = تعذّر فتحه.

folders-far-inside-itself = { $name } (لا يمكن وضع مجلد داخل نفسه)

folders-far-taken-by-folder = { $name } (يوجد هناك مجلد بهذا الاسم)

folders-far-taken-by-file = { $name } (يوجد هناك ملف بهذا الاسم)

folders-far-address-unknown = عنوان الخادم غير معروف.

folders-far-copy-wrong-length = النسخة المرسلة ليست بالطول الصحيح

folders-far-only-server-file = لا يُسمّى بهذه الطريقة إلا ملف على خادم.

folders-far-aside-both = القديم موجود كاملاً باسم { $old }، والجديد باسم { $new }: { $why }

folders-far-aside-kept = تعذّرت إزالة القديم، وهو موجود كاملاً باسم { $old }

folders-far-aside-kept-why = تعذّرت إزالة القديم، وهو موجود كاملاً باسم { $old }: { $why }

folders-far-not-empty = { $name } ليس فارغاً.

folders-far-unsafe-name = تُخطّي: أرسل الخادم اسماً غير آمن.

folders-far-unsafe-names =
    { $n ->
        [zero] لم يُتخطَّ شيء: { $names }.
        [one] تُخطّي واحد: أرسل الخادم اسماً غير آمن: { $names }.
        [two] تُخطّي اثنان: أرسل الخادم اسماً غير آمن: { $names }.
        [few] تُخطّيت { $n }: أرسل الخادم اسماً غير آمن: { $names }.
        [many] تُخطّي { $n }: أرسل الخادم اسماً غير آمن: { $names }.
       *[other] تُخطّي { $n }: أرسل الخادم اسماً غير آمن: { $names }.
    }

folders-far-fetching = جارٍ جلب { $name }…

folders-far-cannot-fetch = تعذّر جلب { $name }: { $why }.

folders-far-opened-copy = فُتحت نسخة من { $name }. حفظها يرسل التغييرات إلى الخادم.

folders-far-saved = حُفظ { $name } على الخادم.

folders-far-not-saved = تعذّر حفظ { $name } على الخادم: { $why }. النسخة هنا تحتفظ بالتغييرات.

folders-far-renamed = أُعيدت تسمية { $was } إلى { $now }.

folders-far-connect-title = الاتصال بخادم

folders-far-connect-close = إغلاق

folders-far-connect-address = العنوان، مثل sftp://jo@example.com

folders-far-connect-password = كلمة المرور، إن كان يطلبها

folders-far-connect-key = ملف المفتاح، إن اخترت واحداً

folders-far-connect-pick-key = اختر ملف المفتاح

folders-far-connect-choose = اختيار

folders-far-connect-ssh-setup = يُستخدم إعداد ssh الخاص بك: ملف إعداده ووكيله ومفاتيحه والخوادم المعروفة لديه.

folders-far-connect-new-host = لم تُزَر { $host } من قبل، لذلك لا يستطيع Trindle أن يعرف أنه الخادم الصحيح. ثق به إذا كنت تتوقع الوصول إليه.

folders-far-connect-host-changed = لم يعد لدى { $host } الهوية التي كانت له من قبل. قد يعني هذا أن أحداً يعترض الطريق، لذلك لم يُفتح. إذا غيّر مالكه ذلك، فأزل القديم من الخوادم المعروفة لديك أولاً.

folders-far-connect-needs-password = هذا الخادم يريد كلمة مرور. اكتبها أعلاه وحاول مرة أخرى.

folders-far-connect-needs-key = هذا الخادم يريد مفتاحاً أو كلمة مرور. سمِّ ملف مفتاحك أعلاه، أو أضفه إلى وكيل ssh لديك، وحاول مرة أخرى.

folders-far-connect-cert-new = تعذّر فحص شهادة { $host }: { $why }. إن استطعت، فقارن بصمتها بالبصمة التي يعطيها مالك الخادم، وثق بها فقط إذا تطابقتا.

folders-far-connect-cert-changed = شهادة { $host } ليست التي وثقت بها من قبل. قد يكون مالكها جددها، وقد يكون أحد يعترض الطريق. ثق بالجديدة فقط إذا أكد مالكها هذه البصمة.

folders-far-connect-connecting = جارٍ الاتصال

folders-far-connect-trust = ثق به واتصل

folders-far-connect-go = اتصال

folders-far-phones = تظهر الهواتف في الشريط الجانبي تلقائياً بمجرد توصيلها وفتح قفلها.

