
# machine translated (Turkish), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = Çöp kutusu boşaltılsın mı?

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
        [one] İçindeki öğe kalıcı olarak silinecek. Bunu geri alamazsınız.
       *[other] İçindeki { $n } öğenin tümü kalıcı olarak silinecek. Bunu geri alamazsınız.
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = Çöp kutusunu boşalt

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
        [one] Bu öğe kalıcı olarak silinsin mi?
       *[other] Bu { $n } öğe kalıcı olarak silinsin mi?
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = Bunu geri alamazsınız.

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = Kalıcı olarak sil

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = Kalsınlar

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = Nereye gitsinler?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Bulundukları yerde kalsınlar




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = Simgeler

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = Liste

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = Sıkı

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = Sütunları seç

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = Ad

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = Sırala

# Name read out for the toolbar's Group by strip.
folders-view-group-by = Grupla





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Önizleme panelini gizle (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Önizleme panelini göster (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Önizleme paneli

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Sekmeler

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Sol bölme sekmeleri

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Sağ bölme sekmeleri

# Tip of the button that opens a new tab.
folders-view-tab-new = Yeni sekme (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = Sekmeyi kapat (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = Ev

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Henüz sabitlenen yok. Sabitlemek için bir klasöre sağ tıklayın.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = Sabitlenenler

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Yerler

# Small heading in the side bar over the drives plugged in.
folders-view-drives = Sürücüler

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Paylaşımlar

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = { $name } öğesini kapat



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Bir sunucuya bağlan

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = İki bölmeyi bağla. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = İki bölmeyi bağla

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] Sol bölmeye kopyala
       *[other] Sağ bölmeye kopyala
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] Sol bölmeye taşı
       *[other] Sağ bölmeye taşı
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Bu klasörü sol bölmede göster
       *[other] Bu klasörü sağ bölmede göster
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Bölmeleri değiştir

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Sol bölmeden farklı olanları seç
       *[other] Sağ bölmeden farklı olanları seç
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] Sol bölmeyle karşılaştır
       *[other] Sağ bölmeyle karşılaştır
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Bir klasör, dosya veya sunucu adresi yazın

# Name read out for the path above a pane's list.
folders-view-where = Konum

# Tip of the pane's Back button.
folders-view-back = Geri (Alt+Sol)

# Tip of the pane's Forward button.
folders-view-forward = İleri (Alt+Sağ)

# Tip of the pane's button that goes up one folder.
folders-view-up = Bir klasör yukarı (Alt+Yukarı)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Süzün veya bu klasörde ve alt klasörlerinde aramak için Enter'a basın

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Adlarda ara, alt klasörler dahil

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = Dosyaların içinde ara, alt klasörler dahil


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Tür

# Name read out for the list that picks which size to show.
folders-view-find-size = Boyut

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Son değiştirilme

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = { $was } için yeni ad

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } boş

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } seçili, boyut hesaplanıyor…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } seçili, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Bu işlemle ilgili dosyaları göster

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Yeniden dene

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
# SAFETY: needs a human review before it ships
folders-view-cancel = İptal

# Line on a pane whose filter matches nothing.
folders-view-none-matches = Burada süzgece uyan bir şey yok

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Her şeyi göster

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = Yakın zamanda hiçbir şey açmadınız

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Eve git

# Line on the Trash page when it is empty.
folders-view-none-trash = Çöp kutusu boş

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = Gösterilecek sürücü yok

# Line on a search page that found nothing.
folders-view-none-found = Hiçbir şey bulunamadı

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Aramayı temizle

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = Bu arşiv boş

# Button on an empty archive page: goes up one folder.
folders-view-go-up = Bir klasör yukarı çık

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = Bu klasör boş

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = Yeni klasör

# Button on an empty folder's page that makes a new document.
folders-view-new-document = Yeni belge

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
        [one] 1 öğe
       *[other] { $n } öğe
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Bağlı değil

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = { $name } paylaşımını yeniden bağla

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } bağlı değil.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Hepsini yeniden bağla

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] Bir paylaşım bağlı değil.
       *[other] { $n } paylaşım bağlı değil.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = { $name } paylaşımını listeden kaldır

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = İki bölme

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = İki bölmeyi yan yana göster (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Tek bölme göster (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = Görünüm

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = Yeni

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = Daha fazla seçenek

# Row of the More menu that undoes the last change.
folders-view-more-undo = Geri al

# Row of the More menu that does again what was undone.
folders-view-more-redo = Yinele

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Ters sırala

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Gizli dosyaları göster (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Gizli dosyaları gizle (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Gizli dosyalar

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Yakındaki bilgisayarlar

# Tip of the Search button in the top row while the path shows.
folders-view-search = Ara (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = Aramayı kapat (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = Bu sayfada arama yapamazsınız

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Ara

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = { $was } için kenar çubuğu adı
