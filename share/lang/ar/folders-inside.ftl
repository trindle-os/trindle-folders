
# machine translated (Arabic), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = حدّد شيئاً في الأرشيف أولاً.

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = لا يمكنك إخراج الأشياء من أرشيف إلا إلى مجلد على هذا الحاسوب.

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = تعذّر على Folders قراءة { $name }: { $why }.

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
        [zero] لم يتعذّر على Folders إخراج أي عنصر.
        [one] تعذّر على Folders إخراج عنصر واحد، لأن اسمه يشير إلى خارج المجلد أو لأن شيئاً يعترض الطريق.
        [two] تعذّر على Folders إخراج عنصرين، لأن اسميهما يشيران إلى خارج المجلد أو لأن شيئاً يعترض الطريق.
        [few] تعذّر على Folders إخراج { $n } عناصر، لأن أسماءها تشير إلى خارج المجلد أو لأن شيئاً يعترض الطريق.
        [many] تعذّر على Folders إخراج { $n } عنصراً، لأن أسماءها تشير إلى خارج المجلد أو لأن شيئاً يعترض الطريق.
       *[other] تعذّر على Folders إخراج { $n } عنصر، لأن أسماءها تشير إلى خارج المجلد أو لأن شيئاً يعترض الطريق.
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = تعذّر على Folders إخراجه من الأرشيف: { $why }.

