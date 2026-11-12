# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-cancelled = لم تُدخل كلمة مرور المسؤول
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-wrong = لم تُقبل كلمة مرور المسؤول
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-not-allowed = لا يُسمح لحسابك بفعل ذلك حتى مع كلمة مرور المسؤول
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unasked = لم تكن هناك طريقة لطلب كلمة مرور المسؤول منك
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unclear = لم تُقبل كلمة مرور المسؤول، أو لا يُسمح لحسابك بفعل ذلك
# Sentence of its own saying a step was refused: the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-sentence-cancelled = لم تُدخل كلمة مرور المسؤول
# Sentence of its own saying a step was refused: the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-sentence-wrong = لم تُقبل كلمة مرور المسؤول
# Sentence of its own saying a step was refused: this login may never do it.
# SAFETY: needs a human review before it ships
common-refused-sentence-not-allowed = لا يُسمح لحسابك بفعل ذلك حتى مع كلمة مرور المسؤول
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
# SAFETY: needs a human review before it ships
common-refused-sentence-unasked = لم تكن هناك طريقة لطلب كلمة مرور المسؤول منك
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
# SAFETY: needs a human review before it ships
common-refused-sentence-unclear = لم تُقبل كلمة مرور المسؤول، أو لا يُسمح لحسابك بفعل ذلك
# Red words when a step was refused because the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-not-done-cancelled = لم يتم ذلك لأنك لم تُدخل كلمة مرور المسؤول.
# Red words when a step was refused because the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-not-done-wrong = لم يتم ذلك لأن كلمة مرور المسؤول لم تُقبل.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-not-allowed = لم يتم ذلك. لا يُسمح لحسابك بفعله حتى مع كلمة مرور المسؤول. اخرج ثم ادخل مرة أخرى، ثم حاول من جديد.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unasked = لم يتم ذلك. لم تكن هناك طريقة لطلب كلمة مرور المسؤول منك. اخرج ثم ادخل مرة أخرى، ثم حاول من جديد.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unclear = لم يتم ذلك. لم تُقبل كلمة مرور المسؤول، أو لا يُسمح لحسابك بفعله. إن لم يُطلب منك إدخال كلمة مرور، فاخرج ثم ادخل مرة أخرى، ثم حاول من جديد.
# Red words when a step behind the administrator's password failed and said nothing of why.
# SAFETY: needs a human review before it ships
common-refused-no-reason = لم ينجح ذلك ولم يُذكر السبب.
# Red words when the keyring will not open without the person's password.
common-keyring-locked = سلسلة المفاتيح مقفلة. افتح قفلها، أو اخرج ثم ادخل مرة أخرى، ثم حاول من جديد.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = لا توجد سلسلة مفاتيح على هذا الحاسوب لحفظه فيها.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = تعذّر على Folders الوصول إلى سلسلة المفاتيح في الوقت المناسب.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = توقف Folders عن انتظار سلسلة المفاتيح.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = تعذّر على Folders الوصول إلى سلسلة المفاتيح ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] ما زال Folders ينتظر سلسلة المفاتيح، لذلك لم يُزَل { $what } بعد.
       *[other] ما زال Folders ينتظر سلسلة المفاتيح، لذلك لم يُحفَظ { $what } بعد.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] بعد تأخير، أُزيل { $what } من سلسلة المفاتيح.
       *[other] بعد تأخير، حُفظ { $what } في سلسلة المفاتيح.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] توقف Folders عن انتظار سلسلة المفاتيح، فلم يُزَل { $what }.
       *[other] توقف Folders عن انتظار سلسلة المفاتيح، فلم يُحفَظ { $what }.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] تعذّر على Folders إزالة { $what } من سلسلة المفاتيح. { $why }
       *[other] تعذّر على Folders حفظ { $what } في سلسلة المفاتيح. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = المزيد
common-more-spoken = افتح المساعدة حول هذا
# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
folders-volumes-no-password = لم تُدخل كلمة مرور، فلم يتغير شيء.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }، فلم يتغير شيء.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = ما زال شيء يستخدمه. أغلق النوافذ أو البرامج التي تعرض ملفاته، ثم حاول مرة أخرى.
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = لا توجد مساحة كافية لما عليه بهذا الحجم.
# SAFETY: needs a human review before it ships
# Heading over the password box, and the tip shown in the box while it is empty.
folders-prompt-password = كلمة المرور
# SAFETY: needs a human review before it ships
# Main button at the bottom of the password window that checks the password.
folders-prompt-unlock = فتح القفل
# SAFETY: needs a human review before it ships
# Button at the bottom of the password window that closes it without unlocking.
folders-prompt-cancel = إلغاء
# SAFETY: needs a human review before it ships
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = اكتب كلمة المرور أولًا.
# Shown under the password box while the password is being checked.
folders-prompt-checking = جارٍ التحقق…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = أكبر من 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = أكبر من شاشتك
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = أكبر من 20 MB (200 MB إن كانت متحركة)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = أكبر من 20 MB
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = أكبر من 200 MB
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } أو { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = مقفل: { $name }

# Name given to a drive that has none.
folders-volumes-drive = المحرك

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = لم تفتح عبارة المرور هذه القرص.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = هو مفتوح بالفعل.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = يلزم إدخال عبارة مرور لفتحه.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = عبارة المرور لـ { $name }

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = افتح قفل المحرك

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = تعذّر على Folders فتح نافذة عبارة المرور.
