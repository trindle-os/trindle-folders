
# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = 这台电脑不被允许打开 { $server } 上的 { $export }。

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } 没有共享名为 { $export } 的文件夹。

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = 下次会再次询问你

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = 剩余不到一天

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
       *[other] 还剩 { $n } 天
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
folders-shares-held =
    { $past ->
        [yes] { $server } 现在位于 { $now }。你以前在另一个地址（{ $before }）使用过它的登录。
       *[no] { $server } 位于 { $now }。你的登录以前没有在这个地址使用过。
    }{ $full ->
        [yes] {" "}Folders 只记得最近 { $most } 个地址。
       *[no] {""}
    }网络上的任何设备都可以占用一个名称，所以 Folders 没有发送你保存的登录。如果这确实是你的 { $server }，请输入用户名和密码在此使用。

# SAFETY: needs a human review before it ships
# Words on the sign-in card when no address could be found for the computer to show.
folders-shares-nowhere = Folders 找不到 { $server } 的地址，所以没有向它发送任何内容。你在这里输入的内容无法安全发送。请改用它的地址打开，例如 smb://192.168.1.20/。

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
folders-shares-lapsed = { $server } 位于 { $now }。你的登录上次在那里使用，距今已超过 { $days } 天。此后另一台设备可能已被分配了这个地址，所以 Folders 没有发送你保存的登录。如果这确实是你的 { $server }，请输入用户名和密码在此使用。

# SAFETY: needs a human review before it ships
# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
folders-shares-unknown = { $server } 位于 { $now }。你以前在那里使用过它的登录，但 Folders 没有记下是什么时候，或是哪个登录。此后另一台设备可能已被分配了这个地址，所以 Folders 没有发送你保存的登录。如果这确实是你的 { $server }，请输入用户名和密码在此使用。

# SAFETY: needs a human review before it ships
# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
folders-shares-unshown = { $server } 位于 { $now }，你以前的登录就是用在这个地址。这次 Folders 是在局域网中找到它的，任何设备都可以使用这个名称。你以前没有见过这个地址，所以 Folders 没有发送你的登录。如果这确实是你的 { $server }，请输入用户名和密码在此使用。

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
folders-shares-asking =
    请输入 { $server } 上某个账户的用户名和密码。{ $heard ->
        [yes] {" "}它位于 { $at }。网络上的任何设备都可以占用一个名称，所以输入之前请确认这是它的地址。
       *[no] {""}
    }

# SAFETY: needs a human review before it ships
# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
folders-shares-where =
    你输入的内容会发送到 { $address }{ $heard ->
        [yes] {" "}，不会发送到其他地方。打开“记住”后，Folders 只会为该地址保存你的登录。
       *[no] {"。"}
    }

# SAFETY: needs a human review before it ships
# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
folders-shares-again = Folders 在局域网中的 { $at }（端口 { $port }）找到了 { $server }。局域网中任何设备都可以使用这个名称，所以 Folders 没有发送你的登录。如果这个地址是对的，请按“打开”。

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders 无法连接密钥环：{ $why }。

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = 已打开，但 Folders 无法保存登录。{ $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } 不提供 NFS 第 4 版，而 Folders 只能打开这一个版本。

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = 共享已打开，但 Folders 找不到它在这台电脑上的位置。

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = 共享已打开，但 Folders 找不到它在这台电脑上的位置。

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = 这不是一个地址。

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = 用于 smb://{ $server } 的地址

# SAFETY: needs a human review before it ships
# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
folders-shares-password-label = { $user } 在 { $what } 上的密码

# SAFETY: needs a human review before it ships
# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
folders-shares-remember = 记住 90 天

# SAFETY: needs a human review before it ships
# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = 这 90 天会在每次使用时重新开始计算。

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $server } 上的 { $share }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = 打开 { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = 关闭

# SAFETY: needs a human review before it ships
# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = 用户名，例如 jo 或 WORKGROUP\jo

# SAFETY: needs a human review before it ships
# Hint in the empty password box of the sign-in card.
folders-shares-password = 密码

# SAFETY: needs a human review before it ships
# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
folders-shares-forget = 忘记以前的地址

# SAFETY: needs a human review before it ships
# Name a screen reader gives the switch that forgets the earlier addresses of a login.
folders-shares-forget-spoken = 忘记此登录以前使用过的地址

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = 正在打开…

# Name of the main button of the sign-in card.
folders-shares-open = 打开

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = 这不是电脑名称。

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = 这不是共享文件夹的路径。

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = 正在打开 { $host } 上的 { $export }…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = 正在打开 { $server } 上的 { $share }…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = 请先完成 { $it } 的登录。

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = 已打开。{ $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } 没有向你共享任何文件夹。

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = 正在关闭 { $name }…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders 无法关闭 { $name }：{ $why }。


# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = 正在重新连接 { $name }…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders 无法保存你的共享，下次需要重新添加。

# Red words when a share's user name cannot be used.
folders-shares-bad-user = 该用户名不能用于共享。

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = 该域不能用于共享。

# Red words when a share's password holds a line break.
folders-shares-bad-password = 共享的密码中不能有换行。

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } 需要用户名和密码才能打开 { $it }。

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = { $it } 的用户名或密码不正确。

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = 你在 { $server } 上的账户已被锁定或停用，或密码已过期。

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } 没有名为 { $share } 的共享文件夹。

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = 该账户无权打开 { $it }。

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } 没有回应。它可能已休眠或关机。请唤醒它，然后再试。

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders 在此网络上找不到名为 { $server } 的电脑。

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } 只使用 SMB1，这是一种已不再安全的旧共享方式。Folders 不使用它。

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = 这需要 Debian 的 { $package } 软件包，但它尚未安装。

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders 无法打开该共享。

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders 无法打开该共享文件夹。

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders 无法列出它的共享文件夹。
