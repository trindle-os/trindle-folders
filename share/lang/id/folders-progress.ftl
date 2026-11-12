
# machine translated (Indonesian), not yet reviewed by a person.

# End of a piece of work's line when speed and time left are known; { $files } is yes when the files are counted ({ $done } of { $of }), { $rate } is the speed such as 24 MB, { $left } is the time left.
folders-progress-pace-known =
    { $files ->
        [yes] { $done } dari { $of } file, { $rate } per detik, { $left }
       *[no] { $rate } per detik, { $left }
    }

# End of a piece of work's line before speed and time left can be told; { $files } is yes when the files are counted ({ $done } of { $of }).
folders-progress-pace-wait =
    { $files ->
        [yes] { $done } dari { $of } file, menghitung perkiraan waktu…
       *[no] menghitung perkiraan waktu…
    }

# The one line of a piece of work in the foot: { $doing } is what is being done, such as Copying 3 items to Documents; { $how } is how fast and how long, or that it is paused.
folders-progress-line = { $doing }: { $how }

# The same with the words about the disk added; { $line } is the line above, { $heavy } the words about the disk.
folders-progress-line-heavy = { $line }. { $heavy }

# A speed or size in bytes; { $n } is the figure.
folders-progress-speed-bytes = { $n } byte

# A speed in kilobytes; { $n } is the figure.
folders-progress-speed-kb = { $n } KB

# A speed in megabytes; { $n } is the figure.
folders-progress-speed-mb = { $n } MB

# A speed in gigabytes; { $n } is the figure.
folders-progress-speed-gb = { $n } GB

# Time left in a piece of work's row: under eight seconds.
folders-progress-left-few = tinggal beberapa detik

# Time left in a piece of work's row, in seconds; { $n } is the rounded figure.
folders-progress-left-seconds =
    { $n ->
       *[other] sekitar { $n } detik lagi
    }

# Time left in a piece of work's row: about one minute.
folders-progress-left-minute = sekitar satu menit lagi

# Time left in a piece of work's row, in minutes; { $n } is the figure.
folders-progress-left-minutes =
    { $n ->
       *[other] sekitar { $n } menit lagi
    }

# Time left in a piece of work's row, in hours; { $n } is the figure.
folders-progress-left-hours =
    { $n ->
       *[other] sekitar { $n } jam lagi
    }

# Line at the top of a foot standing for work that has no room of its own; { $n } is how many.
folders-progress-more =
    { $n ->
       *[other] { $n } pekerjaan lagi sedang berjalan
    }

# End of the line of a copy or move row; { $verb } is moving or copying.
folders-progress-heavy =
    { $verb ->
        [moving] Memindahkan membuat disk sibuk, jadi program lain mungkin lebih lambat sampai selesai.
       *[copying] Menyalin membuat disk sibuk, jadi program lain mungkin lebih lambat sampai selesai.
    }

# End of a piece of work's line while it waits for the person's answer.
folders-progress-waiting = menunggu jawaban

# End of a piece of work's line while it is paused.
folders-progress-paused = dijeda

# Tip of the button on a paused piece of work's row that sets it going again.
folders-progress-carry-on = Lanjutkan

# Tip of the button on a piece of work's row that pauses it.
folders-progress-pause = Jeda

# Tip of the button on a piece of work's row that stops it.
# SAFETY: needs a human review before it ships
folders-progress-stop = Hentikan

# Under a file or folder in the name taken question: it is newer than the other and larger.
folders-progress-mark-newer-larger = lebih baru dan lebih besar

# Under a file or folder in the name taken question: it is newer than the other.
folders-progress-mark-newer = lebih baru

# Under a file in the name taken question: it is larger than the other.
folders-progress-mark-larger = lebih besar

# Line under a file or folder in the name taken question; { $when } is when it was changed.
folders-progress-changed = diubah { $when }

# Heading of the name taken question; { $kind } is folder, file or unknown, and { $name } is the name.
folders-progress-taken =
    { $kind ->
        [folder] Folder bernama { $name } sudah ada di sana
        [file] File bernama { $name } sudah ada di sana
       *[unknown] { $name } sudah ada di sana
    }

# Heading over the file or folder that is being brought, in the name taken question.
folders-progress-coming = Yang baru

# Heading over the file or folder that is already there, in the name taken question.
folders-progress-there = Yang sudah ada di sana

# Tip of the button in the name taken question that skips this one.
folders-progress-skip = Lewati

# Tip of the button in the name taken question that keeps both.
folders-progress-keep-both = Simpan keduanya

# Word on the button in the name taken question that replaces the one already there.
# SAFETY: needs a human review before it ships
folders-progress-replace = Ganti

# Screen reader name of the Replace button for a folder in the name taken question.
# SAFETY: needs a human review before it ships
folders-progress-replace-folder-spoken = Ganti, dan pindahkan folder yang sudah ada ke Tempat sampah

# Main button of the name taken question for a folder: join the two folders.
# SAFETY: needs a human review before it ships
folders-progress-merge = Gabungkan

# Line over the switch in the name taken question for folders.
folders-progress-all-folders = Gunakan jawaban ini untuk setiap folder yang sudah ada.

# Line over the switch in the name taken question for files.
folders-progress-all-files = Gunakan jawaban ini untuk setiap file yang sudah ada.

# Name and tip of the switch in the name taken question that uses the answer for the rest.
folders-progress-same-for-rest = Lakukan hal yang sama untuk sisanya
