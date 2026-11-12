
# machine translated (Arabic), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = لا يسمح { $server } لهذا الحاسوب بفتح { $export }.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = لا يشارك { $server } مجلداً باسم { $export }.

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = ستُسأل مرة أخرى في المرة القادمة

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = بقي أقل من يوم

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [zero] لم يبقَ أي يوم
        [one] بقي يوم واحد
        [two] بقي يومان
        [few] بقيت { $n } أيام
        [many] بقي { $n } يومًا
       *[other] بقي { $n } يوم
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
folders-shares-held =
    { $past ->
        [yes] { $server } موجود الآن على العنوان { $now }. استُخدم تسجيل دخولك له سابقًا على عنوان مختلف ({ $before }).
       *[no] { $server } على العنوان { $now }. لم يُستخدم تسجيل دخولك له على هذا العنوان من قبل.
    }{ $full ->
        [yes] {" "}يتذكر Folders آخر { $most } من العناوين فقط.
       *[no] {""}
    } يمكن لأي جهاز على الشبكة أن يدّعي هذا الاسم، لذلك لم يرسل Folders تسجيل الدخول المحفوظ. إن كان هذا { $server } الخاص بك، فاكتب اسم المستخدم وكلمة المرور لاستخدامه هنا.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when no address could be found for the computer to show.
folders-shares-nowhere = تعذّر على Folders العثور على عنوان { $server }، لذلك لم يُرسَل إليه شيء. لا يمكن إرسال ما تكتبه هنا بأمان. افتحه بعنوانه بدلًا من ذلك، مثل smb://192.168.1.20/.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
folders-shares-lapsed = { $server } على العنوان { $now }. استُخدم تسجيل دخولك له هناك آخر مرة قبل أكثر من { $days } يومًا. ربما أُعطي جهاز آخر هذا العنوان منذ ذلك الحين، لذلك لم يرسل Folders تسجيل الدخول المحفوظ. إن كان هذا { $server } الخاص بك، فاكتب اسم المستخدم وكلمة المرور لاستخدامه هنا.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
folders-shares-unknown = { $server } على العنوان { $now }. استُخدم له تسجيل دخول هناك من قبل، لكن Folders لم يدوّن متى ولا أي تسجيل دخول. ربما أُعطي جهاز آخر هذا العنوان منذ ذلك الحين، لذلك لم يرسل Folders تسجيل الدخول المحفوظ. إن كان هذا { $server } الخاص بك، فاكتب اسم المستخدم وكلمة المرور لاستخدامه هنا.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
folders-shares-unshown = { $server } على العنوان { $now }، حيث استُخدم تسجيل دخولك له سابقًا. لكن Folders عثر عليه هذه المرة على الشبكة المحلية، حيث يمكن لأي جهاز استخدام هذا الاسم. لم ترَ هذا العنوان من قبل، لذلك لم يرسل Folders تسجيل دخولك. إن كان هذا { $server } الخاص بك، فاكتب اسم المستخدم وكلمة المرور لاستخدامه هنا.

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
folders-shares-asking =
    اكتب اسم المستخدم وكلمة المرور لحساب على { $server }.{ $heard ->
        [yes] {" "}إنه على العنوان { $at }. يمكن لأي جهاز على الشبكة أن يدّعي اسمًا، فتأكد أن هذا هو عنوانه قبل أن تكتب.
       *[no] {""}
    }

# SAFETY: needs a human review before it ships
# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
folders-shares-where =
    ما تكتبه يذهب إلى { $address }{ $heard ->
        [yes] {" "}ولا يذهب إلى مكان آخر. وعند تشغيل التذكر، يحتفظ Folders بتسجيل دخولك لهذا العنوان فقط.
       *[no] {"."}
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
folders-shares-again = عثر Folders على { $server } في الشبكة المحلية على { $at }، المنفذ { $port }. يمكن لأي جهاز استخدام هذا الاسم هناك، لذلك لم يرسل Folders تسجيل دخولك. إن كان هذا هو العنوان الصحيح، فاضغط فتح.

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = تعذّر على Folders الوصول إلى سلسلة المفاتيح: { $why }.

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = فُتح، لكن تعذّر على Folders حفظ تسجيل الدخول. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = لا يوفّر { $server } الإصدار 4 من NFS، وهو الإصدار الوحيد الذي يستطيع Folders فتحه.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = فُتحت المشاركة، لكن تعذّر على Folders معرفة مكانها على هذا الحاسوب.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = فُتحت المشاركة، لكن تعذّر على Folders معرفة مكانها على هذا الحاسوب.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = هذا ليس عنوانًا.

# SAFETY: needs a human review before it ships
# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = العناوين المستخدمة لـ smb://{ $server }

# SAFETY: needs a human review before it ships
# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
folders-shares-password-label = كلمة مرور { $user } على { $what }

# SAFETY: needs a human review before it ships
# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
folders-shares-remember = تذكّر لمدة 90 يومًا

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = تبدأ مدة الـ 90 يومًا من جديد في كل مرة تستخدمه.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } على { $server }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = فتح { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = إغلاق

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = اسم المستخدم، مثل jo أو WORKGROUP\jo

# SAFETY: needs a human review before it ships
# Hint in the empty password box of the sign-in card.
folders-shares-password = كلمة المرور

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
folders-shares-forget = نسيان العناوين السابقة

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
folders-shares-forget-spoken = نسيان العناوين السابقة لتسجيل الدخول هذا

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = جارٍ الفتح…

# Name of the main button of the sign-in card.
folders-shares-open = فتح

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = هذا ليس اسم حاسوب.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = هذا ليس مسار مجلد مشترك.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = جارٍ فتح { $export } على { $host }…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = جارٍ فتح { $share } على { $server }…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = أنهِ تسجيل الدخول إلى { $it } أولًا.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = فُتح. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = لا يشارك { $server } أي مجلدات معك.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = جارٍ إغلاق { $name }…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = تعذّر على Folders إغلاق { $name }: { $why }.

# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = جارٍ إعادة الاتصال بـ { $name }…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = تعذّر على Folders حفظ مشاركاتك للمرة القادمة.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = لا يمكن استخدام اسم المستخدم هذا لمشاركة.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = لا يمكن استخدام هذا النطاق لمشاركة.

# Red words when a share's password holds a line break.
folders-shares-bad-password = لا يمكن أن تحتوي كلمة مرور المشاركة على سطر جديد.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = يحتاج { $server } إلى اسم مستخدم وكلمة مرور لفتح { $it }.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = اسم المستخدم أو كلمة المرور غير صحيحين لفتح { $it }.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = حسابك على { $server } مقفل أو معطّل، أو انتهت صلاحية كلمة مروره.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = لا يوجد على { $server } مجلد مشترك باسم { $share }.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = لا يُسمح لهذا الحساب بفتح { $it }.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = لم يردّ { $server }. ربما هو في وضع السكون أو مطفأ: أيقظه وحاول مرة أخرى.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = تعذّر على Folders العثور على حاسوب باسم { $server } في هذه الشبكة.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = لا يستخدم { $server } سوى SMB1، وهي طريقة مشاركة قديمة لم تعد آمنة. لا يستخدمها Folders.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = يحتاج هذا إلى حزمة { $package } من Debian، وهي غير مثبّتة.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = تعذّر على Folders فتح المشاركة.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = تعذّر على Folders فتح المجلد المشترك.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = تعذّر على Folders عرض مجلداته المشتركة.
