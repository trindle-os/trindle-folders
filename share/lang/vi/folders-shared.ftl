# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-cancelled = bạn chưa nhập mật khẩu quản trị viên
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-wrong = mật khẩu quản trị viên không được chấp nhận
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-not-allowed = tài khoản của bạn không được phép làm việc đó, kể cả khi có mật khẩu quản trị viên
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unasked = không có cách nào để hỏi bạn mật khẩu quản trị viên
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unclear = mật khẩu quản trị viên không được chấp nhận, hoặc tài khoản của bạn không được phép làm việc đó
# Sentence of its own saying a step was refused: the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-sentence-cancelled = Bạn chưa nhập mật khẩu quản trị viên
# Sentence of its own saying a step was refused: the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-sentence-wrong = Mật khẩu quản trị viên không được chấp nhận
# Sentence of its own saying a step was refused: this login may never do it.
# SAFETY: needs a human review before it ships
common-refused-sentence-not-allowed = Tài khoản của bạn không được phép làm việc đó, kể cả khi có mật khẩu quản trị viên
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
# SAFETY: needs a human review before it ships
common-refused-sentence-unasked = Không có cách nào để hỏi bạn mật khẩu quản trị viên
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
# SAFETY: needs a human review before it ships
common-refused-sentence-unclear = Mật khẩu quản trị viên không được chấp nhận, hoặc tài khoản của bạn không được phép làm việc đó
# Red words when a step was refused because the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-not-done-cancelled = Việc đó chưa được làm vì bạn chưa nhập mật khẩu quản trị viên.
# Red words when a step was refused because the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-not-done-wrong = Việc đó chưa được làm vì mật khẩu quản trị viên không được chấp nhận.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-not-allowed = Việc đó chưa được làm. Tài khoản của bạn không được phép làm việc đó, kể cả khi có mật khẩu quản trị viên. Hãy đăng xuất rồi đăng nhập lại, sau đó thử lại.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unasked = Việc đó chưa được làm. Không có cách nào để hỏi bạn mật khẩu quản trị viên. Hãy đăng xuất rồi đăng nhập lại, sau đó thử lại.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unclear = Việc đó chưa được làm. Mật khẩu quản trị viên không được chấp nhận, hoặc tài khoản của bạn không được phép làm việc đó. Nếu bạn không được hỏi mật khẩu, hãy đăng xuất rồi đăng nhập lại, sau đó thử lại.
# Red words when a step behind the administrator's password failed and said nothing of why.
# SAFETY: needs a human review before it ships
common-refused-no-reason = Việc đó không thành công, và không có lý do nào được nêu.
# Red words when the keyring will not open without the person's password.
# SAFETY: needs a human review before it ships
common-keyring-locked = Vòng khoá đang bị khoá. Hãy mở khoá nó, hoặc đăng xuất rồi đăng nhập lại, sau đó thử lại.
# Red words when this computer has no keyring to keep a login or key in.
# SAFETY: needs a human review before it ships
common-keyring-missing = Máy tính này không có vòng khoá để lưu nó.
# Red words when the keyring is there and gives no answer in time.
# SAFETY: needs a human review before it ships
common-keyring-silent = Folders không kết nối được với vòng khoá kịp lúc.
# Red words when work with the keyring never ended, long after its deadline.
# SAFETY: needs a human review before it ships
common-keyring-never = Folders đã ngừng chờ vòng khoá.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
common-keyring-turn-failed = Folders không truy cập được vòng khoá ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
# SAFETY: needs a human review before it ships
common-keyring-waiting =
    { $action ->
        [removing] Folders vẫn đang chờ vòng khoá, nên { $what } chưa được xoá.
       *[other] Folders vẫn đang chờ vòng khoá, nên { $what } chưa được lưu.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
# SAFETY: needs a human review before it ships
common-keyring-late-done =
    { $action ->
        [removing] Sau một lúc chờ, { $what } đã được xoá khỏi vòng khoá.
       *[other] Sau một lúc chờ, { $what } đã được lưu vào vòng khoá.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
# SAFETY: needs a human review before it ships
common-keyring-late-never =
    { $action ->
        [removing] Folders đã ngừng chờ vòng khoá, nên { $what } chưa được xoá.
       *[other] Folders đã ngừng chờ vòng khoá, nên { $what } chưa được lưu.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
# SAFETY: needs a human review before it ships
common-keyring-late-refused =
    { $action ->
        [removing] Folders không xoá được { $what } khỏi vòng khoá. { $why }
       *[other] Folders không lưu được { $what } vào vòng khoá. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Thêm
common-more-spoken = Mở Trợ giúp về mục này
# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
folders-volumes-no-password = Bạn chưa nhập mật khẩu, nên chưa có gì bị thay đổi.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, nên chưa có gì bị thay đổi.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = Có thứ gì đó vẫn đang dùng nó. Hãy đóng mọi cửa sổ hoặc chương trình đang hiện tệp của nó, rồi thử lại.
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = Không đủ dung lượng cho những gì đang có trên nó ở kích thước đó.
# Heading over the password box, and the tip shown in the box while it is empty.
# SAFETY: needs a human review before it ships
folders-prompt-password = Mật khẩu
# Main button at the bottom of the password window that checks the password.
# SAFETY: needs a human review before it ships
folders-prompt-unlock = Mở khóa
# Button at the bottom of the password window that closes it without unlocking.
# SAFETY: needs a human review before it ships
folders-prompt-cancel = Hủy
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = Hãy nhập mật khẩu trước.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Đang kiểm tra…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = lớn hơn 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = lớn hơn màn hình của bạn
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = trên 20 MB (200 MB nếu là hình động)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = trên 20 MB
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = trên 200 MB
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } hoặc { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Đã khóa: { $name }

# Name given to a drive that has none.
folders-volumes-drive = Ổ đĩa

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = Cụm mật khẩu đó không mở khóa được ổ đĩa.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = Nó đã được mở rồi.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = Bạn cần có cụm mật khẩu để mở nó.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = Cụm mật khẩu cho { $name }

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Mở khóa ổ đĩa

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders không mở được cửa sổ cụm mật khẩu.
