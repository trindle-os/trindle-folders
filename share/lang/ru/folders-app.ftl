# machine translated (Russian), not yet reviewed by a person.

# Note over the panes when the panes are linked and the other pane has no folder that matches the one opened here.
folders-app-link-no-folder = В другой панели нет папки с таким именем

# Note over the panes when the panes are linked and the other pane has no folder with the name of the one opened here; name is that folder's name.
folders-app-link-no-folder-named = В другой панели нет папки «{ $name }»

# Said at the foot of the window after a folder is opened in a new tab behind the current one; name is the folder's name.
folders-app-opened-tab = Папка «{ $name }» открыта в новой вкладке.

# Red words at the foot of the window when a new Folders window could not be started.
folders-app-no-window = Не удалось открыть новое окно.

# Red words at the foot of the window when the choice of one pane or two could not be saved.
folders-app-panes-not-kept = Не удалось сохранить число панелей на следующий раз.

# Red words at the foot of the window when the path typed in the path bar leads nowhere; text is what was typed.
folders-app-nothing-at = По адресу { $text } ничего нет.

# Red words at the foot of the window when the program chosen to open a file could not be started; name is the program's name.
folders-app-not-started = Не удалось запустить { $name }.

# Red words at the foot of the window when the list of pinned folders could not be saved.
folders-app-pins-not-saved = Не удалось сохранить закреплённые папки.

# Said at the foot of the window when a folder is dropped on the sidebar and is already pinned.
folders-app-pinned-already = Она уже закреплена на боковой панели.

# Said at the foot of the window after one folder is pinned to the sidebar; name is the folder's name.
folders-app-pinned-one = Папка «{ $name }» закреплена на боковой панели.

# Said at the foot of the window after several folders are pinned to the sidebar; n is how many.
folders-app-pinned-many =
    { $n ->
        [one] Закреплена { $n } папка на боковой панели.
        [few] Закреплено { $n } папки на боковой панели.
        [many] Закреплено { $n } папок на боковой панели.
       *[other] Закреплено { $n } папки на боковой панели.
    }

# What several dragged items are called in the words at the foot of the window; n is how many.
folders-app-items =
    { $n ->
        [one] { $n } элемент
        [few] { $n } элемента
        [many] { $n } элементов
       *[other] { $n } элемента
    }

# Said at the foot of the window while items are being dragged; what is the item's name or how many there are.
folders-app-dragging = Перемещение: { $what }. Удерживайте Ctrl, чтобы копировать.

# Said at the foot of the window while items are held after a right-button drag; what is the item's name or how many there are.
folders-app-holding = Перетаскивание: { $what }. Отпустите, чтобы выбрать действие.

# Said at the foot of the window when the last tab of a pane is closed and the other pane still has tabs.
folders-app-only-tab = Это единственная вкладка в этой панели, поэтому она остаётся открытой, пока в другой панели есть вкладки.

# Red words at the foot of the window when the choice to link the panes could not be saved.
folders-app-link-not-kept = Не удалось сохранить, связаны ли панели.

# Red words at the foot of the window when the view chosen for a folder could not be saved.
folders-app-view-not-kept = Не удалось сохранить вид этой папки.

# Red words at the foot of the window when the choice of grouping could not be saved.
folders-app-grouping-not-kept = Не удалось сохранить группировку на следующий раз.

# Said in a window run as administrator when a file, an archive or a preview is asked for.
folders-app-root-folders-only = В окне администратора можно работать только с папками. Открывать файлы, заглядывать внутрь них и просматривать их с правами администратора нельзя.

# Red words when the list of what opens what cannot be read, so the choice is not written; why it cannot be read.
folders-app-opener-unreadable = Folders не удалось прочитать, какое приложение открывает каждый тип файлов: { $why }. Ничего не изменено.
