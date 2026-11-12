
# machine translated (Hindi), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = ट्रैश खाली करें?

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
        [one] इसमें जो चीज़ है वह हमेशा के लिए मिटा दी जाएगी। इसे वापस नहीं लिया जा सकता।
       *[other] इसमें सभी { $n } चीज़ें हमेशा के लिए मिटा दी जाएँगी। इसे वापस नहीं लिया जा सकता।
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = ट्रैश खाली करें

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
        [one] इस चीज़ को हमेशा के लिए मिटाएँ?
       *[other] इन { $n } चीज़ों को हमेशा के लिए मिटाएँ?
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = इसे वापस नहीं लिया जा सकता।

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = हमेशा के लिए मिटाएँ

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = इन्हें रखें

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = इन्हें कहाँ जाना चाहिए?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = इन्हें जहाँ हैं वहीं रहने दें




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = आइकन

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = सूची

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = सघन

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = स्तंभ चुनें

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = नाम

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = इसके अनुसार क्रम दें

# Name read out for the toolbar's Group by strip.
folders-view-group-by = इसके अनुसार समूह बनाएँ





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = पूर्वावलोकन पैनल छिपाएँ (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = पूर्वावलोकन पैनल दिखाएँ (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = पूर्वावलोकन पैनल

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = टैब

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = बाएँ पैन के टैब

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = दाएँ पैन के टैब

# Tip of the button that opens a new tab.
folders-view-tab-new = नया टैब (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = टैब बंद करें (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = होम

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = अभी कुछ पिन नहीं किया गया। पिन करने के लिए फ़ोल्डर पर राइट-क्लिक करें।

# Small heading in the side bar over the pinned folders.
folders-view-pinned = पिन किए हुए

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = जगहें

# Small heading in the side bar over the drives plugged in.
folders-view-drives = ड्राइव

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = शेयर

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = { $name } बंद करें



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = सर्वर से जुड़ें

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = दोनों पैन को जोड़ें। { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = दोनों पैन को जोड़ें

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] बाएँ पैन में कॉपी करें
       *[other] दाएँ पैन में कॉपी करें
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] बाएँ पैन में ले जाएँ
       *[other] दाएँ पैन में ले जाएँ
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] यह फ़ोल्डर बाएँ पैन में दिखाएँ
       *[other] यह फ़ोल्डर दाएँ पैन में दिखाएँ
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = पैन की अदला-बदली करें

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] जो बाएँ पैन से अलग है उसे चुनें
       *[other] जो दाएँ पैन से अलग है उसे चुनें
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] बाएँ पैन से तुलना करें
       *[other] दाएँ पैन से तुलना करें
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = फ़ोल्डर, फ़ाइल या सर्वर का पता लिखें

# Name read out for the path above a pane's list.
folders-view-where = जगह

# Tip of the pane's Back button.
folders-view-back = पीछे (Alt+बायाँ)

# Tip of the pane's Forward button.
folders-view-forward = आगे (Alt+दायाँ)

# Tip of the pane's button that goes up one folder.
folders-view-up = एक फ़ोल्डर ऊपर (Alt+ऊपर)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = छानें, या इस फ़ोल्डर और इसके उप-फ़ोल्डरों में खोजने के लिए Enter दबाएँ

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = उप-फ़ोल्डरों समेत नाम खोजें

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = उप-फ़ोल्डरों समेत फ़ाइलों के अंदर खोजें


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = प्रकार

# Name read out for the list that picks which size to show.
folders-view-find-size = आकार

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = आखिरी बार बदला गया

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = { $was } का नया नाम

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } खाली

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } चुने गए, आकार का हिसाब लगाया जा रहा है…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } चुने गए, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }। { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = इससे जुड़ी फ़ाइलें दिखाएँ

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = फिर कोशिश करें

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
# SAFETY: needs a human review before it ships
folders-view-cancel = रद्द करें

# Line on a pane whose filter matches nothing.
folders-view-none-matches = यहाँ कुछ भी छन्ने से मेल नहीं खाता

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = सब दिखाएँ

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = आपने हाल में कुछ नहीं खोला

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = होम पर जाएँ

# Line on the Trash page when it is empty.
folders-view-none-trash = ट्रैश खाली है

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = दिखाने के लिए कोई ड्राइव नहीं

# Line on a search page that found nothing.
folders-view-none-found = कुछ नहीं मिला

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = खोज साफ़ करें

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = यह संग्रह खाली है

# Button on an empty archive page: goes up one folder.
folders-view-go-up = एक फ़ोल्डर ऊपर जाएँ

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = यह फ़ोल्डर खाली है

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = नया फ़ोल्डर

# Button on an empty folder's page that makes a new document.
folders-view-new-document = नया दस्तावेज़

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] 1 चीज़
       *[other] { $n } चीज़ें
    }



# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = जुड़ा नहीं है

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = { $name } फिर से जोड़ें

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } जुड़ा नहीं है।

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = सभी फिर से जोड़ें


# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] एक शेयर जुड़ा नहीं है।
       *[other] { $n } शेयर जुड़े नहीं हैं।
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = { $name } को भूल जाएँ

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = दो पैन

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = दो पैन साथ-साथ दिखाएँ (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = एक पैन दिखाएँ (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = दृश्य

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = नया

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = और विकल्प

# Row of the More menu that undoes the last change.
folders-view-more-undo = पूर्ववत करें

# Row of the More menu that does again what was undone.
folders-view-more-redo = फिर से करें

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = उल्टे क्रम में छाँटें

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = छिपी फ़ाइलें दिखाएँ (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = छिपी फ़ाइलें फिर छिपाएँ (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = छिपी फ़ाइलें

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = आस-पास के कंप्यूटर

# Tip of the Search button in the top row while the path shows.
folders-view-search = खोजें (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = खोज बंद करें (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = इस पेज पर खोज नहीं की जा सकती

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = खोजें

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = { $was } के लिए साइडबार में नाम
