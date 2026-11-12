
# machine translated (Russian), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = В предыдущей копии ничего нельзя менять. Скопируйте нужное из неё, чтобы сохранить.

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] сегодня в { $time }
        [yesterday] вчера в { $time }
       *[other] { $date } в { $time }
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] Сегодня в { $time }
        [yesterday] Вчера в { $time }
       *[other] { $date } в { $time }
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = Это предыдущая копия от { $when }. Менять её нельзя, но можно скопировать из неё нужное.

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = Это предыдущая копия. Менять её нельзя, но можно скопировать из неё нужное.

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = Раньше: { $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = Отменить возврат { $name }

# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = Возвращено. Текущий файл лежит рядом под другим именем.

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = Возвращено на прежнее место.

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = Совпадает с нынешним.

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = Отличается от нынешнего.

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = Не удалось сравнить: { $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = Предыдущие версии

# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = Удалённое отсюда

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = Предыдущие версии { $name }

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = Нет предыдущих версий, которые отличаются от нынешней.

# Note at the foot of the earlier versions card about what putting one back does.
folders-earlier-versions-foot = При возврате нынешняя версия сохраняется рядом. Это можно отменить.

# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = Удалённое из { $name }

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = Ничего из предыдущих копий не удалено из этой папки.

# Note at the foot of the deleted things card about what putting one back does.
folders-earlier-deleted-foot = При возврате элемент снова окажется в этой папке. Это можно отменить.

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = Папка в том виде, как была тогда

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }, из копии от { $date }

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = Папка, последний раз видели { $when }

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }, последний раз видели { $when }

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = Выполняется…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = Открыть папку за тот день в новой вкладке

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = Открыть

# Tip and name of the round button on a row of the earlier versions card that puts it back.
folders-earlier-put-back-tip = Вернуть

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = Сравнить с нынешним

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = Это не прежняя версия.

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = Прежнюю версию можно вернуть только в домашнюю папку.

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = Сюда файл вернуть нельзя.

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = Не удалось заново создать папку, в которой он был: { $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = Не удалось скопировать прежнюю версию: { $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = Не удалось убрать то, что лежит там сейчас: { $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = Не удалось вернуть прежнюю версию на место: { $why }. Нынешний файл теперь называется { $name }.

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = Не удалось поставить прежнюю версию на место: { $why }
