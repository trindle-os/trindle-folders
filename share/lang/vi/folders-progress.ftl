
# machine translated (Vietnamese), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done }/{ $of } tệp, { $rate } mỗi giây, { $left }
       *[no] { $rate } mỗi giây, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done }/{ $of } tệp, đang tính thời gian…
       *[no] đang tính thời gian…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } byte

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = còn vài giây

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
       *[other] còn khoảng { $n } giây
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = còn khoảng một phút

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
       *[other] còn khoảng { $n } phút
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
       *[other] còn khoảng { $n } giờ
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
       *[other] Còn { $n } công việc khác đang chạy
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Di chuyển khiến ổ đĩa luôn bận, nên các chương trình khác có thể chậm hơn cho đến khi xong.
       *[copying] Sao chép khiến ổ đĩa luôn bận, nên các chương trình khác có thể chậm hơn cho đến khi xong.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = đang chờ câu trả lời

# End of a piece of work's line while it is paused.
folders-progress-paused = đã tạm dừng

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Tiếp tục

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Tạm dừng

# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = Dừng

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = mới hơn và lớn hơn

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = mới hơn

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = lớn hơn

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = sửa { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] Đã có sẵn một thư mục tên { $name }
        [file] Đã có sẵn một tệp tên { $name }
       *[unknown] { $name } đã có sẵn
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = Mục mới

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = Mục đã có sẵn

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Bỏ qua

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = Giữ cả hai

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = Thay thế

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = Thay thế, và chuyển thư mục đã có sẵn vào Thùng rác

# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = Gộp

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = Dùng câu trả lời này cho mọi thư mục đã có sẵn.

# Line over the switch in the name taken question for files.
folders-progress-all-files = Dùng câu trả lời này cho mọi tệp đã có sẵn.

# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = Làm tương tự cho phần còn lại
