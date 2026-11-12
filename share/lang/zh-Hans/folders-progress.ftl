
# machine translated (Simplified Chinese), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done } / { $of } 个文件，每秒 { $rate }，{ $left }
       *[no] 每秒 { $rate }，{ $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } / { $of } 个文件，正在估算所需时间…
       *[no] 正在估算所需时间…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }：{ $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }。{ $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } 字节

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = 还剩几秒

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
       *[other] 还剩约 { $n } 秒
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = 还剩约一分钟

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
       *[other] 还剩约 { $n } 分钟
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
       *[other] 还剩约 { $n } 小时
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
       *[other] 还有 { $n } 项任务正在进行
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] 移动时磁盘会一直很忙，所以在完成之前，其他程序可能会变慢。
       *[copying] 复制时磁盘会一直很忙，所以在完成之前，其他程序可能会变慢。
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = 等待你的回答

# End of a piece of work's line while it is paused.
folders-progress-paused = 已暂停

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = 继续

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = 暂停

# SAFETY: needs a human review before it ships
# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = 停止

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = 更新且更大

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = 更新

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = 更大

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = 修改于 { $when }

# SAFETY: needs a human review before it ships
# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] 那里已有名为 { $name } 的文件夹
        [file] 那里已有名为 { $name } 的文件
       *[unknown] 那里已有 { $name }
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = 新的这个

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = 已在那里的这个

# SAFETY: needs a human review before it ships
# Tip of the button in the name taken question that skips this one.
folders-progress-skip = 跳过

# SAFETY: needs a human review before it ships
# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = 两个都保留

# SAFETY: needs a human review before it ships
# Word on the button in the name taken question that replaces the one already there.
folders-progress-replace = 替换

# SAFETY: needs a human review before it ships
# Screen reader name of the Replace button for a folder in the name taken question.
folders-progress-replace-folder-spoken = 替换，并把原有的文件夹移到回收站

# SAFETY: needs a human review before it ships
# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = 合并

# SAFETY: needs a human review before it ships
# Line over the switch in the name taken question for folders.
folders-progress-all-folders = 对每个已存在的文件夹都用这个回答。

# SAFETY: needs a human review before it ships
# Line over the switch in the name taken question for files.
folders-progress-all-files = 对每个已存在的文件都用这个回答。

# SAFETY: needs a human review before it ships
# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = 其余的也这样做

