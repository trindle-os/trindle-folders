# machine translated (Arabic), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = إفراغ سلة المهملات؟

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
        [one] سيُحذف العنصر الموجود فيها نهائيًا. لا يمكنك التراجع عن هذا.
        [two] سيُحذف العنصران فيها نهائيًا. لا يمكنك التراجع عن هذا.
        [few] ستُحذف جميع العناصر الـ{ $n } فيها نهائيًا. لا يمكنك التراجع عن هذا.
        [many] ستُحذف جميع العناصر الـ{ $n } فيها نهائيًا. لا يمكنك التراجع عن هذا.
       *[other] ستُحذف جميع العناصر الـ{ $n } فيها نهائيًا. لا يمكنك التراجع عن هذا.
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = أفرغ سلة المهملات

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
        [one] حذف هذا العنصر نهائيًا؟
        [two] حذف هذين العنصرين نهائيًا؟
        [few] حذف هذه العناصر الـ{ $n } نهائيًا؟
        [many] حذف هذه العناصر الـ{ $n } نهائيًا؟
       *[other] حذف هذه العناصر الـ{ $n } نهائيًا؟
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = لا يمكنك التراجع عن هذا.

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = احذف نهائيًا

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = أبقِها

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = إلى أين تذهب؟

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = اتركها حيث هي




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = أيقونات

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = قائمة

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = مضغوط

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = اختر الأعمدة

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = الاسم

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = رتّب حسب

# Name read out for the toolbar's Group by strip.
folders-view-group-by = جمّع حسب





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = أخفِ لوحة المعاينة (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = أظهر لوحة المعاينة (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = لوحة المعاينة

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = الألسنة

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = ألسنة اللوحة اليسرى

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = ألسنة اللوحة اليمنى

# Tip of the button that opens a new tab.
folders-view-tab-new = لسان جديد (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = أغلق اللسان (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = الرئيسي

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = لا شيء مثبّت بعد. انقر بالزر الأيمن على مجلد لتثبيته.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = المثبّتة

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = الأماكن

# Small heading in the side bar over the drives plugged in.
folders-view-drives = الأقراص

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = المشاركات

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = أغلق { $name }



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = اتصل بخادم

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = اربط اللوحتين. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = اربط اللوحتين

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] انسخ إلى اللوحة اليسرى
       *[other] انسخ إلى اللوحة اليمنى
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] انقل إلى اللوحة اليسرى
       *[other] انقل إلى اللوحة اليمنى
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] أظهر هذا المجلد في اللوحة اليسرى
       *[other] أظهر هذا المجلد في اللوحة اليمنى
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = بدّل اللوحتين

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] حدّد ما يختلف عن اللوحة اليسرى
       *[other] حدّد ما يختلف عن اللوحة اليمنى
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] قارن مع اللوحة اليسرى
       *[other] قارن مع اللوحة اليمنى
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = اكتب مجلدًا أو ملفًا أو عنوان خادم

# Name read out for the path above a pane's list.
folders-view-where = الموقع

# Tip of the pane's Back button.
folders-view-back = رجوع (Alt+Left)

# Tip of the pane's Forward button.
folders-view-forward = تقدّم (Alt+Right)

# Tip of the pane's button that goes up one folder.
folders-view-up = مجلد أعلى (Alt+Up)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = رشّح، أو اضغط Enter للبحث في هذا المجلد ومجلداته الفرعية

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = ابحث في الأسماء، بما فيها المجلدات الفرعية

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = ابحث داخل الملفات، بما فيها المجلدات الفرعية


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = النوع

# Name read out for the list that picks which size to show.
folders-view-find-size = الحجم

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = آخر تغيير

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = اسم جديد لـ{ $was }

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } حرة

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } محدد، جارٍ حساب الحجم…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } محدد، { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = اعرض الملفات المعنية بهذا

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = حاول مرة أخرى

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
# SAFETY: needs a human review before it ships
folders-view-cancel = إلغاء

# Line on a pane whose filter matches nothing.
folders-view-none-matches = لا شيء هنا يطابق المرشّح

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = أظهر كل شيء

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = لم تفتح شيئًا مؤخرًا

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = اذهب إلى الرئيسي

# Line on the Trash page when it is empty.
folders-view-none-trash = سلة المهملات فارغة

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = لا أقراص لعرضها

# Line on a search page that found nothing.
folders-view-none-found = لم يُعثر على شيء

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = امسح البحث

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = هذا الأرشيف فارغ

# Button on an empty archive page: goes up one folder.
folders-view-go-up = اصعد مجلدًا

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = هذا المجلد فارغ

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = مجلد جديد

# Button on an empty folder's page that makes a new document.
folders-view-new-document = مستند جديد

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] عنصر واحد
        [two] عنصران
        [few] { $n } عناصر
        [many] { $n } عنصرًا
       *[other] { $n } عنصر
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = غير متصل

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = إعادة الاتصال بـ { $name }

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } غير متصل.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = إعادة اتصال الكل

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [zero] لا توجد مشاركات غير متصلة.
        [one] مشاركة واحدة غير متصلة.
        [two] مشاركتان غير متصلتين.
        [few] { $n } مشاركات غير متصلة.
        [many] { $n } مشاركة غير متصلة.
       *[other] { $n } مشاركة غير متصلة.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = نسيان { $name }

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = لوحتان

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = أظهر لوحتين جنبًا إلى جنب (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = أظهر لوحة واحدة (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = عرض

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = جديد

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = المزيد من الخيارات

# Row of the More menu that undoes the last change.
folders-view-more-undo = تراجع

# Row of the More menu that does again what was undone.
folders-view-more-redo = إعادة

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = رتّب بالاتجاه المعاكس

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = أظهر الملفات المخفية (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = أخفِ الملفات المخفية (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = الملفات المخفية

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = حواسيب قريبة

# Tip of the Search button in the top row while the path shows.
folders-view-search = بحث (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = إغلاق البحث (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = لا يمكنك البحث في هذه الصفحة

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = بحث

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = الاسم في الشريط الجانبي لـ { $was }
