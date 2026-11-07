

folders-tools-pass-once = Một lần

folders-tools-pass-three = 3 lần

folders-tools-pass-seven = 7 lần

folders-tools-case-keep = Giữ nguyên

folders-tools-case-lower = Toàn chữ thường

folders-tools-case-upper = Toàn chữ hoa

folders-tools-case-words = Mỗi từ

folders-tools-case-first = Từ đầu tiên

folders-tools-ending-keep = Giữ nguyên

folders-tools-ending-lower = Toàn chữ thường

folders-tools-ending-upper = Toàn chữ hoa

folders-tools-ending-new = Đuôi mới

folders-tools-ending-none = Không có đuôi

folders-tools-archive-name = Kho nén

folders-tools-clash-empty = sẽ không còn tên nào

folders-tools-clash-bad = tên này không được phép

folders-tools-clash-long = tên này quá dài

folders-tools-clash-twice = hai mục sẽ có cùng tên này

folders-tools-clash-taken = đã có mục khác dùng tên này

folders-tools-clash-not-text = tên cũ không phải văn bản thuần nên không đổi được

folders-tools-caveat-flash = Đây là ổ flash, không phải ổ quay, nên các bản cũ có thể vẫn còn ở những phần mà Trindle không với tới được.

folders-tools-caveat-copy-on-write = Hệ thống tệp { $fs } ghi thay đổi sang chỗ mới, nên dữ liệu cũ có thể vẫn còn ở chỗ cũ.

folders-tools-caveat-network = Đây là thư mục chia sẻ { $fs } trên máy tính khác, máy đó có thể giữ bản sao riêng.

folders-tools-row-only-left = Chỉ có bên trái

folders-tools-row-only-right = Chỉ có bên phải

folders-tools-row-same = Giống nhau

folders-tools-row-different =
    Khác nhau: { $why ->
        [kind] một bên là thư mục hoặc liên kết
        [size] kích thước khác nhau
        [date] ngày khác nhau
        [content] nội dung khác nhau
        [target] liên kết trỏ đến nơi khác
        [inside] có thứ bên trong khác nhau
       *[unread] không đọc được một bên
    }{ $newer ->
        [left] , bên trái mới hơn
        [right] , bên phải mới hơn
       *[none] { "" }
    }

folders-tools-place-here = Tại đây

folders-tools-only-files-wiped = Chỉ có thể xóa sạch tệp.

folders-tools-compare-need-folders = Cả hai ngăn phải hiện một thư mục thì mới so sánh được.

folders-tools-rename-undo =
    { $n ->
        [1] Hoàn tác việc đổi tên 1 mục
       *[other] Hoàn tác việc đổi tên { $n } mục
    }

folders-tools-rename-done =
    { $n ->
        [1] Đã đổi tên 1 mục.
       *[other] Đã đổi tên { $n } mục.
    }

folders-tools-rename-stopped = Việc đổi tên dừng giữa chừng: { $error }.

folders-tools-name-empty = Tên không được để trống.

folders-tools-looked-at = Đã xem { $done }

folders-tools-read-of = Đã đọc { $done } trên { $total }

folders-tools-listing-files = Đang liệt kê tệp

folders-tools-comparing = Đang so sánh

folders-tools-looking = Đang xem

folders-tools-compress-title =
    { $n ->
        [1] Nén { $name }
       *[other] Nén { $n } mục
    }

folders-tools-pack-name-tip = Tên kho nén

folders-tools-pack-kind-tip = Loại kho nén

folders-tools-pack-made-in = Tạo trong { $place }

folders-tools-compress = Nén

folders-tools-extract-to = Giải nén { $name } vào

folders-tools-extract-new-folder = Một thư mục mới tên { $name } được tạo ở đó.

folders-tools-resize-title =
    { $n ->
        [1] Đổi kích thước { $name }
       *[other] Đổi kích thước { $n } ảnh
    }

folders-tools-resize-size-tip = Kích thước mới

folders-tools-resize-note = Một tỷ lệ của kích thước ban đầu, hoặc số điểm ảnh theo cạnh dài hơn; ảnh không bao giờ được phóng to. Một bản sao được tạo bên cạnh mỗi ảnh, đặt tên theo kích thước mới; ảnh gốc được giữ nguyên.

folders-tools-resize = Đổi kích thước

folders-tools-wipe-title =
    { $n ->
        [1] Xóa sạch { $name }?
       *[other] Xóa sạch { $n } tệp?
    }

folders-tools-wipe-warning = Mỗi tệp được ghi đè trước khi bị xóa, nên không thể khôi phục. Không thể hoàn tác việc này.

folders-tools-wipe-write-over = Ghi đè

folders-tools-wipe-passes-tip = Mỗi tệp được ghi đè bao nhiêu lần

folders-tools-wipe-button = Xóa sạch an toàn

folders-tools-rename-number-tip = Đánh số

folders-tools-rename-at-start = Ở đầu

folders-tools-rename-at-end = Ở cuối

folders-tools-rename-where-tip = Vị trí của số

folders-tools-rename-first-number-tip = Số đầu tiên

folders-tools-rename-ending-tip = Việc làm với đuôi tệp

folders-tools-rename-new-ending-tip = Đuôi mới, ví dụ txt

folders-tools-rename-cannot = Không thể: { $why }

folders-tools-rename-was = trước là { $old }

folders-tools-rename-stays = giữ nguyên

folders-tools-rename-title =
    { $n ->
        [1] Đổi tên 1 mục
       *[other] Đổi tên { $n } mục
    }

folders-tools-rename-replace = Thay thế

folders-tools-rename-find-tip = Tìm

folders-tools-rename-with-tip = Thay bằng

folders-tools-rename-wildcard-tip = Dùng * và ? làm ký tự đại diện

folders-tools-rename-wildcard = Ký tự đại diện * và ?

folders-tools-rename-match-case-tip = Khớp chính xác chữ hoa

folders-tools-rename-match-case = Phân biệt hoa thường

folders-tools-rename-add = Thêm

folders-tools-rename-before-tip = Trước tên

folders-tools-rename-after-tip = Sau tên

folders-tools-rename-number = Số

folders-tools-rename-letters = Chữ cái

folders-tools-rename-case-tip = Chữ hoa trong tên

folders-tools-rename-ending = Đuôi tệp

folders-tools-rename-summary =
    { $clash ->
        [yes] Một số tên không dùng được; xem các hàng được đánh dấu. Trong { $place }.
       *[no] { $changes } trên { $total } tên sẽ thay đổi. Trong { $place }.
    }

folders-tools-rename-button = Đổi tên

folders-tools-home = Thư mục chính

folders-tools-dupes-title =
    { $n ->
        [1] Tệp trùng lặp trong { $folder }
       *[other] Tệp trùng lặp trong { $n } thư mục
    }

folders-tools-dupes-stopped = Việc tìm kiếm đã dừng: { $error }.

folders-tools-dupes-none = Không tìm thấy tệp trùng lặp nào.

folders-tools-dupes-unread =
    { $n ->
        [1] Không đọc được 1 tệp.
       *[other] Không đọc được { $n } tệp.
    }

folders-tools-dupes-sets =
    { $n ->
        [1] 1 nhóm bản sao; chuyển các bản thừa vào Thùng rác sẽ giải phóng { $size }. Các bản thừa đã được chọn sẵn; nhấn Ctrl và bấm để chọn hoặc bỏ chọn một bản.
       *[other] { $n } nhóm bản sao; chuyển các bản thừa vào Thùng rác sẽ giải phóng { $size }. Các bản thừa đã được chọn sẵn; nhấn Ctrl và bấm để chọn hoặc bỏ chọn một bản.
    }

folders-tools-dupes-row = Nhóm { $set }, { $size }, trong { $at }

folders-tools-dupes-unread-left-out =
    { $n ->
        [1] Không đọc được 1 tệp nên đã bỏ qua.
       *[other] Không đọc được { $n } tệp nên đã bỏ qua.
    }

folders-tools-dupes-keep-one = Mọi bản sao của một nhóm đều đang được chọn; hãy giữ lại ít nhất một bản.

folders-tools-dupes-pick-extras = Chọn các bản thừa

folders-tools-dupes-trash = Chuyển vào Thùng rác

folders-tools-diff-title = So sánh hai tệp

folders-tools-diff-failed = Không thể so sánh: { $error }.

folders-tools-diff-same-bytes = Hai tệp giống nhau từng byte.

folders-tools-diff-same-as-shorter = Hai tệp giống nhau đến hết phần ngắn hơn; một tệp là { $one } và tệp kia là { $other }.

folders-tools-diff-from-byte = Hai tệp khác nhau từ byte { $n } trở đi.

folders-tools-diff-lines =
    { $n ->
        [0] Hai tệp giống nhau.
        [1] 1 dòng khác nhau.
       *[other] { $n } dòng khác nhau.
    }

folders-tools-diff-too-different = Hai tệp khác nhau quá nhiều để so từng dòng, nên mọi dòng giữa điểm khác đầu tiên và điểm khác cuối cùng đều hiện là đã thay đổi.

folders-tools-diff-cut = Chỉ so sánh 4 MB đầu tiên của mỗi tệp.

folders-tools-compare-title = So sánh { $left } với { $right }

folders-tools-compare-bytes = Đọc từng byte (chậm hơn)

folders-tools-compare-bytes-tip = So sánh nội dung bên trong mỗi tệp

folders-tools-compare-show-same = Hiện cả những mục giống nhau

folders-tools-compare-show-same-tip = Hiện cả những tên giống nhau

folders-tools-compare-stopped = Việc so sánh đã dừng: { $error }.

folders-tools-compare-counts = { $left } chỉ có bên trái, { $right } chỉ có bên phải, { $different } khác nhau, { $same } giống nhau.

folders-tools-compare-all-same = Hai thư mục có nội dung giống nhau.

folders-tools-compare-again = So sánh lại

folders-tools-compare-copy-left = Sao chép { $n } mục thiếu hoặc mới hơn sang bên trái

folders-tools-compare-copy-right = Sao chép { $n } mục thiếu hoặc mới hơn sang bên phải
