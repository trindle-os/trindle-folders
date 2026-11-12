
# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = 隐藏的，或在隐藏的文件夹中

# SAFETY: needs a human review before it ships
# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = 里面有隐藏的项目

# SAFETY: needs a human review before it ships
# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = 里面的项目太多，无法检查，所以 Folders 会忽略其中的隐藏项目

# SAFETY: needs a human review before it ships
# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = 在你的主文件夹和驱动器之外

# SAFETY: needs a human review before it ships
# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = 链接指向隐藏的项目，或指向你的主文件夹和驱动器之外

# SAFETY: needs a human review before it ships
# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = 里面有链接，指向隐藏的项目，或指向你的主文件夹和驱动器之外

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [1] 把 { $name } 移到这里吗？
       *[other] 把这 { $n } 项移到这里吗？
    }

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [1] 把 { $name } 带到这里吗？
       *[other] 把这 { $n } 项带到这里吗？
    }

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [1] 把 { $name } 复制到这里吗？
       *[other] 把这 { $n } 项复制到这里吗？
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be moved: why it is asked.
folders-outside-body-move =
    { $n ->
       *[other] 另一个程序把这些内容拖放到了 Folders 上。任何程序都可以拖放任何文件，而其中有些并不是你的普通文件。在你同意之前，Folders 不会移动它们。
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
       *[other] 另一个程序把这些内容拖放到了 Folders 上。任何程序都可以拖放任何文件，而其中有些并不是你的普通文件。在你同意之前，Folders 不会把它们放进这里。
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
       *[other] 另一个程序把这些内容拖放到了 Folders 上。任何程序都可以拖放任何文件，而其中有些并不是你的普通文件。在你同意之前，Folders 不会复制它们。
    }

# SAFETY: needs a human review before it ships
# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
       *[other] 不要动
    }

# SAFETY: needs a human review before it ships
# Button on the question card that moves what was dropped.
folders-outside-do-move =
    { $n ->
       *[other] 移动
    }

# SAFETY: needs a human review before it ships
# Button on the question card that brings what was dropped.
folders-outside-do-bring =
    { $n ->
       *[other] 带入
    }

# SAFETY: needs a human review before it ships
# Button on the question card that copies what was dropped.
folders-outside-do-copy =
    { $n ->
       *[other] 复制
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }，来自 { $from }：{ $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }，来自 { $from }

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = 另有 { $n } 项。

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = 放入 { $into }。

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = 请先回答关于你所拖放内容的问题。

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders 仍在检查你上次拖放的内容。

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = 这里只能拖放文件和文件夹。

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = 只有文件夹才能固定到侧边栏。

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = 只能把项目拖放到文件夹上。

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } 已不在原处，所以没有复制或移动任何东西。

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = 文件夹不能放进它自己里面。

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = 它们已经在那个文件夹里了。

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders 没能及时连上 { $name } 所在的驱动器，所以没有复制或移动任何东西。

# Words when the question card is answered with no.
folders-outside-nothing-brought = 没有复制或移动任何东西。

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
       *[other] { $n } 项
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = 另有 { $n } 项

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name }{ $why }。

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
       *[other] 有 { $n } 个项目在你开始拖动后发生了变化，所以 Folders 把它们留在了原处：{ $names }。
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
       *[other] 有 { $n } 项仍留在原处：{ $each }。
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name }（{ $why }）

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders 无法确定另一个程序是否收到了你拖动的全部内容，所以把原件留在了这里。另一个程序拿到的是副本。

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = 另一个程序没有接收你拖动的全部内容，所以 Folders 没有移动任何东西。

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = 另一个程序没有接收你拖动的全部内容，所以它那里的副本可能不完整。

