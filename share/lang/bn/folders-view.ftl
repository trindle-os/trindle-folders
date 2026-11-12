
# machine translated (Bengali), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = ট্র্যাশ খালি করবেন?

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
        [one] এতে থাকা আইটেমটি চিরতরে মুছে ফেলা হবে। এটি ফেরানো যাবে না।
       *[other] এতে থাকা সব { $n }টি আইটেম চিরতরে মুছে ফেলা হবে। এটি ফেরানো যাবে না।
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = ট্র্যাশ খালি করুন

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
        [one] এই আইটেমটি চিরতরে মুছবেন?
       *[other] এই { $n }টি আইটেম চিরতরে মুছবেন?
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = এটি ফেরানো যাবে না।

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = চিরতরে মুছুন

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = রেখে দিন

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = এগুলো কোথায় যাবে?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = যেখানে আছে সেখানেই থাকুক




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = আইকন

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = তালিকা

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = সংক্ষিপ্ত

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = কলাম বেছে নিন

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = নাম

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = সাজানোর ধরন

# Name read out for the toolbar's Group by strip.
folders-view-group-by = ভাগ করার ধরন





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = প্রিভিউ প্যানেল লুকান (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = প্রিভিউ প্যানেল দেখান (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = প্রিভিউ প্যানেল

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = ট্যাব

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = বাঁ প্যানের ট্যাব

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = ডান প্যানের ট্যাব

# Tip of the button that opens a new tab.
folders-view-tab-new = নতুন ট্যাব (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = ট্যাব বন্ধ করুন (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = হোম

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = এখনও কিছু পিন করা নেই। পিন করতে ফোল্ডারে ডান ক্লিক করুন।

# Small heading in the side bar over the pinned folders.
folders-view-pinned = পিন করা

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = জায়গা

# Small heading in the side bar over the drives plugged in.
folders-view-drives = ড্রাইভ

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = শেয়ার

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = { $name } বন্ধ করুন



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = সার্ভারে যুক্ত হন

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = দুটি প্যান জুড়ে দিন। { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = দুটি প্যান জুড়ে দিন

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] বাঁ প্যানে কপি করুন
       *[other] ডান প্যানে কপি করুন
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] বাঁ প্যানে সরান
       *[other] ডান প্যানে সরান
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] এই ফোল্ডার বাঁ প্যানে দেখান
       *[other] এই ফোল্ডার ডান প্যানে দেখান
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = প্যান দুটি অদলবদল করুন

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] বাঁ প্যানের সঙ্গে যা আলাদা তা বেছে নিন
       *[other] ডান প্যানের সঙ্গে যা আলাদা তা বেছে নিন
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] বাঁ প্যানের সঙ্গে তুলনা করুন
       *[other] ডান প্যানের সঙ্গে তুলনা করুন
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = ফোল্ডার, ফাইল বা সার্ভারের ঠিকানা লিখুন

# Name read out for the path above a pane's list.
folders-view-where = অবস্থান

# Tip of the pane's Back button.
folders-view-back = পেছনে (Alt+Left)

# Tip of the pane's Forward button.
folders-view-forward = সামনে (Alt+Right)

# Tip of the pane's button that goes up one folder.
folders-view-up = এক ফোল্ডার ওপরে (Alt+Up)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = ছাঁকুন, বা এই ফোল্ডার ও এর সাবফোল্ডারে খুঁজতে Enter চাপুন

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = সাবফোল্ডারসহ নাম খুঁজুন

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = সাবফোল্ডারসহ ফাইলের ভেতরে খুঁজুন


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = ধরন

# Name read out for the list that picks which size to show.
folders-view-find-size = আকার

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = শেষ বদল

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = { $was }-এর নতুন নাম

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } খালি

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count }টি বাছা হয়েছে, আকার হিসাব চলছে…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count }টি বাছা হয়েছে, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }। { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = এ সম্পর্কিত ফাইলগুলো দেখান

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = আবার চেষ্টা করুন

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
# SAFETY: needs a human review before it ships
folders-view-cancel = বাতিল করুন

# Line on a pane whose filter matches nothing.
folders-view-none-matches = ছাঁকনির সঙ্গে এখানে কিছু মেলেনি

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = সব দেখান

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = আপনি সম্প্রতি কিছু খোলেননি

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = হোমে যান

# Line on the Trash page when it is empty.
folders-view-none-trash = ট্র্যাশ খালি

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = দেখানোর মতো কোনো ড্রাইভ নেই

# Line on a search page that found nothing.
folders-view-none-found = কিছু পাওয়া যায়নি

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = অনুসন্ধান মুছুন

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = এই আর্কাইভ খালি

# Button on an empty archive page: goes up one folder.
folders-view-go-up = এক ফোল্ডার ওপরে যান

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = এই ফোল্ডার খালি

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = নতুন ফোল্ডার

# Button on an empty folder's page that makes a new document.
folders-view-new-document = নতুন নথি

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] { $n }টি আইটেম
       *[other] { $n }টি আইটেম
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = যুক্ত নেই

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = { $name } আবার যুক্ত করুন

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } যুক্ত নেই।

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = সব আবার যুক্ত করুন

# machine translated (Bengali)

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] একটি শেয়ার যুক্ত নেই।
       *[other] { $n }টি শেয়ার যুক্ত নেই।
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
# SAFETY: needs a human review before it ships
folders-view-forget-share = { $name } ভুলে যান

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = দুটি প্যান

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = দুটি প্যান পাশাপাশি দেখান (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = একটি প্যান দেখান (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = দেখা

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = নতুন

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = আরও বিকল্প

# Row of the More menu that undoes the last change.
folders-view-more-undo = আনডু

# Row of the More menu that does again what was undone.
folders-view-more-redo = রিডু

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = উল্টো দিকে সাজান

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = লুকানো ফাইল দেখান (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = লুকানো ফাইল আবার লুকান (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = লুকানো ফাইল

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = কাছের কম্পিউটার

# Tip of the Search button in the top row while the path shows.
folders-view-search = খুঁজুন (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = খোঁজা বন্ধ করুন (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = এই পাতায় খোঁজা যায় না

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = খুঁজুন

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = { $was }-এর সাইডবারের নাম
