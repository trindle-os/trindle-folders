# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = 你没有输入管理员密码
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = 管理员密码未被接受
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = 即使输入管理员密码，你的账户也不被允许这样做
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = 没有办法向你询问管理员密码
# SAFETY: needs a human review before it ships
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = 管理员密码未被接受，或者你的账户不被允许这样做
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = 你没有输入管理员密码
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = 管理员密码未被接受
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = 即使输入管理员密码，你的账户也不被允许这样做
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = 没有办法向你询问管理员密码
# SAFETY: needs a human review before it ships
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = 管理员密码未被接受，或者你的账户不被允许这样做
# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = 没有完成，因为你没有输入管理员密码。
# SAFETY: needs a human review before it ships
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = 没有完成，因为管理员密码未被接受。
# SAFETY: needs a human review before it ships
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = 没有完成。即使输入管理员密码，你的账户也不被允许这样做。请注销后重新登录，再试一次。
# SAFETY: needs a human review before it ships
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = 没有完成。没有办法向你询问管理员密码。请注销后重新登录，再试一次。
# SAFETY: needs a human review before it ships
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = 没有完成。管理员密码未被接受，或者你的账户不被允许这样做。如果没有要求你输入密码，请注销后重新登录，再试一次。
# SAFETY: needs a human review before it ships
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = 没有成功，也没有给出原因。
# SAFETY: needs a human review before it ships
# Red words when the keyring will not open without the person's password.
common-keyring-locked = 密钥环已锁定。请将其解锁，或注销后重新登录，然后再试一次。
# SAFETY: needs a human review before it ships
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = 这台电脑上没有可用来保存它的密钥环。
# SAFETY: needs a human review before it ships
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Folders 没能及时联系上密钥环。
# SAFETY: needs a human review before it ships
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Folders 已不再等待密钥环。
# SAFETY: needs a human review before it ships
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Folders 无法访问密钥环（{ $why }）。
# SAFETY: needs a human review before it ships
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Folders 仍在等待密钥环，所以{ $what }还没有被移除。
       *[other] Folders 仍在等待密钥环，所以{ $what }还没有被保存。
    }
# SAFETY: needs a human review before it ships
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] 延迟之后，{ $what }已从密钥环中移除。
       *[other] 延迟之后，{ $what }已保存到密钥环中。
    }
# SAFETY: needs a human review before it ships
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Folders 已不再等待密钥环，所以{ $what }没有被移除。
       *[other] Folders 已不再等待密钥环，所以{ $what }没有被保存。
    }
# SAFETY: needs a human review before it ships
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Folders 无法从密钥环中移除{ $what }。{ $why }
       *[other] Folders 无法把{ $what }保存到密钥环中。{ $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = 更多
common-more-spoken = 打开关于此项的帮助
# SAFETY: needs a human review before it ships
# Red words when the password box was closed with no password. Said whole, as a sentence.
folders-volumes-no-password = 你没有输入密码，所以没有做任何更改。
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }，所以没有做任何更改。
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = 仍有东西在使用它。请关闭所有显示其文件的窗口或程序，然后再试。
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = 按该大小，空间不够放下里面的内容。
# SAFETY: needs a human review before it ships
# Heading over the password box, and the tip shown in the box while it is empty.
folders-prompt-password = 密码
# SAFETY: needs a human review before it ships
# Main button at the bottom of the password window that checks the password.
folders-prompt-unlock = 解锁
# SAFETY: needs a human review before it ships
# Button at the bottom of the password window that closes it without unlocking.
folders-prompt-cancel = 取消
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = 请先输入密码。
# Shown under the password box while the password is being checked.
folders-prompt-checking = 正在检查…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = 大于 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = 大于您的屏幕
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = 超过 20 MB（动态的超过 200 MB）
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = 超过 20 MB
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = 超过 200 MB
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first }或{ $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = 已锁定：{ $name }

# Name given to a drive that has none.
folders-volumes-drive = 驱动器

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = 这个口令无法解锁该驱动器。

# Said when a drive that is open already is opened again.
folders-volumes-already-open = 它已经打开了。

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = 需要口令才能打开它。

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = { $name } 的口令

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = 解锁驱动器

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders 无法打开口令窗口。
