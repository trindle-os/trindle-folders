
# machine translated (Russian), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = Сначала введите слова, которые нужно найти внутри файлов.

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = Введите, что искать, или выберите вид, размер или время.

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = Искать можно только в папках на этом компьютере.

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = Показано: { $shown } из { $all }

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = В папке { $folder }

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = В папке, где вы искали

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
    В папке { $folder }. Строка { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(и ещё { $more })
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    В папке, где вы искали. Строка { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(и ещё { $more })
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] Найдено { $n } из { $all }
       *[other] { $n ->
            [one] Найден { $n }
            [few] Найдено { $n }
            [many] Найдено { $n }
           *[other] Найдено { $n }
        }
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
