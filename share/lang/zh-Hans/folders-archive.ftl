
# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Reason an archive would not open: it is locked with a password and none was given.
folders-archive-trouble-locked = 它已用密码锁定

# SAFETY: needs a human review before it ships
# Reason an archive would not open: the password given is not the one.
folders-archive-trouble-wrong-key = 这个密码打不开它

# SAFETY: needs a human review before it ships
# Reason an archive would not open: what came out with the password makes no sense, so the password is wrong or the archive is damaged.
folders-archive-trouble-wrong-or-bad = 这个密码打不开它，或者压缩包已损坏

# SAFETY: needs a human review before it ships
# Reason an archive would not open: it is a RAR locked with a password.
folders-archive-trouble-locked-rar = 它是带密码的 RAR 文件，Folders 无法解锁，但文件本身没有损坏

# SAFETY: needs a human review before it ships
# Reason an archive would not open: it is locked and packed a way Folders does not know.
folders-archive-trouble-locked-foreign = 它使用了 Folders 无法解锁的密码保护方式

# Reason a file would not open as an archive: it is not a kind Folders reads.
folders-archive-trouble-unknown = 它不是 Folders 能打开的压缩包类型

# Reason an archive would not unpack: it holds too many things.
folders-archive-trouble-too-many = 其中有超过一百万个项目，对 Folders 来说太多，无法解压

# Reason an archive would not unpack: the place it was to go has too little room.
folders-archive-trouble-no-room = 这里没有足够的空间存放其中的内容

# Reason an archive would not open: reading it takes more memory or time than is allowed.
folders-archive-trouble-gave-up = 打开它所需的内存或时间超出了 Folders 允许的范围

# Reason an archive would not open: the program that reads this kind is not on the computer.
folders-archive-trouble-no-reader = Trindle 中用来打开这种压缩包的部分缺失了，请安装更新来恢复

# Reason an archive would not open: it is packed a way Folders does not know.
folders-archive-trouble-foreign = 它使用了 Folders 无法读取的压缩方式

# Reason a split archive would not open: one of its parts is not there.
folders-archive-trouble-parts = 缺少其中一部分，而且所有分卷必须放在同一个文件夹里

# Reason given for an archive that cannot be read when the file is damaged or ends too soon.
folders-archive-why-damaged = 它已损坏或不完整

# Reason given for an archive that cannot be written when the drive is full.
folders-archive-why-no-room = 没有剩余空间了

# Reason given for an archive that cannot be unpacked when the folder cannot be written in.
folders-archive-why-read-only = 你没有权限，或者驱动器是只读的

# Reason given for an archive that cannot be read when the file has gone.
folders-archive-why-gone = 它已经不在了

# Reason given for an archive that cannot be read: it needs more memory than is allowed.
folders-archive-why-memory = 读取它所需的内存超出了 Folders 允许的范围

# Reason given for an archive that cannot be read when no better reason is known.
folders-archive-why-other = 出现了意外错误

# Reason a file inside an archive cannot be opened: it is too big.
folders-archive-inside-too-big = 它太大，无法在压缩包内直接打开

# Reason a file cannot be taken from an archive: it is not in it.
folders-archive-inside-missing = 它不在压缩包中

# Reason a 7z archive would not unpack: it needs more memory than is allowed.
folders-archive-7z-memory = 这个 7z 压缩包解压所需的内存超出了 Folders 允许的范围

# Reason making an archive failed: a file grew while it was being packed.
folders-archive-pack-grew = 有文件在 Folders 压缩时变得太大

# Reason making an archive failed: something with the archive's name is already there.
folders-archive-pack-exists = 已经有同名的内容

# Reason making an archive failed: an item has no name to put in it.
folders-archive-pack-no-name = 有一个项目没有 Folders 能放进压缩包的名称

# Reason making an archive failed: Folders reads this kind but cannot write it.
folders-archive-pack-cannot-make = Folders 能打开这种压缩包，但不能创建

# Reason a 7z archive cannot be read; the cause follows, already written out.
folders-archive-damaged-7z = 7z 压缩包已损坏：{ $why }

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock too slow to open.
folders-archive-7z-a-lock-too-slow-to-open = 解锁它需要的时间太长

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock with no settings.
folders-archive-7z-a-lock-with-no-settings = 缺少密码保护的设置

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock with short settings.
folders-archive-7z-a-lock-with-short-settings = 密码保护的设置不完整

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock with no key.
folders-archive-7z-a-lock-with-no-key = 它的密码保护没有可用的密钥

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a locked block is not whole.
folders-archive-7z-a-locked-block-is-not-whole = 其中被加密的一部分不完整

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header ends too soon.
folders-archive-7z-the-header-ends-too-soon = 它的内容列表结束得太早

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a count is far too big.
folders-archive-7z-a-count-is-far-too-big = 其中有个数字大得离谱

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block has no packed bytes.
folders-archive-7z-a-block-has-no-packed-bytes = 其中有一部分没有压缩数据

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block is out of reach.
folders-archive-7z-a-block-is-out-of-reach = 其中有一部分指向了文件末尾之后

# Cause given after "damaged 7z:" when a 7z archive cannot be read: too short.
folders-archive-7z-too-short = 文件太短

# Cause given after "damaged 7z:" when a 7z archive cannot be read: no 7z mark.
folders-archive-7z-no-7z-mark = 它的开头不像 7z 文件

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the start header's check does not match.
folders-archive-7z-the-start-header-s-check-does-not-match = 文件开头未通过校验

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header is out of reach.
folders-archive-7z-the-header-is-out-of-reach = 它的内容列表指向了文件末尾之后

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header's check does not match.
folders-archive-7z-the-header-s-check-does-not-match = 它的内容列表未通过校验

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a packed header with no block.
folders-archive-7z-a-packed-header-with-no-block = 它被压缩的内容列表没有数据

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header is far too big.
folders-archive-7z-the-header-is-far-too-big = 它的内容列表大得离谱

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the packed header's check does not match.
folders-archive-7z-the-packed-header-s-check-does-not-match = 它被压缩的内容列表未通过校验

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown header.
folders-archive-7z-an-unknown-header = 它的内容列表格式是 Folders 不认识的

# Cause given after "damaged 7z:" when a 7z archive cannot be read: headers packed too many times over.
folders-archive-7z-headers-packed-too-many-times-over = 它的内容列表被重复压缩了太多次

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the header.
folders-archive-7z-an-unknown-part-in-the-header = 它的内容列表中有未知的内容

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the streams.
folders-archive-7z-an-unknown-part-in-the-streams = 它的数据中有未知的内容

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the packed streams.
folders-archive-7z-an-unknown-part-in-the-packed-streams = 它的压缩数据中有未知的内容

# Cause given after "damaged 7z:" when a 7z archive cannot be read: packed streams with no sizes.
folders-archive-7z-packed-streams-with-no-sizes = 缺少它的压缩数据的大小信息

# Cause given after "damaged 7z:" when a 7z archive cannot be read: no list of blocks.
folders-archive-7z-no-list-of-blocks = 缺少它各部分的列表

# Cause given after "damaged 7z:" when a 7z archive cannot be read: blocks kept elsewhere.
folders-archive-7z-blocks-kept-elsewhere = 它各部分的列表存放在 Folders 无法读取的地方

# Cause given after "damaged 7z:" when a 7z archive cannot be read: blocks with no sizes.
folders-archive-7z-blocks-with-no-sizes = 缺少它各部分的大小信息

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the blocks.
folders-archive-7z-an-unknown-part-in-the-blocks = 它各部分的列表中有未知的内容

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block with no sensible coders.
folders-archive-7z-a-block-with-no-sensible-coders = 其中有一部分没有合理的压缩步骤

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a coder with too many ends.
folders-archive-7z-a-coder-with-too-many-ends = 其中有个压缩步骤的输入或输出过多

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block's coders do not join up.
folders-archive-7z-a-block-s-coders-do-not-join-up = 其中有一部分的压缩步骤无法衔接

# Cause given after "damaged 7z:" when a 7z archive cannot be read: far too many files.
folders-archive-7z-far-too-many-files = 它列出的文件多得离谱

# Cause given after "damaged 7z:" when a 7z archive cannot be read: files in a block with no sizes.
folders-archive-7z-files-in-a-block-with-no-sizes = 其中某一部分里有些文件没有记录大小

# Cause given after "damaged 7z:" when a 7z archive cannot be read: files bigger than their block.
folders-archive-7z-files-bigger-than-their-block = 有些文件比容纳它们的那一部分还大

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the files' sizes.
folders-archive-7z-an-unknown-part-in-the-files-sizes = 它的文件大小信息中有未知的内容

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a part is far too big.
folders-archive-7z-a-part-is-far-too-big = 其中有一部分大得离谱

# Cause given after "damaged 7z:" when a 7z archive cannot be read: names kept elsewhere.
folders-archive-7z-names-kept-elsewhere = 它的文件名存放在 Folders 无法读取的地方

# Cause given after "damaged 7z:" when a 7z archive cannot be read: too few names.
folders-archive-7z-too-few-names = 缺少一些文件名

# Cause given after "damaged 7z:" when a 7z archive cannot be read: times kept elsewhere.
folders-archive-7z-times-kept-elsewhere = 它的文件日期存放在 Folders 无法读取的地方

# Cause given after "damaged 7z:" when a 7z archive cannot be read: attributes kept elsewhere.
folders-archive-7z-attributes-kept-elsewhere = 它的文件详细信息存放在 Folders 无法读取的地方

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block runs past the end.
folders-archive-7z-a-block-runs-past-the-end = 其中有一部分超出了文件末尾

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block's coders go round in a loop.
folders-archive-7z-a-block-s-coders-go-round-in-a-loop = 其中有一部分的压缩步骤形成了循环

# Cause given after "damaged 7z:" when a 7z archive cannot be read: LZMA with no settings.
folders-archive-7z-lzma-with-no-settings = 它的 LZMA 压缩没有设置

# Cause given after "damaged 7z:" when a 7z archive cannot be read: LZMA2 with no settings.
folders-archive-7z-lzma2-with-no-settings = 它的 LZMA2 压缩没有设置

# Cause given after "damaged 7z:" when a 7z archive cannot be read: LZMA2 with impossible settings.
folders-archive-7z-lzma2-with-impossible-settings = 它的 LZMA2 压缩设置无效

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block would not open.
folders-archive-7z-a-block-would-not-open = Folders 无法打开其中的一部分

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block ends too soon.
folders-archive-7z-a-block-ends-too-soon = 其中有一部分结束得太早

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a file ends too soon.
folders-archive-7z-a-file-ends-too-soon = 其中有个文件结束得太早

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a file's check does not match.
folders-archive-7z-a-file-s-check-does-not-match = 其中有个文件未通过校验

# Cause given after "damaged 7z:" when a 7z archive cannot be read: files' sizes and checks do not match up.
folders-archive-7z-files-sizes-and-checks-do-not-match-up = 它的文件大小与校验对不上

# Cause given after "damaged 7z:" when a 7z archive cannot be read: more files than blocks hold.
folders-archive-7z-more-files-than-blocks-hold = 它列出的文件比各部分能容纳的还多

# Cause given after "damaged 7z:" when a 7z archive cannot be read: more files than sizes.
folders-archive-7z-more-files-than-sizes = 它的有些文件没有记录大小

# Reason a zip archive cannot be read; the cause follows, already written out.
folders-archive-damaged-zip = zip 压缩包已损坏：{ $why }

# Cause given after "damaged zip:" when a zip archive cannot be read: no end record.
folders-archive-zip-no-end-record = 缺少它末尾的记录

# Cause given after "damaged zip:" when a zip archive cannot be read: no ZIP64 end record.
folders-archive-zip-no-zip64-end-record = 缺少它末尾的 ZIP64 记录

# Cause given after "damaged zip:" when a zip archive cannot be read: directory out of place.
folders-archive-zip-directory-out-of-place = 它的内容列表位置不对

# Cause given after "damaged zip:" when a zip archive cannot be read: directory too big.
folders-archive-zip-directory-too-big = 它的内容列表太大

# Cause given after "damaged zip:" when a zip archive cannot be read: record runs past the directory.
folders-archive-zip-record-runs-past-the-directory = 有一个条目超出了内容列表的末尾

# Cause given after "damaged zip:" when a zip archive cannot be read: no local header.
folders-archive-zip-no-local-header = 缺少其中某个文件前面的详细信息

# Cause given after "damaged zip:" when a zip archive cannot be read: entry runs past the end.
folders-archive-zip-entry-runs-past-the-end = 其中有个文件超出了压缩包的末尾

# Cause given after "damaged zip:" when a zip archive cannot be read: an entry's check does not match.
folders-archive-zip-an-entry-s-check-does-not-match = 其中有个文件未通过校验

# Cause given after "damaged zip:" when a zip archive cannot be read: an entry is longer than it says.
folders-archive-zip-an-entry-is-longer-than-it-says = 其中有个文件比记录的大小更长

# Cause given after "damaged zip:" when a zip archive cannot be read: a locked entry is cut short.
folders-archive-zip-a-locked-entry-is-cut-short = 其中有个有密码保护的文件不完整

# Cause given after "damaged zip:" when a zip archive cannot be read: a lock with a key of no known length.
folders-archive-zip-a-lock-with-a-key-of-no-known-length = 它的密码保护使用了 Folders 不认识的密钥长度

# Cause given after "damaged zip:" when a zip archive cannot be read: a locked entry's check does not match.
folders-archive-zip-a-locked-entry-s-check-does-not-match = 其中有个有密码保护的文件未通过校验
