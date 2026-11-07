

folders-shares-nfs-denied = لا يسمح { $server } لهذا الحاسوب بفتح { $export }.

folders-shares-nfs-no-share = لا يشارك { $server } أي شيء باسم { $export }.

folders-shares-left-none = يُطلب مجددًا عند الفتح التالي

folders-shares-left-under-day = بقي أقل من يوم

folders-shares-left-days =
    { $n ->
        [zero] لم يبقَ أي يوم
        [one] بقي يوم واحد
        [two] بقي يومان
        [few] بقيت { $n } أيام
        [many] بقي { $n } يومًا
       *[other] بقي { $n } يوم
    }

folders-shares-held =
    { $past ->
        [yes] يردّ { $server } الآن على العنوان { $now }، وهو عنوان مختلف عن العنوان الذي استُخدم فيه تسجيل دخولك له سابقًا ({ $before }).
       *[no] يردّ { $server } على العنوان { $now }، وهو عنوان لم يُستخدم فيه تسجيل دخولك له من قبل.
    }{ $full ->
        [yes] {" "}تُحفظ فقط آخر { $most } عناوين استُخدم فيها.
       *[no] {""}
    } يمكن لأي جهاز في الشبكة أن يردّ على اسم، لذلك لم يُرسل تسجيل الدخول المحفوظ له. إن كان هذا { $server } الخاص بك، فاكتب اسم المستخدم وكلمة المرور لاستخدامه هنا.

folders-shares-nowhere = تعذّر العثور مسبقًا على العنوان الذي يردّ عليه { $server }، لذلك لم يُرسل إليه شيء، وما يُكتب هنا لا يوجد مكان آمن يذهب إليه. افتحه بعنوانه بدلًا من ذلك، مثل smb://192.168.1.20/.

folders-shares-lapsed = يردّ { $server } على العنوان { $now }، وقد استُخدم فيه تسجيل دخولك له آخر مرة قبل أكثر من { $days } يومًا. ربما أُعطي جهاز آخر هذا العنوان منذ ذلك الحين، لذلك لم يُرسل تسجيل الدخول المحفوظ له. إن كان هذا { $server } الخاص بك، فاكتب اسم المستخدم وكلمة المرور لاستخدامه هنا.

folders-shares-unknown = يردّ { $server } على العنوان { $now }، وقد استُخدم فيه تسجيل دخول له من قبل، لكن لم يُسجَّل متى أو أي تسجيل دخول. ربما أُعطي جهاز آخر هذا العنوان منذ ذلك الحين، لذلك لم يُرسل تسجيل الدخول المحفوظ له. إن كان هذا { $server } الخاص بك، فاكتب اسم المستخدم وكلمة المرور لاستخدامه هنا.

folders-shares-unshown = يردّ { $server } على العنوان { $now }، وهو العنوان الذي ذهب إليه تسجيل دخولك له سابقًا. لكن هذا العنوان سُمع هذه المرة على الشبكة فقط، حيث يمكن لأي جهاز أن يردّ على اسم، ولم يُعرض عليك هنا من قبل، لذلك لم يُرسل تسجيل الدخول. إن كان هذا { $server } الخاص بك، فاكتب اسم المستخدم وكلمة المرور لاستخدامه هنا.

folders-shares-asking =
    يطلب { $server } اسم مستخدم وكلمة مرور. هما الخاصان بحساب على ذلك الحاسوب.{ $heard ->
        [yes] {" "}يردّ على العنوان { $at }. يمكن لأي جهاز في الشبكة أن يردّ على اسم، فتأكد أن هذا هو عنوانه قبل أن تكتب.
       *[no] {""}
    }

folders-shares-where =
    ما تكتبه يذهب إلى { $address }{ $heard ->
        [yes] {" "}ولا يذهب إلى مكان آخر. وعند تشغيل التذكر، يُحفظ تسجيل دخولك لهذا العنوان فقط.
       *[no] {"."}
    }

folders-shares-again = يردّ { $server } على العنوان { $at }، المنفذ { $port }. هذا العنوان سُمع على الشبكة فقط، حيث يمكن لأي جهاز أن يردّ على اسم، لذلك لم يُرسل تسجيل دخولك له. إن كان هذا عنوانه، فاضغط فتح.

folders-shares-keyring-unreachable = تعذّر الوصول إلى سلسلة المفاتيح: { $why }.

folders-shares-unkept = فُتح، لكن لم يُحفظ تسجيل الدخول. { $why }

folders-shares-nfs-version = لا يوفّر { $server } الإصدار 4 من NFS، وهو النوع الوحيد الذي يفتحه Trindle.

folders-shares-nfs-opened-unknown = فُتح، لكن مكانه غير معروف.

folders-shares-opened-unknown = فُتحت المشاركة، لكن مكانها غير معروف.

folders-shares-not-address = هذا ليس عنوانًا.

folders-shares-seen-label = العناوين التي فُتح عليها smb://{ $server }

folders-shares-password-label = كلمة مرور { $user } على { $what }

folders-shares-remember = تذكّر لمدة 90 يومًا

folders-shares-remember-line = تبدأ من جديد في كل مرة تستخدمه.

folders-shares-share-on = { $share } على { $server }

folders-shares-card-title = فتح { $it }

folders-shares-close = إغلاق

folders-shares-user-placeholder = اسم المستخدم، مثل jo أو WORKGROUP\jo

folders-shares-password = كلمة المرور

folders-shares-forget = نسيان العناوين السابقة

folders-shares-forget-spoken = نسيان العناوين التي استُخدم فيها تسجيل الدخول هذا سابقًا

folders-shares-opening = جارٍ الفتح

folders-shares-open = فتح

folders-shares-bad-computer = هذا ليس اسم حاسوب.

folders-shares-bad-path = هذا ليس مسار مجلد مشترك.

folders-shares-opening-export = جارٍ فتح { $export } على { $host }…

folders-shares-opening-share = جارٍ فتح { $share } على { $server }…

folders-shares-answer-card = أجب عن بطاقة { $it } أولًا.

folders-shares-open-waiting = فُتح. { $words }

folders-shares-none = لا يشارك { $server } أي مجلدات معك.

folders-shares-closing = جارٍ إغلاق { $name }…

folders-shares-close-failed = تعذّر إغلاق { $name }: { $why }.

folders-shares-reconnecting = جارٍ إعادة الاتصال بـ { $name }…

folders-shares-not-remembered = تعذّر تذكّر المشاركات للمرة القادمة.
