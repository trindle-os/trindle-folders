
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders 无法读取此文件夹中的内容。

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = 只能共享文件夹本身，不能共享指向文件夹的链接。

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = 只能共享属于你的文件夹。这个文件夹属于其他人。

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = 你的主文件夹

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = 此文件夹

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first }和{ $second }

# SAFETY: needs a human review before it ships
# Name of the switch that shares the folder.
folders-sharing-share = 共享此文件夹

# One line under the share switch when the folder is shared.
folders-sharing-shared = 已在家庭网络中共享。


# SAFETY: needs a human review before it ships
# Tip of the share switch.
folders-sharing-share-tip = 在家庭网络中共享

# Label over the box for the name other computers see.
folders-sharing-name = 名称

# Tip of the box for the name other computers see.
folders-sharing-name-tip = 其他电脑看到的名称

# SAFETY: needs a human review before it ships
# Name of the switch that lets others change files, and its tip.
folders-sharing-write = 允许其他人更改文件

# SAFETY: needs a human review before it ships
# One line under the switch that lets others change files.
folders-sharing-write-sub = 他们可以在其中添加、更改和删除文件。

# SAFETY: needs a human review before it ships
# Name of the switch that lets anyone in without a password.
folders-sharing-guests = 允许任何人进入

# SAFETY: needs a human review before it ships
# One line under the let in anyone switch.
folders-sharing-guests-sub = 无需密码。

# SAFETY: needs a human review before it ships
# Tip of the let in anyone switch.
folders-sharing-guests-tip = 允许任何人进入，无需密码

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] 共享后，你家里的其他电脑（包括 Windows 电脑）会在“网络”下看到它。第一次共享时，Trindle 会下载 Samba（用于共享文件夹的程序），这需要一分钟左右。
       *[no] 共享后，你家里的其他电脑（包括 Windows 电脑）会在“网络”下看到它。
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] 在其他电脑上，请在“网络”下查找 { $host } 上的 { $name }。
       *[no] 在其他电脑上，请在“网络”下查找 { $name }。
    }

# SAFETY: needs a human review before it ships
# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = 其他电脑暂时还无法访问此文件夹。请在“设置”中打开“网络”，并打开“允许其他电脑连接”。

# SAFETY: needs a human review before it ships
# Tip of the button that lets other computers connect.
folders-sharing-let-connect = 允许其他电脑连接

# SAFETY: needs a human review before it ships
# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
folders-sharing-visitors =
    { $n ->
       *[other] 没有密码的访客暂时无法打开它，因为 { $names } 不对其他账户开放。如果你让他们通过，他们可以访问此文件夹，但看不到沿途的其他任何内容。
    }

# SAFETY: needs a human review before it ships
# Tip of the button that lets visitors through to the folder.
folders-sharing-let-through = 让访客进入此文件夹

# SAFETY: needs a human review before it ships
# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
folders-sharing-sign-in-set = 其他人以 { $user } 的身份登录，使用共享密码。

# SAFETY: needs a human review before it ships
# Tip of the button that changes the sharing password.
folders-sharing-change-password = 更改共享密码

# SAFETY: needs a human review before it ships
# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
folders-sharing-set-password = 其他人以 { $user } 的身份登录。请设置一个至少 { $n } 个字符的共享密码。它与你的登录密码是分开的。

# SAFETY: needs a human review before it ships
# Label beside the sharing password box.
folders-sharing-password = 密码

# SAFETY: needs a human review before it ships
# Tip of the sharing password box.
folders-sharing-password-tip = 共享密码

# SAFETY: needs a human review before it ships
# Tip of the button that keeps the sharing password.
folders-sharing-keep-password = 保存此密码

# SAFETY: needs a human review before it ships
# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = 在网络上共享 { $name }

# Red words when the share's name is used by another share.
folders-sharing-name-taken = 你已经有一个同名的共享文件夹。请换一个。

# Calm words while sharing is being stopped.
folders-sharing-stopping = 正在停止共享…

# SAFETY: needs a human review before it ships
# Red words when the sharing password is too short; { $n } is the fewest letters.
folders-sharing-password-short = 密码至少需要 { $n } 个字符。

# SAFETY: needs a human review before it ships
# Calm words while the sharing password is being kept.
folders-sharing-keeping-password = 正在保存密码。会先要求你输入管理员密码。

# SAFETY: needs a human review before it ships
# Calm words while other computers are being let connect.
folders-sharing-letting-connect = 正在允许其他电脑连接。会先要求你输入管理员密码。

# SAFETY: needs a human review before it ships
# Calm words while Samba is fetched and set up the first time.
folders-sharing-fetching = 正在下载并设置 Samba（用于共享文件夹的程序）。会先要求你输入管理员密码。这只在第一次发生。

# SAFETY: needs a human review before it ships
# Calm words while sharing is set up the first time and Samba is already there.
folders-sharing-setting-up = 正在设置共享。会先要求你输入管理员密码。这只在第一次发生。

# Calm words while an existing share is changed.
folders-sharing-changing = 正在更改共享…

# Calm words while a share is made.
folders-sharing-sharing-it = 正在共享此文件夹…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = 请给共享文件夹起个名称。

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = 共享文件夹的名称不能超过 { $most } 个字符。

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = 共享文件夹的名称不能包含 % < > * ? | / \ + = ; : " 或逗号。

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = 以 $ 结尾的名称会隐藏该共享，所以不能使用。

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = 共享文件夹的名称不能以短横线开头。

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba 已在使用名称 { $name }。请选择别的名称。

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = 共享

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = 只能共享属于你的文件夹。

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = 这个名称已是这台电脑上的一个账户名，所以不能用于共享。请选择别的名称。

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = 这台电脑的 Samba 设置不允许无密码共享。

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = 这台电脑无法再共享更多文件夹。

# Red words when the folder to share has gone.
folders-sharing-folder-gone = 那个文件夹已经不在了。

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba 没有给出原因。

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Samba 的消息：{ $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } 属于别人，所以你无法在这里让访客访问它。

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders 无法让访客访问 { $path }：{ $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle 无法下载 Samba，因为这台电脑似乎未联网。请连接互联网后再试。

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = 其他程序正在安装，所以 Trindle 无法安装 Samba。请几分钟后再试。
