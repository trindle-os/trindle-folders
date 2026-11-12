
# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
folders-remote-mod-heard-card = Folders 在局域网中的 { $at }（端口 { $port }）找到了 { $host }。局域网中任何设备都可以使用这个名称，所以 Folders 没有发送你的密码。如果这个地址是对的，请按“连接”。

# SAFETY: needs a human review before it ships
# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
folders-remote-mod-heard-stopped = Folders 在局域网中的 { $at } 找到了 { $host }。局域网中任何设备都可以使用这个名称，所以 Folders 没有发送你的密码。请再次打开 { $host } 以核对它的地址。

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders 无法读取该地址：{ $why }。

# SAFETY: needs a human review before it ships
# Red words when a server needs a password or key and none could be asked for.
folders-remote-mod-needs-password = 此服务器需要密码或密钥。

# SAFETY: needs a human review before it ships
# Red words when a server turned down the login.
folders-remote-mod-login-refused = 服务器不接受该登录。

# SAFETY: needs a human review before it ships
# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
folders-remote-mod-host-key-unknown = 你以前没有连接过 { $host }，所以 Folders 还不知道它的身份。

# SAFETY: needs a human review before it ships
# Red words when a server's identity differs from the one remembered; the variable is the server's name.
folders-remote-mod-host-key-changed = { $host } 的身份与上次不同。它可能被重建过，也可能有人在拦截这个连接。

# SAFETY: needs a human review before it ships
# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
folders-remote-mod-cert-unknown = Folders 无法检查 { $host } 的证书：{ $why }。

# SAFETY: needs a human review before it ships
# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
folders-remote-mod-cert-changed = { $host } 的证书在你信任它之后发生了变化。

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders 无法连接：{ $why }。

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders 找不到 { $what }。

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } 已经存在。

# SAFETY: needs a human review before it ships
# Red words when a server or phone refused permission; the variable is what it was refused for.
folders-remote-mod-denied = 你没有权限使用 { $what }。

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what }正在被其他程序使用。

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = 不支持{ $what }。

# SAFETY: needs a human review before it ships
# Red words when a copy or other work with a server was cancelled.
folders-remote-mod-cancelled = 已取消。

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = 跟随链接

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = 在打开 Windows 共享之前向其复制或从中复制

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = 在打开 NFS 共享之前向其复制或从中复制

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = 开头不是 sftp://、ftp:// 或 mtp://

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = 开头必须是 sftp://、ftp://、ftps://、ftpes://、dav://、davs://、smb://、nfs:// 或 mtp://

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = [ 缺少对应的 ]

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = 没有写明服务器

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = 服务器名称无效

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = 路径中含有换行

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = 服务器名称后面还有多余内容

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } 不是有效的端口号

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = 已跳过，因为服务器发来的名称不安全

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = 一个没有名称的项目

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = 回复内容太大，所以 Folders 已将其中止。

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } 现在是文件夹，所以 Folders 没有删除它。
