# machine translated (Vietnamese), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = không có máy chủ nào tên { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } không chấp nhận kết nối

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } không trả lời kịp

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = không có đường nào để đến { $host } trên mạng

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
folders-remote-ftp-no-protect = Máy chủ này không thể giữ an toàn cho việc truyền tệp, nên Folders chưa gửi gì cả: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Máy chủ không phản hồi như mong đợi khi Folders kết nối: { $said }

# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = Máy chủ này không có FTP an toàn. Hãy thử địa chỉ có ftp:// ở đầu.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = kết nối an toàn

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = việc đăng nhập

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = kết nối dữ liệu

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = chế độ truyền nhị phân

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = thư mục bạn bắt đầu

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = Đăng nhập bằng tài khoản FTP

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders không mở được kết nối dữ liệu tới máy chủ.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = FTP an toàn

# Red words when the server stopped answering.
folders-remote-ftp-stopped = Máy chủ đã ngừng trả lời.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = Đã mất kết nối với máy chủ.

# Red words when the server closed the connection.
folders-remote-ftp-closed = Máy chủ đã đóng kết nối.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = Việc đó trên máy chủ này

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Đã xảy ra lỗi với { $what } trên máy chủ: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = máy chủ gửi về thứ không phải phản hồi FTP


