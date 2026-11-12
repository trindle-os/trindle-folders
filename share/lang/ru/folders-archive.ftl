
# machine translated (Russian), not yet reviewed by a person.

# Reason an archive would not open: it is locked with a password and none was given.
# SAFETY: needs a human review before it ships
folders-archive-trouble-locked = он защищён паролем

# Reason an archive would not open: the password given is not the one.
# SAFETY: needs a human review before it ships
folders-archive-trouble-wrong-key = этот пароль не подходит

# Reason an archive would not open: what came out with the password makes no sense, so the password is wrong or the archive is damaged.
# SAFETY: needs a human review before it ships
folders-archive-trouble-wrong-or-bad = этот пароль не подходит, или архив повреждён

# Reason an archive would not open: it is a RAR locked with a password.
# SAFETY: needs a human review before it ships
folders-archive-trouble-locked-rar = это RAR-архив с паролем, который Folders не может открыть, но сам файл не повреждён

# Reason an archive would not open: it is locked and packed a way Folders does not know.
# SAFETY: needs a human review before it ships
folders-archive-trouble-locked-foreign = в нём используется вид защиты паролем, который Folders не может снять

# Reason a file would not open as an archive: it is not a kind Folders reads.
folders-archive-trouble-unknown = это не тот вид архива, который Folders может открыть

# Reason an archive would not unpack: it holds too many things.
folders-archive-trouble-too-many = в нём больше миллиона элементов, а это слишком много для распаковки в Folders

# Reason an archive would not unpack: the place it was to go has too little room.
folders-archive-trouble-no-room = здесь не хватает места для его содержимого

# Reason an archive would not open: reading it takes more memory or time than is allowed.
folders-archive-trouble-gave-up = для открытия нужно больше памяти или времени, чем разрешает Folders

# Reason an archive would not open: the program that reads this kind is not on the computer.
folders-archive-trouble-no-reader = нет части Trindle, которая открывает такие архивы, поэтому установите обновления, чтобы вернуть её

# Reason an archive would not open: it is packed a way Folders does not know.
folders-archive-trouble-foreign = в нём использован вид сжатия, который Folders не умеет читать

# Reason a split archive would not open: one of its parts is not there.
folders-archive-trouble-parts = не хватает одной из его частей, а все части должны лежать в одной папке

# Reason given for an archive that cannot be read when the file is damaged or ends too soon.
folders-archive-why-damaged = он повреждён или неполон

# Reason given for an archive that cannot be written when the drive is full.
folders-archive-why-no-room = не осталось места

# Reason given for an archive that cannot be unpacked when the folder cannot be written in.
folders-archive-why-read-only = нет разрешения, или диск доступен только для чтения

# Reason given for an archive that cannot be read when the file has gone.
folders-archive-why-gone = его больше нет

# Reason given for an archive that cannot be read: it needs more memory than is allowed.
folders-archive-why-memory = для чтения нужно больше памяти, чем разрешает Folders

# Reason given for an archive that cannot be read when no better reason is known.
folders-archive-why-other = произошла непредвиденная ошибка

# Reason a file inside an archive cannot be opened: it is too big.
folders-archive-inside-too-big = он слишком велик, чтобы открыть его изнутри архива

# Reason a file cannot be taken from an archive: it is not in it.
folders-archive-inside-missing = его нет в архиве

# Reason a 7z archive would not unpack: it needs more memory than is allowed.
folders-archive-7z-memory = для распаковки этого архива 7z нужно больше памяти, чем разрешает Folders

# Reason making an archive failed: a file grew while it was being packed.
folders-archive-pack-grew = файл стал слишком большим, пока Folders упаковывал его

# Reason making an archive failed: something with the archive's name is already there.
folders-archive-pack-exists = там уже есть что-то с таким именем

# Reason making an archive failed: an item has no name to put in it.
folders-archive-pack-no-name = у одного из элементов нет имени, которое Folders мог бы записать в архив

# Reason making an archive failed: Folders reads this kind but cannot write it.
folders-archive-pack-cannot-make = Folders может открывать архивы такого вида, но не может их создавать

# Reason a 7z archive cannot be read; the cause follows, already written out.
folders-archive-damaged-7z = архив 7z повреждён: { $why }

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock too slow to open.
folders-archive-7z-a-lock-too-slow-to-open = чтобы снять защиту паролем, потребуется слишком много времени

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock with no settings.
folders-archive-7z-a-lock-with-no-settings = нет настроек защиты паролем

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock with short settings.
folders-archive-7z-a-lock-with-short-settings = настройки защиты паролем неполные

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock with no key.
folders-archive-7z-a-lock-with-no-key = у защиты паролем нет подходящего ключа

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a locked block is not whole.
folders-archive-7z-a-locked-block-is-not-whole = защищённая часть архива неполна

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header ends too soon.
folders-archive-7z-the-header-ends-too-soon = список его содержимого обрывается слишком рано

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a count is far too big.
folders-archive-7z-a-count-is-far-too-big = одно из чисел в нём чрезмерно велико

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block has no packed bytes.
folders-archive-7z-a-block-has-no-packed-bytes = в одной из частей нет упакованных данных

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block is out of reach.
folders-archive-7z-a-block-is-out-of-reach = одна из частей указывает за конец файла

# Cause given after "damaged 7z:" when a 7z archive cannot be read: too short.
folders-archive-7z-too-short = файл слишком короткий

# Cause given after "damaged 7z:" when a 7z archive cannot be read: no 7z mark.
folders-archive-7z-no-7z-mark = он не начинается как файл 7z

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the start header's check does not match.
folders-archive-7z-the-start-header-s-check-does-not-match = начало файла не проходит проверку

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header is out of reach.
folders-archive-7z-the-header-is-out-of-reach = список его содержимого указывает за конец файла

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header's check does not match.
folders-archive-7z-the-header-s-check-does-not-match = список его содержимого не проходит проверку

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a packed header with no block.
folders-archive-7z-a-packed-header-with-no-block = в упакованном списке его содержимого нет данных

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header is far too big.
folders-archive-7z-the-header-is-far-too-big = список его содержимого чрезмерно велик

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the packed header's check does not match.
folders-archive-7z-the-packed-header-s-check-does-not-match = упакованный список его содержимого не проходит проверку

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown header.
folders-archive-7z-an-unknown-header = список его содержимого записан в формате, который Folders не распознаёт

# Cause given after "damaged 7z:" when a 7z archive cannot be read: headers packed too many times over.
folders-archive-7z-headers-packed-too-many-times-over = список его содержимого упакован слишком много раз

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the header.
folders-archive-7z-an-unknown-part-in-the-header = в списке его содержимого есть что-то неизвестное

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the streams.
folders-archive-7z-an-unknown-part-in-the-streams = в его данных есть что-то неизвестное

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the packed streams.
folders-archive-7z-an-unknown-part-in-the-packed-streams = в его упакованных данных есть что-то неизвестное

# Cause given after "damaged 7z:" when a 7z archive cannot be read: packed streams with no sizes.
folders-archive-7z-packed-streams-with-no-sizes = отсутствуют размеры его упакованных данных

# Cause given after "damaged 7z:" when a 7z archive cannot be read: no list of blocks.
folders-archive-7z-no-list-of-blocks = отсутствует список его частей

# Cause given after "damaged 7z:" when a 7z archive cannot be read: blocks kept elsewhere.
folders-archive-7z-blocks-kept-elsewhere = список его частей хранится там, где Folders не может его прочитать

# Cause given after "damaged 7z:" when a 7z archive cannot be read: blocks with no sizes.
folders-archive-7z-blocks-with-no-sizes = отсутствуют размеры его частей

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the blocks.
folders-archive-7z-an-unknown-part-in-the-blocks = в списке его частей есть что-то неизвестное

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block with no sensible coders.
folders-archive-7z-a-block-with-no-sensible-coders = у одной из его частей нет подходящих шагов упаковки

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a coder with too many ends.
folders-archive-7z-a-coder-with-too-many-ends = у одного из шагов упаковки слишком много входов или выходов

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block's coders do not join up.
folders-archive-7z-a-block-s-coders-do-not-join-up = шаги упаковки одной из частей не сочетаются друг с другом

# Cause given after "damaged 7z:" when a 7z archive cannot be read: far too many files.
folders-archive-7z-far-too-many-files = в нём указано чрезмерно много файлов

# Cause given after "damaged 7z:" when a 7z archive cannot be read: files in a block with no sizes.
folders-archive-7z-files-in-a-block-with-no-sizes = у некоторых файлов в одной из частей не записан размер

# Cause given after "damaged 7z:" when a 7z archive cannot be read: files bigger than their block.
folders-archive-7z-files-bigger-than-their-block = некоторые файлы больше части, в которой они лежат

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the files' sizes.
folders-archive-7z-an-unknown-part-in-the-files-sizes = в размерах его файлов есть что-то неизвестное

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a part is far too big.
folders-archive-7z-a-part-is-far-too-big = одна из его частей чрезмерно велика

# Cause given after "damaged 7z:" when a 7z archive cannot be read: names kept elsewhere.
folders-archive-7z-names-kept-elsewhere = имена его файлов хранятся там, где Folders не может их прочитать

# Cause given after "damaged 7z:" when a 7z archive cannot be read: too few names.
folders-archive-7z-too-few-names = не хватает некоторых имён файлов

# Cause given after "damaged 7z:" when a 7z archive cannot be read: times kept elsewhere.
folders-archive-7z-times-kept-elsewhere = даты его файлов хранятся там, где Folders не может их прочитать

# Cause given after "damaged 7z:" when a 7z archive cannot be read: attributes kept elsewhere.
folders-archive-7z-attributes-kept-elsewhere = сведения о его файлах хранятся там, где Folders не может их прочитать

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block runs past the end.
folders-archive-7z-a-block-runs-past-the-end = одна из его частей выходит за конец файла

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block's coders go round in a loop.
folders-archive-7z-a-block-s-coders-go-round-in-a-loop = шаги упаковки одной из частей идут по кругу

# Cause given after "damaged 7z:" when a 7z archive cannot be read: LZMA with no settings.
folders-archive-7z-lzma-with-no-settings = у его упаковки LZMA нет настроек

# Cause given after "damaged 7z:" when a 7z archive cannot be read: LZMA2 with no settings.
folders-archive-7z-lzma2-with-no-settings = у его упаковки LZMA2 нет настроек

# Cause given after "damaged 7z:" when a 7z archive cannot be read: LZMA2 with impossible settings.
folders-archive-7z-lzma2-with-impossible-settings = у его упаковки LZMA2 недопустимые настройки

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block would not open.
folders-archive-7z-a-block-would-not-open = Folders не удалось открыть одну из его частей

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block ends too soon.
folders-archive-7z-a-block-ends-too-soon = одна из его частей обрывается слишком рано

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a file ends too soon.
folders-archive-7z-a-file-ends-too-soon = один из файлов в нём обрывается слишком рано

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a file's check does not match.
folders-archive-7z-a-file-s-check-does-not-match = один из файлов в нём не проходит проверку

# Cause given after "damaged 7z:" when a 7z archive cannot be read: files' sizes and checks do not match up.
folders-archive-7z-files-sizes-and-checks-do-not-match-up = размеры и проверки его файлов не сходятся

# Cause given after "damaged 7z:" when a 7z archive cannot be read: more files than blocks hold.
folders-archive-7z-more-files-than-blocks-hold = в нём указано больше файлов, чем вмещают его части

# Cause given after "damaged 7z:" when a 7z archive cannot be read: more files than sizes.
folders-archive-7z-more-files-than-sizes = для некоторых его файлов не записан размер

# Reason a zip archive cannot be read; the cause follows, already written out.
folders-archive-damaged-zip = архив zip повреждён: { $why }

# Cause given after "damaged zip:" when a zip archive cannot be read: no end record.
folders-archive-zip-no-end-record = отсутствует запись в его конце

# Cause given after "damaged zip:" when a zip archive cannot be read: no ZIP64 end record.
folders-archive-zip-no-zip64-end-record = отсутствует запись ZIP64 в его конце

# Cause given after "damaged zip:" when a zip archive cannot be read: directory out of place.
folders-archive-zip-directory-out-of-place = список его содержимого находится не на своём месте

# Cause given after "damaged zip:" when a zip archive cannot be read: directory too big.
folders-archive-zip-directory-too-big = список его содержимого слишком велик

# Cause given after "damaged zip:" when a zip archive cannot be read: record runs past the directory.
folders-archive-zip-record-runs-past-the-directory = одна из записей выходит за конец списка содержимого

# Cause given after "damaged zip:" when a zip archive cannot be read: no local header.
folders-archive-zip-no-local-header = отсутствуют сведения, которые должны стоять перед файлом

# Cause given after "damaged zip:" when a zip archive cannot be read: entry runs past the end.
folders-archive-zip-entry-runs-past-the-end = один из файлов в нём выходит за конец архива

# Cause given after "damaged zip:" when a zip archive cannot be read: an entry's check does not match.
folders-archive-zip-an-entry-s-check-does-not-match = один из его файлов не проходит проверку

# Cause given after "damaged zip:" when a zip archive cannot be read: an entry is longer than it says.
folders-archive-zip-an-entry-is-longer-than-it-says = один из файлов в нём длиннее, чем указано в записанном размере

# Cause given after "damaged zip:" when a zip archive cannot be read: a locked entry is cut short.
folders-archive-zip-a-locked-entry-is-cut-short = файл в нём, защищённый паролем, неполон

# Cause given after "damaged zip:" when a zip archive cannot be read: a lock with a key of no known length.
folders-archive-zip-a-lock-with-a-key-of-no-known-length = защита паролем использует длину ключа, которую Folders не распознаёт

# Cause given after "damaged zip:" when a zip archive cannot be read: a locked entry's check does not match.
folders-archive-zip-a-locked-entry-s-check-does-not-match = файл в нём, защищённый паролем, не проходит проверку

