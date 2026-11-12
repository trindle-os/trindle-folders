# machine translated (Swahili), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = Kumwaga Tupio?

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
        [one] Kitu kimoja kilichomo kitafutwa kabisa. Huwezi kutendua hili.
       *[other] Vitu vyote { $n } vilivyomo vitafutwa kabisa. Huwezi kutendua hili.
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = Mwaga Tupio

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
        [one] Kufuta kitu hiki kabisa?
       *[other] Kufuta vitu hivi { $n } kabisa?
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = Huwezi kutendua hili.

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = Futa kabisa

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = Viweke

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = Vipelekwe wapi?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Viache vilipo




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = Aikoni

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = Orodha

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = Fupi

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = Chagua safu

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = Jina

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = Panga kwa

# Name read out for the toolbar's Group by strip.
folders-view-group-by = Kusanya kwa





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Ficha paneli ya hakiki (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Onyesha paneli ya hakiki (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Paneli ya hakiki

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Vichupo

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Vichupo vya kidirisha cha kushoto

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Vichupo vya kidirisha cha kulia

# Tip of the button that opens a new tab.
folders-view-tab-new = Kichupo kipya (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = Funga kichupo (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = Nyumbani

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Hakuna kilichobandikwa bado. Bofya kulia folda ili kuibandika.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = Zilizobandikwa

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Mahali

# Small heading in the side bar over the drives plugged in.
folders-view-drives = Diski

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Zilizoshirikiwa

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = Funga { $name }



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Unganisha kwenye seva

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = Unganisha vidirisha viwili. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = Unganisha vidirisha viwili

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] Nakili kwenye kidirisha cha kushoto
       *[other] Nakili kwenye kidirisha cha kulia
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] Hamisha kwenye kidirisha cha kushoto
       *[other] Hamisha kwenye kidirisha cha kulia
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Onyesha folda hii kwenye kidirisha cha kushoto
       *[other] Onyesha folda hii kwenye kidirisha cha kulia
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Badilishana vidirisha

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Chagua kinachotofautiana na kidirisha cha kushoto
       *[other] Chagua kinachotofautiana na kidirisha cha kulia
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] Linganisha na kidirisha cha kushoto
       *[other] Linganisha na kidirisha cha kulia
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Andika folda, faili au anwani ya seva

# Name read out for the path above a pane's list.
folders-view-where = Mahali

# Tip of the pane's Back button.
folders-view-back = Nyuma (Alt+Left)

# Tip of the pane's Forward button.
folders-view-forward = Mbele (Alt+Right)

# Tip of the pane's button that goes up one folder.
folders-view-up = Juu folda moja (Alt+Up)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Chuja, au bonyeza Enter kutafuta kwenye folda hii na folda zake ndogo

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Tafuta majina, pamoja na folda ndogo

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = Tafuta ndani ya faili, pamoja na folda ndogo


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Aina

# Name read out for the list that picks which size to show.
folders-view-find-size = Ukubwa

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Ilipobadilishwa mara ya mwisho

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = Jina jipya la { $was }

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = nafasi { $size }

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } vimechaguliwa, inakokotoa ukubwa…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } vimechaguliwa, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Onyesha faili zinazohusika

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Jaribu tena

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
# SAFETY: needs a human review before it ships
folders-view-cancel = Ghairi

# Line on a pane whose filter matches nothing.
folders-view-none-matches = Hakuna kinacholingana na kichujio

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Onyesha vyote

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = Hujafungua chochote hivi karibuni

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Nenda Nyumbani

# Line on the Trash page when it is empty.
folders-view-none-trash = Tupio ni tupu

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = Hakuna diski za kuonyesha

# Line on a search page that found nothing.
folders-view-none-found = Hakuna kilichopatikana

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Futa utafutaji

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = Kumbukumbu hii ni tupu

# Button on an empty archive page: goes up one folder.
folders-view-go-up = Nenda juu folda moja

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = Folda hii ni tupu

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = Folda mpya

# Button on an empty folder's page that makes a new document.
folders-view-new-document = Hati mpya

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] Kitu 1
       *[other] Vitu { $n }
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Haijaunganishwa

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = Unganisha tena { $name }

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } haijaunganishwa.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Unganisha zote tena

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] Kushiriki kumoja hakujaunganishwa.
       *[other] Kushiriki { $n } hakujaunganishwa.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = Sahau { $name }

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = Vidirisha viwili

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = Onyesha vidirisha viwili kando kwa kando (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Onyesha kidirisha kimoja (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = Mwonekano

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = Mpya

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = Chaguo zaidi

# Row of the More menu that undoes the last change.
folders-view-more-undo = Tendua

# Row of the More menu that does again what was undone.
folders-view-more-redo = Rudia

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Panga upande mwingine

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Onyesha faili zilizofichwa (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Ficha faili zilizofichwa (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Faili zilizofichwa

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Kompyuta za karibu

# Tip of the Search button in the top row while the path shows.
folders-view-search = Tafuta (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = Funga utafutaji (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = Huwezi kutafuta kwenye ukurasa huu

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Tafuta

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = Jina la utepe wa pembeni kwa { $was }
