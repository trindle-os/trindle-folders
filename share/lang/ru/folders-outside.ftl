# machine translated (Russian), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = скрытый или лежит в скрытой папке

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = внутри есть скрытые элементы

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = внутри слишком много элементов для проверки, поэтому Folders не будет переносить скрытые элементы

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = вне вашей домашней папки и ваших дисков

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = ссылка на скрытое или на то, что вне вашей домашней папки и ваших дисков

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = внутри есть ссылка на скрытое или на то, что вне вашей домашней папки и ваших дисков

# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [1] Переместить сюда { $name }?
        [few] Переместить сюда эти { $n } элемента?
        [many] Переместить сюда эти { $n } элементов?
       *[other] Переместить сюда эти { $n } элемента?
    }

# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [1] Принести сюда { $name }?
        [few] Принести сюда эти { $n } элемента?
        [many] Принести сюда эти { $n } элементов?
       *[other] Принести сюда эти { $n } элемента?
    }

# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [1] Скопировать сюда { $name }?
        [few] Скопировать сюда эти { $n } элемента?
        [many] Скопировать сюда эти { $n } элементов?
       *[other] Скопировать сюда эти { $n } элемента?
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be moved: why it is asked.
folders-outside-body-move =
    { $n ->
        [one] Другая программа перетащила это в Folders. Любая программа может перетащить любой файл, а это не один из ваших обычных файлов. Folders не будет перемещать его, пока вы не разрешите.
        [few] Другая программа перетащила их в Folders. Любая программа может перетащить любой файл, а некоторые из них не относятся к вашим обычным файлам. Folders не будет перемещать их, пока вы не разрешите.
        [many] Другая программа перетащила их в Folders. Любая программа может перетащить любой файл, а некоторые из них не относятся к вашим обычным файлам. Folders не будет перемещать их, пока вы не разрешите.
       *[other] Другая программа перетащила их в Folders. Любая программа может перетащить любой файл, а некоторые из них не относятся к вашим обычным файлам. Folders не будет перемещать их, пока вы не разрешите.
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
        [one] Другая программа перетащила это в Folders. Любая программа может перетащить любой файл, а это не один из ваших обычных файлов. Folders не будет переносить его сюда, пока вы не разрешите.
        [few] Другая программа перетащила их в Folders. Любая программа может перетащить любой файл, а некоторые из них не относятся к вашим обычным файлам. Folders не будет переносить их сюда, пока вы не разрешите.
        [many] Другая программа перетащила их в Folders. Любая программа может перетащить любой файл, а некоторые из них не относятся к вашим обычным файлам. Folders не будет переносить их сюда, пока вы не разрешите.
       *[other] Другая программа перетащила их в Folders. Любая программа может перетащить любой файл, а некоторые из них не относятся к вашим обычным файлам. Folders не будет переносить их сюда, пока вы не разрешите.
    }

# SAFETY: needs a human review before it ships
# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
        [one] Другая программа перетащила это в Folders. Любая программа может перетащить любой файл, а это не один из ваших обычных файлов. Folders не будет копировать его, пока вы не разрешите.
        [few] Другая программа перетащила их в Folders. Любая программа может перетащить любой файл, а некоторые из них не относятся к вашим обычным файлам. Folders не будет копировать их, пока вы не разрешите.
        [many] Другая программа перетащила их в Folders. Любая программа может перетащить любой файл, а некоторые из них не относятся к вашим обычным файлам. Folders не будет копировать их, пока вы не разрешите.
       *[other] Другая программа перетащила их в Folders. Любая программа может перетащить любой файл, а некоторые из них не относятся к вашим обычным файлам. Folders не будет копировать их, пока вы не разрешите.
    }

# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
        [one] Оставить
        [few] Оставить
        [many] Оставить
       *[other] Оставить
    }

# Button on the question card that moves what was dropped.
folders-outside-do-move =
    { $n ->
        [one] Переместить
        [few] Переместить
        [many] Переместить
       *[other] Переместить
    }

# Button on the question card that brings what was dropped.
folders-outside-do-bring =
    { $n ->
        [one] Принести
        [few] Принести
        [many] Принести
       *[other] Принести
    }

# Button on the question card that copies what was dropped.
folders-outside-do-copy =
    { $n ->
        [one] Скопировать
        [few] Скопировать
        [many] Скопировать
       *[other] Скопировать
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, из { $from }: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, из { $from }

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = И ещё: { $n }.

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = Куда: { $into }.

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Сначала ответьте на вопрос о том, что вы перетащили.

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders всё ещё проверяет последнее, что вы перетащили.

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Сюда можно перетаскивать только файлы и папки.

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = В боковой панели можно закреплять только папки.

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = Перетаскивать можно только на папку.

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } больше нет на месте, поэтому ничего не скопировано и не перемещено.

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = Нельзя поместить папку внутрь самой себя.

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = Они уже в этой папке.

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders не дождался ответа от диска, на котором лежит { $name }, поэтому ничего не скопировано и не перемещено.

# Words when the question card is answered with no.
folders-outside-nothing-brought = Ничего не скопировано и не перемещено.

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] { $n } элемент
        [few] { $n } элемента
        [many] { $n } элементов
       *[other] { $n } элемента
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = и ещё { $n }

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] { $n } элемент изменился после того, как вы начали его перетаскивать, поэтому Folders оставил его на месте: { $names }.
        [few] { $n } элемента изменились после того, как вы начали их перетаскивать, поэтому Folders оставил их на месте: { $names }.
        [many] { $n } элементов изменились после того, как вы начали их перетаскивать, поэтому Folders оставил их на месте: { $names }.
       *[other] { $n } элемента изменились после того, как вы начали их перетаскивать, поэтому Folders оставил их на месте: { $names }.
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] { $n } элемент остался на месте: { $each }.
        [few] { $n } элемента остались на месте: { $each }.
        [many] { $n } элементов остались на месте: { $each }.
       *[other] { $n } элемента остались на месте: { $each }.
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders не может понять, получила ли другая программа всё, что вы перетащили, поэтому оригиналы остались здесь. У другой программы есть копия.

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = Другая программа приняла не всё, что вы перетащили, поэтому Folders ничего не переместил.

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = Другая программа приняла не всё, что вы перетащили, поэтому её копия может быть неполной.
