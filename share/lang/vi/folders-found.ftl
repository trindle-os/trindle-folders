# machine translated (Vietnamese), not yet reviewed by a person.

folders-found-need-inside-words = Hãy nhập trước những từ cần tìm bên trong tệp.

folders-found-need-something = Hãy nhập nội dung cần tìm, hoặc chọn một loại, một cỡ hay một khoảng thời gian.

folders-found-only-here = Chỉ tìm được trong các thư mục trên máy này.

folders-found-shown = Hiện { $shown } trên { $all }

folders-found-in-folder = Trong { $folder }

folders-found-in-root = Trong thư mục đã tìm

folders-found-in-folder-line =
    Trong { $folder }. Dòng { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(và { $more } dòng khác)
    }

folders-found-in-root-line =
    Trong thư mục đã tìm. Dòng { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(và { $more } dòng khác)
    }

folders-found-status =
    { $n ->
       *[other] Tìm thấy { $n }
    }{ $filtered ->
        [yes] { " " }trên { $all }
       *[other]
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

