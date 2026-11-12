# machine translated (Vietnamese), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = Hãy nhập trước những từ cần tìm bên trong tệp.

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = Hãy nhập nội dung cần tìm, hoặc chọn một loại, một cỡ hay một khoảng thời gian.

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = Chỉ tìm được trong các thư mục trên máy này.

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = Hiện { $shown } trên { $all }

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = Trong { $folder }

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = Trong thư mục bạn đã tìm

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    Trong { $folder }. Dòng { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(và { $more } dòng khác)
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    Trong thư mục bạn đã tìm. Dòng { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(và { $more } dòng khác)
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] Tìm thấy { $n } trên { $all }
       *[other] Tìm thấy { $n }
    }{ $looking ->
        [yes] , vẫn đang tìm…
       *[other]
    }{ $full ->
        [yes] . Chỉ hiện { $most } mục đầu tiên
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } không đọc được
    }{ $skipped ->
        [0]
       *[other] . { $skipped } bị bỏ qua vì quá lớn hoặc không phải văn bản
    }


