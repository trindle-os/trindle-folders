
# machine translated (Turkish), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Sunucudaki dosyalarda kes, kopyala ve yapıştır çalışmaz. Bunun yerine bölmeler arasındaki kopyala ve taşı düğmelerini kullanın.

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = Bir arşivin içindeki hiçbir şeyi değiştiremezsiniz. Önce ihtiyacınız olanları çıkarın.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = Bunlar başka bir uygulamada kesilmişti, bu yüzden Folders onları kopyaladı.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = Bu zaten kopyalanıyor veya taşınıyor.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = Bu hâlâ buraya kopyalanıyor.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10.000 öğe veya daha fazlası

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] 1 öğe
       *[other] { $n } öğe
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] Bir öğe için { $verb } işlemi yapılamadı: { $names }.
       *[other] { $n } öğe için { $verb } işlemi yapılamadı: { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } ve { $more } tane daha

# Words on a job card while things are copied: what, and where to.
folders-act-copying = { $what } { $place } konumuna kopyalanıyor

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = { $what } { $place } konumuna kopyalandı.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = kopyalama

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = { $what } kopyalamayı geri al

# Words on a job card while things are moved: what, and where to.
folders-act-moving = { $what } { $place } konumuna taşınıyor

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = { $what } { $place } konumuna taşındı.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = taşıma

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = { $what } taşımayı geri al

# Words on a job card while things are put in the Trash.
folders-act-trashing = { $what } Çöp Kutusuna koyuluyor

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = { $what } Çöp Kutusuna konuldu.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = Çöp Kutusuna koyma

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = { $what } Çöp Kutusuna koymayı geri al

# SAFETY: needs a human review before it ships
# Words on a job card while things are deleted for good.
folders-act-deleting = { $what } siliniyor

# SAFETY: needs a human review before it ships
# Words under the file lines when things have been deleted for good.
folders-act-deleted = { $what } kalıcı olarak silindi.

# SAFETY: needs a human review before it ships
# The word for what was tried, put in 'could not be ...' when deleting failed.
folders-act-verb-deleted = silme

# Words on a job card while things are put back from the Trash.
folders-act-restoring = { $what } geri konuluyor

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = { $what } geri konuldu.

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = { $what } geri koymayı geri al

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = geri koyma

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Geri alınıyor

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Geri alındı.

# Words on a job card while things are packed into an archive.
folders-act-compressing = { $what } sıkıştırılıyor

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = { $file } dosyası { $place } konumunda oluşturuldu.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = sıkıştırma

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = { $what } sıkıştırmayı geri al

# Words on a job card while an archive is unpacked.
folders-act-extracting = { $what } çıkarılıyor

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = { $what } { $place } konumuna çıkarıldı.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = arşivden çıkarma

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = { $what } çıkarmayı geri al

# SAFETY: needs a human review before it ships
# Words on a job card while files are wiped securely.
folders-act-wiping = { $what } güvenli biçimde siliniyor

# SAFETY: needs a human review before it ships
# Words under the file lines when files have been wiped securely.
folders-act-wiped = { $what } güvenli biçimde silindi.

# SAFETY: needs a human review before it ships
# The word for what was tried, put in 'could not be ...' when wiping failed.
folders-act-verb-wiped = güvenli silme

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = { $what } arşivden alınıyor

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = { $what } arşivden { $place } konumuna alındı.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = arşivden alma

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = { $what } arşivden almayı geri al

# Words on a job card while pictures are turned.
folders-act-turning = { $what } döndürülüyor

# Words under the file lines when pictures have been turned.
folders-act-turned = { $what } döndürüldü.

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = döndürme

# Words on a job card while pictures are resized.
folders-act-resizing = { $what } yeniden boyutlandırılıyor

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = { $what } için yeniden boyutlandırılmış bir kopya oluşturuldu.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = yeniden boyutlandırma

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = { $what } yeniden boyutlandırmayı geri al

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = Yeni klasör

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = Yeni belge

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders { $base } öğesini oluşturamadı: { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = Bir ad yazın.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = Ad içinde eğik çizgi olamaz.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = Bu adı sistem kullanıyor. Başka bir ad seçin.

# Red words under a name being typed when it is too long.
folders-act-name-long = Bu ad çok uzun.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders bu işi başlatamadı.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = Yinele: { $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = Geri al: { $what }

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = İş bitmeden durduruldu.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [one] Folders bir öğeyi atladı.
       *[other] Folders { $n } öğeyi atladı.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Önce bir şey seçin.

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = Bu zaten Çöp Kutusunda.

# SAFETY: needs a human review before it ships
# Words under the file lines when a server's files are put in the Trash.
folders-act-far-trash = Sunucudaki dosyalar Çöp Kutusuna konulamaz. Bunun yerine Kalıcı olarak sil'i kullanın.

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Önce kopyalanacak veya taşınacak bir şey seçin.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = Öğeleri bir arşivden yalnızca bir klasöre çıkarabilirsiniz.

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = Öğeleri Çöp Kutusuna kopyalayamazsınız.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = Diğer bölme bir klasör göstermiyor.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Önce kopyalanacak bir şey seçin.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = { $what } kesildi.

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = { $what } kopyalandı.

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = Yalnızca bir klasöre yapıştırabilirsiniz.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = Panoda dosya yok.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = Hiçbir şey kesilmedi veya kopyalanmadı.

# SAFETY: needs a human review before it ships
# Words under the file lines when Delete is used on the Network list.
folders-act-network-delete = Ağ konumlarını buradan silemezsiniz.

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = Çöp Kutusu zaten boş.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Önce Çöp Kutusunda bir şey seçin.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = { $name } oluşturuluyor

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = Yeni bir klasörü ya da belgeyi yalnızca bir klasörün içinde oluşturabilirsiniz.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = { $name } oluşturmayı geri al

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders taşıdığı öğelerdeki klasör renklerini koruyamadı.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = Yinelenecek bir şey yok.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = Geri alınacak bir şey yok.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = Yineleniyor: { $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = Yinelendi: { $what }.

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = yeniden yapma

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = Geri alınıyor: { $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = Geri alındı: { $what }.

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Ev

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Çöp Kutusu

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = Öğeleri yalnızca bir klasörün ya da Çöp Kutusunun üzerine bırakabilirsiniz.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] 1 bağlantı
       *[other] { $n } bağlantı
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = { $what } oluşturuldu.

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] Bir bağlantı oluşturulamadı: { $names }.
       *[other] { $n } bağlantı oluşturulamadı: { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = Yanıt bekleyen bir soru var. Folders'ı kapatmadan önce yanıtlayın ya da işi durdurun.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = Bir iş çalışırken Folders kapatılamaz. Bitmesini bekleyin ya da durdurun, sonra Folders'ı kapatın.
