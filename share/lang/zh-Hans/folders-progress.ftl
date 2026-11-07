

folders-progress-pace-known =
    { $files ->
        [yes] { $done } / { $of } 个文件，每秒 { $rate }，{ $left }
       *[no] 每秒 { $rate }，{ $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $done } / { $of } 个文件，正在估算所需时间…
       *[no] 正在估算所需时间…
    }

folders-progress-line = { $doing }：{ $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } 字节

folders-progress-speed-kb = { $n } KB

folders-progress-speed-mb = { $n } MB

folders-progress-speed-gb = { $n } GB

folders-progress-left-few = 还剩几秒

folders-progress-left-seconds =
    { $n ->
       *[other] 还剩约 { $n } 秒
    }

folders-progress-left-minute = 还剩约一分钟

folders-progress-left-minutes =
    { $n ->
       *[other] 还剩约 { $n } 分钟
    }

folders-progress-left-hours =
    { $n ->
       *[other] 还剩约 { $n } 小时
    }

folders-progress-more =
    { $n ->
       *[other] 还有 { $n } 项任务正在进行
    }

folders-progress-heavy =
    { $verb ->
        [moving] 移动会大量占用磁盘，所以在结束之前，其他程序可能会变慢。
       *[copying] 复制会大量占用磁盘，所以在结束之前，其他程序可能会变慢。
    }

folders-progress-waiting = 等待你的回答

folders-progress-paused = 已暂停

folders-progress-carry-on = 继续

folders-progress-pause = 暂停

folders-progress-stop = 停止

folders-progress-mark-newer-larger = 更新，更大

folders-progress-mark-newer = 更新

folders-progress-mark-larger = 更大

folders-progress-changed = 修改于 { $when }

folders-progress-taken =
    { $kind ->
        [folder] 那里已有名为 { $name } 的文件夹
        [file] 那里已有名为 { $name } 的文件
       *[unknown] 那里已有 { $name }
    }

folders-progress-coming = 要带入的这个

folders-progress-there = 已在那里的这个

folders-progress-skip = 跳过

folders-progress-keep-both = 两个都保留

folders-progress-replace = 替换

folders-progress-replace-folder-spoken = 替换：已在那里的文件夹会被移到回收站

folders-progress-merge = 合并

folders-progress-all-folders = 对每个已在那里的文件夹都给出相同的回答。

folders-progress-all-files = 对每个已在那里的文件都给出相同的回答。

folders-progress-same-for-rest = 其余的也这样做

