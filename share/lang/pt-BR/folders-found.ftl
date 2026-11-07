

folders-found-need-inside-words = Digite primeiro as palavras a procurar dentro dos arquivos.

folders-found-need-something = Digite o que procurar, ou escolha um tipo, um tamanho ou um período.

folders-found-only-here = Só as pastas deste computador podem ser pesquisadas.

folders-found-shown = { $shown } de { $all } mostrados

folders-found-in-folder = Em { $folder }

folders-found-in-root = Na pasta pesquisada

folders-found-in-folder-line =
        Em { $folder }. Linha { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(e mais { $more })
    }

folders-found-in-root-line =
        Na pasta pesquisada. Linha { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(e mais { $more })
    }

folders-found-status =
    { $n ->
        [one] 1 encontrado
       *[other] { $n } encontrados
        [many] { $n } encontrados
    }{ $filtered ->
        [yes] { " " }de { $all }
       *[other]
    }{ $looking ->
        [yes] , ainda procurando…
       *[other]
    }{ $full ->
        [yes] . Só os primeiros { $most } são mostrados
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } não puderam ser lidos
    }{ $skipped ->
        [0]
       *[other] . { $skipped } ignorados por serem grandes demais ou não serem texto
    }

