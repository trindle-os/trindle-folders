

folders-tools-pass-once = 一次

folders-tools-pass-three = 3 次

folders-tools-pass-seven = 7 次

folders-tools-case-keep = 保持原样

folders-tools-case-lower = 全部小写

folders-tools-case-upper = 全部大写

folders-tools-case-words = 每个单词首字母大写

folders-tools-case-first = 仅第一个单词首字母大写

folders-tools-ending-keep = 保持原样

folders-tools-ending-lower = 全部小写

folders-tools-ending-upper = 全部大写

folders-tools-ending-new = 新的扩展名

folders-tools-ending-none = 不要扩展名

folders-tools-archive-name = 压缩包

folders-tools-clash-empty = 不会留下任何名称

folders-tools-clash-bad = 不允许使用这个名称

folders-tools-clash-long = 这个名称太长

folders-tools-clash-twice = 有两个会得到这个名称

folders-tools-clash-taken = 已有其他项目使用这个名称

folders-tools-clash-not-text = 旧名称不是纯文本

folders-tools-caveat-flash = 这个磁盘是闪存，不是旋转磁盘，所以旧副本可能仍保留在 Trindle 无法触及的部分中。

folders-tools-caveat-copy-on-write = { $fs } 文件系统会把更改写到新的位置，所以旧数据可能仍留在原处。

folders-tools-caveat-network = 这是另一台电脑上的 { $fs } 共享，它可能自己保留着副本。

folders-tools-row-only-left = 仅在左边

folders-tools-row-only-right = 仅在右边

folders-tools-row-same = 相同

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

folders-tools-place-here = 此处

folders-tools-only-files-wiped = 只能擦除文件。

folders-tools-compare-need-folders = 两个窗格都必须显示文件夹才能比较。

folders-tools-rename-undo =
    { $n ->
       *[other] 撤销重命名 { $n } 项
    }

folders-tools-rename-done =
    { $n ->
       *[other] 已重命名 { $n } 项。
    }

folders-tools-rename-stopped = 重命名中途停止：{ $error }。

folders-tools-name-empty = 名称不能为空。

folders-tools-looked-at = 已查看 { $done }

folders-tools-read-of = 已读取 { $done }，共 { $total }

folders-tools-listing-files = 正在列出文件

folders-tools-comparing = 正在比较

folders-tools-looking = 正在查看

folders-tools-compress-title =
    { $n ->
        [1] 压缩 { $name }
       *[other] 压缩 { $n } 项
    }

folders-tools-pack-name-tip = 压缩包的名称

folders-tools-pack-kind-tip = 压缩包的类型

folders-tools-pack-made-in = 创建在 { $place }

folders-tools-compress = 压缩

folders-tools-extract-to = 将 { $name } 提取到

folders-tools-extract-new-folder = 会在那里创建一个名为 { $name } 的新文件夹。

folders-tools-resize-title =
    { $n ->
        [1] 调整 { $name } 的大小
       *[other] 调整 { $n } 张图片的大小
    }

folders-tools-resize-size-tip = 新尺寸

folders-tools-resize-note = 可以按原尺寸的比例，或按较长边的像素数；图片绝不会被放大。每张图片旁边会生成一个副本，名称中带有新尺寸；图片本身保持不变。

folders-tools-resize = 调整大小

folders-tools-wipe-title =
    { $n ->
        [1] 擦除 { $name } 吗？
       *[other] 擦除 { $n } 个文件吗？
    }

folders-tools-wipe-warning = 每个文件在删除前都会被覆盖写入，所以无法找回。此操作无法撤销。

folders-tools-wipe-write-over = 覆盖写入

folders-tools-wipe-passes-tip = 每个文件覆盖写入的次数

folders-tools-wipe-button = 安全擦除

folders-tools-rename-number-tip = 给它们编号

folders-tools-rename-at-start = 在开头

folders-tools-rename-at-end = 在末尾

folders-tools-rename-where-tip = 编号放在名称的什么位置

folders-tools-rename-first-number-tip = 起始编号

folders-tools-rename-ending-tip = 如何处理扩展名

folders-tools-rename-new-ending-tip = 新扩展名，例如 txt

folders-tools-rename-cannot = 无法重命名：{ $why }

folders-tools-rename-was = 原名 { $old }

folders-tools-rename-stays = 保持不变

folders-tools-rename-title =
    { $n ->
       *[other] 重命名 { $n } 项
    }

folders-tools-rename-replace = 替换

folders-tools-rename-find-tip = 查找

folders-tools-rename-with-tip = 替换为

folders-tools-rename-wildcard-tip = 将 * 和 ? 用作通配符

folders-tools-rename-wildcard = 通配符 * 和 ?

folders-tools-rename-match-case-tip = 严格匹配大小写

folders-tools-rename-match-case = 区分大小写

folders-tools-rename-add = 添加

folders-tools-rename-before-tip = 名称之前

folders-tools-rename-after-tip = 名称之后

folders-tools-rename-number = 编号

folders-tools-rename-letters = 字母

folders-tools-rename-case-tip = 名称中的大小写

folders-tools-rename-ending = 扩展名

folders-tools-rename-summary =
    { $clash ->
        [yes] 有些名称无法使用；请看标记的行。位于 { $place }。
       *[no] { $total } 个名称中有 { $changes } 个会改变。位于 { $place }。
    }

folders-tools-rename-button = 重命名

folders-tools-home = 主文件夹

folders-tools-dupes-title =
    { $n ->
        [1] { $folder } 中的重复项
       *[other] { $n } 个文件夹中的重复项
    }

folders-tools-dupes-stopped = 搜索已停止：{ $error }。

folders-tools-dupes-none = 没有找到重复项。

folders-tools-dupes-unread =
    { $n ->
       *[other] 有 { $n } 个文件无法读取。
    }

folders-tools-dupes-sets =
    { $n ->
       *[other] { $n } 组副本；把多余的副本移到回收站可释放 { $size }。多余的副本已被选中；按住 Ctrl 并点击可选中或取消选中其中一个。
    }

folders-tools-dupes-row = 第 { $set } 组，{ $size }，位于 { $at }

folders-tools-dupes-unread-left-out =
    { $n ->
       *[other] 有 { $n } 个文件无法读取，已排除在外。
    }

folders-tools-dupes-keep-one = 一组中的每个副本都被选中了；请至少保留一个。

folders-tools-dupes-pick-extras = 选中多余的副本

folders-tools-dupes-trash = 移到回收站

folders-tools-diff-title = 比较这两个

folders-tools-diff-failed = 无法比较：{ $error }。

folders-tools-diff-same-bytes = 两者逐字节完全相同。

folders-tools-diff-same-as-shorter = 在较短的那个所及的范围内两者相同；一个是 { $one }，另一个是 { $other }。

folders-tools-diff-from-byte = 从第 { $n } 字节起不同。

folders-tools-diff-lines =
    { $n ->
        [0] 两者相同。
       *[other] 有 { $n } 行不同。
    }

folders-tools-diff-too-different = 两者差异太大，无法逐行匹配，所以第一处和最后一处差异之间的每一行都显示为已更改。

folders-tools-diff-cut = 每个文件只比较前 4 MB。

folders-tools-compare-title = 比较 { $left } 与 { $right }

folders-tools-compare-bytes = 读取每个字节（较慢）

folders-tools-compare-bytes-tip = 比较每个文件的内容

folders-tools-compare-show-same = 显示相同的项目

folders-tools-compare-show-same-tip = 同时显示相同的名称

folders-tools-compare-stopped = 比较已停止：{ $error }。

folders-tools-compare-counts = 仅在左边 { $left } 项，仅在右边 { $right } 项，不同 { $different } 项，相同 { $same } 项。

folders-tools-compare-all-same = 两个文件夹的内容相同。

folders-tools-compare-again = 再次比较

folders-tools-compare-copy-left = 把 { $n } 个缺少或较新的项目复制到左边

folders-tools-compare-copy-right = 把 { $n } 个缺少或较新的项目复制到右边

