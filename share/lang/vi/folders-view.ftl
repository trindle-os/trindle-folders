
# machine translated (Vietnamese), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = Dọn sạch Thùng rác?

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
       *[other] Tất cả { $n } mục trong đó sẽ bị xoá vĩnh viễn. Bạn không thể hoàn tác việc này.
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = Dọn sạch Thùng rác

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
       *[other] Xoá vĩnh viễn { $n } mục này?
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = Bạn không thể hoàn tác việc này.

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = Xoá vĩnh viễn

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = Giữ lại

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = Đưa chúng tới đâu?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Để chúng nguyên chỗ cũ




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = Biểu tượng

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = Danh sách

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = Gọn

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = Chọn cột

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = Tên

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = Sắp xếp theo

# Name read out for the toolbar's Group by strip.
folders-view-group-by = Nhóm theo





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Ẩn bảng xem trước (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Hiện bảng xem trước (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Bảng xem trước

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Thẻ

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Thẻ ngăn trái

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Thẻ ngăn phải

# Tip of the button that opens a new tab.
folders-view-tab-new = Thẻ mới (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = Đóng thẻ (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = Thư mục chính

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Chưa ghim gì. Bấm chuột phải vào thư mục để ghim.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = Đã ghim

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Vị trí

# Small heading in the side bar over the drives plugged in.
folders-view-drives = Ổ đĩa

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Thư mục chia sẻ

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = Đóng { $name }



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Kết nối tới máy chủ

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = Liên kết hai ngăn. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = Liên kết hai ngăn

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] Sao chép sang ngăn trái
       *[other] Sao chép sang ngăn phải
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] Di chuyển sang ngăn trái
       *[other] Di chuyển sang ngăn phải
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Hiện thư mục này ở ngăn trái
       *[other] Hiện thư mục này ở ngăn phải
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Đổi chỗ hai ngăn

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Chọn những mục khác với ngăn trái
       *[other] Chọn những mục khác với ngăn phải
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] So sánh với ngăn trái
       *[other] So sánh với ngăn phải
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Nhập thư mục, tệp hoặc địa chỉ máy chủ

# Name read out for the path above a pane's list.
folders-view-where = Vị trí

# Tip of the pane's Back button.
folders-view-back = Quay lại (Alt+Trái)

# Tip of the pane's Forward button.
folders-view-forward = Tiến tới (Alt+Phải)

# Tip of the pane's button that goes up one folder.
folders-view-up = Lên một thư mục (Alt+Lên)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Lọc, hoặc nhấn Enter để tìm trong thư mục này và các thư mục con

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Tìm theo tên, kể cả trong thư mục con

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = Tìm trong nội dung tệp, kể cả trong thư mục con


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Loại

# Name read out for the list that picks which size to show.
folders-view-find-size = Kích cỡ

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Lần sửa gần nhất

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = Tên mới cho { $was }

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = còn trống { $size }

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = Đã chọn { $count }, đang tính kích cỡ…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = Đã chọn { $count }, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Hiện các tệp liên quan

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Thử lại

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
# SAFETY: needs a human review before it ships
folders-view-cancel = Huỷ

# Line on a pane whose filter matches nothing.
folders-view-none-matches = Không có gì khớp với bộ lọc

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Hiện tất cả

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = Bạn chưa mở gì gần đây

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Về thư mục chính

# Line on the Trash page when it is empty.
folders-view-none-trash = Thùng rác trống

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = Không có ổ đĩa nào để hiện

# Line on a search page that found nothing.
folders-view-none-found = Không tìm thấy gì

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Xoá tìm kiếm

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = Tệp nén này trống

# Button on an empty archive page: goes up one folder.
folders-view-go-up = Lên một thư mục

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = Thư mục này trống

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = Thư mục mới

# Button on an empty folder's page that makes a new document.
folders-view-new-document = Tài liệu mới

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
       *[other] { $n } mục
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Chưa kết nối

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = Kết nối lại { $name }

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } chưa được kết nối.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Kết nối lại tất cả

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
       *[other] { $n } thư mục mạng chưa được kết nối.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = Quên { $name }

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = Hai ngăn

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = Hiển thị hai ngăn cạnh nhau (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Hiển thị một ngăn (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = Xem

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = Mới

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = Thêm tùy chọn

# Row of the More menu that undoes the last change.
folders-view-more-undo = Hoàn tác

# Row of the More menu that does again what was undone.
folders-view-more-redo = Làm lại

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Sắp xếp ngược lại

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Hiện tệp ẩn (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Ẩn tệp ẩn (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Tệp ẩn

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Máy tính gần đây

# Tip of the Search button in the top row while the path shows.
folders-view-search = Tìm kiếm (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = Đóng tìm kiếm (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = Bạn không thể tìm kiếm trên trang này

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Tìm kiếm

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = Tên trên thanh bên cho { $was }
