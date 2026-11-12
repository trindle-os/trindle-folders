
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name }（{ $why }）

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = 没有名为 { $host } 的服务器

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = 没有全部送达，所以原件没有移动

# Reason a link was not sent to a server.
folders-far-not-followed = 这是链接，所以 Folders 没有发送它

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = 它不是文件或文件夹，所以 Folders 没有发送它

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders 无法判断它是什么类型的项目，所以把它留在了原处

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = 在这种服务器上，文件可能会替换同名的另一个文件。

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = 不能对服务器上的文件执行此操作

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders 无法读取它

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders 无法打开它。

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name }（文件夹不能放进它自己里面）

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name }（已有同名文件夹）

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name }（已有同名文件）

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders 不知道服务器的地址。

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = 发送的副本大小不对

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = 这只适用于服务器上的文件。

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders 无法放入新文件，所以旧文件仍在原处，现在名为 { $old }，新文件名为 { $new }：{ $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders 无法删除旧文件。它仍在那里，现在名为 { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders 无法删除旧文件，所以它仍在那里，现在名为 { $old }：{ $why }

# SAFETY: needs a human review before it ships
# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } 不是空的。

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders 跳过了一个文件，因为它的名称不安全，无法使用。

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
       *[other] Folders 跳过了 { $n } 个项目，因为它们的名称不安全，无法使用：{ $names }。
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = 正在下载 { $name }…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders 无法下载 { $name }：{ $why }。

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = 已打开 { $name } 的副本。保存时，你的更改会送回服务器。

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = 已将 { $name } 保存到服务器。

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders 无法将 { $name } 保存到服务器：{ $why }。你的更改仍保留在这台电脑上的副本中。

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = 已将 { $was } 改名为 { $now }。

# Heading of the Connect to a server card.
folders-far-connect-title = 连接到服务器

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = 关闭

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = 地址，例如 sftp://jo@example.com

# SAFETY: needs a human review before it ships
# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
folders-far-connect-password = 密码（如需要）

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = 密钥文件（如果你使用）

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = 选择密钥文件


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders 会使用你的 ssh 设置、ssh 代理、密钥和已知服务器列表。

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card for a server not visited before; the server's name.
folders-far-connect-new-host = 你以前没有连接过 { $host }，所以 Folders 无法确认它是否正确。只有在你确实想连接它时才信任它。

# SAFETY: needs a human review before it ships
# Red note on the Connect to a server card when a known server has a different identity; the server's name.
folders-far-connect-host-changed = { $host } 的身份与上次不同。可能有人试图拦截你的连接，所以 Folders 没有连接。如果是所有者更改了它，请先从你的已知主机中删除它的旧记录。

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
folders-far-connect-needs-password = 此服务器需要密码。请在上方输入，然后再试一次。

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card when a server asks for a key or password.
folders-far-connect-needs-key = 此服务器需要密钥或密码。请在上方选择你的密钥文件，或将其添加到你的 ssh 代理，然后再试一次。

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
folders-far-connect-cert-new = Folders 无法检查 { $host } 的证书：{ $why }。如果可以，请把它的指纹与所有者给你的指纹对比。只有两者一致才信任它。

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
folders-far-connect-cert-changed = 自你信任它以来，{ $host } 的证书已更改。可能是所有者续期了证书，也可能有人试图拦截你的连接。只有在所有者确认此指纹后才信任它。

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = 正在连接

# SAFETY: needs a human review before it ships
# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
folders-far-connect-trust = 信任并连接

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = 连接

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = 手机插入并解锁后，会自动显示在侧边栏中。

