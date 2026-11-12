# Words of the two-pane strip in Folders (the buttons between the panes).

## Two panes (src/files/pair.rs)

# Note after "pick what differs" in the dual view; says how many things here are not in the other pane or differ by size or date.
folders-pair-differing =
    { $n ->
        [0] Nothing here differs from the other pane by name, size or date. Subfolders are only compared by name.
        [one] 1 thing here is missing from the other pane or differs in size or date. Subfolders are only compared by name.
       *[other] { $n } things here are missing from the other pane or differ in size or date. Subfolders are only compared by name.
    }
# Red note when a button of the strip between the panes is used and only one pane shows.
folders-pair-needs-both = This needs both panes. Press F3 to show them.
# Red note when the person asks for the same folder in the other pane while the pane shows search results.
folders-pair-no-search = Search results can't be shown in the other pane.
