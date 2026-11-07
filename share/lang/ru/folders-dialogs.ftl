

folders-dialogs-close = Закрыть

folders-dialogs-home = Домашняя папка

folders-dialogs-open-with-title = Открыть «{ $name }» в программе

folders-dialogs-opens-now = Открывает сейчас

folders-dialogs-no-apps = На этом компьютере нет приложений для файлов такого типа.

folders-dialogs-always-use = Всегда использовать

folders-dialogs-always-sub = Для всех файлов типа { $kind }.

folders-dialogs-always-tip = Сделать её основной программой для этого типа

folders-dialogs-show-suggested = Показать только рекомендуемые приложения

folders-dialogs-show-every = Показать все приложения

folders-dialogs-open = Открыть

folders-dialogs-choose-app = Выберите приложение, чтобы открыть файл.

folders-dialogs-usual-failed = Не удалось сделать «{ $name }» основным приложением.

folders-dialogs-start-failed = Не удалось запустить «{ $name }».

folders-dialogs-properties = Свойства

folders-dialogs-properties-pages = Разделы свойств

folders-dialogs-page-general = Общие

folders-dialogs-page-details = Подробности

folders-dialogs-page-sharing = Общий доступ

folders-dialogs-row-name = Имя

folders-dialogs-row-where = Где

folders-dialogs-row-kind = Тип

folders-dialogs-row-link-to = Ссылка на

folders-dialogs-row-changed = Изменено

folders-dialogs-row-owner = Владелец

folders-dialogs-row-rights = Права

folders-dialogs-row-size = Размер

folders-dialogs-kind-unknown = Неизвестно

folders-dialogs-kind-with-name = { $words } ({ $kind })

folders-dialogs-rights-open-change = { $letters }, вы можете открывать и изменять

folders-dialogs-rights-open-only = { $letters }, вы можете открывать, но не изменять

folders-dialogs-rights-change-only = { $letters }, вы можете изменять, но не открывать

folders-dialogs-rights-none = { $letters }, вы не можете открывать

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

folders-dialogs-size-adding = Подсчёт

folders-dialogs-checksums = Контрольные суммы

folders-dialogs-sums-work-out = Вычислить контрольные суммы

folders-dialogs-sums-working = Вычисляем

folders-dialogs-sums-failed = Не удалось прочитать файл

folders-dialogs-sums-paste = Вставьте контрольную сумму для проверки

folders-dialogs-sums-matches = { $kind } совпадает

folders-dialogs-sums-no-match = Не совпадает

folders-dialogs-sums-check = Проверка

folders-dialogs-send-title =
    { $n ->
        [1] Отправить «{ $name }» в
        [few] Отправить { $n } элемента в
        [many] Отправить { $n } элементов в
       *[other] Отправить { $n } элемента в
    }

folders-dialogs-send-note = Там создаётся копия; сами элементы остаются на месте.

folders-dialogs-send-other = Другая панель, { $name }

folders-dialogs-columns-title = Столбцы в виде списка

folders-dialogs-columns-note = Имя показывается всегда. Столбцы, которые не помещаются рядом с ним, скрываются, пока панель не станет шире.

folders-dialogs-columns-files = Файлы

folders-dialogs-columns-media = Музыка, фото, PDF

folders-dialogs-columns-show = Показывать: { $name }

folders-dialogs-columns-failed = Не удалось сохранить столбцы.

folders-dialogs-colour-title =
    { $n ->
        [1] Цвет для «{ $name }»
        [few] Цвет для { $n } папок
        [many] Цвет для { $n } папок
       *[other] Цвет для { $n } папок
    }

folders-dialogs-colour-none = Без своего цвета

folders-dialogs-colour-failed = Не удалось сохранить цвет папки.
