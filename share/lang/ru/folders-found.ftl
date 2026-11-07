

folders-found-need-inside-words = Сначала введите слова, которые нужно найти внутри файлов.

folders-found-need-something = Введите, что искать, или выберите вид, размер или время.

folders-found-only-here = Искать можно только в папках на этом компьютере.

folders-found-shown = Показано: { $shown } из { $all }

folders-found-in-folder = В папке { $folder }

folders-found-in-root = В папке, где идёт поиск

folders-found-in-folder-line =
    В папке { $folder }. Строка { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(и ещё { $more })
    }

folders-found-in-root-line =
    В папке, где идёт поиск. Строка { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(и ещё { $more })
    }

folders-found-status =
    { $n ->
        [one] Найден { $n }
        [few] Найдено { $n }
        [many] Найдено { $n }
       *[other] Найдено { $n }
    }{ $filtered ->
        [yes] { " " }из { $all }
       *[other]
    }{ $looking ->
        [yes] , поиск продолжается…
       *[other]
    }{ $full ->
        [yes] . Показаны только первые { $most }
       *[other]
    }{ $unread ->
        [0]
       *[other] . Не удалось прочитать: { $unread }
    }{ $skipped ->
        [0]
       *[other] . Пропущено как слишком большие или не текстовые: { $skipped }
    }
