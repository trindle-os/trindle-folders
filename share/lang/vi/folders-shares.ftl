
# machine translated (Vietnamese), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = Máy tính này không được phép mở { $export } trên { $server }.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } không chia sẻ thư mục nào tên { $export }.

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
# SAFETY: needs a human review before it ships
folders-shares-left-none = Bạn sẽ được hỏi lại vào lần sau

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
# SAFETY: needs a human review before it ships
folders-shares-left-under-day = Còn chưa đến một ngày

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
# SAFETY: needs a human review before it ships
folders-shares-left-days =
    { $n ->
       *[other] Còn { $n } ngày
    }

# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
# SAFETY: needs a human review before it ships
folders-shares-held =
    { $past ->
        [yes] Bây giờ { $server } đang ở { $now }. Trước đây đăng nhập của bạn cho nó được dùng ở một địa chỉ khác ({ $before }).
       *[no] { $server } đang ở { $now }. Đăng nhập của bạn cho nó chưa từng được dùng ở địa chỉ này.
    }{ $full ->
        [yes] {" "}Folders chỉ nhớ { $most } địa chỉ gần nhất.
       *[no] {""}
    } Thiết bị nào trong mạng cũng có thể tự nhận một cái tên, nên Folders chưa gửi đăng nhập đã lưu của bạn. Nếu đây là { $server } của bạn, hãy nhập tên người dùng và mật khẩu để dùng ở đây.

# Words on the sign-in card when no address could be found for the computer to show.
# SAFETY: needs a human review before it ships
folders-shares-nowhere = Folders không tìm được địa chỉ của { $server }, nên chưa gửi gì tới đó. Những gì bạn nhập ở đây không thể được gửi đi an toàn. Hãy mở bằng địa chỉ của nó, ví dụ smb://192.168.1.20/.

# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
# SAFETY: needs a human review before it ships
folders-shares-lapsed = { $server } đang ở { $now }. Lần cuối bạn dùng đăng nhập cho nó ở đó là hơn { $days } ngày trước. Từ đó địa chỉ này có thể đã được giao cho thiết bị khác, nên Folders chưa gửi đăng nhập đã lưu của bạn. Nếu đây là { $server } của bạn, hãy nhập tên người dùng và mật khẩu để dùng ở đây.

# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
# SAFETY: needs a human review before it ships
folders-shares-unknown = { $server } đang ở { $now }. Trước đây đã từng dùng một đăng nhập cho nó ở đó, nhưng Folders không ghi lại là lúc nào, hay đăng nhập nào. Từ đó địa chỉ này có thể đã được giao cho thiết bị khác, nên Folders chưa gửi đăng nhập đã lưu của bạn. Nếu đây là { $server } của bạn, hãy nhập tên người dùng và mật khẩu để dùng ở đây.

# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
# SAFETY: needs a human review before it ships
folders-shares-unshown = { $server } đang ở { $now }, nơi đăng nhập của bạn cho nó từng được dùng. Lần này Folders tìm thấy nó trên mạng cục bộ, nơi thiết bị nào cũng có thể dùng tên đó. Bạn chưa từng thấy địa chỉ này, nên Folders chưa gửi đăng nhập của bạn. Nếu đây là { $server } của bạn, hãy nhập tên người dùng và mật khẩu để dùng ở đây.

# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
# SAFETY: needs a human review before it ships
folders-shares-asking =
    Hãy nhập tên người dùng và mật khẩu của một tài khoản trên { $server }.{ $heard ->
        [yes] {" "}Nó đang ở { $at }. Thiết bị nào trong mạng cũng có thể tự nhận một cái tên, nên hãy kiểm tra xem đây có đúng là địa chỉ của nó không trước khi nhập.
       *[no] {""}
    }

# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
# SAFETY: needs a human review before it ships
folders-shares-where =
    Những gì bạn nhập sẽ được gửi tới { $address }{ $heard ->
        [yes] {" "}và không đi đâu khác. Khi bật Ghi nhớ, Folders chỉ lưu đăng nhập của bạn cho địa chỉ này.
       *[no] {"."}
    }

# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
# SAFETY: needs a human review before it ships
folders-shares-again = Folders tìm thấy { $server } trên mạng cục bộ tại { $at }, cổng { $port }. Ở đó, thiết bị nào cũng có thể dùng tên này, nên Folders chưa gửi đăng nhập của bạn. Nếu đây là địa chỉ đúng, hãy bấm Mở.

# Red words when the keyring cannot be reached; the reason follows.
# SAFETY: needs a human review before it ships
folders-shares-keyring-unreachable = Folders không truy cập được vòng khoá: { $why }.

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
# SAFETY: needs a human review before it ships
folders-shares-unkept = Đã mở, nhưng Folders không lưu được đăng nhập. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } không hỗ trợ NFS phiên bản 4, phiên bản duy nhất Folders mở được.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = Đã mở, nhưng Folders không tìm thấy nó nằm ở đâu trên máy tính này.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = Thư mục chia sẻ đã mở, nhưng Folders không tìm thấy nó nằm ở đâu trên máy tính này.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = Đó không phải là một địa chỉ.

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = Các địa chỉ đã dùng cho smb://{ $server }

# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
# SAFETY: needs a human review before it ships
folders-shares-password-label = Mật khẩu của { $user } trên { $what }

# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
# SAFETY: needs a human review before it ships
folders-shares-remember = Ghi nhớ trong 90 ngày

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = 90 ngày được tính lại từ đầu mỗi lần bạn dùng.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } trên { $server }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = Mở { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Đóng

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = Tên người dùng, như jo hoặc WORKGROUP\jo

# Hint in the empty password box of the sign-in card.
# SAFETY: needs a human review before it ships
folders-shares-password = Mật khẩu

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
# SAFETY: needs a human review before it ships
folders-shares-forget = Quên các địa chỉ trước đó

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
# SAFETY: needs a human review before it ships
folders-shares-forget-spoken = Quên các địa chỉ trước đó của đăng nhập này

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Đang mở…

# Name of the main button of the sign-in card.
folders-shares-open = Mở

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = Đó không phải tên máy tính.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = Đó không phải đường dẫn tới một thư mục chia sẻ.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = Đang mở { $export } trên { $host }…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = Đang mở { $share } trên { $server }…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Hãy hoàn tất đăng nhập vào { $it } trước.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = Đã mở. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } không chia sẻ thư mục nào với bạn.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = Đang đóng { $name }…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders không đóng được { $name }: { $why }.

# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = Đang kết nối lại { $name }…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders không lưu được các thư mục chia sẻ của bạn cho lần sau.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = Không thể dùng tên người dùng đó cho một thư mục chia sẻ.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = Không thể dùng tên miền đó cho một thư mục chia sẻ.

# Red words when a share's password holds a line break.
folders-shares-bad-password = Mật khẩu của thư mục chia sẻ không được có ngắt dòng.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } cần tên người dùng và mật khẩu cho { $it }.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = Tên người dùng hoặc mật khẩu cho { $it } không đúng.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = Tài khoản của bạn trên { $server } bị khóa hoặc bị tắt, hoặc mật khẩu đã hết hạn.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } không có thư mục chia sẻ nào tên { $share }.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = Tài khoản đó không được phép mở { $it }.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } không phản hồi. Có thể máy đang ngủ hoặc đã tắt. Hãy đánh thức máy rồi thử lại.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders không tìm thấy máy tính nào tên { $server } trên mạng này.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } chỉ dùng SMB1, một cách chia sẻ cũ và không còn an toàn. Folders không dùng nó.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = Việc này cần gói { $package } của Debian, nhưng gói đó chưa được cài.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders không mở được thư mục chia sẻ.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders không mở được thư mục được chia sẻ.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders không liệt kê được các thư mục chia sẻ của nó.
