# machine translated (Indonesian), not yet reviewed by a person.

# Title of the question window asked before the Trash is emptied.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-title = Kosongkan Tempat sampah?

# Text of the question asked before the Trash is emptied; n is how many items the Trash holds.
# SAFETY: needs a human review before it ships
folders-view-empty-trash-body =
    { $n ->
       *[other] Semua { $n } item di dalamnya akan dihapus secara permanen. Tindakan ini tidak dapat dibatalkan.
    }

# Red button of the question asked before the Trash is emptied: empties it at once.
# SAFETY: needs a human review before it ships
folders-view-empty-trash = Kosongkan Tempat sampah

# Title of the question window asked before items are deleted for good; n is how many.
# SAFETY: needs a human review before it ships
folders-view-delete-title =
    { $n ->
        [1] Hapus item ini secara permanen?
       *[other] Hapus { $n } item ini secara permanen?
    }

# Text of the question asked before items are deleted for good.
# SAFETY: needs a human review before it ships
folders-view-delete-body = Tindakan ini tidak dapat dibatalkan.

# Red button of the question asked before items are deleted for good: deletes them at once.
# SAFETY: needs a human review before it ships
folders-view-delete-permanently = Hapus permanen

# Tip of the round close button of the question asked before things are deleted: leaves them as they are.
# SAFETY: needs a human review before it ships
folders-view-keep-them = Biarkan saja

# Title of the window that lists the places items dropped on the side bar can go.
folders-view-drop-title = Mau ditaruh di mana?

# Tip of the round close button of the window that asks where dropped items should go: leaves them where they are.
folders-view-drop-leave = Biarkan di tempat semula




# Tip of the toolbar button that shows a folder as icons.
folders-view-icons = Ikon

# Tip of the toolbar button that shows a folder as a list.
folders-view-list = Daftar

# Tip of the toolbar button that shows a folder as a compact list.
folders-view-compact = Ringkas

# Tip of the toolbar button that opens the choice of columns of the list view.
folders-view-columns = Pilih kolom

# First answer of the toolbar's Sort by list: sort by name.
folders-view-sort-name = Nama

# Name read out for the toolbar's Sort by list.
folders-view-sort-by = Urutkan menurut

# Name read out for the toolbar's Group by strip.
folders-view-group-by = Kelompokkan menurut





# Tip of the toolbar button that hides the preview panel.
folders-view-panel-hide = Sembunyikan panel pratinjau (Alt+P)

# Tip of the toolbar button that shows the preview panel.
folders-view-panel-show = Tampilkan panel pratinjau (Alt+P)

# Name read out for the toolbar button that shows or hides the preview panel.
folders-view-panel-spoken = Panel pratinjau

# Name read out for the row of tabs above the list when there is one pane.
folders-view-tabs = Tab

# Name read out for the row of tabs above the left pane's list.
folders-view-tabs-left = Tab panel kiri

# Name read out for the row of tabs above the right pane's list.
folders-view-tabs-right = Tab panel kanan

# Tip of the button that opens a new tab.
folders-view-tab-new = Tab baru (Ctrl+T)

# Tip of the button that closes the tab showing.
folders-view-tab-close = Tutup tab (Ctrl+W)






# Side bar row that opens the Home folder.
folders-view-home = Beranda

# Side bar line under Pinned while no folder is pinned.
folders-view-pinned-none = Belum ada yang disematkan. Klik kanan sebuah folder untuk menyematkannya.

# Small heading in the side bar over the pinned folders.
folders-view-pinned = Disematkan

# Small heading in the side bar over the places (Recent, Trash and so on).
folders-view-places = Tempat

# Small heading in the side bar over the drives plugged in.
folders-view-drives = Drive

# Small heading in the side bar over the Windows shares that are open.
folders-view-shares = Folder bersama

# Tip of the side bar button that closes an open Windows share; name is the share.
folders-view-share-close = Tutup { $name }



# Tip of the button that opens the window to connect to a server, and the button of the empty Network page that does the same.
folders-view-connect = Sambungkan ke server

# Tip of the strip button that links the two panes, while linked; note says what linking found.
folders-view-link-noted = Tautkan kedua panel. { $note }

# Tip of the strip button that links the two panes so they move together.
folders-view-link = Tautkan kedua panel

# Tip of the strip button that copies to the other pane; way is left or right.
folders-view-copy-to =
    { $way ->
        [left] Salin ke panel kiri
       *[other] Salin ke panel kanan
    }

# Tip of the strip button that moves to the other pane; way is left or right.
folders-view-move-to =
    { $way ->
        [left] Pindahkan ke panel kiri
       *[other] Pindahkan ke panel kanan
    }

# Tip of the strip button that shows this folder in the other pane; way is left or right.
folders-view-same-in =
    { $way ->
        [left] Tampilkan folder ini di panel kiri
       *[other] Tampilkan folder ini di panel kanan
    }

# Tip of the strip button that swaps what the two panes show.
folders-view-swap = Tukar kedua panel

# Tip of the strip button that picks what differs from the other pane; way is left or right.
folders-view-pick-differing =
    { $way ->
        [left] Pilih yang berbeda dari panel kiri
       *[other] Pilih yang berbeda dari panel kanan
    }

# Tip of the strip button that compares the two panes side by side; way is left or right.
folders-view-compare =
    { $way ->
        [left] Bandingkan dengan panel kiri
       *[other] Bandingkan dengan panel kanan
    }

# Placeholder of the box where a folder, file or server address is typed.
folders-view-locate-hint = Ketik folder, file, atau alamat server

# Name read out for the path above a pane's list.
folders-view-where = Lokasi

# Tip of the pane's Back button.
folders-view-back = Kembali (Alt+Kiri)

# Tip of the pane's Forward button.
folders-view-forward = Maju (Alt+Kanan)

# Tip of the pane's button that goes up one folder.
folders-view-up = Naik satu folder (Alt+Atas)



# Placeholder of the filter box above a pane's list.
folders-view-filter-hint = Saring, atau tekan Enter untuk mencari di folder ini dan subfoldernya

# Tip of the button beside the filter box that finds things by name here and below.
folders-view-find-name = Cari nama, termasuk subfolder

# Tip of the button beside the filter box that finds words inside files here and below.
folders-view-find-inside = Cari di dalam file, termasuk subfolder


# Name read out for the list that picks which kind of thing to show.
folders-view-find-kind = Jenis

# Name read out for the list that picks which size to show.
folders-view-find-size = Ukuran

# Name read out for the list that picks when things were last changed.
folders-view-find-changed = Terakhir diubah

# Placeholder of the box in which an item is renamed; was is its present name.
folders-view-rename-hint = Nama baru untuk { $was }

# Status line at a pane's foot with nothing picked: how many things the folder holds and the room left on its disk.
folders-view-status-free = { $size } tersisa

# Status line at a pane's foot while the size of what is picked is still being added up.
folders-view-status-selected-working = { $count } dipilih, menghitung ukuran…

# Status line at a pane's foot with things picked: how many, and their size together.
folders-view-status-selected = { $count } dipilih, { $size }

# Status line at a pane's foot: one thing said, then another after it as a new sentence.
folders-view-status-then = { $first }. { $then }

# Tip of the small round i at a pane's foot that lists the files words there are about, each with its reason.
folders-view-listed = Tampilkan file yang terkait

# Tip of the button on an empty or failed pane that reads the place again.
folders-view-try-again = Coba lagi

# Tip of the button on a pane that waits for a place which is not answering: gives up waiting.
# SAFETY: needs a human review before it ships
folders-view-cancel = Batal

# Line on a pane whose filter matches nothing.
folders-view-none-matches = Tidak ada yang cocok dengan saringan

# Button on a pane whose filter matches nothing: clears the filter.
folders-view-show-everything = Tampilkan semuanya

# Line on the Recent page when nothing was opened lately.
folders-view-none-recent = Anda belum membuka apa pun baru-baru ini

# Button on the empty Recent page that goes to the Home folder.
folders-view-go-home = Ke Beranda

# Line on the Trash page when it is empty.
folders-view-none-trash = Tempat sampah kosong

# Line on the Computer page when there are no drives to show.
folders-view-none-drives = Tidak ada drive untuk ditampilkan

# Line on a search page that found nothing.
folders-view-none-found = Tidak ada yang ditemukan

# Button on a search page that found nothing: clears the search.
folders-view-clear-search = Hapus pencarian

# Line on a pane showing an archive with nothing in it.
folders-view-none-archive = Arsip ini kosong

# Button on an empty archive page: goes up one folder.
folders-view-go-up = Naik satu folder

# Line on a pane showing a folder with nothing in it.
folders-view-none-folder = Folder ini kosong

# Button on an empty folder's page that makes a new folder.
folders-view-new-folder = Folder baru

# Button on an empty folder's page that makes a new document.
folders-view-new-document = Dokumen baru

# How many things a folder holds, in the status line and elsewhere.
folders-view-items =
    { $n ->
       *[other] { $n } item
    }

# Quiet words at the end of a pinned row whose network share is not connected.
folders-view-not-connected = Tidak terhubung

# Tip of the round button that mounts a network share again; its name.
folders-view-reconnect = Hubungkan kembali { $name }

# Line under Pinned saying a network share is not connected; its name.
folders-view-share-down = { $name } tidak terhubung.

# Tip and line of the button that mounts every network share that is not connected.
folders-view-reconnect-all = Hubungkan kembali semua

# Line beside Reconnect all: how many network shares are not connected.
folders-view-shares-down =
    { $n ->
        [one] Satu folder bersama tidak terhubung.
       *[other] { $n } folder bersama tidak terhubung.
    }

# Tip of the round button that lets a network share that is not connected go from the list; its name.
folders-view-forget-share = Lupakan { $name }

# Name read out for the top bar's button that shows two panes side by side, or one.
folders-view-two-panes = Dua panel

# Tip of the top bar's two panes button, while one pane shows.
folders-view-panes-two = Tampilkan dua panel berdampingan (F3)

# Tip of the top bar's two panes button, while two panes show.
folders-view-panes-one = Tampilkan satu panel (F3)

# Tip and spoken name of the top bar's button that opens the View menu.
folders-view-view = Tampilan

# Words of the top bar's button that opens the New menu (New folder, New document).
folders-view-new = Baru

# Tip and spoken name of the top bar's button that opens the More menu (Undo, Redo, settings).
folders-view-more = Opsi lainnya

# Row of the More menu that undoes the last change.
folders-view-more-undo = Urungkan

# Row of the More menu that does again what was undone.
folders-view-more-redo = Ulangi

# Row of the View menu that turns the sort round.
folders-view-sort-other-way = Urutkan terbalik

# Tip of the toolbar button that shows hidden files (Ctrl+H).
folders-view-hidden-show = Tampilkan berkas tersembunyi (Ctrl+H)

# Tip of the toolbar button that hides hidden files again (Ctrl+H).
folders-view-hidden-hide = Sembunyikan berkas tersembunyi (Ctrl+H)

# Name read out for the toolbar button that shows or hides hidden files.
folders-view-hidden-spoken = Berkas tersembunyi

# Row under the Network heading of the side bar: the computers found nearby.
folders-view-network-nearby = Komputer di sekitar

# Tip of the Search button in the top row while the path shows.
folders-view-search = Cari (Ctrl+F)

# Tip of the Search button while the search box shows.
folders-view-search-close = Tutup pencarian (Escape)

# Tip of the Search button, greyed, on the page of every drive.
folders-view-search-none = Anda tidak dapat mencari di halaman ini

# Name read out for the Search button, which says whether it is pressed.
folders-view-search-spoken = Cari

# Placeholder of the box in which the name the side bar shows for a pinned folder is typed; was is the folder's own name.
folders-view-pin-name-hint = Nama bilah samping untuk { $was }
