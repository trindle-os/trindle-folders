

folders-shares-nfs-denied = { $server } 不允许这台电脑打开 { $export }。

folders-shares-nfs-no-share = { $server } 没有名为 { $export } 的共享。

folders-shares-left-none = 下次打开时会再次询问

folders-shares-left-under-day = 剩余不到一天

folders-shares-left-days =
    { $n ->
       *[other] 还剩 { $n } 天
    }

folders-shares-held =
    { $past ->
        [yes] { $server } 现在在 { $now } 上回应，这与你以前使用该登录的地址（{ $before }）不同。
       *[no] { $server } 在 { $now } 上回应，你的登录以前没有在这个地址上使用过。
    }{ $full ->
        [yes] {" "}只记得它最近使用过的 { $most } 个地址。
       *[no] {""}
    }网络上的任何设备都可以冒用一个名称来回应，所以没有发送为它保存的登录。如果这确实是你的 { $server }，请输入用户名和密码在此使用。

folders-shares-nowhere = 事先找不到 { $server } 所回应的地址，所以没有向它发送任何内容，在这里输入的内容也没有安全的去处。请改用它的地址打开，例如 smb://192.168.1.20/。

folders-shares-lapsed = { $server } 在 { $now } 上回应，你的登录上次在这个地址使用是在 { $days } 天以前。此后另一台设备可能已被分配了这个地址，所以没有发送为它保存的登录。如果这确实是你的 { $server }，请输入用户名和密码在此使用。

folders-shares-unknown = { $server } 在 { $now } 上回应，以前曾在这个地址使用过它的登录，但没有记下使用的时间或是哪个登录。此后另一台设备可能已被分配了这个地址，所以没有发送为它保存的登录。如果这确实是你的 { $server }，请输入用户名和密码在此使用。

folders-shares-unshown = { $server } 在 { $now } 上回应，这是你的登录以前发送到的地址。这次该地址只是在网络上听到的，任何设备都可以冒用一个名称来回应，而且这个地址以前没有在这里向你显示过，所以没有发送登录。如果这确实是你的 { $server }，请输入用户名和密码在此使用。

folders-shares-asking =
    { $server } 需要用户名和密码，应是那台电脑上某个账户的用户名和密码。{ $heard ->
        [yes] {" "}它在 { $at } 上回应。网络上的任何设备都可以冒用一个名称来回应，所以输入之前请确认这是它的地址。
       *[no] {""}
    }

folders-shares-where =
    你输入的内容会发送到 { $address }{ $heard ->
        [yes] {" "}，不会发送到其他地方。打开“记住”后，你的登录只会为该地址保存。
       *[no] {"。"}
    }

folders-shares-again = { $server } 在 { $at }（端口 { $port }）上回应。这个地址只是在网络上听到的，任何设备都可以冒用一个名称来回应，所以没有发送你的登录。如果这确实是它的地址，请按“打开”。

folders-shares-keyring-unreachable = 无法连接密钥环：{ $why }。

folders-shares-unkept = 已打开，但没有保存登录。{ $why }

folders-shares-nfs-version = { $server } 不提供 NFS 第 4 版，而 Trindle 只打开这一种。

folders-shares-nfs-opened-unknown = 已打开，但不知道它在哪里。

folders-shares-opened-unknown = 共享已打开，但不知道它在哪里。

folders-shares-not-address = 这不是一个地址。

folders-shares-seen-label = smb://{ $server } 打开过的地址

folders-shares-password-label = { $user } 在 { $what } 上的密码

folders-shares-remember = 记住 90 天

folders-shares-remember-line = 每次使用时重新开始计算。

folders-shares-share-on = { $server } 上的 { $share }

folders-shares-card-title = 打开 { $it }

folders-shares-close = 关闭

folders-shares-user-placeholder = 用户名，例如 jo 或 WORKGROUP\jo

folders-shares-password = 密码

folders-shares-forget = 忘记以前的地址

folders-shares-forget-spoken = 忘记此登录以前使用过的地址

folders-shares-opening = 正在打开

folders-shares-open = 打开

folders-shares-bad-computer = 这不是电脑的名称。

folders-shares-bad-path = 这不是共享文件夹的路径。

folders-shares-opening-export = 正在打开 { $host } 上的 { $export }…

folders-shares-opening-share = 正在打开 { $server } 上的 { $share }…

folders-shares-answer-card = 请先回应 { $it } 的卡片。

folders-shares-open-waiting = 已打开。{ $words }

folders-shares-none = { $server } 没有向你共享任何文件夹。

folders-shares-closing = 正在关闭 { $name }…

folders-shares-close-failed = 无法关闭 { $name }：{ $why }。

folders-shares-reconnecting = 正在重新连接 { $name }…

folders-shares-not-remembered = 无法记住这些共享，下次需要重新添加。
