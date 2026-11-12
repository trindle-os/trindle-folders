
# machine translated (Vietnamese), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = Không thể thay đổi gì trong bản sao cũ. Hãy sao chép các mục ra ngoài để giữ lại.

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] lúc { $time } hôm nay
        [yesterday] lúc { $time } hôm qua
       *[other] lúc { $time } ngày { $date }
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] Lúc { $time } hôm nay
        [yesterday] Lúc { $time } hôm qua
       *[other] Lúc { $time } ngày { $date }
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = Đây là bản sao cũ từ { $when }. Bạn không thể thay đổi nó, nhưng có thể sao chép các mục ra ngoài.

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = Đây là bản sao cũ. Bạn không thể thay đổi nó, nhưng có thể sao chép các mục ra ngoài.

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = Bản cũ: { $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = Hoàn tác việc khôi phục { $name }

# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = Folders đã khôi phục nó. Bản hiện tại nằm bên cạnh, dưới một tên mới.

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = Folders đã khôi phục nó về chỗ cũ.

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = Giống bản hiện tại.

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = Khác với bản hiện tại.

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = Folders không thể so sánh chúng: { $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = Các phiên bản cũ

# Menu row on the folder's menu that shows what was deleted from the folder.
# SAFETY: needs a human review before it ships
folders-earlier-menu-deleted = Các mục đã xóa khỏi đây

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = Các phiên bản cũ của { $name }

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = Không có phiên bản cũ nào khác với bản hiện tại.

# Note at the foot of the earlier versions card about what putting one back does.
folders-earlier-versions-foot = Khôi phục một bản sẽ giữ bản hiện tại bên cạnh nó. Bạn có thể hoàn tác việc này.

# Title of the card that lists what was deleted from a folder.
# SAFETY: needs a human review before it ships
folders-earlier-deleted-title = Các mục đã xóa khỏi { $name }

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = Không có gì từ các bản sao cũ bị xóa khỏi thư mục này.

# Note at the foot of the deleted things card about what putting one back does.
folders-earlier-deleted-foot = Khôi phục một mục sẽ trả nó về thư mục này. Bạn có thể hoàn tác việc này.

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = Thư mục như lúc đó

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }, từ bản sao tạo ngày { $date }

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = Một thư mục, thấy lần cuối { $when }

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }, thấy lần cuối { $when }

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = Đang xử lý…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = Mở thư mục của ngày đó trong thẻ mới

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = Mở

# Tip and name of the round button on a row of the earlier versions card that puts it back.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-tip = Khôi phục

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = So sánh với bản hiện tại

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = Đó không phải là một phiên bản cũ.

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = Bạn chỉ có thể khôi phục phiên bản cũ vào trong thư mục chính của mình.

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = Folders không thể khôi phục tệp về đó.

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = Folders không thể tạo lại thư mục chứa nó: { $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = Folders không thể sao chép phiên bản cũ: { $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = Folders không thể dời bản hiện tại sang một bên: { $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = Folders không thể khôi phục phiên bản cũ: { $why }. Bản hiện tại nay có tên là { $name }.

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = Folders không thể khôi phục phiên bản cũ: { $why }
