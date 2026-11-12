
# machine translated (Indonesian), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = tersembunyi, atau berada di folder tersembunyi

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = berisi item tersembunyi

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = memiliki terlalu banyak item untuk diperiksa, jadi Folders akan melewatkan item tersembunyinya

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = berada di luar folder Home dan drive Anda

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = tautan ke sesuatu yang tersembunyi, atau di luar folder Home dan drive Anda

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = berisi tautan ke sesuatu yang tersembunyi, atau di luar folder Home dan drive Anda

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [1] Pindahkan { $name } ke sini?
       *[other] Pindahkan { $n } item ini ke sini?
    }

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [1] Bawa { $name } ke sini?
       *[other] Bawa { $n } item ini ke sini?
    }

# SAFETY: needs a human review before it ships
# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [1] Salin { $name } ke sini?
       *[other] Salin { $n } item ini ke sini?
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be moved: why it is asked.
folders-outside-body-move =
    { $n ->
       *[other] Program lain menjatuhkannya ke Folders. Program mana pun dapat menjatuhkan file apa pun, dan yang dijatuhkan di sini belum tentu file biasa milik Anda. Folders tidak akan memindahkannya sampai Anda mengizinkan.
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
       *[other] Program lain menjatuhkannya ke Folders. Program mana pun dapat menjatuhkan file apa pun, dan yang dijatuhkan di sini belum tentu file biasa milik Anda. Folders tidak akan membawanya ke sini sampai Anda mengizinkan.
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
       *[other] Program lain menjatuhkannya ke Folders. Program mana pun dapat menjatuhkan file apa pun, dan yang dijatuhkan di sini belum tentu file biasa milik Anda. Folders tidak akan menyalinnya sampai Anda mengizinkan.
    }

# SAFETY: needs a human review before it ships
# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
       *[other] Biarkan saja
    }

# SAFETY: needs a human review before it ships
# Button on the question card that moves what was dropped.
folders-outside-do-move =
    { $n ->
       *[other] Pindahkan
    }

# SAFETY: needs a human review before it ships
# Button on the question card that brings what was dropped.
folders-outside-do-bring =
    { $n ->
       *[other] Bawa
    }

# SAFETY: needs a human review before it ships
# Button on the question card that copies what was dropped.
folders-outside-do-copy =
    { $n ->
       *[other] Salin
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, dari { $from }: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, dari { $from }

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = Dan { $n } lagi.

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = Ke { $into }.

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Jawab dulu pertanyaan tentang yang Anda jatuhkan.

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders masih memeriksa yang terakhir Anda jatuhkan.

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Hanya file dan folder yang dapat dijatuhkan di sini.

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = Hanya folder yang dapat disematkan ke bilah samping.

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = Item hanya dapat dijatuhkan pada folder.

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } sudah tidak ada, jadi tidak ada yang disalin atau dipindahkan.

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = Folder tidak dapat dimasukkan ke dalam dirinya sendiri.

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = Item itu sudah ada di folder tersebut.

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders tidak dapat menjangkau drive tempat { $name } berada tepat waktu, jadi tidak ada yang disalin atau dipindahkan.

# Words when the question card is answered with no.
folders-outside-nothing-brought = Tidak ada yang disalin atau dipindahkan.

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [1] 1 item
       *[other] { $n } item
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = dan { $n } lagi

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
       *[other] { $n } item berubah setelah Anda mulai menyeretnya, jadi Folders membiarkannya di tempat semula: { $names }.
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
       *[other] { $n } item masih berada di tempat semula: { $each }.
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders tidak dapat memastikan apakah program lain menerima semua yang Anda seret, jadi aslinya dibiarkan di sini. Program lain memiliki salinannya.

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = Program lain tidak mengambil semua yang Anda seret, jadi Folders tidak memindahkan apa pun.

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = Program lain tidak mengambil semua yang Anda seret, jadi salinannya mungkin tidak lengkap.
