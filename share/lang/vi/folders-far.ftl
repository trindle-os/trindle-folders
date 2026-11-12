
# machine translated (Vietnamese), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = không có máy chủ nào tên { $host }

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = chưa gửi đủ nên bản gốc vẫn ở chỗ cũ

# Reason a link was not sent to a server.
folders-far-not-followed = đây là liên kết nên Folders không gửi nó

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = đây không phải tệp hay thư mục nên Folders không gửi nó

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders không xác định được đây là loại mục gì nên đã để nguyên ở đó

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = Trên loại máy chủ này, một tệp có thể thay thế một tệp khác cùng tên.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = Bạn không thể làm việc đó với tệp trên máy chủ

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders không đọc được nó

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders không mở được nó.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (thư mục không thể nằm trong chính nó)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (đã có thư mục cùng tên)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (đã có tệp cùng tên)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders không biết địa chỉ của máy chủ.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = bản đã gửi có kích thước sai

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = Việc này chỉ dùng được với tệp trên máy chủ.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders không đặt được tệp mới vào chỗ, nên tệp cũ vẫn ở đó, nay có tên là { $old }, còn tệp mới có tên là { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders không xóa được tệp cũ. Nó vẫn ở đó, nay có tên là { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders không xóa được tệp cũ nên nó vẫn ở đó, nay có tên là { $old }: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } không trống.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders đã bỏ qua một tệp vì tên của nó không an toàn để dùng.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
       *[other] Folders đã bỏ qua { $n } mục vì tên của chúng không an toàn để dùng: { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = Đang tải xuống { $name }…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders không thể tải xuống { $name }: { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = Đã mở một bản sao của { $name }. Khi bạn lưu, các thay đổi sẽ được gửi lại máy chủ.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = Đã lưu { $name } trên máy chủ.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders không lưu được { $name } trên máy chủ: { $why }. Các thay đổi của bạn vẫn còn trong bản sao trên máy tính này.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = Đã đổi tên { $was } thành { $now }.

# Heading of the Connect to a server card.
folders-far-connect-title = Kết nối tới máy chủ

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Đóng

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Địa chỉ, ví dụ sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
# SAFETY: needs a human review before it ships
folders-far-connect-password = Mật khẩu, nếu cần

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = Tệp khóa, nếu bạn dùng

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = Chọn tệp khóa


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders dùng thiết lập ssh, ssh agent, khóa và danh sách máy chủ đã biết của bạn.

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = Bạn chưa từng kết nối tới { $host }, nên Folders không thể kiểm tra đây có đúng máy chủ không. Chỉ tin cậy nếu bạn đang định kết nối tới nó.

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = { $host } có danh tính khác với lần trước. Có thể ai đó đang cố chặn kết nối của bạn, nên Folders đã không kết nối. Nếu chủ sở hữu đã thay đổi, hãy xóa mục cũ của nó khỏi danh sách máy chủ đã biết trước.

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-password = Máy chủ này cần mật khẩu. Hãy nhập mật khẩu ở trên rồi thử lại.

# Note on the Connect to a server card when a server asks for a key or password.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-key = Máy chủ này cần khóa hoặc mật khẩu. Hãy chọn tệp khóa ở trên, hoặc thêm khóa vào ssh agent, rồi thử lại.

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = Folders không thể kiểm tra chứng chỉ của { $host }: { $why }. Nếu có thể, hãy so sánh dấu vân tay của nó với dấu vân tay mà chủ sở hữu đưa cho bạn. Chỉ tin cậy nếu hai bên khớp nhau.

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = Chứng chỉ của { $host } đã thay đổi kể từ khi bạn tin cậy nó. Chủ sở hữu có thể đã gia hạn, hoặc có người đang cố chặn kết nối của bạn. Chỉ tin cậy nếu chủ sở hữu xác nhận dấu vân tay này.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Đang kết nối

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = Tin cậy và kết nối

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Kết nối

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Điện thoại sẽ hiện ở thanh bên khi bạn cắm vào và mở khóa.
