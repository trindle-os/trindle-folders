# machine translated (Swahili), not yet reviewed by a person.

folders-found-need-inside-words = Andika kwanza maneno ya kutafuta ndani ya faili.

folders-found-need-something = Andika cha kutafuta, au chagua aina, ukubwa au muda.

folders-found-only-here = Ni folda za kompyuta hii pekee zinazoweza kutafutwa.

folders-found-shown = { $shown } kati ya { $all } vinaonyeshwa

folders-found-in-folder = Ndani ya { $folder }

folders-found-in-root = Ndani ya folda iliyotafutwa

folders-found-in-folder-line =
    Ndani ya { $folder }. Mstari wa { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(na { $more } zaidi)
    }

folders-found-in-root-line =
    Ndani ya folda iliyotafutwa. Mstari wa { $n }: { $line }{ $more ->
        [0]
       *[other] { " " }(na { $more } zaidi)
    }

folders-found-status =
    { $n ->
        [one] 1 imepatikana
       *[other] { $n } zimepatikana
    }{ $filtered ->
        [yes] { " " }kati ya { $all }
       *[other]
    }{ $looking ->
        [yes] , bado inatafuta…
       *[other]
    }{ $full ->
        [yes] . Ni { $most } za kwanza tu zinazoonyeshwa
       *[other]
    }{ $unread ->
        [0]
       *[other] . { $unread } hazikuweza kusomwa
    }{ $skipped ->
        [0]
       *[other] . { $skipped } ziliruka kwa kuwa kubwa mno au si maandishi
    }
