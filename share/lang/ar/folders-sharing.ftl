# machine translated (Arabic), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = لا يستطيع Folders قراءة ما في هذا المجلد.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = يمكن مشاركة المجلد نفسه فقط، لا رابط إليه.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = يمكن مشاركة المجلدات التي تخصك فقط. هذا المجلد يخص شخصًا آخر.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = مجلدك الرئيسي

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = هذا المجلد

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } و{ $second }

# Name of the switch that shares the folder.
folders-sharing-share = شارك هذا المجلد

# One line under the share switch when the folder is shared.
folders-sharing-shared = مشارَك على الشبكة المنزلية.


# Tip of the share switch.
folders-sharing-share-tip = المشاركة على الشبكة المنزلية

# Label over the box for the name other computers see.
folders-sharing-name = الاسم

# Tip of the box for the name other computers see.
folders-sharing-name-tip = الاسم الذي تراه الحواسيب الأخرى

# Name of the switch that lets others change files, and its tip.
# SAFETY: needs a human review before it ships
folders-sharing-write = اسمح للآخرين بتغيير الملفات

# One line under the switch that lets others change files.
# SAFETY: needs a human review before it ships
folders-sharing-write-sub = يمكنهم إضافة الملفات وتغييرها وحذفها فيه.

# Name of the switch that lets anyone in without a password.
# SAFETY: needs a human review before it ships
folders-sharing-guests = اسمح للجميع بالدخول

# One line under the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-sub = لا تُطلب كلمة مرور.

# Tip of the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-tip = اسمح للجميع بالدخول بلا كلمة مرور

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] بعد مشاركته يظهر تحت «الشبكة» على حواسيبك الأخرى في المنزل، ومنها حواسيب ويندوز. في المرة الأولى ينزّل Trindle برنامج Samba الذي يشارك المجلدات، وهذا يستغرق دقيقة.
       *[no] بعد مشاركته يظهر تحت «الشبكة» على حواسيبك الأخرى في المنزل، ومنها حواسيب ويندوز.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] على الحواسيب الأخرى، ابحث تحت «الشبكة» عن { $name } على { $host }.
       *[no] على الحواسيب الأخرى، ابحث تحت «الشبكة» عن { $name }.
    }

# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = لا تستطيع الحواسيب الأخرى الوصول إلى هذا المجلد بعد. في الإعدادات، افتح «الشبكة» وشغّل «اسمح للحواسيب الأخرى بالاتصال».

# Tip of the button that lets other computers connect.
folders-sharing-let-connect = اسمح للحواسيب الأخرى بالاتصال

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
# SAFETY: needs a human review before it ships
folders-sharing-visitors =
    { $n ->
        [one] لا يستطيع الضيوف بلا كلمة مرور فتحه بعد، لأن { $names } مغلق أمام الحسابات الأخرى. إذا سمحت لهم بالمرور، يمكنهم الوصول إلى هذا المجلد لكنهم لا يرون أي شيء آخر في الطريق إليه.
        [two] لا يستطيع الضيوف بلا كلمة مرور فتحه بعد، لأن { $names } مغلقان أمام الحسابات الأخرى. إذا سمحت لهم بالمرور، يمكنهم الوصول إلى هذا المجلد لكنهم لا يرون أي شيء آخر في الطريق إليه.
       *[other] لا يستطيع الضيوف بلا كلمة مرور فتحه بعد، لأن { $names } مغلقة أمام الحسابات الأخرى. إذا سمحت لهم بالمرور، يمكنهم الوصول إلى هذا المجلد لكنهم لا يرون أي شيء آخر في الطريق إليه.
    }

# Tip of the button that lets visitors through to the folder.
# SAFETY: needs a human review before it ships
folders-sharing-let-through = اسمح للضيوف بالوصول إلى هذا المجلد

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
# SAFETY: needs a human review before it ships
folders-sharing-sign-in-set = يسجّل الآخرون الدخول باسم { $user } بكلمة مرور المشاركة.

# Tip of the button that changes the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-change-password = غيّر كلمة مرور المشاركة

# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-set-password = يسجّل الآخرون الدخول باسم { $user }. عيّن كلمة مرور للمشاركة لا يقل طولها عن { $n } من الأحرف. وهي منفصلة عن كلمة مرور دخولك.

# Label beside the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password = كلمة المرور

# Tip of the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password-tip = كلمة مرور المشاركة

# Tip of the button that keeps the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-keep-password = حفظ كلمة المرور

# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = شارك { $name } على الشبكة

# Red words when the share's name is used by another share.
folders-sharing-name-taken = لديك مجلد مشترك بهذا الاسم بالفعل. اختر اسمًا آخر.

# Calm words while sharing is being stopped.
folders-sharing-stopping = جارٍ إيقاف المشاركة…

# Red words when the sharing password is too short; { $n } is the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-password-short = يجب ألا يقل طول كلمة المرور عن { $n } من الأحرف.

# Calm words while the sharing password is being kept.
# SAFETY: needs a human review before it ships
folders-sharing-keeping-password = جارٍ حفظ كلمة المرور. ستُسأل عن كلمة مرور المسؤول أولًا.

# Calm words while other computers are being let connect.
# SAFETY: needs a human review before it ships
folders-sharing-letting-connect = جارٍ السماح للحواسيب الأخرى بالاتصال. ستُسأل عن كلمة مرور المسؤول أولًا.

# Calm words while Samba is fetched and set up the first time.
# SAFETY: needs a human review before it ships
folders-sharing-fetching = جارٍ تنزيل Samba، البرنامج الذي يشارك المجلدات، وإعداده. ستُسأل عن كلمة مرور المسؤول أولًا. يحدث هذا في المرة الأولى فقط.

# Calm words while sharing is set up the first time and Samba is already there.
# SAFETY: needs a human review before it ships
folders-sharing-setting-up = جارٍ إعداد المشاركة. ستُسأل عن كلمة مرور المسؤول أولًا. يحدث هذا في المرة الأولى فقط.

# Calm words while an existing share is changed.
folders-sharing-changing = جارٍ تغيير المشاركة…

# Calm words while a share is made.
folders-sharing-sharing-it = جارٍ مشاركة المجلد…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = أعطِ المجلد المشترك اسمًا.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = لا يمكن أن يزيد اسم المجلد المشترك على { $most } من الأحرف.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = لا يمكن أن يحتوي اسم المجلد المشترك على أي من % < > * ? | / \ + = ; : " أو فاصلة.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = اسم ينتهي بـ $ سيُخفي المشاركة، لذلك لا يمكنك استخدامه.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = لا يمكن أن يبدأ اسم المجلد المشترك بشرطة.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = يستخدم Samba الاسم { $name } بالفعل. اختر اسمًا آخر.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = مشترك

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = يمكن مشاركة المجلدات التي تخصك فقط.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = هذا الاسم هو اسم حساب موجود على هذا الحاسوب، فلا يمكنك استخدامه لمشاركة. اختر اسمًا آخر.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = إعدادات Samba في هذا الحاسوب لا تسمح بمشاركات بلا كلمة مرور.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = لا يستطيع هذا الحاسوب مشاركة المزيد من المجلدات.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = هذا المجلد لم يعد موجودًا.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = لم يذكر Samba السبب.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = رسالة Samba: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } يخص شخصًا آخر، فلا يمكنك منح الضيوف حق الوصول إليه من هنا.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = تعذّر على Folders منح الضيوف حق الوصول إلى { $path }: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = تعذّر على Trindle تنزيل Samba لأن هذا الحاسوب يبدو غير متصل. اتصل بالإنترنت وحاول مرة أخرى.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = تعذّر تثبيت Samba بينما يجري تثبيت شيء آخر. حاول مرة أخرى بعد بضع دقائق.
