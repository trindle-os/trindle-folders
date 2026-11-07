# Words of the two-pane strip in Folders (the buttons between the panes).

folders-pair-differing =
    { $n ->
        [0] Nothing here differs from the other pane by name, size or date. Folders are matched by name only.
        [one] 1 thing here is not in the other pane, or differ from it by size or date. Folders are matched by name only.
       *[other] { $n } things here are not in the other pane, or differ from it by size or date. Folders are matched by name only.
    }
folders-pair-needs-both = This needs both panes: press F3 for the dual view.
folders-pair-no-search = What a search found cannot be shown in the other pane.
