# machine translated (Vietnamese), not yet reviewed by a person.

# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
folders-remote-mod-heard-card = Folders tìm thấy { $host } trên mạng cục bộ tại { $at }, cổng { $port }. Ở đó, thiết bị nào cũng có thể dùng tên này, nên Folders chưa gửi mật khẩu của bạn. Nếu đây là địa chỉ đúng, hãy nhấn Kết nối.

# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
folders-remote-mod-heard-stopped = Folders tìm thấy { $host } trên mạng cục bộ tại { $at }. Ở đó, thiết bị nào cũng có thể dùng tên này, nên Folders chưa gửi mật khẩu của bạn. Hãy mở lại { $host } để kiểm tra địa chỉ của nó.

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders không đọc được địa chỉ đó: { $why }.

# Red words when a server needs a password or key and none could be asked for.
# SAFETY: needs a human review before it ships
folders-remote-mod-needs-password = Máy chủ này cần mật khẩu hoặc khóa.

# Red words when a server turned down the login.
# SAFETY: needs a human review before it ships
folders-remote-mod-login-refused = Máy chủ không chấp nhận lần đăng nhập đó.

# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-unknown = Bạn chưa từng kết nối tới { $host }, nên Folders chưa biết danh tính của nó.

# Red words when a server's identity differs from the one remembered; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-changed = { $host } có danh tính khác với lần trước. Có thể nó đã được dựng lại, hoặc có người đang chặn kết nối.

# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-unknown = Folders không kiểm tra được chứng chỉ của { $host }: { $why }.

# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-changed = Chứng chỉ của { $host } đã thay đổi kể từ lúc bạn tin cậy nó.

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders không kết nối được: { $why }.

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders không tìm thấy { $what }.

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } đã có sẵn.

# Red words when a server or phone refused permission; the variable is what it was refused for.
# SAFETY: needs a human review before it ships
folders-remote-mod-denied = Bạn không có quyền dùng { $what }.

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } đang được chương trình khác dùng.

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } không được hỗ trợ.

# Red words when a copy or other work with a server was cancelled.
folders-remote-mod-cancelled = Đã hủy.

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = Việc đi theo liên kết

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = Sao chép đến hoặc từ thư mục chia sẻ Windows trước khi mở nó

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = Sao chép đến hoặc từ thư mục chia sẻ NFS trước khi mở nó

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = địa chỉ không bắt đầu bằng sftp://, ftp:// hay mtp://

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = địa chỉ phải bắt đầu bằng sftp://, ftp://, ftps://, ftpes://, dav://, davs://, smb://, nfs:// hoặc mtp://

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = có [ mà thiếu ]

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = không nêu tên máy chủ

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = tên máy chủ không hợp lệ

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = đường dẫn có chứa ngắt dòng

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = có thêm nội dung thừa sau tên máy chủ

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } không phải là số cổng hợp lệ

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = đã bỏ qua vì máy chủ gửi một tên không an toàn

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = một mục không có tên

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = Phản hồi quá lớn, nên Folders đã dừng nó lại.

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } giờ là một thư mục, nên Folders không xoá nó.
