

folders-progress-pace-known =
    { $files ->
        [yes] { $done }/{ $of } tệp, { $rate } mỗi giây, { $left }
       *[no] { $rate } mỗi giây, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $done }/{ $of } tệp, đang tính thời gian…
       *[no] đang tính thời gian…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } byte

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = còn vài giây

folders-progress-left-seconds =
    { $n ->
       *[other] còn khoảng { $n } giây
    }

folders-progress-left-minute = còn khoảng một phút

folders-progress-left-minutes =
    { $n ->
       *[other] còn khoảng { $n } phút
    }

folders-progress-left-hours =
    { $n ->
       *[other] còn khoảng { $n } giờ
    }

folders-progress-more =
    { $n ->
       *[other] Còn { $n } công việc khác đang chạy
    }

folders-progress-heavy =
    { $verb ->
        [moving] Di chuyển dùng ổ đĩa nhiều, nên các chương trình khác có thể chậm hơn cho đến khi xong.
       *[copying] Sao chép dùng ổ đĩa nhiều, nên các chương trình khác có thể chậm hơn cho đến khi xong.
    }

folders-progress-waiting = đang chờ câu trả lời

folders-progress-paused = đã tạm dừng

folders-progress-carry-on = Tiếp tục

folders-progress-pause = Tạm dừng

folders-progress-stop = Dừng

folders-progress-mark-newer-larger = mới hơn, lớn hơn

folders-progress-mark-newer = mới hơn

folders-progress-mark-larger = lớn hơn

folders-progress-changed = sửa { $when }

folders-progress-taken =
    { $kind ->
        [folder] Đã có sẵn một thư mục tên { $name }
        [file] Đã có sẵn một tệp tên { $name }
       *[unknown] { $name } đã có sẵn
    }

folders-progress-coming = Mục đang được đưa vào

folders-progress-there = Mục đã có sẵn

folders-progress-skip = Bỏ qua

folders-progress-keep-both = Giữ cả hai

folders-progress-replace = Thay thế

folders-progress-replace-folder-spoken = Thay thế: thư mục đã có sẵn sẽ được chuyển vào Thùng rác

folders-progress-merge = Gộp

folders-progress-all-folders = Cùng một câu trả lời cho mọi thư mục đã có sẵn.

folders-progress-all-files = Cùng một câu trả lời cho mọi tệp đã có sẵn.

folders-progress-same-for-rest = Làm tương tự cho phần còn lại
