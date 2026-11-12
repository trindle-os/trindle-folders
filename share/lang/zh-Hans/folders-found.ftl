
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = 请先输入要在文件内容中查找的字词。

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = 请输入要查找的内容，或选择类型、大小或时间。

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = 只能搜索这台电脑上的文件夹。

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = 显示 { $shown } 项，共 { $all } 项

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = 位于 { $folder }

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = 位于你搜索的文件夹

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    位于 { $folder }。第 { $n } 行：{ $line }{ $more ->
        [0]
       *[other] { " " }（另有 { $more } 行匹配）
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    位于你搜索的文件夹。第 { $n } 行：{ $line }{ $more ->
        [0]
       *[other] { " " }（另有 { $more } 行匹配）
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] 在 { $all } 项中找到 { $n } 项
       *[other] 找到 { $n } 项
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
       *[other] 。{ $skipped } 项因太大或不是文本而被跳过
    }

