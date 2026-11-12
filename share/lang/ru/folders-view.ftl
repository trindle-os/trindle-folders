
# machine translated (Russian), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = Очистить Корзину?

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
        [one] { $n } элемент в ней будет удалён навсегда. Это нельзя отменить.
        [few] Все { $n } элемента в ней будут удалены навсегда. Это нельзя отменить.
        [many] Все { $n } элементов в ней будут удалены навсегда. Это нельзя отменить.
       *[other] Все { $n } элемента в ней будут удалены навсегда. Это нельзя отменить.
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = Очистить Корзину

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
        [one] Удалить навсегда { $n } элемент?
        [few] Удалить навсегда { $n } элемента?
        [many] Удалить навсегда { $n } элементов?
       *[other] Удалить навсегда { $n } элемента?
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = Это нельзя отменить.

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = Удалить навсегда

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = Оставить

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = Куда их поместить?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Оставить на месте




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = Значки

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = Список

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = Компактный список

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = Выбрать столбцы

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = Имя

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = Сортировать по

# Name read out for the toolbar's Group by strip.
folders-view-group-by = Группировать по





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Скрыть панель просмотра (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Показать панель просмотра (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Панель просмотра

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Вкладки

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Вкладки левой панели

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Вкладки правой панели

# Tip of the button that opens a new tab.
folders-view-tab-new = Новая вкладка (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = Закрыть вкладку (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = Домашняя папка

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Пока ничего не закреплено. Щёлкните папку правой кнопкой, чтобы закрепить её.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = Закреплённые

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Места

# Small heading in the side bar over the drives plugged in.
folders-view-drives = Накопители

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Общие папки

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = Закрыть «{ $name }»



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Подключиться к серверу

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = Связать две панели. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = Связать две панели

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] Копировать в левую панель
       *[other] Копировать в правую панель
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] Переместить в левую панель
       *[other] Переместить в правую панель
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Показать эту папку в левой панели
       *[other] Показать эту папку в правой панели
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Поменять панели местами

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Выделить то, что отличается от левой панели
       *[other] Выделить то, что отличается от правой панели
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] Сравнить с левой панелью
       *[other] Сравнить с правой панелью
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Введите папку, файл или адрес сервера

# Name read out for the path above a pane's list.
folders-view-where = Расположение

# Tip of the pane's Back button.
folders-view-back = Назад (Alt+Left)

# Tip of the pane's Forward button.
folders-view-forward = Вперёд (Alt+Right)

# Tip of the pane's button that goes up one folder.
folders-view-up = На одну папку вверх (Alt+Up)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Фильтр, или нажмите Enter, чтобы искать в этой папке и её подпапках

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Искать по именам, включая подпапки

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = Искать внутри файлов, включая подпапки


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Тип

# Name read out for the list that picks which size to show.
folders-view-find-size = Размер

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Изменено

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = Новое имя для «{ $was }»

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = свободно { $size }

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = Выбрано: { $count }, считаем размер…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = Выбрано: { $count }, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Показать файлы, о которых идёт речь

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Повторить

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
# SAFETY: needs a human review before it ships
folders-view-cancel = Отмена

# Line on a pane whose filter matches nothing.
folders-view-none-matches = Ничего не подходит под фильтр

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Показать всё

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = Вы недавно ничего не открывали

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Перейти в домашнюю папку

# Line on the Trash page when it is empty.
folders-view-none-trash = Корзина пуста

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = Нет накопителей для показа

# Line on a search page that found nothing.
folders-view-none-found = Ничего не найдено

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Очистить поиск

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = Этот архив пуст

# Button on an empty archive page: goes up one folder.
folders-view-go-up = На одну папку вверх

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = Эта папка пуста

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = Новая папка

# Button on an empty folder's page that makes a new document.
folders-view-new-document = Новый документ

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] { $n } элемент
        [few] { $n } элемента
        [many] { $n } элементов
       *[other] { $n } элемента
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Не подключено

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = Подключить { $name } снова

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } не подключено.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Подключить все снова

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] Одна сетевая папка не подключена.
        [few] { $n } сетевые папки не подключены.
        [many] { $n } сетевых папок не подключены.
       *[other] { $n } сетевой папки не подключены.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = Забыть { $name }

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = Две панели

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = Показать две панели рядом (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Показать одну панель (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = Вид

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = Создать

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = Ещё

# Row of the More menu that undoes the last change.
folders-view-more-undo = Отменить

# Row of the More menu that does again what was undone.
folders-view-more-redo = Повторить

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Сортировать в обратном порядке

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Показать скрытые файлы (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Скрыть скрытые файлы (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Скрытые файлы

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Компьютеры поблизости

# Tip of the Search button in the top row while the path shows.
folders-view-search = Поиск (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = Закрыть поиск (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = На этой странице поиск недоступен

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Поиск

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = Имя на боковой панели для { $was }
