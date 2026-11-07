

folders-view-empty-trash-title = Очистить Корзину?

folders-view-empty-trash-body =
    { $n ->
        [one] { $n } элемент в ней будет удалён навсегда. Это нельзя отменить.
        [few] Все { $n } элемента в ней будут удалены навсегда. Это нельзя отменить.
        [many] Все { $n } элементов в ней будут удалены навсегда. Это нельзя отменить.
       *[other] Все { $n } элемента в ней будут удалены навсегда. Это нельзя отменить.
    }

folders-view-empty-trash = Очистить Корзину

folders-view-delete-title =
    { $n ->
        [one] Удалить навсегда { $n } элемент?
        [few] Удалить навсегда { $n } элемента?
        [many] Удалить навсегда { $n } элементов?
       *[other] Удалить навсегда { $n } элемента?
    }

folders-view-delete-body = Это нельзя отменить.

folders-view-delete-permanently = Удалить навсегда

folders-view-keep-them = Оставить

folders-view-drop-title = Куда их поместить?

folders-view-drop-leave = Оставить на месте

folders-view-single = Одна

folders-view-dual = Две

folders-view-panes = Панели: одна или две (F3)

folders-view-icons = Значки

folders-view-list = Список

folders-view-compact = Компактный список

folders-view-columns = Выбрать столбцы

folders-view-sort-name = Имя

folders-view-sort-by = Сортировать по

folders-view-group-by = Группировать по

folders-view-sorted-up = По возрастанию: сначала меньшие или старые

folders-view-sorted-down = По убыванию: сначала большие или новые

folders-view-hide-hidden = Скрыть скрытые файлы

folders-view-show-hidden = Показать скрытые файлы

folders-view-panel-hide = Скрыть панель просмотра (Alt+P)

folders-view-panel-show = Показать панель просмотра (Alt+P)

folders-view-panel-spoken = Панель просмотра

folders-view-tabs = Вкладки

folders-view-tabs-left = Вкладки левой панели

folders-view-tabs-right = Вкладки правой панели

folders-view-tab-new = Новая вкладка (Ctrl+T)

folders-view-tab-close = Закрыть вкладку (Ctrl+W или щелчок по ней средней кнопкой)

folders-view-undo = { $label } (Ctrl+Z)

folders-view-undo-none = Нечего отменять

folders-view-redo = { $label } (Ctrl+Y)

folders-view-redo-none = Нечего повторять

folders-view-new-folder-tip = Новая папка (Ctrl+Shift+N)

folders-view-home = Домашняя папка

folders-view-pinned-none = Пока ничего не закреплено. Щёлкните папку правой кнопкой, чтобы закрепить её.

folders-view-pinned = Закреплённые

folders-view-places = Места

folders-view-drives = Накопители

folders-view-shares = Общие папки

folders-view-share-close = Закрыть «{ $name }»

folders-view-connect = Подключиться к серверу

folders-view-link-noted = Связать две панели. { $note }

folders-view-link = Связать две панели

folders-view-copy-to =
    { $way ->
        [left] Копировать в левую панель
       *[other] Копировать в правую панель
    }

folders-view-move-to =
    { $way ->
        [left] Переместить в левую панель
       *[other] Переместить в правую панель
    }

folders-view-same-in =
    { $way ->
        [left] Показать эту папку в левой панели
       *[other] Показать эту папку в правой панели
    }

folders-view-swap = Поменять панели местами

folders-view-pick-differing =
    { $way ->
        [left] Выбрать то, что отличается от левой панели
       *[other] Выбрать то, что отличается от правой панели
    }

folders-view-compare =
    { $way ->
        [left] Сравнить с левой панелью
       *[other] Сравнить с правой панелью
    }

folders-view-locate-hint = Введите папку, файл или адрес сервера

folders-view-where = Где находится эта панель

folders-view-back = Назад (Alt+Left)

folders-view-forward = Вперёд (Alt+Right)

folders-view-up = На одну папку вверх (Alt+Up)

folders-view-locate = Ввести путь (Ctrl+L или Alt+D)

folders-view-filter = Фильтр или поиск здесь (Ctrl+F)

folders-view-filter-hint = Фильтр, или нажмите Enter, чтобы искать в этой папке и глубже

folders-view-find-name = Искать по имени здесь и глубже

folders-view-find-inside = Искать слова внутри файлов здесь и глубже

folders-view-find-close = Закрыть и показать всё

folders-view-find-kind = Какие элементы показывать

folders-view-find-size = Какой размер показывать

folders-view-find-changed = Когда изменено в последний раз

folders-view-rename-hint = Новое имя для «{ $was }»

folders-view-status-free = { $items }, свободно { $size }

folders-view-status-selected-working = Выбрано: { $count }, считаем размер…

folders-view-status-selected = Выбрано: { $count }, { $size }

folders-view-status-then = { $first }. { $then }

folders-view-listed = Какие файлы и почему

folders-view-try-again = Повторить

folders-view-cancel = Отмена

folders-view-none-matches = Ничего не подходит под фильтр

folders-view-show-everything = Показать всё

folders-view-none-recent = Недавно ничего не открывалось

folders-view-go-home = Перейти в домашнюю папку

folders-view-none-trash = Корзина пуста

folders-view-none-drives = Нет накопителей для показа

folders-view-none-found = Ничего не найдено

folders-view-clear-search = Очистить поиск

folders-view-none-archive = Этот архив пуст

folders-view-go-up = На одну папку вверх

folders-view-none-folder = Эта папка пуста

folders-view-new-folder = Новая папка

folders-view-new-document = Новый документ

folders-view-items =
    { $n ->
        [one] { $n } элемент
        [few] { $n } элемента
        [many] { $n } элементов
       *[other] { $n } элемента
    }

folders-view-not-connected = Не подключено

folders-view-reconnect = Подключить { $name } снова

folders-view-share-down = { $name } не подключено.

folders-view-reconnect-all = Подключить все снова

folders-view-shares-down =
    { $n ->
        [one] Одна сетевая папка не подключена.
        [few] { $n } сетевые папки не подключены.
        [many] { $n } сетевых папок не подключены.
       *[other] { $n } сетевой папки не подключены.
    }

folders-view-forget-share = Забыть { $name }

