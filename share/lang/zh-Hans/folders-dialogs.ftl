
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = 关闭

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = 主文件夹

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = 用以下程序打开 { $name }

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = 当前应用

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = 这台电脑上没有设置为可打开此类文件的应用。

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = 始终使用此应用

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = 用于所有 { $kind } 文件。

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = 设为默认应用

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = 只显示推荐的应用

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = 显示所有应用

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = 打开

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = 请先选择一个应用。

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders 无法将 { $name } 设为默认应用。

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders 无法启动 { $name }。

# Heading of the Properties window.
folders-dialogs-properties = 属性

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = 属性页面

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = 常规

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = 详细信息

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = 共享



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = 名称

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = 位置

# Label in Properties of the kind of file.
folders-dialogs-row-kind = 种类

# Label in Properties of where a link points.
folders-dialogs-row-link-to = 链接到

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = 修改时间

# Label in Properties of who owns the item.
folders-dialogs-row-owner = 所有者

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = 权限

# Label in Properties of the size of the item.
folders-dialogs-row-size = 大小

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = 未知

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words }（{ $kind }）

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }，你可以打开和修改

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }，你可以打开，但不能修改

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }，你可以修改，但不能打开

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }，你不能打开

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] ，{ $items }
       *[other]
    }{ $unread ->
        [yes] ，有些无法读取
       *[other]
    }{ $adding ->
        [yes] ，仍在计算
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = 正在计算大小

# Label in Properties of the row about checksums.
folders-dialogs-checksums = 校验和

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = 计算校验和

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = 正在计算校验和

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders 无法读取该文件。

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = 粘贴校验和以进行比较

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } 一致

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = 不一致

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = 核对


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [1] 将 { $name } 发送到
       *[other] 将 { $n } 个项目发送到
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = 会在目标位置创建一份副本，原项目保持不动。

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = 另一个窗格：{ $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = 列表视图的列

# Note in the Columns window.
folders-dialogs-columns-note = 名称始终显示。放不下的列会先隐藏，窗格变宽后再显示。

# Heading over the columns that suit every file.
folders-dialogs-columns-files = 文件

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = 音乐、照片、PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = 显示{ $name }

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders 无法保存列的设置。

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [1] 为 { $name } 选择颜色
       *[other] 为 { $n } 个文件夹选择颜色
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = 无颜色

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders 无法保存文件夹颜色。


folders-dialogs-no-match = 没有符合这些字词的应用。


folders-dialogs-find-app = 查找应用


folders-dialogs-find-place = 查找位置


folders-dialogs-no-match-place = 没有符合这些字词的位置。
