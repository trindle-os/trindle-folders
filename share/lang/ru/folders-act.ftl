
# machine translated (Russian), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Вырезать, копировать и вставлять файлы на сервере нельзя. Вместо этого используйте кнопки копирования и перемещения между панелями.

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = Внутри архива ничего нельзя менять. Сначала извлеките нужное.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = Эти элементы вырезали в другом приложении, поэтому Folders их скопировал.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = Уже копируется или перемещается.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = Это ещё копируется сюда.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10 000 элементов или больше

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] { $n } элемент
        [few] { $n } элемента
        [many] { $n } элементов
       *[other] { $n } элемента
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] Не удалось { $verb } { $n } элемент: { $names }.
        [few] Не удалось { $verb } { $n } элемента: { $names }.
        [many] Не удалось { $verb } { $n } элементов: { $names }.
       *[other] Не удалось { $verb } { $n } элемента: { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } и ещё { $more }

# Words on a job card while things are copied: what, and where to.
folders-act-copying = Копируем { $what } в { $place }

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = Скопировано: { $what } в { $place }.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = скопировать

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = Отменить копирование: { $what }

# Words on a job card while things are moved: what, and where to.
folders-act-moving = Перемещаем { $what } в { $place }

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = Перемещено: { $what } в { $place }.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = переместить

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = Отменить перемещение: { $what }

# Words on a job card while things are put in the Trash.
folders-act-trashing = Перемещаем в Корзину: { $what }

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = Перемещено в Корзину: { $what }.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = переместить в Корзину

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = Отменить перемещение в Корзину: { $what }

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = Удаляем { $what }

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = Удалено навсегда: { $what }.

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = удалить

# Words on a job card while things are put back from the Trash.
folders-act-restoring = Возвращаем { $what }

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = Возвращено: { $what }.

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = Отменить возврат: { $what }

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = вернуть

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Отменяем

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Отменено.

# Words on a job card while things are packed into an archive.
folders-act-compressing = Сжимаем { $what }

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = Создан файл { $file } в { $place }.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = сжать

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = Отменить сжатие: { $what }

# Words on a job card while an archive is unpacked.
folders-act-extracting = Распаковываем { $what }

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = Распаковано: { $what } в { $place }.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = распаковать

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = Отменить распаковку: { $what }

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = Стираем без возможности восстановления: { $what }

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = Стёрто без возможности восстановления: { $what }.

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = стереть

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = Извлекаем { $what }

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = Из архива извлечено: { $what } в { $place }.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = извлечь

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = Отменить извлечение: { $what }

# Words on a job card while pictures are turned.
folders-act-turning = Поворачиваем { $what }

# Words under the file lines when pictures have been turned.
folders-act-turned = Повёрнуто: { $what }.

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = повернуть

# Words on a job card while pictures are resized.
folders-act-resizing = Меняем размер: { $what }

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = Создана копия с другим размером: { $what }.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = изменить размер

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = Отменить изменение размера: { $what }

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = Новая папка

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = Новый документ

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Не удалось создать { $base }: { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = Введите имя.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = Имя не может содержать косую черту.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = Это имя занято системой. Выберите другое.

# Red words under a name being typed when it is too long.
folders-act-name-long = Слишком длинное имя.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Не удалось начать работу.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = Повторить: { $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = Отменить: { $what }

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = Работа остановлена до завершения.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped =
    { $done } { $n ->
        [one] Folders пропустил { $n } элемент.
        [few] Folders пропустил { $n } элемента.
        [many] Folders пропустил { $n } элементов.
       *[other] Folders пропустил { $n } элемента.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Сначала выберите что-нибудь.

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = Это уже в Корзине.

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = Файлы на сервере нельзя поместить в Корзину. Используйте «Удалить навсегда».

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Сначала выберите, что копировать или перемещать.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = Из архива можно извлекать только в папку.

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = Копии нельзя поместить в Корзину.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = В другой панели не открыта папка.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Сначала выберите, что копировать.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = Вырезано: { $what }. Вставьте в папку.

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = Скопировано: { $what }. Вставьте в папку.

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = Вставлять можно только в папку.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = В буфере обмена нет файлов.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = Ничего не вырезано и не скопировано.

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = Сетевые расположения нельзя удалять отсюда.

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = Корзина уже пуста.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Сначала выберите что-нибудь в Корзине.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = Создаём { $name }

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = Новую папку или документ можно создать только внутри папки.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = Отменить создание: { $name }

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders не удалось сохранить цвета папок у перемещённых элементов.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = Повторять нечего.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = Отменять нечего.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = Повторяем: { $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = Повторено: { $what }.

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = повторить

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = Отменяем: { $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = Отменено: { $what }.

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Домашняя папка

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Корзина

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = Перетаскивать можно только в папку или в Корзину.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] { $n } ссылка
        [few] { $n } ссылки
        [many] { $n } ссылок
       *[other] { $n } ссылки
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = Создано: { $what }.

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] Не удалось создать { $n } ссылку: { $names }.
        [few] Не удалось создать { $n } ссылки: { $names }.
        [many] Не удалось создать { $n } ссылок: { $names }.
       *[other] Не удалось создать { $n } ссылки: { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = Ждёт ответа вопрос. Ответьте на него или остановите работу, прежде чем закрыть Folders.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = Folders не закроется, пока выполняется работа. Дождитесь её окончания или остановите её, затем закройте Folders.
