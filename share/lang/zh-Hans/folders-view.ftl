
# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Title of the question window asked before the Trash is emptied.
folders-view-empty-trash-title = 清空回收站吗？

# SAFETY: needs a human review before it ships
# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
folders-view-empty-trash-body =
    { $n ->
       *[other] 其中全部 { $n } 项都会被永久删除。此操作无法撤销。
    }

# SAFETY: needs a human review before it ships
# Red button of the question asked before the Trash is emptied: empties it at once.
folders-view-empty-trash = 清空回收站

# SAFETY: needs a human review before it ships
# Title of the question window asked before items are deleted for good; n is how many.
folders-view-delete-title =
    { $n ->
       *[other] 永久删除这 { $n } 项吗？
    }

# SAFETY: needs a human review before it ships
# Text of the question asked before items are deleted for good.
folders-view-delete-body = 此操作无法撤销。

# SAFETY: needs a human review before it ships
# Red button of the question asked before items are deleted for good: deletes them at once.
folders-view-delete-permanently = 永久删除

# SAFETY: needs a human review before it ships
# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
folders-view-keep-them = 保留它们

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = 它们应该放到哪里？

# SAFETY: needs a human review before it ships
# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = 让它们留在原处




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = 图标

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = 列表

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = 紧凑

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = 选择列

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = 名称

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = 排序方式

# Name read out for the toolbar's Group by strip.
folders-view-group-by = 分组方式





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = 隐藏预览面板（Alt+P）

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = 显示预览面板（Alt+P）

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = 预览面板

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = 标签页

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = 左窗格标签页

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = 右窗格标签页

# Tip of the button that opens a new tab.
folders-view-tab-new = 新建标签页（Ctrl+T）

# Tip of the button that closes the tab showing.
folders-view-tab-close = 关闭标签页（Ctrl+W）






# Side bar row that opens the Home folder.
folders-view-home = 主文件夹

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = 还没有固定任何内容。右键点击文件夹即可固定。

# Small heading in the side bar over the pinned folders.
folders-view-pinned = 已固定

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = 位置

# Small heading in the side bar over the drives plugged in.
folders-view-drives = 驱动器

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = 共享

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = 关闭 { $name }



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = 连接到服务器

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = 联动两个窗格。{ $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = 联动两个窗格

# SAFETY: needs a human review before it ships
# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] 复制到左窗格
       *[other] 复制到右窗格
    }

# SAFETY: needs a human review before it ships
# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] 移动到左窗格
       *[other] 移动到右窗格
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] 在左窗格中显示此文件夹
       *[other] 在右窗格中显示此文件夹
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = 交换两个窗格

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] 选中与左窗格不同的项目
       *[other] 选中与右窗格不同的项目
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] 与左窗格比较
       *[other] 与右窗格比较
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = 输入文件夹、文件或服务器地址

# Name read out for the path above a pane's list.
folders-view-where = 位置

# Tip of the pane's Back button.
folders-view-back = 后退（Alt+向左键）

# Tip of the pane's Forward button.
folders-view-forward = 前进（Alt+向右键）

# Tip of the pane's button that goes up one folder.
folders-view-up = 上一级文件夹（Alt+向上键）



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = 筛选，或按 Enter 搜索此文件夹及其子文件夹

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = 按名称搜索，包括子文件夹

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = 搜索文件内容，包括子文件夹


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = 类型

# Name read out for the list that picks which size to show.
folders-view-find-size = 大小

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = 最后修改时间

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = { $was } 的新名称

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = 可用 { $size }

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = 已选择 { $count } 项，正在计算大小…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = 已选择 { $count } 项，{ $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }。{ $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = 显示与此相关的文件

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = 重试

# SAFETY: needs a human review before it ships
# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
folders-view-cancel = 取消

# Line on a pane whose filter matches nothing.
folders-view-none-matches = 这里没有与筛选条件匹配的内容

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = 显示全部

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = 你最近没有打开过任何内容

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = 前往主文件夹

# Line on the Trash page when it is empty.
folders-view-none-trash = 回收站是空的

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = 没有可显示的驱动器

# Line on a search page that found nothing.
folders-view-none-found = 什么也没找到

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = 清除搜索

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = 此压缩包是空的

# Button on an empty archive page: goes up one folder.
folders-view-go-up = 上一级文件夹

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = 此文件夹是空的

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = 新建文件夹

# Button on an empty folder's page that makes a new document.
folders-view-new-document = 新建文档

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
       *[other] { $n } 项
    }


# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = 未连接

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = 重新连接 { $name }

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } 未连接。

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = 全部重新连接

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] 有 1 个共享未连接。
       *[other] 有 { $n } 个共享未连接。
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = 忘记 { $name }

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = 双窗格

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = 并排显示两个窗格（F3）

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = 显示一个窗格（F3）

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = 查看

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = 新建

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = 更多选项

# Row of the More menu that undoes the last change.
folders-view-more-undo = 撤销

# Row of the More menu that does again what was undone.
folders-view-more-redo = 重做

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = 反向排序

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = 显示隐藏文件（Ctrl+H）

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = 隐藏隐藏文件（Ctrl+H）

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = 隐藏文件

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = 附近的电脑

# Tip of the Search button in the top row while the path shows.
folders-view-search = 搜索（Ctrl+F）

# Tip of the Search button while the search box shows.
folders-view-search-close = 关闭搜索（Escape）

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = 无法在此页面上搜索

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = 搜索

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = { $was } 的侧边栏名称
