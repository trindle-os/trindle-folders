
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = 没有名为 { $host } 的服务器

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } 不接受连接

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } 没有及时响应

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = 在网络上无法连接到 { $host }

# SAFETY: needs a human review before it ships
# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
folders-remote-ftp-no-protect = 此服务器无法保证文件传输的安全，所以 Folders 没有发送任何内容：{ $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Folders 连接时，服务器没有按预期回复：{ $said }

# SAFETY: needs a human review before it ships
# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = 此服务器不提供安全 FTP。请试试在地址开头加上 ftp://。

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = 安全连接

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = 登录

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = 数据连接

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = 二进制传输模式

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = 登录后进入的文件夹

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = 使用 FTP 账户登录

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders 无法打开与服务器的数据连接。

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = 安全 FTP

# Red words when the server stopped answering.
folders-remote-ftp-stopped = 服务器不再响应了。

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = 与服务器的连接已断开。

# Red words when the server closed the connection.
folders-remote-ftp-closed = 服务器关闭了连接。

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = 在此服务器上执行该操作

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = 服务器上的 { $what } 出了问题：{ $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = 服务器发来的内容不是 FTP 回复

