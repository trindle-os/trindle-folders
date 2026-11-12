# machine translated (Turkish), not yet reviewed by a person.

# Choice in the Wipe window: write over each file one time.
folders-tools-pass-once = Bir kez

# Choice in the Wipe window: write over each file three times.
folders-tools-pass-three = 3 kez

# Choice in the Wipe window: write over each file seven times.
folders-tools-pass-seven = 7 kez

# Choice in the rename window, under Letters: leave the capitals of the names as they are.
folders-tools-case-keep = Olduğu gibi

# Choice in the rename window, under Letters: make every name small letters.
folders-tools-case-lower = Hepsi küçük harf

# Choice in the rename window, under Letters: make every name capital letters.
folders-tools-case-upper = Hepsi büyük

# Choice in the rename window, under Letters: start each word with a capital.
folders-tools-case-words = Her sözcüğü büyük harfle başlat

# Choice in the rename window, under Letters: start only the first word with a capital.
folders-tools-case-first = İlk sözcüğü büyük harfle başlat

# Choice in the rename window, under Ending: leave the file ending as it is.
folders-tools-ending-keep = Olduğu gibi

# Choice in the rename window, under Ending: make the ending small letters.
folders-tools-ending-lower = Hepsi küçük harf

# Choice in the rename window, under Ending: make the ending capital letters.
folders-tools-ending-upper = Hepsi büyük

# Choice in the rename window, under Ending: give every file a new ending of the person's choice.
folders-tools-ending-new = Yeni uzantı

# Choice in the rename window, under Ending: take the ending off.
folders-tools-ending-none = Uzantısız

# The name offered for a new archive when several things from different places are packed together.
folders-tools-archive-name = Arşiv

# Beside a row of the rename list that cannot be done: the name would be left empty.
folders-tools-clash-empty = ad kalmazdı

# Beside a row of the rename list that cannot be done: the new name is not allowed.
folders-tools-clash-bad = bu ad kullanılamaz

# Beside a row of the rename list that cannot be done: the new name is too long.
folders-tools-clash-long = bu ad çok uzun

# Beside a row of the rename list that cannot be done: two files would get the same new name.
folders-tools-clash-twice = iki öğe bu adı alacak

# Beside a row of the rename list that cannot be done: another file already has the new name.
folders-tools-clash-taken = bu adı zaten başka bir şey kullanıyor

# Beside a row of the rename list that cannot be done: the old name is not plain text, so it cannot be changed.
folders-tools-clash-not-text = eski adda Folders'ın okuyamadığı karakterler var

# Warning in the Wipe window about a flash disk, where old copies may stay out of reach.
folders-tools-caveat-flash = Bu bir flash bellek ya da SSD, bu yüzden eski kopyalar Folders'ın erişemediği bölümlerinde kalabilir.

# Warning in the Wipe window about a filesystem that writes changes elsewhere; { $fs } is the filesystem's name.
folders-tools-caveat-copy-on-write = { $fs } dosya sistemi değişiklikleri yeni bir yere kaydeder, bu yüzden eski veriler hâlâ orada olabilir.

# Warning in the Wipe window about a share on another computer; { $fs } is the kind of share.
folders-tools-caveat-network = Bu, başka bir bilgisayardaki bir { $fs } paylaşımı ve kendi kopyalarını tutuyor olabilir.

# Line in the compare list for a name that is only in the left folder.
folders-tools-row-only-left = Yalnızca solda

# Line in the compare list for a name that is only in the right folder.
folders-tools-row-only-right = Yalnızca sağda

# Line in the compare list for a name that is the same on both sides.
folders-tools-row-same = Aynı

# Line in the compare list for a name that differs: { $why } says how (kind, size, date, content, target, inside, unread) and { $newer } says which side is newer (left, right, none).
folders-tools-row-different =
    Farklı: { $why ->
        [kind] biri klasör ya da bağlantı
        [size] boyutlar farklı
        [date] tarihler farklı
        [content] içerikler farklı
        [target] bağlantılar başka yere işaret ediyor
        [inside] içindeki bir şey farklı
       *[unread] bir taraf okunamadı
    }{ $newer ->
        [left] , solu daha yeni
        [right] , sağı daha yeni
       *[none] { "" }
    }

# A place in the Extract to list: the folder the archive sits in.
folders-tools-place-here = Burası

# SAFETY: needs a human review before it ships
# Red notice when Wipe is chosen on things that are not files.
folders-tools-only-files-wiped = Yalnızca dosyalar güvenli biçimde silinebilir.

# Red notice when comparing the two panes while one of them does not show a folder.
folders-tools-compare-need-folders = Karşılaştırmak için her bölmede bir klasör açın.

# Undo entry after renaming many files; { $n } is how many were renamed.
folders-tools-rename-undo =
    { $n ->
        [one] 1 öğeyi yeniden adlandırmayı geri al
       *[other] { $n } öğeyi yeniden adlandırmayı geri al
    }

# Notice after renaming many files; { $n } is how many were renamed.
folders-tools-rename-done =
    { $n ->
        [one] 1 öğe yeniden adlandırıldı.
       *[other] { $n } öğe yeniden adlandırıldı.
    }

# Red notice when renaming many files fails part way; { $error } says why.
folders-tools-rename-stopped = Yeniden adlandırma yarıda durdu: { $error }.

# Red words under the archive name in the Compress window when the name is empty.
folders-tools-name-empty = Arşiv için bir ad yazın.

# Beside the ring while a search is running and the end is not known; { $done } is how many have been looked at.
folders-tools-looked-at = { $done } denetlendi

# Beside the ring while a search is running and the end is known; { $done } and { $total } are sizes such as 3 MB.
folders-tools-read-of = { $total } içinden { $done } okundu

# Beside the ring at the start of the duplicates search.
folders-tools-listing-files = Dosyalar listeleniyor

# Beside the ring while two files are compared.
folders-tools-comparing = Karşılaştırılıyor

# Beside the ring while two folders are compared.
folders-tools-looking = Klasörler karşılaştırılıyor

# Title of the Compress window; { $name } is the one thing packed, { $n } how many.
folders-tools-compress-title =
    { $n ->
        [one] { $name } öğesini sıkıştır
       *[other] { $n } öğeyi sıkıştır
    }

# Typing box tip in the Compress window: the name of the archive to make.
folders-tools-pack-name-tip = Arşivin adı

# Choice tip in the Compress window: the kind of archive to make.
folders-tools-pack-kind-tip = Arşiv türü

# Note in the Compress window saying where the archive is made; { $place } is the folder.
folders-tools-pack-made-in = Arşiv şuraya kaydedilecek: { $place }

# Main button of the Compress window.
folders-tools-compress = Sıkıştır

# Title of the Extract to window; { $name } is the archive.
folders-tools-extract-to = { $name } arşivini şuraya çıkar

# Note in the Extract to window; { $name } is the name of the folder that is made.
folders-tools-extract-new-folder = Dosyalar { $name } adlı yeni bir klasöre konacak.

# Title of the Resize window; { $name } is the one picture, { $n } how many.
folders-tools-resize-title =
    { $n ->
        [one] { $name } resmini yeniden boyutlandır
       *[other] { $n } resmi yeniden boyutlandır
    }

# Choice tip in the Resize window: the size the copies are made at.
folders-tools-resize-size-tip = Yeni boyut

# Note in the Resize window explaining the sizes and that copies are made.
folders-tools-resize-note = Boyut, uzun kenarın piksel cinsinden uzunluğudur. Folders resimleri yalnızca küçültür. Yeniden boyutlandırılmış her kopyayı özgün resmin yanına, adında yeni boyutla birlikte kaydeder.

# Main button of the Resize window.
folders-tools-resize = Yeniden boyutlandır

# SAFETY: needs a human review before it ships
# Title of the Wipe window; { $name } is the one file, { $n } how many.
folders-tools-wipe-title =
    { $n ->
        [one] { $name } güvenli biçimde silinsin mi?
       *[other] { $n } dosya güvenli biçimde silinsin mi?
    }

# SAFETY: needs a human review before it ships
# Warning at the top of the Wipe window.
folders-tools-wipe-warning = Folders her dosyayı silmeden önce üzerine yazar, bu yüzden onu geri getiremezsiniz.

# Name beside the choice of how many times to write over, in the Wipe window.
folders-tools-wipe-write-over = Üzerine yaz

# Choice tip in the Wipe window: how many times each file is written over.
folders-tools-wipe-passes-tip = Her dosyanın kaç kez üzerine yazılacağı

# SAFETY: needs a human review before it ships
# Red main button of the Wipe window.
folders-tools-wipe-button = Güvenli biçimde sil

# Switch tip in the rename window: put numbers in the names.
folders-tools-rename-number-tip = Numaralandır

# Choice in the rename window: the number goes at the start of the name.
folders-tools-rename-at-start = Başta

# Choice in the rename window: the number goes at the end of the name.
folders-tools-rename-at-end = Sonda

# Choice tip in the rename window: where the number goes in the name.
folders-tools-rename-where-tip = Numaranın yeri

# Typing box tip in the rename window: the number to start counting from.
folders-tools-rename-first-number-tip = İlk numara

# Choice tip in the rename window: what happens to the file endings.
folders-tools-rename-ending-tip = Uzantıyla ne yapılacağı

# Typing box tip in the rename window: the new ending to give every file.
folders-tools-rename-new-ending-tip = Yeni uzantı, örneğin txt

# Detail of a row in the rename list that cannot be done; { $why } says why.
folders-tools-rename-cannot = Folders bunu yeniden adlandıramıyor: { $why }

# Detail of a row in the rename list whose name changes; { $old } is the old name.
folders-tools-rename-was = eski adı { $old }

# Detail of a row in the rename list whose name does not change.
folders-tools-rename-stays = olduğu gibi kalır

# Title of the rename window; { $n } is how many items.
folders-tools-rename-title =
    { $n ->
        [one] 1 öğeyi yeniden adlandır
       *[other] { $n } öğeyi yeniden adlandır
    }

# Name of the find and replace boxes in the rename window.
folders-tools-rename-replace = Değiştir

# Typing box tip in the rename window: the text to find.
folders-tools-rename-find-tip = Bul

# Typing box tip in the rename window: the text to put in its place.
folders-tools-rename-with-tip = Şununla değiştir

# Switch tip in the rename window: treat * and ? as wildcards.
folders-tools-rename-wildcard-tip = * ve ? karakterlerini joker olarak kullan

# Label beside the wildcards switch in the rename window.
folders-tools-rename-wildcard = Jokerler * ve ?

# Switch tip in the rename window: capitals must match exactly.
folders-tools-rename-match-case-tip = Büyük harfleri tam eşleştir

# Label beside the match case switch in the rename window.
folders-tools-rename-match-case = Büyük/küçük harfe duyarlı

# Name of the before and after boxes in the rename window.
folders-tools-rename-add = Ekle

# Typing box tip in the rename window: text added before the name.
folders-tools-rename-before-tip = Adın öncesine

# Typing box tip in the rename window: text added after the name.
folders-tools-rename-after-tip = Adın sonrasına

# Name of the numbering row in the rename window.
folders-tools-rename-number = Numara

# Name over the capitals choice in the rename window.
folders-tools-rename-letters = Büyük harfler

# Choice tip in the rename window: what happens to the capitals in the names.
folders-tools-rename-case-tip = Adlardaki büyük harfler

# Name over the ending choice in the rename window.
folders-tools-rename-ending = Uzantı

# Note under the rename list; { $clash } is yes when some names cannot be used, otherwise no; { $changes } of { $total } names change; { $place } is the folder.
folders-tools-rename-summary =
    { $clash ->
        [yes] { $place } içinde bazı adlar kullanılamıyor. İşaretli satırlara bakın.
       *[no] { $place } içinde { $total } adın { $changes } tanesi değişecek.
    }

# Main button of the rename window.
folders-tools-rename-button = Yeniden adlandır

# Name of the folder searched for duplicates when it is the home folder.
folders-tools-home = Ana klasör

# Title of the duplicates window; { $folder } is the one folder searched, { $n } how many.
folders-tools-dupes-title =
    { $n ->
        [one] { $folder } içindeki yinelenenler
       *[other] { $n } klasördeki yinelenenler
    }

# Red words in the duplicates window when the search fails; { $error } says why.
folders-tools-dupes-stopped = Arama durdu: { $error }.

# Line in the duplicates window when no duplicates were found.
folders-tools-dupes-none = Yinelenen bulunamadı.

# Note in the duplicates window when none were found; { $n } files could not be read.
folders-tools-dupes-unread =
    { $n ->
        [one] 1 dosya okunamadı.
       *[other] { $n } dosya okunamadı.
    }

# Note above the duplicates list; { $n } is how many sets of copies, { $size } how much the extras take.
folders-tools-dupes-sets =
    { $n ->
        [one] 1 kopya kümesi. Fazla kopyalar seçili ve bunları Çöp kutusuna taşımak { $size } yer açar.
       *[other] { $n } kopya kümesi. Fazla kopyalar seçili ve bunları Çöp kutusuna taşımak { $size } yer açar.
    }

# Detail of a row in the duplicates list; { $set } is the set's number, { $size } the file's size, { $at } the folder it is in.
folders-tools-dupes-row = Küme { $set }, { $size }, { $at } içinde

# Note under the duplicates list; { $n } files could not be read.
folders-tools-dupes-unread-left-out =
    { $n ->
        [one] 1 dosya okunamadı ve dışarıda bırakıldı.
       *[other] { $n } dosya okunamadı ve dışarıda bırakıldı.
    }

# Note in the duplicates window when every copy of a set is picked.
folders-tools-dupes-keep-one = Bir kümedeki her kopya seçili. En az birinin seçimini kaldırın.

# Tip of the button in the duplicates window that picks the extra copies.
folders-tools-dupes-pick-extras = Fazla kopyaları seç

# SAFETY: needs a human review before it ships
# Main button of the duplicates window: move the picked files to the Trash.
folders-tools-dupes-trash = Çöp kutusuna taşı

# Title of the window that compares two files.
folders-tools-diff-title = İki dosyayı karşılaştır

# Red words in the file compare window when it fails; { $error } says why.
folders-tools-diff-failed = Folders bunları karşılaştıramadı: { $error }.

# Line in the file compare window when two files are the same, byte for byte.
folders-tools-diff-same-bytes = İki dosya bire bir aynı.

# Line in the file compare window when one file is the start of the other; { $one } and { $other } are their sizes.
folders-tools-diff-same-as-shorter = Kısa olanın sonuna kadar aynılar. Biri { $one }, diğeri { $other }.

# Line in the file compare window for the first byte that differs; { $n } is its number.
folders-tools-diff-from-byte = { $n }. bayttan itibaren farklılar.

# Note in the file compare window; { $n } is how many lines differ.
folders-tools-diff-lines =
    { $n ->
        [0] İki dosya aynı.
        [one] 1 satır farklı.
       *[other] { $n } satır farklı.
    }

# Note in the file compare window when the files are too different to match line by line.
folders-tools-diff-too-different = Satır satır karşılaştırılamayacak kadar farklılar. İlk ve son fark arasındaki her şey değişmiş olarak görünür.

# Note in the file compare window when only the start of each file is compared.
folders-tools-diff-cut = Her dosyanın yalnızca ilk 4 MB'ı karşılaştırılır.

# Title of the window that compares the two panes; { $left } and { $right } are the folders' names.
folders-tools-compare-title = { $left } ile { $right } karşılaştırması

# Label of the switch in the pane compare window that reads every byte.
folders-tools-compare-bytes = İçeriği karşılaştır (daha yavaş)

# Switch tip in the pane compare window: compare the contents of each file.
folders-tools-compare-bytes-tip = Her dosyanın içindekini karşılaştır

# Label of the switch in the pane compare window that also lists names that are the same.
folders-tools-compare-show-same = Eşleşen adları göster

# Switch tip in the pane compare window: list the names that are the same too.
folders-tools-compare-show-same-tip = Eşleşen adları da listele

# Red words in the pane compare window when it fails; { $error } says why.
folders-tools-compare-stopped = Karşılaştırma durdu: { $error }.

# Note in the pane compare window with the four counts.
folders-tools-compare-counts = Yalnızca solda { $left }, yalnızca sağda { $right }, farklı { $different }, aynı { $same }.

# Line in the pane compare window when nothing differs.
folders-tools-compare-all-same = İki klasörün içeriği aynı.

# Tip of the button in the pane compare window that compares again.
folders-tools-compare-again = Yeniden karşılaştır

# SAFETY: needs a human review before it ships
# Button in the pane compare window; { $n } is how many names are copied to the left folder.
folders-tools-compare-copy-left = Eksik ya da daha yeni { $n } öğeyi sola kopyala

# SAFETY: needs a human review before it ships
# Button in the pane compare window; { $n } is how many names are copied to the right folder.
folders-tools-compare-copy-right = Eksik ya da daha yeni { $n } öğeyi sağa kopyala

# Line in the Wipe window naming the folders that were picked too, which are not wiped.
# SAFETY: needs a human review before it ships
folders-tools-wipe-left-out = Yalnızca dosyaların üzerine yazılarak silinir, bu yüzden bu klasörler olduğu gibi bırakılacak: { $names }.
