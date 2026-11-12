
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red words when the search button is pressed for words inside files and the typing box is empty.
folders-found-need-inside-words = Digite primeiro as palavras a procurar dentro dos arquivos.

# Red words when the search button is pressed with nothing to look for and no kind, size or time chosen.
folders-found-need-something = Digite algo para procurar, ou escolha um tipo, um tamanho ou um período.

# Red words when the search button is pressed in a place that is not a folder on this computer.
folders-found-only-here = Você só pode pesquisar pastas deste computador.

# Status line when a filter or choices narrow a folder and nothing is picked: how many of all the entries are shown.
folders-found-shown = { $shown } de { $all } mostrados

# Status line when one search result is picked and it sits in a folder below the one searched; the folder follows.
folders-found-in-folder = Em { $folder }

# Status line when one search result is picked and it sits directly in the folder searched.
folders-found-in-root = Na pasta que você pesquisou

# Status line when one result found by words inside files is picked and it sits below the folder searched: the folder, the line number, the line, and how many more lines match (0 for none).
folders-found-in-folder-line =
        Em { $folder }. Linha { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(e mais { $more })
    }

# Status line when one result found by words inside files is picked and it sits directly in the folder searched: the line number, the line, and how many more lines match (0 for none).
folders-found-in-root-line =
    Na pasta que você pesquisou. Linha { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(e mais { $more })
    }

# Status line while a search runs and after it ends: how many were found, and when a filter is on, out of how many; while looking it says so; at the limit it says how many are shown; then how many files could not be read and how many were passed over. The variables filtered, looking, full are yes or no, most is the limit as written, unread and skipped are counts (0 for none).
folders-found-status =
    { $filtered ->
        [yes] { $n } de { $all } encontrados
       *[other] { $n ->
            [one] 1 encontrado
           *[other] { $n } encontrados
        }
    }{ $looking ->
        [yes] , ainda procurando…
       *[other]
    }{ $full ->
        [yes] . Mostrando só os primeiros { $most }
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } não puderam ser lidos
    }{ $skipped ->
        [0]
       *[other] . { $skipped } ignorados por serem grandes demais ou não serem texto
    }

