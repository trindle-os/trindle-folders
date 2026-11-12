
# machine translated (Russian), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done } из { $of } файлов, { $rate } в секунду, { $left }
       *[no] { $rate } в секунду, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } из { $of } файлов, считаем, сколько ещё…
       *[no] считаем, сколько ещё…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } байт

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } КБ

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } МБ

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } ГБ

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = осталось несколько секунд

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] осталось около { $n } секунды
        [few] осталось около { $n } секунд
        [many] осталось около { $n } секунд
       *[other] осталось около { $n } секунды
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = осталось около минуты

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] осталось около { $n } минуты
        [few] осталось около { $n } минут
        [many] осталось около { $n } минут
       *[other] осталось около { $n } минуты
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] осталось около { $n } часа
        [few] осталось около { $n } часов
        [many] осталось около { $n } часов
       *[other] осталось около { $n } часа
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] Выполняется ещё { $n } задача
        [few] Выполняется ещё { $n } задачи
        [many] Выполняется ещё { $n } задач
       *[other] Выполняется ещё { $n } задачи
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Перемещение сильно нагружает диск, поэтому другие программы могут работать медленнее, пока оно не закончится.
       *[copying] Копирование сильно нагружает диск, поэтому другие программы могут работать медленнее, пока оно не закончится.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = ждёт ответа

# End of a piece of work's line while it is paused.
folders-progress-paused = приостановлено

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Возобновить

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Приостановить

# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = Остановить

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = новее и больше

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = новее

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = больше

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = изменено { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] Папка «{ $name }» уже существует
        [file] Файл «{ $name }» уже существует
       *[unknown] «{ $name }» уже существует
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = Новый

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = Тот, который уже есть

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Пропустить

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = Оставить оба

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = Заменить

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = Заменить и переместить уже имеющуюся папку в Корзину

# Main button of the name taken question for a folder: join the two folders.
# SAFETY: needs a human review before it ships
folders-progress-merge = Объединить

# Line over the switch in the name taken question for folders.
# SAFETY: needs a human review before it ships
folders-progress-all-folders = Использовать этот ответ для всех папок, которые уже есть.

# Line over the switch in the name taken question for files.
# SAFETY: needs a human review before it ships
folders-progress-all-files = Использовать этот ответ для всех файлов, которые уже есть.

# Name and tip of the switch in the name taken question that uses the answer for the rest.
# SAFETY: needs a human review before it ships
folders-progress-same-for-rest = Сделать так же для остальных
