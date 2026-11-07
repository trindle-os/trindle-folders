

folders-sharing-cannot-read = 无法读取此文件夹。

folders-sharing-only-folder = 只能共享文件夹本身，不能共享指向文件夹的链接。

folders-sharing-only-yours = 只能共享属于你的文件夹。这个文件夹属于其他人。

folders-sharing-your-home = 你的主文件夹

folders-sharing-this-folder = 此文件夹

folders-sharing-and = { $first }和{ $second }

folders-sharing-share = 共享此文件夹

folders-sharing-shared = 已在家庭网络中共享。

folders-sharing-not-shared = 未共享。

folders-sharing-share-tip = 与家庭网络中的其他电脑共享此文件夹

folders-sharing-name = 名称

folders-sharing-name-tip = 其他电脑看到的名称

folders-sharing-write = 其他人可以更改文件

folders-sharing-write-sub = 他们可以在其中添加、更改和删除文件。

folders-sharing-guests = 允许任何人进入

folders-sharing-guests-sub = 不要求密码。

folders-sharing-guests-tip = 允许任何人进入，无需密码

folders-sharing-explain =
    { $fetch ->
        [yes] 共享后，家里的其他电脑（包括 Windows 电脑）会在“网络”下看到它。第一次共享时，Trindle 会获取 Samba（用于共享文件夹的程序），大约需要一分钟。
       *[no] 共享后，家里的其他电脑（包括 Windows 电脑）会在“网络”下看到它。
    }

folders-sharing-find-it =
    { $host_known ->
        [yes] 其他电脑会在“网络”下找到它，名称为 { $host } 上的 { $name }。
       *[no] 其他电脑会在“网络”下找到它，名称为 { $name }。
    }

folders-sharing-cannot-reach = 其他电脑暂时无法访问它：“设置”的“网络”中，“允许其他电脑连接”已关闭。

folders-sharing-let-connect = 允许其他电脑连接

folders-sharing-visitors =
    { $n ->
       *[other] 没有密码的访客暂时无法打开它：{ $names } 不对其他账户开放。让他们通过只会打开通往此文件夹的路径，他们看不到那里的其他内容。
    }

folders-sharing-let-through = 让访客通过并进入此文件夹

folders-sharing-sign-in-set = 其他人以 { $user } 的身份登录，使用为共享设置的密码。

folders-sharing-change-password = 更改其他电脑使用的密码

folders-sharing-set-password = 其他人以 { $user } 的身份登录。请设置其他电脑打开你的共享文件夹所用的密码，至少 { $n } 个字符。它与你的登录密码是分开保存的。

folders-sharing-password = 密码

folders-sharing-password-tip = 共享密码

folders-sharing-keep-password = 保存此密码

folders-sharing-card-title = 在网络上共享 { $name }

folders-sharing-name-taken = 你的另一个共享文件夹已使用了这个名称。请换一个。

folders-sharing-stopping = 正在停止共享…

folders-sharing-password-short = 密码至少需要 { $n } 个字符。

folders-sharing-keeping-password = 正在保存密码；会先询问管理员密码。

folders-sharing-letting-connect = 正在允许其他电脑连接；会先询问管理员密码。

folders-sharing-fetching = 正在获取并设置 Samba（用于共享文件夹的程序）。这只在第一次发生，并且会先询问管理员密码。

folders-sharing-setting-up = 正在设置共享。这只在第一次发生，并且会先询问管理员密码。

folders-sharing-changing = 正在更改共享…

folders-sharing-sharing-it = 正在共享…

