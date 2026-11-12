
# machine translated (Vietnamese), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = Đóng

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = Thư mục chính

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = Mở { $name } bằng

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = Ứng dụng hiện tại

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = Không có ứng dụng nào trên máy này được thiết lập để mở loại tệp này.

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = Luôn dùng ứng dụng này

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = Dùng cho mọi tệp { $kind }.

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = Đặt làm ứng dụng mặc định

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = Chỉ hiện các ứng dụng được gợi ý

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = Hiện tất cả ứng dụng

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = Mở

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = Hãy chọn một ứng dụng trước.

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders không thể đặt { $name } làm ứng dụng mặc định.

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders không thể khởi động { $name }.

# Heading of the Properties window.
folders-dialogs-properties = Thuộc tính

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = Các trang Thuộc tính

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = Chung

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = Chi tiết

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = Chia sẻ



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = Tên

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = Vị trí

# Label in Properties of the kind of file.
folders-dialogs-row-kind = Loại

# Label in Properties of where a link points.
folders-dialogs-row-link-to = Liên kết tới

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = Đã sửa

# Label in Properties of who owns the item.
folders-dialogs-row-owner = Chủ sở hữu

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = Quyền

# Label in Properties of the size of the item.
folders-dialogs-row-size = Kích cỡ

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = Không rõ

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, bạn có thể mở và sửa

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, bạn có thể mở nhưng không thể sửa

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, bạn có thể sửa nhưng không thể mở

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, bạn không thể mở

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , một số không đọc được
       *[other]
    }{ $adding ->
        [yes] , vẫn đang cộng dồn
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = Đang tính kích thước

# Label in Properties of the row about checksums.
folders-dialogs-checksums = Mã kiểm tra

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = Tính mã kiểm tra

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = Đang tính mã kiểm tra

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders không đọc được tệp.

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = Dán mã kiểm tra để so sánh

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } khớp

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = Không khớp

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = So sánh


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [1] Gửi { $name } tới
       *[other] Gửi { $n } mục tới
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = Thao tác này tạo một bản sao ở đó. Các bản gốc vẫn ở chỗ cũ.

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = Ngăn bên kia, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = Các cột trong dạng xem Danh sách

# Note in the Columns window.
folders-dialogs-columns-note = Cột Tên luôn hiện. Các cột không đủ chỗ sẽ được ẩn cho đến khi ngăn rộng hơn.

# Heading over the columns that suit every file.
folders-dialogs-columns-files = Tệp

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = Nhạc, ảnh, PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = Hiện { $name }

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders không lưu được các cột.

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [1] Màu cho { $name }
       *[other] Màu cho { $n } thư mục
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = Không có màu

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders không lưu được màu thư mục.


folders-dialogs-no-match = Không có ứng dụng nào khớp với những từ đó.


folders-dialogs-find-app = Tìm một ứng dụng


folders-dialogs-find-place = Tìm một nơi


folders-dialogs-no-match-place = Không có nơi nào khớp với những từ đó.
