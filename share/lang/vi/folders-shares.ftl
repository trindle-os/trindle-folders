

folders-shares-nfs-denied = { $server } không cho máy tính này mở { $export }.

folders-shares-nfs-no-share = { $server } không chia sẻ thứ gì tên là { $export }.

folders-shares-left-none = Sẽ hỏi lại ở lần mở tiếp theo

folders-shares-left-under-day = Còn chưa đến một ngày

folders-shares-left-days =
    { $n ->
       *[other] Còn { $n } ngày
    }

folders-shares-held =
    { $past ->
        [yes] Bây giờ { $server } trả lời tại { $now }, một địa chỉ khác với địa chỉ mà thông tin đăng nhập của bạn cho nó đã dùng trước đây ({ $before }).
       *[no] { $server } trả lời tại { $now }, một địa chỉ mà thông tin đăng nhập của bạn cho nó chưa từng dùng.
    }{ $full ->
        [yes] {" "}Chỉ { $most } địa chỉ dùng gần nhất được ghi nhớ.
       *[no] {""}
    } Thiết bị nào trong mạng cũng có thể trả lời một cái tên, nên thông tin đăng nhập đã lưu cho nó chưa được gửi đi. Nếu đây là { $server } của bạn, hãy nhập tên người dùng và mật khẩu để dùng ở đây.

folders-shares-nowhere = Không tìm thấy trước địa chỉ mà { $server } trả lời, nên chưa gửi gì tới đó và những gì nhập ở đây không có nơi nào an toàn để đi. Hãy mở bằng địa chỉ của nó, ví dụ smb://192.168.1.20/.

folders-shares-lapsed = { $server } trả lời tại { $now }, nơi thông tin đăng nhập của bạn cho nó được dùng lần cuối cách đây hơn { $days } ngày. Từ đó địa chỉ này có thể đã được giao cho thiết bị khác, nên thông tin đăng nhập đã lưu cho nó chưa được gửi đi. Nếu đây là { $server } của bạn, hãy nhập tên người dùng và mật khẩu để dùng ở đây.

folders-shares-unknown = { $server } trả lời tại { $now }, nơi từng dùng một thông tin đăng nhập cho nó, nhưng không ghi lại lúc nào hoặc thông tin nào. Từ đó địa chỉ này có thể đã được giao cho thiết bị khác, nên thông tin đăng nhập đã lưu cho nó chưa được gửi đi. Nếu đây là { $server } của bạn, hãy nhập tên người dùng và mật khẩu để dùng ở đây.

folders-shares-unshown = { $server } trả lời tại { $now }, địa chỉ mà thông tin đăng nhập của bạn cho nó đã được gửi tới trước đây. Lần này địa chỉ đó chỉ được nghe thấy trên mạng, nơi thiết bị nào cũng có thể trả lời một cái tên, và nó chưa từng được hiện cho bạn ở đây, nên thông tin đăng nhập chưa được gửi đi. Nếu đây là { $server } của bạn, hãy nhập tên người dùng và mật khẩu để dùng ở đây.

folders-shares-asking =
    { $server } cần tên người dùng và mật khẩu. Đó là của một tài khoản trên máy tính đó.{ $heard ->
        [yes] {" "}Nó trả lời tại { $at }. Thiết bị nào trong mạng cũng có thể trả lời một cái tên, nên hãy kiểm tra đó đúng là địa chỉ của nó trước khi nhập.
       *[no] {""}
    }

folders-shares-where =
    Những gì bạn nhập sẽ được gửi tới { $address }{ $heard ->
        [yes] {" "}và không đi đâu khác. Khi bật Ghi nhớ, thông tin đăng nhập của bạn chỉ được lưu cho địa chỉ đó.
       *[no] {"."}
    }

folders-shares-again = { $server } trả lời tại { $at }, cổng { $port }. Địa chỉ đó chỉ được nghe thấy trên mạng, nơi thiết bị nào cũng có thể trả lời một cái tên, nên thông tin đăng nhập của bạn cho nó chưa được gửi đi. Nếu đó đúng là địa chỉ của nó, hãy bấm Mở.

folders-shares-keyring-unreachable = Không thể truy cập vòng khoá: { $why }.

folders-shares-unkept = Đã mở, nhưng thông tin đăng nhập không được lưu. { $why }

folders-shares-nfs-version = { $server } không hỗ trợ NFS phiên bản 4, loại duy nhất Trindle mở được.

folders-shares-nfs-opened-unknown = Đã mở, nhưng không biết nó ở đâu.

folders-shares-opened-unknown = Thư mục chia sẻ đã mở, nhưng không biết nó ở đâu.

folders-shares-not-address = Đó không phải là một địa chỉ.

folders-shares-seen-label = Các địa chỉ mà smb://{ $server } đã được mở tại

folders-shares-password-label = Mật khẩu của { $user } trên { $what }

folders-shares-remember = Ghi nhớ trong 90 ngày

folders-shares-remember-line = Tính lại từ đầu mỗi lần bạn dùng.

folders-shares-share-on = { $share } trên { $server }

folders-shares-card-title = Mở { $it }

folders-shares-close = Đóng

folders-shares-user-placeholder = Tên người dùng, như jo, hoặc WORKGROUP\jo

folders-shares-password = Mật khẩu

folders-shares-forget = Quên các địa chỉ trước đó

folders-shares-forget-spoken = Quên các địa chỉ mà thông tin đăng nhập này đã dùng trước đây

folders-shares-opening = Đang mở

folders-shares-open = Mở

folders-shares-bad-computer = Đó không phải là tên của một máy tính.

folders-shares-bad-path = Đó không phải là đường dẫn của một thư mục chia sẻ.

folders-shares-opening-export = Đang mở { $export } trên { $host }…

folders-shares-opening-share = Đang mở { $share } trên { $server }…

folders-shares-answer-card = Hãy trả lời thẻ của { $it } trước.

folders-shares-open-waiting = Đã mở. { $words }

folders-shares-none = { $server } không chia sẻ thư mục nào với bạn.

folders-shares-closing = Đang đóng { $name }…

folders-shares-close-failed = Không thể đóng { $name }: { $why }.

folders-shares-reconnecting = Đang kết nối lại { $name }…

folders-shares-not-remembered = Không thể ghi nhớ các thư mục chia sẻ cho lần sau.
