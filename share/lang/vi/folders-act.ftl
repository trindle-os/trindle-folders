
# machine translated (Vietnamese), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Không dùng được cắt, sao chép và dán với tệp trên máy chủ. Thay vào đó, hãy dùng các nút sao chép và di chuyển giữa hai ngăn.

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = Bạn không thể thay đổi gì bên trong tệp nén. Hãy giải nén những gì bạn cần trước.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = Những mục này được cắt trong ứng dụng khác, nên Folders đã sao chép chúng.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = Mục này đang được sao chép hoặc di chuyển.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = Mục này vẫn đang được sao chép vào đây.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10.000 mục trở lên

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
       *[other] { $n } mục
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
       *[other] { $n } mục không thể { $verb }: { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } và { $more } mục nữa

# Words on a job card while things are copied: what, and where to.
folders-act-copying = Đang sao chép { $what } tới { $place }

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = Đã sao chép { $what } tới { $place }.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = sao chép

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = Hoàn tác việc sao chép { $what }

# Words on a job card while things are moved: what, and where to.
folders-act-moving = Đang di chuyển { $what } tới { $place }

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = Đã di chuyển { $what } tới { $place }.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = di chuyển

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = Hoàn tác việc di chuyển { $what }

# Words on a job card while things are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-trashing = Đang chuyển { $what } vào thùng rác

# Words under the file lines when things have been put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-trashed = Đã chuyển { $what } vào thùng rác.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
# SAFETY: needs a human review before it ships
folders-act-verb-trashed = chuyển vào thùng rác

# Tip of the Undo button after things were put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-undo-trashing = Hoàn tác việc chuyển { $what } vào thùng rác

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = Đang xóa { $what }

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = Đã xóa vĩnh viễn { $what }.

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = xóa

# Words on a job card while things are put back from the Trash.
folders-act-restoring = Đang khôi phục { $what }

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = Đã khôi phục { $what }.

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = Hoàn tác việc khôi phục { $what }

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = khôi phục

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Đang hoàn tác

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Đã hoàn tác.

# Words on a job card while things are packed into an archive.
folders-act-compressing = Đang nén { $what }

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = Đã tạo { $file } trong { $place }.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = nén

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = Hoàn tác việc nén { $what }

# Words on a job card while an archive is unpacked.
folders-act-extracting = Đang giải nén { $what }

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = Đã giải nén { $what } vào { $place }.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = giải nén

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = Hoàn tác việc giải nén { $what }

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = Đang xóa sạch { $what }

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = Đã xóa sạch { $what }.

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = xóa sạch

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = Đang lấy { $what } ra

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = Đã lấy { $what } ra khỏi tệp nén vào { $place }.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = lấy ra

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = Hoàn tác việc lấy { $what } ra

# Words on a job card while pictures are turned.
folders-act-turning = Đang xoay { $what }

# Words under the file lines when pictures have been turned.
folders-act-turned = Đã xoay { $what }.

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = xoay

# Words on a job card while pictures are resized.
folders-act-resizing = Đang đổi kích thước { $what }

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = Đã tạo bản sao đổi kích thước của { $what }.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = đổi kích thước

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = Hoàn tác việc đổi kích thước { $what }

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = Thư mục mới

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = Tài liệu mới

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders không thể tạo { $base }: { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = Hãy nhập tên.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = Tên không được chứa dấu gạch chéo.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = Hệ thống đang dùng tên đó. Hãy chọn tên khác.

# Red words under a name being typed when it is too long.
folders-act-name-long = Tên đó quá dài.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders không thể bắt đầu công việc này.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = Làm lại { $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = Hoàn tác { $what }

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = Công việc đã bị dừng trước khi xong.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
       *[other] Folders đã bỏ qua { $n } mục.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Hãy chọn một mục trước.

# Words under the file lines when something already in the Trash is put there again.
# SAFETY: needs a human review before it ships
folders-act-in-trash = Mục đó đã ở trong thùng rác rồi.

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = Tệp trên máy chủ không thể chuyển vào thùng rác. Hãy dùng Xóa vĩnh viễn.

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Hãy chọn thứ cần sao chép hoặc di chuyển trước.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = Chỉ có thể lấy mục ra khỏi tệp nén vào một thư mục.

# Words under the file lines when a copy is sent to the Trash.
# SAFETY: needs a human review before it ships
folders-act-copy-trash = Không thể chuyển bản sao vào thùng rác.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = Ngăn bên kia không hiển thị thư mục nào.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Hãy chọn thứ cần sao chép trước.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = Đã cắt { $what }.

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = Đã sao chép { $what }.

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = Chỉ có thể dán vào một thư mục.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = Không có tệp nào trong bộ nhớ tạm.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = Chưa cắt hay sao chép gì.

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = Không thể xóa các địa điểm mạng từ đây.

# Words under the file lines when Empty the Trash is used on an empty Trash.
# SAFETY: needs a human review before it ships
folders-act-trash-empty = Thùng rác đã trống rồi.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Hãy chọn một mục trong thùng rác trước.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = Đang tạo { $name }

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = Bạn chỉ có thể tạo thư mục hoặc tài liệu mới bên trong một thư mục.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = Hoàn tác việc tạo { $name }

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders không giữ được màu thư mục trên các mục đã di chuyển.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = Không có gì để làm lại.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = Không có gì để hoàn tác.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = Đang làm lại { $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = Đã làm lại { $what }.

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = làm lại

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = Đang hoàn tác { $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = Đã hoàn tác { $what }.

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Thư mục nhà

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Thùng rác

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = Chỉ có thể thả vào một thư mục hoặc thùng rác.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
       *[other] { $n } liên kết
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = Đã tạo { $what }.

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
       *[other] Không thể tạo { $n } liên kết: { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = Có một câu hỏi đang chờ bạn trả lời. Hãy trả lời, hoặc dừng công việc, trước khi đóng Folders.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = Folders không thể đóng khi đang có công việc chạy. Hãy chờ cho xong, hoặc dừng lại, rồi đóng Folders.
