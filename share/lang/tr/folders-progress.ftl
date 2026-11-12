
# machine translated (Turkish), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $of } dosyadan { $done }, saniyede { $rate }, { $left }
       *[no] saniyede { $rate }, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $of } dosyadan { $done }, ne kadar süreceği hesaplanıyor…
       *[no] ne kadar süreceği hesaplanıyor…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } bayt

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = birkaç saniye kaldı

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
        [one] yaklaşık 1 saniye kaldı
       *[other] yaklaşık { $n } saniye kaldı
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = yaklaşık bir dakika kaldı

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
        [one] yaklaşık 1 dakika kaldı
       *[other] yaklaşık { $n } dakika kaldı
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
        [one] yaklaşık 1 saat kaldı
       *[other] yaklaşık { $n } saat kaldı
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
        [one] 1 iş daha çalışıyor
       *[other] { $n } iş daha çalışıyor
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Taşırken disk meşgul olur, bu yüzden işlem bitene kadar diğer programlar yavaşlayabilir.
       *[copying] Kopyalarken disk meşgul olur, bu yüzden işlem bitene kadar diğer programlar yavaşlayabilir.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = yanıt bekleniyor

# End of a piece of work's line while it is paused.
folders-progress-paused = duraklatıldı

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Devam et

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Duraklat

# Tip of the button on a piece of work's row that stops it.
folders-progress-stop = Durdur

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = daha yeni ve daha büyük

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = daha yeni

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = daha büyük

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = { $when } değişti

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] { $name } adlı bir klasör zaten var
        [file] { $name } adlı bir dosya zaten var
       *[unknown] { $name } zaten var
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = Yeni olan

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = Zaten orada olan

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Atla

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = İkisini de tut

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = Değiştir

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = Değiştir ve zaten orada olan klasörü Çöp kutusuna taşı

# Main button of the name taken question for a folder: join the two folders.
folders-progress-merge = Birleştir

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = Bu yanıtı zaten orada olan her klasör için kullan.

# Line over the switch in the name taken question for files.
folders-progress-all-files = Bu yanıtı zaten orada olan her dosya için kullan.

# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = Kalanlar için de aynısını yap

