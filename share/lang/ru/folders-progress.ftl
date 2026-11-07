

folders-progress-pace-known =
    { $files ->
        [yes] { $done } из { $of } файлов, { $rate } в секунду, { $left }
       *[no] { $rate } в секунду, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] { $done } из { $of } файлов, считаем, сколько ещё…
       *[no] считаем, сколько ещё…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = { $n } байт

folders-progress-speed-kb = { $n } КБ

folders-progress-speed-mb = { $n } МБ

folders-progress-speed-gb = { $n } ГБ

folders-progress-left-few = осталось несколько секунд

folders-progress-left-seconds =
    { $n ->
        [one] осталось около { $n } секунды
        [few] осталось около { $n } секунд
        [many] осталось около { $n } секунд
       *[other] осталось около { $n } секунды
    }

folders-progress-left-minute = осталось около минуты

folders-progress-left-minutes =
    { $n ->
        [one] осталось около { $n } минуты
        [few] осталось около { $n } минут
        [many] осталось около { $n } минут
       *[other] осталось около { $n } минуты
    }

folders-progress-left-hours =
    { $n ->
        [one] осталось около { $n } часа
        [few] осталось около { $n } часов
        [many] осталось около { $n } часов
       *[other] осталось около { $n } часа
    }

folders-progress-more =
    { $n ->
        [one] Выполняется ещё { $n } задача
        [few] Выполняется ещё { $n } задачи
        [many] Выполняется ещё { $n } задач
       *[other] Выполняется ещё { $n } задачи
    }

folders-progress-heavy =
    { $verb ->
        [moving] Перемещение сильно нагружает диск, поэтому другие программы могут работать медленнее, пока оно не закончится.
       *[copying] Копирование сильно нагружает диск, поэтому другие программы могут работать медленнее, пока оно не закончится.
    }

folders-progress-waiting = ждёт ответа

folders-progress-paused = приостановлено

folders-progress-carry-on = Продолжить

folders-progress-pause = Приостановить

folders-progress-stop = Остановить

folders-progress-mark-newer-larger = новее, больше

folders-progress-mark-newer = новее

folders-progress-mark-larger = больше

folders-progress-changed = изменено { $when }

folders-progress-taken =
    { $kind ->
        [folder] Папка «{ $name }» уже существует
        [file] Файл «{ $name }» уже существует
       *[unknown] «{ $name }» уже существует
    }

folders-progress-coming = Тот, который переносится

folders-progress-there = Тот, который уже есть

folders-progress-skip = Пропустить

folders-progress-keep-both = Оставить оба

folders-progress-replace = Заменить

folders-progress-replace-folder-spoken = Заменить: папка, которая уже есть, будет перемещена в Корзину

folders-progress-merge = Объединить

folders-progress-all-folders = Один и тот же ответ для всех папок, которые уже есть.

folders-progress-all-files = Один и тот же ответ для всех файлов, которые уже есть.

folders-progress-same-for-rest = Сделать так же для остальных
