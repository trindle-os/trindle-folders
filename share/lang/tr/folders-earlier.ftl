
# machine translated (Turkish), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = Eski bir kopyadaki hiçbir şeyi değiştiremezsiniz. Saklamak için öğeleri oradan kopyalayın.

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] bugün saat { $time }
        [yesterday] dün saat { $time }
       *[other] { $date } saat { $time }
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] Bugün saat { $time }
        [yesterday] Dün saat { $time }
       *[other] { $date } saat { $time }
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = Bu, { $when } tarihli eski bir kopya. Değiştiremezsiniz ama içindeki öğeleri kopyalayabilirsiniz.

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = Bu, eski bir kopya. Değiştiremezsiniz ama içindeki öğeleri kopyalayabilirsiniz.

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = Eski: { $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = { $name } geri koymayı geri al

# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = Folders geri koydu. Şu anki dosya yeni bir adla yanında duruyor.

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = Folders eski yerine geri koydu.

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = Şu anki dosyayla aynı.

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = Şu anki dosyadan farklı.

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = Folders bunları karşılaştıramadı: { $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = Eski sürümler

# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = Buradan silinenler

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = { $name } için eski sürümler

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = Şu anki sürümden farklı eski sürüm yok.

# Note at the foot of the earlier versions card about what putting one back does.
folders-earlier-versions-foot = Birini geri koymak şu anki sürümü yanında saklar. Bunu geri alabilirsiniz.

# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = { $name } klasöründen silinenler

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = Eski kopyalardaki hiçbir şey bu klasörden silinmemiş.

# Note at the foot of the deleted things card about what putting one back does.
folders-earlier-deleted-foot = Birini geri koymak onu bu klasöre döndürür. Bunu geri alabilirsiniz.

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = Klasörün o zamanki hâli

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }, { $date } tarihinde yapılan kopyadan

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = Klasör, en son { $when } görüldü

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }, en son { $when } görüldü

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = Çalışıyor…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = O günün klasörünü yeni sekmede aç

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = Aç

# Tip and name of the round button on a row of the earlier versions card that puts it back.
folders-earlier-put-back-tip = Geri koy

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = Şu anki dosyayla karşılaştır

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = Bu, önceki bir sürüm değil.

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = Önceki bir sürümü yalnızca ev klasörünüzün içine geri koyabilirsiniz.

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = Folders bir dosyayı oraya geri koyamaz.

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = Folders içinde bulunduğu klasörü yeniden oluşturamadı: { $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = Folders önceki sürümü kopyalayamadı: { $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = Folders şu anki dosyayı kenara alamadı: { $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = Folders önceki sürümü geri koyamadı: { $why }. Şu anki dosyanın adı artık { $name }.

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = Folders önceki sürümü geri koyamadı: { $why }
