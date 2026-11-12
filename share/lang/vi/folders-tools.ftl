
# machine translated (Vietnamese), not yet reviewed by a person.

# Choice in the Wipe window: write over each file one time.
folders-tools-pass-once = Một lần

# Choice in the Wipe window: write over each file three times.
folders-tools-pass-three = 3 lần

# Choice in the Wipe window: write over each file seven times.
folders-tools-pass-seven = 7 lần

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = Giữ nguyên

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = Toàn chữ thường

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = Toàn chữ hoa

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = Viết hoa mỗi từ

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = Viết hoa từ đầu tiên

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = Giữ nguyên

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = Toàn chữ thường

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = Toàn chữ hoa

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = Đuôi mới

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = Không có đuôi

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = Kho nén

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = sẽ không còn tên nào

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = tên này không được phép

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = tên này quá dài

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = hai mục sẽ có cùng tên này

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = đã có mục khác dùng tên này

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = tên cũ có ký tự mà Folders không đọc được

# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
# SAFETY: needs a human review before it ships
folders-tools-caveat-flash = Đây là ổ flash hoặc SSD, nên các bản cũ có thể vẫn còn ở những phần mà Folders không với tới được.

# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
# SAFETY: needs a human review before it ships
folders-tools-caveat-copy-on-write = Hệ thống tệp { $fs } lưu thay đổi vào chỗ mới, nên dữ liệu cũ có thể vẫn còn ở đó.

# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
# SAFETY: needs a human review before it ships
folders-tools-caveat-network = Đây là thư mục chia sẻ { $fs } trên máy tính khác, máy đó có thể giữ bản sao riêng.

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = Chỉ có bên trái

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = Chỉ có bên phải

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = Giống nhau

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
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

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = Tại đây

# Red notice when Wipe is chosen on things that are not files.
# SAFETY: needs a human review before it ships
folders-tools-only-files-wiped = Chỉ có thể xóa sạch tệp.

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = Hãy mở một thư mục trong mỗi ngăn để so sánh chúng.

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
        [1] Hoàn tác việc đổi tên 1 mục
       *[other] Hoàn tác việc đổi tên { $n } mục
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [1] Đã đổi tên 1 mục.
       *[other] Đã đổi tên { $n } mục.
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = Việc đổi tên dừng giữa chừng: { $error }.

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = Hãy nhập tên cho tệp nén.

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = Đã kiểm tra { $done }

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = Đã đọc { $done } trên { $total }

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = Đang liệt kê tệp

# Beside the ring while two files are compared.
folders-tools-comparing = Đang so sánh

# Beside the ring while two folders are compared.
folders-tools-looking = Đang so sánh các thư mục

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [1] Nén { $name }
       *[other] Nén { $n } mục
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = Tên kho nén

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = Loại kho nén

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = Tệp nén sẽ được lưu trong { $place }

# Main button of the Compress window.
folders-tools-compress = Nén

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = Giải nén { $name } vào

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = Các tệp sẽ được đặt trong một thư mục mới tên { $name }.

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [1] Đổi kích thước { $name }
       *[other] Đổi kích thước { $n } ảnh
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = Kích thước mới

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = Kích thước là độ dài cạnh dài hơn, tính bằng điểm ảnh. Folders chỉ thu nhỏ ảnh. Mỗi bản đã đổi cỡ được lưu cạnh ảnh gốc, với kích thước mới trong tên.

# Main button of the Resize window.
folders-tools-resize = Đổi kích thước

# Title of the Wipe window; { $name } is the one file, { $n } how many.
# SAFETY: needs a human review before it ships
folders-tools-wipe-title =
    { $n ->
        [1] Xóa sạch { $name }?
       *[other] Xóa sạch { $n } tệp?
    }

# Warning at the top of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-warning = Folders ghi đè lên mỗi tệp trước khi xoá, nên bạn không thể lấy lại nó.

# Name beside the choice of how many times to write over, in the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-write-over = Ghi đè

# Choice tip in the Wipe window: how many times each file is written over.
# SAFETY: needs a human review before it ships
folders-tools-wipe-passes-tip = Số lần ghi đè lên mỗi tệp

# Red main button of the Wipe window.
# SAFETY: needs a human review before it ships
folders-tools-wipe-button = Xóa sạch an toàn

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = Đánh số

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = Ở đầu

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = Ở cuối

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = Vị trí của số

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = Số đầu tiên

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = Cách xử lý đuôi tệp

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = Đuôi mới, ví dụ txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders không đổi tên được mục này: { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = trước là { $old }

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = giữ nguyên

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [1] Đổi tên 1 mục
       *[other] Đổi tên { $n } mục
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = Thay thế

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = Tìm

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = Thay bằng

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = Dùng * và ? làm ký tự đại diện

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = Ký tự đại diện * và ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = Khớp chính xác chữ hoa

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = Phân biệt hoa thường

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = Thêm

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = Trước tên

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = Sau tên

# Name of the numbering row in the rename window.
folders-tools-rename-number = Số

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = Chữ hoa

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = Chữ hoa trong tên

# Name over the ending choice in the rename window.
folders-tools-rename-ending = Đuôi tệp

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] Trong { $place }, một số tên không dùng được. Xem các hàng được đánh dấu.
       *[no] Trong { $place }, { $changes } trên { $total } tên sẽ thay đổi.
    }

# Main button of the rename window.
folders-tools-rename-button = Đổi tên

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = Thư mục chính

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [1] Tệp trùng lặp trong { $folder }
       *[other] Tệp trùng lặp trong { $n } thư mục
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = Việc tìm kiếm đã dừng: { $error }.

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = Không tìm thấy tệp trùng lặp nào.

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [1] Không đọc được 1 tệp.
       *[other] Không đọc được { $n } tệp.
    }

# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
        [1] 1 nhóm bản sao. Các bản thừa đã được chọn, và chuyển chúng vào Thùng rác sẽ giải phóng { $size }.
       *[other] { $n } nhóm bản sao. Các bản thừa đã được chọn, và chuyển chúng vào Thùng rác sẽ giải phóng { $size }.
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = Nhóm { $set }, { $size }, trong { $at }

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [1] Không đọc được 1 tệp nên đã bỏ qua.
       *[other] Không đọc được { $n } tệp nên đã bỏ qua.
    }

# Note in the duplicates window when every copy of a set is picked.
# SAFETY: needs a human review before it ships
folders-tools-dupes-keep-one = Mọi bản sao trong một nhóm đều đang được chọn. Hãy bỏ chọn ít nhất một bản.

# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = Chọn các bản thừa

# Main button of the duplicates window: move the picked files to the Trash.
# SAFETY: needs a human review before it ships
folders-tools-dupes-trash = Chuyển vào Thùng rác

# Title of the window that compares two files.
folders-tools-diff-title = So sánh hai tệp

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders không so sánh được: { $error }.

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = Hai tệp hoàn toàn giống nhau.

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = Hai tệp giống nhau đến hết tệp ngắn hơn. Một tệp là { $one } và tệp kia là { $other }.

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = Hai tệp khác nhau từ byte { $n } trở đi.

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] Hai tệp giống nhau.
        [1] 1 dòng khác nhau.
       *[other] { $n } dòng khác nhau.
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = Hai tệp khác nhau quá nhiều để so từng dòng. Mọi thứ giữa điểm khác đầu tiên và điểm khác cuối cùng đều hiện là đã thay đổi.

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = Chỉ so sánh 4 MB đầu tiên của mỗi tệp.

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = So sánh { $left } với { $right }

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = So sánh nội dung (chậm hơn)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = So sánh nội dung bên trong mỗi tệp

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = Hiện các tên giống nhau

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = Hiện cả những tên giống nhau

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = Việc so sánh đã dừng: { $error }.

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = { $left } chỉ có bên trái, { $right } chỉ có bên phải, { $different } khác nhau, { $same } giống nhau.

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = Hai thư mục có nội dung giống nhau.

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = So sánh lại

# Button in the pane compare window; { $n } is how many names are copied to the left folder.
# SAFETY: needs a human review before it ships
folders-tools-compare-copy-left = Sao chép { $n } mục thiếu hoặc mới hơn sang bên trái

# Button in the pane compare window; { $n } is how many names are copied to the right folder.
# SAFETY: needs a human review before it ships
folders-tools-compare-copy-right = Sao chép { $n } mục thiếu hoặc mới hơn sang bên phải

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = Chỉ tệp mới bị xoá sạch, nên các thư mục này sẽ được để nguyên: { $names }.
