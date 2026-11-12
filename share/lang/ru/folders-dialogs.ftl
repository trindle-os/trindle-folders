
# machine translated (Russian), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = Закрыть

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = Домашняя папка

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = Открыть «{ $name }» в программе

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = Текущее приложение

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = На этом компьютере нет приложений, настроенных для открытия файлов такого типа.

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = Всегда использовать это приложение

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = Использовать для всех файлов типа { $kind }.

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = Сделать основным приложением

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = Показать только рекомендуемые приложения

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = Показать все приложения

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = Открыть

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = Сначала выберите приложение.

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Не удалось сделать «{ $name }» основным приложением.

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Не удалось запустить «{ $name }».

# Heading of the Properties window.
folders-dialogs-properties = Свойства

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = Разделы свойств

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = Общие

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = Подробности

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = Общий доступ



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = Имя

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = Где

# Label in Properties of the kind of file.
folders-dialogs-row-kind = Тип

# Label in Properties of where a link points.
folders-dialogs-row-link-to = Ссылка на

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = Изменено

# Label in Properties of who owns the item.
folders-dialogs-row-owner = Владелец

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = Права доступа

# Label in Properties of the size of the item.
folders-dialogs-row-size = Размер

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = Неизвестно

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, вы можете открывать и изменять

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, вы можете открывать, но не изменять

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, вы можете изменять, но не открывать

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, вы не можете открывать

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , часть не удалось прочитать
       *[other]
    }{ $adding ->
        [yes] , подсчёт продолжается
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = Вычисляем размер

# Label in Properties of the row about checksums.
folders-dialogs-checksums = Контрольные суммы

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = Вычислить контрольные суммы

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = Вычисляем контрольные суммы

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders не удалось прочитать файл.

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = Вставьте контрольную сумму для сравнения

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } совпадает

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = Не совпадает

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = Проверка


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [1] Отправить «{ $name }» в
        [few] Отправить { $n } элемента в
        [many] Отправить { $n } элементов в
       *[other] Отправить { $n } элемента в
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = Здесь создаётся копия. Оригиналы остаются на месте.

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = Другая панель, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = Столбцы в виде списка

# Note in the Columns window.
folders-dialogs-columns-note = Имя показывается всегда. Столбцы, которые не помещаются, скрываются, пока панель не станет шире.

# Heading over the columns that suit every file.
folders-dialogs-columns-files = Файлы

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = Музыка, фото, PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = Показывать: { $name }

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Не удалось сохранить столбцы.

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [1] Цвет для «{ $name }»
        [few] Цвет для { $n } папок
        [many] Цвет для { $n } папок
       *[other] Цвет для { $n } папок
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = Без цвета

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Не удалось сохранить цвет папки.


folders-dialogs-no-match = Нет приложений, подходящих под эти слова.


folders-dialogs-find-app = Найти приложение


folders-dialogs-find-place = Найти место


folders-dialogs-no-match-place = Нет мест, подходящих под эти слова.
