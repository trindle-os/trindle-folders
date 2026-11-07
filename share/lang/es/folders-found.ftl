

folders-found-need-inside-words = Escribe primero las palabras que buscar dentro de los archivos.

folders-found-need-something = Escribe qué buscar, o elige un tipo, un tamaño o una fecha.

folders-found-only-here = Solo se pueden buscar carpetas de este equipo.

folders-found-shown = Se muestran { $shown } de { $all }

folders-found-in-folder = En { $folder }

folders-found-in-root = En la carpeta buscada

folders-found-in-folder-line =
    En { $folder }. Línea { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(y { $more } más)
    }

folders-found-in-root-line =
    En la carpeta buscada. Línea { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(y { $more } más)
    }

folders-found-status =
    { $n ->
        [one] 1 encontrado
        [many] { $n } encontrados
       *[other] { $n } encontrados
    }{ $filtered ->
        [yes] { " " }de { $all }
       *[other]
    }{ $looking ->
        [yes] , aún buscando…
       *[other]
    }{ $full ->
        [yes] . Solo se muestran los primeros { $most }
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } no se pudieron leer
    }{ $skipped ->
        [0]
       *[other] . { $skipped } omitidos por ser demasiado grandes o no ser texto
    }

