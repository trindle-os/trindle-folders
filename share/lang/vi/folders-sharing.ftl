
# machine translated (Vietnamese), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders không đọc được nội dung của thư mục này.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = Bạn chỉ có thể chia sẻ chính thư mục, không phải liên kết tới thư mục.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = Bạn chỉ có thể chia sẻ thư mục của chính mình. Thư mục này thuộc về người khác.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = thư mục chính của bạn

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = thư mục này

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } và { $second }

# Name of the switch that shares the folder.
folders-sharing-share = Chia sẻ thư mục này

# One line under the share switch when the folder is shared.
folders-sharing-shared = Đang chia sẻ trong mạng gia đình.


# Tip of the share switch.
folders-sharing-share-tip = Chia sẻ trên mạng gia đình

# Label over the box for the name other computers see.
folders-sharing-name = Tên

# Tip of the box for the name other computers see.
folders-sharing-name-tip = Tên mà các máy tính khác nhìn thấy

# Name of the switch that lets others change files, and its tip.
# SAFETY: needs a human review before it ships
folders-sharing-write = Cho phép người khác thay đổi tệp

# One line under the switch that lets others change files.
# SAFETY: needs a human review before it ships
folders-sharing-write-sub = Họ có thể thêm, thay đổi và xóa tệp trong đó.

# Name of the switch that lets anyone in without a password.
# SAFETY: needs a human review before it ships
folders-sharing-guests = Cho mọi người vào

# One line under the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-sub = Không cần mật khẩu.

# Tip of the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-tip = Cho mọi người vào mà không cần mật khẩu

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Sau khi được chia sẻ, thư mục hiện trong mục Mạng trên các máy tính khác của bạn ở nhà, kể cả máy Windows. Lần đầu, Trindle tải Samba, chương trình dùng để chia sẻ thư mục. Việc này mất khoảng một phút.
       *[no] Sau khi được chia sẻ, thư mục hiện trong mục Mạng trên các máy tính khác của bạn ở nhà, kể cả máy Windows.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] Trên các máy tính khác, hãy tìm { $name } trên { $host } trong mục Mạng.
       *[no] Trên các máy tính khác, hãy tìm { $name } trong mục Mạng.
    }

# Note when other computers are not allowed to connect yet.
# SAFETY: needs a human review before it ships
folders-sharing-cannot-reach = Các máy tính khác chưa truy cập được thư mục này. Trong Cài đặt, mở Mạng và bật "Cho phép máy tính khác kết nối".

# Tip of the button that lets other computers connect.
# SAFETY: needs a human review before it ships
folders-sharing-let-connect = Cho phép máy tính khác kết nối

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
# SAFETY: needs a human review before it ships
folders-sharing-visitors =
    { $n ->
       *[other] Khách không có mật khẩu chưa thể mở được, vì { $names } đang đóng với các tài khoản khác. Nếu bạn cho họ đi qua, họ có thể vào thư mục này nhưng không thấy được gì khác trên đường đi.
    }

# Tip of the button that lets visitors through to the folder.
# SAFETY: needs a human review before it ships
folders-sharing-let-through = Cho khách vào thư mục này

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
# SAFETY: needs a human review before it ships
folders-sharing-sign-in-set = Người khác đăng nhập bằng tên { $user } với mật khẩu chia sẻ.

# Tip of the button that changes the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-change-password = Đổi mật khẩu chia sẻ

# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-set-password = Người khác đăng nhập bằng tên { $user }. Hãy đặt mật khẩu chia sẻ dài ít nhất { $n } ký tự. Mật khẩu này tách biệt với mật khẩu đăng nhập của bạn.

# Label beside the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password = Mật khẩu

# Tip of the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password-tip = Mật khẩu chia sẻ

# Tip of the button that keeps the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-keep-password = Lưu mật khẩu

# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = Chia sẻ { $name } trên mạng

# Red words when the share's name is used by another share.
folders-sharing-name-taken = Bạn đã có một thư mục chia sẻ với tên đó. Hãy chọn tên khác.

# Calm words while sharing is being stopped.
folders-sharing-stopping = Đang dừng chia sẻ…

# Red words when the sharing password is too short; { $n } is the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-password-short = Mật khẩu cần ít nhất { $n } ký tự.

# Calm words while the sharing password is being kept.
# SAFETY: needs a human review before it ships
folders-sharing-keeping-password = Đang lưu mật khẩu. Trước hết bạn sẽ được hỏi mật khẩu quản trị viên.

# Calm words while other computers are being let connect.
# SAFETY: needs a human review before it ships
folders-sharing-letting-connect = Đang cho phép các máy tính khác kết nối. Trước hết bạn sẽ được hỏi mật khẩu quản trị viên.

# Calm words while Samba is fetched and set up the first time.
# SAFETY: needs a human review before it ships
folders-sharing-fetching = Đang tải và thiết lập Samba, chương trình dùng để chia sẻ thư mục. Trước hết bạn sẽ được hỏi mật khẩu quản trị viên. Việc này chỉ xảy ra lần đầu.

# Calm words while sharing is set up the first time and Samba is already there.
# SAFETY: needs a human review before it ships
folders-sharing-setting-up = Đang thiết lập chia sẻ. Trước hết bạn sẽ được hỏi mật khẩu quản trị viên. Việc này chỉ xảy ra lần đầu.

# Calm words while an existing share is changed.
folders-sharing-changing = Đang thay đổi chia sẻ…

# Calm words while a share is made.
folders-sharing-sharing-it = Đang chia sẻ thư mục…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = Hãy đặt tên cho thư mục chia sẻ.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = Tên thư mục chia sẻ chỉ được dài tối đa { $most } ký tự.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = Tên thư mục chia sẻ không được chứa bất kỳ ký tự nào trong % < > * ? | / \ + = ; : " hoặc dấu phẩy.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = Tên kết thúc bằng $ sẽ làm ẩn thư mục chia sẻ, nên bạn không thể dùng nó.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = Tên thư mục chia sẻ không được bắt đầu bằng dấu gạch ngang.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba đã dùng tên { $name }. Hãy chọn tên khác.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Đã chia sẻ

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = Bạn chỉ có thể chia sẻ thư mục của chính mình.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = Tên đó đã là tên một tài khoản trên máy tính này, nên bạn không thể dùng nó cho thư mục chia sẻ. Hãy chọn tên khác.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = Cài đặt Samba của máy tính này không cho phép chia sẻ mà không có mật khẩu.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = Máy tính này không thể chia sẻ thêm thư mục nào nữa.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = Thư mục đó không còn nữa.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba không nêu lý do.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Thông báo của Samba: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } thuộc về người khác, nên bạn không thể cho khách truy cập nó từ đây.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders không thể cho khách truy cập { $path }: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle không tải được Samba vì máy tính này có vẻ đang ngoại tuyến. Hãy kết nối internet rồi thử lại.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Trindle không cài được Samba khi đang có việc cài đặt khác. Hãy thử lại sau vài phút.
