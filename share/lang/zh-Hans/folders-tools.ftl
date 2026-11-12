
# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Choice in the Wipe window: write over each file one time.
folders-tools-pass-once = 一次

# SAFETY: needs a human review before it ships
# Choice in the Wipe window: write over each file three times.
folders-tools-pass-three = 3 次

# SAFETY: needs a human review before it ships
# Choice in the Wipe window: write over each file seven times.
folders-tools-pass-seven = 7 次

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = 保持原样

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = 全部小写

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = 全部大写

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = 每个单词首字母大写

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = 仅第一个单词首字母大写

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = 保持原样

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = 全部小写

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = 全部大写

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = 新扩展名

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = 不要扩展名

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = 压缩包

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = 不会留下任何名称

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = 不允许使用这个名称

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = 这个名称太长

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = 有两个项目会得到这个名称

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = 已有其他项目使用这个名称

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = 旧名称里有 Folders 无法读取的字符

# SAFETY: needs a human review before it ships
# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
folders-tools-caveat-flash = 这是闪存盘或固态硬盘（SSD），所以旧副本可能仍留在 Folders 无法触及的部分中。

# SAFETY: needs a human review before it ships
# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
folders-tools-caveat-copy-on-write = { $fs } 文件系统会把更改保存到新的位置，所以旧数据可能仍然在那里。

# SAFETY: needs a human review before it ships
# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
folders-tools-caveat-network = 这是另一台电脑上的 { $fs } 共享，它可能保留着自己的副本。

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = 仅在左边

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = 仅在右边

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = 相同

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
folders-tools-row-different =
    不同：{ $why ->
        [kind] 一个是文件夹或链接
        [size] 大小不同
        [date] 日期不同
        [content] 内容不同
        [target] 链接指向不同的地方
        [inside] 里面有不同之处
       *[unread] 有一边无法读取
    }{ $newer ->
        [left] ，左边较新
        [right] ，右边较新
       *[none] { "" }
    }

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = 此处

# SAFETY: needs a human review before it ships
# Red notice when Wipe is chosen on things that are not files.
folders-tools-only-files-wiped = 只能擦除文件。

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = 请在每个窗格中打开一个文件夹，再进行比较。

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
       *[other] 撤销重命名 { $n } 项
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
       *[other] 已重命名 { $n } 项。
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = 重命名中途停止：{ $error }。

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = 请为压缩包输入名称。

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = 已检查 { $done }

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = 已读取 { $done }，共 { $total }

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = 正在列出文件

# Beside the ring while two files are compared.
folders-tools-comparing = 正在比较

# Beside the ring while two folders are compared.
folders-tools-looking = 正在比较这些文件夹

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [1] 压缩 { $name }
       *[other] 压缩 { $n } 项
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = 压缩包的名称

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = 压缩包的类型

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = 压缩包将保存到 { $place }

# Main button of the Compress window.
folders-tools-compress = 压缩

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = 将 { $name } 提取到

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = 文件会放进一个名为 { $name } 的新文件夹。

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [1] 调整 { $name } 的大小
       *[other] 调整 { $n } 张图片的大小
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = 新尺寸

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = 尺寸是指较长边的像素长度。Folders 只会缩小图片。它会把每个调整后的副本保存在原图旁边，名称中带有新尺寸。

# Main button of the Resize window.
folders-tools-resize = 调整大小

# SAFETY: needs a human review before it ships
# Title of the Wipe window; { $name } is the one file, { $n } how many.
folders-tools-wipe-title =
    { $n ->
        [1] 擦除 { $name } 吗？
       *[other] 擦除 { $n } 个文件吗？
    }

# SAFETY: needs a human review before it ships
# Warning at the top of the Wipe window.
folders-tools-wipe-warning = Folders 会在删除每个文件前先将其覆盖写入，所以无法找回。

# SAFETY: needs a human review before it ships
# Name beside the choice of how many times to write over, in the Wipe window.
folders-tools-wipe-write-over = 覆盖写入

# SAFETY: needs a human review before it ships
# Choice tip in the Wipe window: how many times each file is written over.
folders-tools-wipe-passes-tip = 每个文件覆盖写入的次数

# SAFETY: needs a human review before it ships
# Red main button of the Wipe window.
folders-tools-wipe-button = 安全擦除

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = 给它们编号

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = 在开头

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = 在末尾

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = 编号放在名称的什么位置

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = 起始编号

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = 如何处理扩展名

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = 新扩展名，例如 txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders 无法重命名此项：{ $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = 原名 { $old }

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = 保持不变

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
       *[other] 重命名 { $n } 项
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = 替换

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = 查找

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = 替换为

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = 将 * 和 ? 用作通配符

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = 通配符 * 和 ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = 严格匹配大小写

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = 区分大小写

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = 添加

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = 名称之前

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = 名称之后

# Name of the numbering row in the rename window.
folders-tools-rename-number = 编号

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = 大小写

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = 名称中的大小写

# Name over the ending choice in the rename window.
folders-tools-rename-ending = 扩展名

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] 在 { $place } 中，有些名称无法使用。请看标记的行。
       *[no] 在 { $place } 中，{ $total } 个名称中有 { $changes } 个会改变。
    }

# Main button of the rename window.
folders-tools-rename-button = 重命名

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = 主文件夹

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [1] { $folder } 中的重复项
       *[other] { $n } 个文件夹中的重复项
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = 搜索已停止：{ $error }。

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = 没有找到重复项。

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
       *[other] 有 { $n } 个文件无法读取。
    }

# SAFETY: needs a human review before it ships
# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
       *[other] { $n } 组副本。多余的副本已被选中，把它们移到回收站可释放 { $size }。
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = 第 { $set } 组，{ $size }，位于 { $at }

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
       *[other] 有 { $n } 个文件无法读取，已排除在外。
    }

# SAFETY: needs a human review before it ships
# Note in the duplicates window when every copy of a set is picked.
folders-tools-dupes-keep-one = 一组中的每个副本都被选中了。请至少保留一个不选。

# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = 选中多余的副本

# SAFETY: needs a human review before it ships
# Main button of the duplicates window: move the picked files to the Trash.
folders-tools-dupes-trash = 移到回收站

# Title of the window that compares two files.
folders-tools-diff-title = 比较两个文件

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders 无法比较它们：{ $error }。

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = 这两个文件完全相同。

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = 两者在较短的那个结束之前都是相同的。一个是 { $one }，另一个是 { $other }。

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = 从第 { $n } 字节起不同。

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] 这两个文件相同。
       *[other] 有 { $n } 行不同。
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = 两者差异太大，无法逐行比较。第一处和最后一处差异之间的所有内容都显示为已更改。

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = 每个文件只比较前 4 MB。

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = 比较 { $left } 与 { $right }

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = 比较内容（较慢）

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = 比较每个文件的内容

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = 显示名称相同的项目

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = 同时列出名称相同的项目

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = 比较已停止：{ $error }。

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = 仅在左边 { $left } 项，仅在右边 { $right } 项，不同 { $different } 项，相同 { $same } 项。

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = 两个文件夹的内容相同。

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = 再次比较

# SAFETY: needs a human review before it ships
# Button in the pane compare window; { $n } is how many names are copied to the left folder.
folders-tools-compare-copy-left = 把 { $n } 个缺少或较新的项目复制到左边

# SAFETY: needs a human review before it ships
# Button in the pane compare window; { $n } is how many names are copied to the right folder.
folders-tools-compare-copy-right = 把 { $n } 个缺少或较新的项目复制到右边

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = 只会擦除文件，所以这些文件夹将保持原样：{ $names }。
