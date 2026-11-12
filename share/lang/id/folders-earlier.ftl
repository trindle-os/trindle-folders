
# machine translated (Indonesian), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = Tidak ada yang dapat diubah di salinan sebelumnya. Salin item keluar darinya untuk menyimpannya.

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] hari ini pukul { $time }
        [yesterday] kemarin pukul { $time }
       *[other] { $date } pukul { $time }
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] Hari ini pukul { $time }
        [yesterday] Kemarin pukul { $time }
       *[other] { $date } pukul { $time }
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = Ini salinan sebelumnya dari { $when }. Anda tidak dapat mengubahnya, tetapi dapat menyalin item keluar darinya.

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = Ini salinan sebelumnya. Anda tidak dapat mengubahnya, tetapi dapat menyalin item keluar darinya.

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = Sebelumnya: { $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = Urungkan pengembalian { $name }

# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = Folders mengembalikannya. Yang sekarang ada di sebelahnya dengan nama baru.

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = Folders mengembalikannya ke tempat semula.

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = Isinya sama dengan yang sekarang.

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = Isinya berbeda dari yang ada sekarang.

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = Folders tidak dapat membandingkannya: { $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = Versi sebelumnya

# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = Item yang dihapus dari sini

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = Versi sebelumnya dari { $name }

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = Tidak ada versi sebelumnya yang berbeda dari yang sekarang.

# Note at the foot of the earlier versions card about what putting one back does.
folders-earlier-versions-foot = Mengembalikan satu versi menyimpan yang sekarang di sebelahnya. Anda dapat mengurungkan langkah ini.

# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = Item yang dihapus dari { $name }

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = Tidak ada item dari salinan sebelumnya yang dihapus dari folder ini.

# Note at the foot of the deleted things card about what putting one back does.
folders-earlier-deleted-foot = Mengembalikan satu item memasukkannya kembali ke folder ini. Anda dapat mengurungkan langkah ini.

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = Folder seperti keadaannya saat itu

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }, dari salinan yang dibuat pada { $date }

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = Sebuah folder, terakhir terlihat { $when }

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }, terakhir terlihat { $when }

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = Sedang bekerja…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = Buka folder hari itu di tab baru

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = Buka

# Tip and name of the round button on a row of the earlier versions card that puts it back.
folders-earlier-put-back-tip = Kembalikan

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = Bandingkan dengan yang sekarang

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = Itu bukan versi sebelumnya.

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = Versi sebelumnya hanya dapat dikembalikan ke dalam folder home.

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = Folders tidak dapat mengembalikan file ke sana.

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = Folders tidak dapat membuat ulang folder tempatnya berada: { $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = Folders tidak dapat menyalin versi sebelumnya: { $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = Folders tidak dapat memindahkan yang sekarang agar tidak menghalangi: { $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = Folders tidak dapat mengembalikan versi sebelumnya: { $why }. Yang sekarang kini bernama { $name }.

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = Folders tidak dapat mengembalikan versi sebelumnya: { $why }
