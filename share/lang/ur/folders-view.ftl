# machine translated (Urdu), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Title of the question window asked before the Trash is emptied.
folders-view-empty-trash-title = ردی دان خالی کریں؟

# SAFETY: needs a human review before it ships
# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
folders-view-empty-trash-body =
    { $n ->
        [one] اس میں موجود ایک شے ہمیشہ کے لیے حذف ہو جائے گی۔ یہ واپس نہیں ہو سکتا۔
       *[other] اس میں موجود تمام { $n } اشیا ہمیشہ کے لیے حذف ہو جائیں گی۔ یہ واپس نہیں ہو سکتا۔
    }

# SAFETY: needs a human review before it ships
# Red button of the question asked before the Trash is emptied: empties it at once.
folders-view-empty-trash = ردی دان خالی کریں

# SAFETY: needs a human review before it ships
# Title of the question window asked before items are deleted for good; n is how many.
folders-view-delete-title =
    { $n ->
        [one] یہ شے ہمیشہ کے لیے حذف کریں؟
       *[other] یہ { $n } اشیا ہمیشہ کے لیے حذف کریں؟
    }

# SAFETY: needs a human review before it ships
# Text of the question asked before items are deleted for good.
folders-view-delete-body = یہ واپس نہیں ہو سکتا۔

# SAFETY: needs a human review before it ships
# Red button of the question asked before items are deleted for good: deletes them at once.
folders-view-delete-permanently = ہمیشہ کے لیے حذف کریں

# SAFETY: needs a human review before it ships
# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
folders-view-keep-them = انہیں رہنے دیں

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = یہ کہاں جائیں؟

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = جہاں ہیں وہیں رہنے دیں




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = آئیکن

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = فہرست

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = مختصر

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = کالم چنیں

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = نام

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = ترتیب بلحاظ

# Name read out for the toolbar's Group by strip.
folders-view-group-by = گروپ بلحاظ





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = پیش منظر کا پینل چھپائیں (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = پیش منظر کا پینل دکھائیں (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = پیش منظر کا پینل

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = ٹیبز

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = بائیں پین کے ٹیبز

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = دائیں پین کے ٹیبز

# Tip of the button that opens a new tab.
folders-view-tab-new = نیا ٹیب (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = ٹیب بند کریں (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = ہوم

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = ابھی کچھ پن نہیں کیا گیا۔ پن کرنے کے لیے فولڈر پر دایاں کلک کریں۔

# Small heading in the side bar over the pinned folders.
folders-view-pinned = پن کیے ہوئے

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = مقامات

# Small heading in the side bar over the drives plugged in.
folders-view-drives = ڈرائیوز

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = شیئرز

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = { $name } بند کریں



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = سرور سے جڑیں

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = دونوں پین کو جوڑیں۔ { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = دونوں پین کو جوڑیں

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] بائیں پین میں نقل کریں
       *[other] دائیں پین میں نقل کریں
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] بائیں پین میں منتقل کریں
       *[other] دائیں پین میں منتقل کریں
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] یہ فولڈر بائیں پین میں دکھائیں
       *[other] یہ فولڈر دائیں پین میں دکھائیں
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = پین آپس میں بدلیں

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] جو بائیں پین سے مختلف ہے اسے منتخب کریں
       *[other] جو دائیں پین سے مختلف ہے اسے منتخب کریں
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] بائیں پین سے موازنہ کریں
       *[other] دائیں پین سے موازنہ کریں
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = فولڈر، فائل یا سرور کا پتہ لکھیں

# Name read out for the path above a pane's list.
folders-view-where = جگہ

# Tip of the pane's Back button.
folders-view-back = پیچھے (Alt+Left)

# Tip of the pane's Forward button.
folders-view-forward = آگے (Alt+Right)

# Tip of the pane's button that goes up one folder.
folders-view-up = ایک فولڈر اوپر (Alt+Up)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = فلٹر کریں، یا اس فولڈر اور اس کے ذیلی فولڈرز میں تلاش کے لیے Enter دبائیں

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = نام تلاش کریں، ذیلی فولڈرز سمیت

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = فائلوں کے اندر تلاش کریں، ذیلی فولڈرز سمیت


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = قسم

# Name read out for the list that picks which size to show.
folders-view-find-size = سائز

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = آخری تبدیلی

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = { $was } کا نیا نام

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } خالی

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } منتخب، سائز نکالا جا رہا ہے…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } منتخب، { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }۔ { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = اس سے متعلق فائلیں دکھائیں

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = دوبارہ کوشش کریں

# SAFETY: needs a human review before it ships
# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
folders-view-cancel = منسوخ کریں

# Line on a pane whose filter matches nothing.
folders-view-none-matches = یہاں کچھ بھی فلٹر سے میل نہیں کھاتا

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = سب کچھ دکھائیں

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = آپ نے حال ہی میں کچھ نہیں کھولا

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = ہوم پر جائیں

# Line on the Trash page when it is empty.
folders-view-none-trash = ردی دان خالی ہے

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = دکھانے کے لیے کوئی ڈرائیو نہیں

# Line on a search page that found nothing.
folders-view-none-found = کچھ نہیں ملا

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = تلاش صاف کریں

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = یہ آرکائیو خالی ہے

# Button on an empty archive page: goes up one folder.
folders-view-go-up = ایک فولڈر اوپر جائیں

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = یہ فولڈر خالی ہے

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = نیا فولڈر

# Button on an empty folder's page that makes a new document.
folders-view-new-document = نئی دستاویز

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] 1 شے
       *[other] { $n } اشیا
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = منسلک نہیں

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = { $name } دوبارہ جوڑیں

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } منسلک نہیں ہے۔

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = سب دوبارہ جوڑیں

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] ایک شیئر جڑا ہوا نہیں ہے۔
       *[other] { $n } شیئر جڑے ہوئے نہیں ہیں۔
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = { $name } کو بھول جائیں

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = دو حصے

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = دو حصے ساتھ ساتھ دکھائیں (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = ایک حصہ دکھائیں (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = منظر

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = نیا

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = مزید اختیارات

# Row of the More menu that undoes the last change.
folders-view-more-undo = کالعدم کریں

# Row of the More menu that does again what was undone.
folders-view-more-redo = دوبارہ کریں

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = الٹی ترتیب دیں

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = چھپی ہوئی فائلیں دکھائیں (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = چھپی ہوئی فائلیں پھر چھپائیں (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = چھپی ہوئی فائلیں

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = قریبی کمپیوٹر

# Tip of the Search button in the top row while the path shows.
folders-view-search = تلاش (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = تلاش بند کریں (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = اس صفحے پر تلاش نہیں کی جا سکتی

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = تلاش

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = { $was } کے لیے سائیڈ بار کا نام
