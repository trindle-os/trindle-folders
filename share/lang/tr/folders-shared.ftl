# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-cancelled = yönetici parolasını girmediniz
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-wrong = yönetici parolası kabul edilmedi
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-not-allowed = hesabınızın bunu yapmaya izni yok, yönetici parolasıyla bile
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unasked = size yönetici parolasını sormanın bir yolu yoktu
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unclear = yönetici parolası kabul edilmedi ya da hesabınızın bunu yapmaya izni yok
# Sentence of its own saying a step was refused: the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-sentence-cancelled = Yönetici parolasını girmediniz
# Sentence of its own saying a step was refused: the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-sentence-wrong = Yönetici parolası kabul edilmedi
# Sentence of its own saying a step was refused: this login may never do it.
# SAFETY: needs a human review before it ships
common-refused-sentence-not-allowed = Hesabınızın bunu yapmaya izni yok, yönetici parolasıyla bile
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
# SAFETY: needs a human review before it ships
common-refused-sentence-unasked = Size yönetici parolasını sormanın bir yolu yoktu
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
# SAFETY: needs a human review before it ships
common-refused-sentence-unclear = Yönetici parolası kabul edilmedi ya da hesabınızın bunu yapmaya izni yok
# Red words when a step was refused because the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-not-done-cancelled = Yönetici parolasını girmediğiniz için bu yapılmadı.
# Red words when a step was refused because the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-not-done-wrong = Yönetici parolası kabul edilmediği için bu yapılmadı.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-not-allowed = Bu yapılmadı. Hesabınızın bunu yapmaya izni yok, yönetici parolasıyla bile. Oturumu kapatıp yeniden açın, sonra tekrar deneyin.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unasked = Bu yapılmadı. Size yönetici parolasını sormanın bir yolu yoktu. Oturumu kapatıp yeniden açın, sonra tekrar deneyin.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unclear = Bu yapılmadı. Yönetici parolası kabul edilmedi ya da hesabınızın bunu yapmaya izni yok. Sizden parola istenmediyse oturumu kapatıp yeniden açın, sonra tekrar deneyin.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Bu işe yaramadı ve bir neden belirtilmedi.
# Red words when the keyring will not open without the person's password.
# SAFETY: needs a human review before it ships
common-keyring-locked = Anahtar halkası kilitli. Kilidini açın ya da oturumu kapatıp yeniden açın, sonra tekrar deneyin.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = Bu bilgisayarda bunu kaydedecek bir anahtar halkası yok.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Folders anahtar halkasına zamanında ulaşamadı.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Folders anahtar halkasını beklemekten vazgeçti.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Folders anahtar halkasına erişemedi ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Folders hâlâ anahtar halkasını bekliyor, bu yüzden { $what } henüz kaldırılmadı.
       *[other] Folders hâlâ anahtar halkasını bekliyor, bu yüzden { $what } henüz kaydedilmedi.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] Bir gecikmeden sonra { $what } anahtar halkasından kaldırıldı.
       *[other] Bir gecikmeden sonra { $what } anahtar halkasına kaydedildi.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Folders anahtar halkasını beklemekten vazgeçti, bu yüzden { $what } kaldırılmadı.
       *[other] Folders anahtar halkasını beklemekten vazgeçti, bu yüzden { $what } kaydedilmedi.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] { $what } anahtar halkasından kaldırılamadı. { $why }
       *[other] { $what } anahtar halkasına kaydedilemedi. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Daha fazla
common-more-spoken = Bu konuda Yardım'ı aç
# SAFETY: needs a human review before it ships
# Red words when the password box was closed with no password. Said whole, as a sentence.
folders-volumes-no-password = Parola girmediniz, bu yüzden hiçbir şey değiştirilmedi.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, bu yüzden hiçbir şey değiştirilmedi.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = Bir şey hâlâ onu kullanıyor. Dosyalarını gösteren pencereleri veya programları kapatın, sonra yeniden deneyin.
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = Üzerindekiler için istenen boyutta yeterli yer yok.
# Heading over the password box, and the tip shown in the box while it is empty.
# SAFETY: needs a human review before it ships
folders-prompt-password = Parola
# Main button at the bottom of the password window that checks the password.
# SAFETY: needs a human review before it ships
folders-prompt-unlock = Kilidi aç
# Button at the bottom of the password window that closes it without unlocking.
# SAFETY: needs a human review before it ships
folders-prompt-cancel = İptal
# Shown under the password box when Unlock is pressed with the box empty.
# SAFETY: needs a human review before it ships
folders-prompt-type-first = Önce parolayı yazın.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Denetleniyor…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = 4K'dan büyük
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = ekranınızdan büyük
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = 20 MB üzeri (hareketliyse 200 MB)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = 20 MB üzeri
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = 200 MB üzeri
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } veya { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Kilitli: { $name }

# Name given to a drive that has none.
folders-volumes-drive = Sürücü

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = Bu parola ibaresi sürücünün kilidini açmadı.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = Zaten açık.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = Açmak için parola ibaresi gerekir.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = { $name } için parola ibaresi

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Sürücünün kilidini aç

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders parola ibaresi penceresini açamadı.
