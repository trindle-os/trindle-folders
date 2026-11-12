
# machine translated (Vietnamese), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = đã ẩn, hoặc nằm trong thư mục ẩn

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = có các mục ẩn bên trong

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = có quá nhiều mục để kiểm tra, nên Folders sẽ bỏ qua các mục ẩn của nó

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = nằm ngoài thư mục nhà và các ổ đĩa của bạn

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = một liên kết tới thứ đã ẩn, hoặc nằm ngoài thư mục nhà và các ổ đĩa của bạn

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = có một liên kết bên trong tới thứ đã ẩn, hoặc nằm ngoài thư mục nhà và các ổ đĩa của bạn

# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
# SAFETY: needs a human review before it ships
folders-outside-title-move =
    { $n ->
        [1] Chuyển { $name } vào đây?
       *[other] Chuyển { $n } mục này vào đây?
    }

# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
# SAFETY: needs a human review before it ships
folders-outside-title-bring =
    { $n ->
        [1] Đưa { $name } vào đây?
       *[other] Đưa { $n } mục này vào đây?
    }

# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
# SAFETY: needs a human review before it ships
folders-outside-title-copy =
    { $n ->
        [1] Sao chép { $name } vào đây?
       *[other] Sao chép { $n } mục này vào đây?
    }

# Words on the question card when the drop would be moved: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-move =
    { $n ->
       *[other] Một chương trình khác đã thả các mục này vào Folders. Chương trình nào cũng có thể thả bất kỳ tệp nào, và có thể có mục không phải tệp thông thường của bạn. Folders sẽ không di chuyển chúng cho đến khi bạn đồng ý.
    }

# Words on the question card when the drop would be brought: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-bring =
    { $n ->
       *[other] Một chương trình khác đã thả các mục này vào Folders. Chương trình nào cũng có thể thả bất kỳ tệp nào, và có thể có mục không phải tệp thông thường của bạn. Folders sẽ không đưa chúng vào cho đến khi bạn đồng ý.
    }

# Words on the question card when the drop would be copied: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-copy =
    { $n ->
       *[other] Một chương trình khác đã thả các mục này vào Folders. Chương trình nào cũng có thể thả bất kỳ tệp nào, và có thể có mục không phải tệp thông thường của bạn. Folders sẽ không sao chép chúng cho đến khi bạn đồng ý.
    }

# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
# SAFETY: needs a human review before it ships
folders-outside-leave =
    { $n ->
       *[other] Để yên
    }

# Button on the question card that moves what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-move =
    { $n ->
       *[other] Chuyển vào
    }

# Button on the question card that brings what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-bring =
    { $n ->
       *[other] Đưa vào
    }

# Button on the question card that copies what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-copy =
    { $n ->
       *[other] Sao chép
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, từ { $from }: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, từ { $from }

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = Và { $n } mục nữa.

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = Vào { $into }.

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Hãy trả lời câu hỏi về những gì bạn đã thả vào trước.

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders vẫn đang kiểm tra mục bạn thả vào gần nhất.

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Ở đây chỉ có thể thả tệp và thư mục.

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = Chỉ có thể ghim thư mục vào thanh bên.

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = Chỉ có thể thả vào một thư mục.

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } không còn ở đó nữa, nên không có gì được sao chép hoặc di chuyển.

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = Không thể đặt một thư mục vào chính nó.

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = Chúng đã ở trong thư mục đó rồi.

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders không kết nối được kịp với ổ đĩa chứa { $name }, nên không có gì được sao chép hoặc di chuyển.

# Words when the question card is answered with no.
folders-outside-nothing-brought = Không có gì được sao chép hoặc di chuyển.

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
       *[other] { $n } mục
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = và { $n } mục nữa

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
       *[other] { $n } mục đã thay đổi sau khi bạn bắt đầu kéo, nên Folders để chúng ở chỗ cũ: { $names }.
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
       *[other] { $n } mục vẫn ở chỗ cũ: { $each }.
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders không biết chương trình kia đã nhận đủ mọi thứ bạn kéo hay chưa, nên đã để các bản gốc ở đây. Chương trình kia có một bản sao.

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = Chương trình kia không nhận hết những gì bạn kéo, nên Folders không di chuyển gì cả.

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = Chương trình kia không nhận hết những gì bạn kéo, nên bản sao của nó có thể chưa đầy đủ.
