

folders-found-need-inside-words = 请先输入要在文件内容中查找的字词。

folders-found-need-something = 请输入要查找的内容，或选择类型、大小或时间。

folders-found-only-here = 只能搜索这台电脑上的文件夹。

folders-found-shown = 显示 { $shown } 项，共 { $all } 项

folders-found-in-folder = 位于 { $folder }

folders-found-in-root = 位于所搜索的文件夹

folders-found-in-folder-line =
    位于 { $folder }。第 { $n } 行：{ $line }{ $more ->
        [0]
       *[other] { " " }（另有 { $more } 行匹配）
    }

folders-found-in-root-line =
    位于所搜索的文件夹。第 { $n } 行：{ $line }{ $more ->
        [0]
       *[other] { " " }（另有 { $more } 行匹配）
    }

folders-found-status =
    找到 { $n } 项{ $filtered ->
        [yes] ，共 { $all } 项
       *[other]
    }{ $looking ->
        [yes] ，仍在查找…
       *[other]
    }{ $full ->
        [yes] 。只显示前 { $most } 项
       *[other]
    }{ $unread ->
        [0]
       *[other] 。{ $unread } 项无法读取
    }{ $skipped ->
        [0]
       *[other] 。{ $skipped } 项因太大或不是文本而跳过
    }

