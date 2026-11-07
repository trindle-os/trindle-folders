# machine translated (Russian), not yet reviewed by a person.

folders-tools-pass-once = Один раз

folders-tools-pass-three = 3 раза

folders-tools-pass-seven = 7 раз

folders-tools-case-keep = Как есть

folders-tools-case-lower = Все строчные

folders-tools-case-upper = Все прописные

folders-tools-case-words = Каждое слово

folders-tools-case-first = Первое слово

folders-tools-ending-keep = Как есть

folders-tools-ending-lower = Все строчные

folders-tools-ending-upper = Все прописные

folders-tools-ending-new = Новое расширение

folders-tools-ending-none = Без расширения

folders-tools-archive-name = Архив

folders-tools-clash-empty = имя останется пустым

folders-tools-clash-bad = такое имя недопустимо

folders-tools-clash-long = это имя слишком длинное

folders-tools-clash-twice = два файла получили бы это имя

folders-tools-clash-taken = это имя уже занято другим файлом

folders-tools-clash-not-text = старое имя не является обычным текстом

folders-tools-caveat-flash = Это флеш-диск, а не вращающийся, поэтому старые копии могут сохраниться в тех частях диска, до которых Trindle не достаёт.

folders-tools-caveat-copy-on-write = Файловая система { $fs } записывает изменения в новое место, поэтому старые данные могут остаться на прежнем.

folders-tools-caveat-network = Это общий ресурс { $fs } на другом компьютере, который может хранить собственные копии.

folders-tools-row-only-left = Только слева

folders-tools-row-only-right = Только справа

folders-tools-row-same = Одинаково

folders-tools-row-different =
    Различается: { $why ->
        [kind] одно из них папка или ссылка
        [size] размеры разные
        [date] даты разные
        [content] содержимое разное
        [target] ссылки указывают на разное
        [inside] внутри есть различия
       *[unread] одну сторону не удалось прочитать
    }{ $newer ->
        [left] , слева новее
        [right] , справа новее
       *[none] { "" }
    }

folders-tools-place-here = Здесь

folders-tools-only-files-wiped = Стереть можно только файлы.

folders-tools-compare-need-folders = Для сравнения в обеих панелях должна быть открыта папка.

folders-tools-rename-undo =
    { $n ->
        [one] Отменить переименование { $n } элемента
        [few] Отменить переименование { $n } элементов
        [many] Отменить переименование { $n } элементов
       *[other] Отменить переименование { $n } элемента
    }

folders-tools-rename-done =
    { $n ->
        [one] Переименован { $n } элемент.
        [few] Переименовано { $n } элемента.
        [many] Переименовано { $n } элементов.
       *[other] Переименовано { $n } элемента.
    }

folders-tools-rename-stopped = Переименование прервалось на середине: { $error }.

folders-tools-name-empty = Имя не может быть пустым.

folders-tools-looked-at = Просмотрено: { $done }

folders-tools-read-of = Прочитано { $done } из { $total }

folders-tools-listing-files = Составляется список файлов

folders-tools-comparing = Сравнение

folders-tools-looking = Просмотр

folders-tools-compress-title =
    { $n ->
        [1] Сжать «{ $name }»
        [few] Сжать { $n } элемента
        [many] Сжать { $n } элементов
       *[other] Сжать { $n } элемента
    }

folders-tools-pack-name-tip = Имя архива

folders-tools-pack-kind-tip = Вид архива

folders-tools-pack-made-in = Будет создан в папке { $place }

folders-tools-compress = Сжать

folders-tools-extract-to = Извлечь «{ $name }» в

folders-tools-extract-new-folder = Там будет создана новая папка «{ $name }».

folders-tools-resize-title =
    { $n ->
        [1] Изменить размер «{ $name }»
        [few] Изменить размер { $n } изображений
        [many] Изменить размер { $n } изображений
       *[other] Изменить размер { $n } изображения
    }

folders-tools-resize-size-tip = Новый размер

folders-tools-resize-note = Доля от исходного размера или число пикселей по длинной стороне; изображение никогда не увеличивается. Рядом с каждым создаётся копия, в имени которой указан новый размер; сами изображения не меняются.

folders-tools-resize = Изменить размер

folders-tools-wipe-title =
    { $n ->
        [1] Стереть «{ $name }»?
        [few] Стереть { $n } файла?
        [many] Стереть { $n } файлов?
       *[other] Стереть { $n } файла?
    }

folders-tools-wipe-warning = Каждый файл перезаписывается перед удалением, поэтому восстановить его будет нельзя. Отменить это нельзя.

folders-tools-wipe-write-over = Перезаписать

folders-tools-wipe-passes-tip = Сколько раз перезаписывать каждый файл

folders-tools-wipe-button = Безопасно стереть

folders-tools-rename-number-tip = Пронумеровать

folders-tools-rename-at-start = В начале

folders-tools-rename-at-end = В конце

folders-tools-rename-where-tip = Где ставить номер

folders-tools-rename-first-number-tip = Первый номер

folders-tools-rename-ending-tip = Что делать с расширением

folders-tools-rename-new-ending-tip = Новое расширение, например txt

folders-tools-rename-cannot = Невозможно: { $why }

folders-tools-rename-was = было { $old }

folders-tools-rename-stays = остаётся как есть

folders-tools-rename-title =
    { $n ->
        [one] Переименовать { $n } элемент
        [few] Переименовать { $n } элемента
        [many] Переименовать { $n } элементов
       *[other] Переименовать { $n } элемента
    }

folders-tools-rename-replace = Заменить

folders-tools-rename-find-tip = Найти

folders-tools-rename-with-tip = Заменить на

folders-tools-rename-wildcard-tip = Считать * и ? подстановочными знаками

folders-tools-rename-wildcard = Подстановочные знаки * и ?

folders-tools-rename-match-case-tip = Точно учитывать заглавные буквы

folders-tools-rename-match-case = Учитывать регистр

folders-tools-rename-add = Добавить

folders-tools-rename-before-tip = Перед именем

folders-tools-rename-after-tip = После имени

folders-tools-rename-number = Номер

folders-tools-rename-letters = Буквы

folders-tools-rename-case-tip = Заглавные буквы в именах

folders-tools-rename-ending = Расширение

folders-tools-rename-summary =
    { $clash ->
        [yes] Некоторые имена использовать нельзя; см. отмеченные строки. В папке { $place }.
       *[no] Изменится имён: { $changes } из { $total }. В папке { $place }.
    }

folders-tools-rename-button = Переименовать

folders-tools-home = Домашняя папка

folders-tools-dupes-title =
    { $n ->
        [1] Дубликаты в «{ $folder }»
        [few] Дубликаты в { $n } папках
        [many] Дубликаты в { $n } папках
       *[other] Дубликаты в { $n } папках
    }

folders-tools-dupes-stopped = Поиск прервался: { $error }.

folders-tools-dupes-none = Дубликаты не найдены.

folders-tools-dupes-unread =
    { $n ->
        [one] Не удалось прочитать { $n } файл.
        [few] Не удалось прочитать { $n } файла.
        [many] Не удалось прочитать { $n } файлов.
       *[other] Не удалось прочитать { $n } файла.
    }

folders-tools-dupes-sets =
    { $n ->
        [one] { $n } набор копий; если убрать лишние в корзину, освободится { $size }. Лишние копии выделены; Ctrl и щелчок выделяет или снимает выделение.
        [few] { $n } набора копий; если убрать лишние в корзину, освободится { $size }. Лишние копии выделены; Ctrl и щелчок выделяет или снимает выделение.
        [many] { $n } наборов копий; если убрать лишние в корзину, освободится { $size }. Лишние копии выделены; Ctrl и щелчок выделяет или снимает выделение.
       *[other] { $n } набора копий; если убрать лишние в корзину, освободится { $size }. Лишние копии выделены; Ctrl и щелчок выделяет или снимает выделение.
    }

folders-tools-dupes-row = Набор { $set }, { $size }, в папке { $at }

folders-tools-dupes-unread-left-out =
    { $n ->
        [one] { $n } файл не удалось прочитать, он пропущен.
        [few] { $n } файла не удалось прочитать, они пропущены.
        [many] { $n } файлов не удалось прочитать, они пропущены.
       *[other] { $n } файла не удалось прочитать, они пропущены.
    }

folders-tools-dupes-keep-one = Выделены все копии набора; оставьте хотя бы одну.

folders-tools-dupes-pick-extras = Выделить лишние копии

folders-tools-dupes-trash = Переместить в корзину

folders-tools-diff-title = Сравнить эти два

folders-tools-diff-failed = Не удалось сравнить: { $error }.

folders-tools-diff-same-bytes = Эти два файла одинаковы байт в байт.

folders-tools-diff-same-as-shorter = Они совпадают на длину более короткого; один размером { $one }, другой { $other }.

folders-tools-diff-from-byte = Они различаются начиная с байта { $n }.

folders-tools-diff-lines =
    { $n ->
        [0] Эти два файла одинаковы.
        [one] Отличается { $n } строка.
        [few] Отличаются { $n } строки.
        [many] Отличаются { $n } строк.
       *[other] Отличается { $n } строки.
    }

folders-tools-diff-too-different = Они слишком различаются, чтобы сравнивать построчно, поэтому все строки между первым и последним отличием показаны как изменённые.

folders-tools-diff-cut = Сравниваются только первые 4 МБ каждого.

folders-tools-compare-title = Сравнить «{ $left }» с «{ $right }»

folders-tools-compare-bytes = Читать каждый байт (медленнее)

folders-tools-compare-bytes-tip = Сравнивать содержимое каждого файла

folders-tools-compare-show-same = Показывать одинаковое

folders-tools-compare-show-same-tip = Показывать и одинаковые имена

folders-tools-compare-stopped = Сравнение прервалось: { $error }.

folders-tools-compare-counts = Только слева: { $left }, только справа: { $right }, различаются: { $different }, одинаковы: { $same }.

folders-tools-compare-all-same = В обеих папках одно и то же.

folders-tools-compare-again = Сравнить снова

folders-tools-compare-copy-left = Копировать влево отсутствующие или более новые: { $n }

folders-tools-compare-copy-right = Копировать вправо отсутствующие или более новые: { $n }
