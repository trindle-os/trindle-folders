
# machine translated (German), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = Papierkorb leeren?

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
        [one] Das eine Objekt darin wird endgültig gelöscht. Das lässt sich nicht rückgängig machen.
       *[other] Alle { $n } Objekte darin werden endgültig gelöscht. Das lässt sich nicht rückgängig machen.
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = Papierkorb leeren

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
        [one] Dieses Objekt endgültig löschen?
       *[other] Diese { $n } Objekte endgültig löschen?
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = Das lässt sich nicht rückgängig machen.

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = Endgültig löschen

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = Behalten

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = Wohin sollen sie?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Dort lassen, wo sie sind




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = Symbole

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = Liste

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = Kompakt

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = Spalten auswählen

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = Name

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = Sortieren nach

# Name read out for the toolbar's Group by strip.
folders-view-group-by = Gruppieren nach





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Vorschaubereich ausblenden (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Vorschaubereich anzeigen (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Vorschaubereich

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Tabs

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Tabs im linken Bereich

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Tabs im rechten Bereich

# Tip of the button that opens a new tab.
folders-view-tab-new = Neuer Tab (Strg+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = Tab schließen (Strg+W)






# Side bar row that opens the Home folder.
folders-view-home = Persönlicher Ordner

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Noch nichts angeheftet. Klicke mit der rechten Maustaste auf einen Ordner, um ihn anzuheften.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = Angeheftet

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Orte

# Small heading in the side bar over the drives plugged in.
folders-view-drives = Laufwerke

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Freigaben

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = { $name } schließen



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Mit einem Server verbinden

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = Die beiden Bereiche verknüpfen. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = Die beiden Bereiche verknüpfen

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] In den linken Bereich kopieren
       *[other] In den rechten Bereich kopieren
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] In den linken Bereich verschieben
       *[other] In den rechten Bereich verschieben
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Diesen Ordner im linken Bereich zeigen
       *[other] Diesen Ordner im rechten Bereich zeigen
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Die Bereiche tauschen

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Auswählen, was sich vom linken Bereich unterscheidet
       *[other] Auswählen, was sich vom rechten Bereich unterscheidet
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] Mit dem linken Bereich vergleichen
       *[other] Mit dem rechten Bereich vergleichen
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Ordner, Datei oder Serveradresse eingeben

# Name read out for the path above a pane's list.
folders-view-where = Ort

# Tip of the pane's Back button.
folders-view-back = Zurück (Alt+Links)

# Tip of the pane's Forward button.
folders-view-forward = Vorwärts (Alt+Rechts)

# Tip of the pane's button that goes up one folder.
folders-view-up = Einen Ordner nach oben (Alt+Hoch)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Filtern, oder Eingabetaste drücken, um in diesem Ordner und seinen Unterordnern zu suchen

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Namen durchsuchen, einschließlich Unterordner

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = In Dateien suchen, einschließlich Unterordner


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Typ

# Name read out for the list that picks which size to show.
folders-view-find-size = Größe

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Zuletzt geändert

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = Neuer Name für { $was }

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } frei

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } ausgewählt, Größe wird berechnet…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } ausgewählt, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Die betroffenen Dateien anzeigen

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Erneut versuchen

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
# SAFETY: needs a human review before it ships
folders-view-cancel = Abbrechen

# Line on a pane whose filter matches nothing.
folders-view-none-matches = Nichts hier passt zum Filter

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Alles anzeigen

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = Du hast zuletzt nichts geöffnet

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Zum Persönlichen Ordner

# Line on the Trash page when it is empty.
folders-view-none-trash = Der Papierkorb ist leer

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = Keine Laufwerke vorhanden

# Line on a search page that found nothing.
folders-view-none-found = Nichts gefunden

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Suche löschen

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = Dieses Archiv ist leer

# Button on an empty archive page: goes up one folder.
folders-view-go-up = Einen Ordner nach oben

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = Dieser Ordner ist leer

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = Neuer Ordner

# Button on an empty folder's page that makes a new document.
folders-view-new-document = Neues Dokument

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] 1 Objekt
       *[other] { $n } Objekte
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Nicht verbunden

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = { $name } erneut verbinden

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } ist nicht verbunden.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Alle erneut verbinden

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] Eine Freigabe ist nicht verbunden.
       *[other] { $n } Freigaben sind nicht verbunden.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
# SAFETY: needs a human review before it ships
folders-view-forget-share = { $name } vergessen

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = Zwei Bereiche

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = Zwei Bereiche nebeneinander zeigen (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Einen Bereich zeigen (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = Ansicht

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = Neu

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = Weitere Optionen

# Row of the More menu that undoes the last change.
folders-view-more-undo = Rückgängig

# Row of the More menu that does again what was undone.
folders-view-more-redo = Wiederholen

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Andersherum sortieren

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Versteckte Dateien zeigen (Strg+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Versteckte Dateien wieder verbergen (Strg+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Versteckte Dateien

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Computer in der Nähe

# Tip of the Search button in the top row while the path shows.
folders-view-search = Suche (Strg+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = Suche schließen (Esc)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = Auf dieser Seite kannst du nicht suchen

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Suche

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = Name in der Seitenleiste für { $was }
