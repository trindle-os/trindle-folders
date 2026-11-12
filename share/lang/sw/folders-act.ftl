# machine translated (Swahili), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = Kukata, kunakili na kubandika hakufanyi kazi na faili zilizo kwenye seva. Tumia vitufe vya kunakili na kuhamisha kati ya vidirisha badala yake.

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = Huwezi kubadilisha chochote ndani ya jalada. Toa unachohitaji kwanza.

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = Hivi vilikatwa kwenye programu nyingine, kwa hiyo Folders imevinakili badala yake.

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = Tayari inanakiliwa au kuhamishwa.

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = Hii bado inanakiliwa hapa.

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = vitu 10,000 au zaidi

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] kitu 1
       *[other] vitu { $n }
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] Kitu kimoja hakikuweza { $verb }: { $names }.
       *[other] Vitu { $n } havikuweza { $verb }: { $names }.
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }. { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } na { $more } zaidi

# Words on a job card while things are copied: what, and where to.
folders-act-copying = Inanakili { $what } hadi { $place }

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = Imenakili { $what } hadi { $place }.

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = kunakiliwa

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = Tendua kunakili { $what }

# Words on a job card while things are moved: what, and where to.
folders-act-moving = Inahamisha { $what } hadi { $place }

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = Imehamisha { $what } hadi { $place }.

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = kuhamishwa

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = Tendua kuhamisha { $what }

# Words on a job card while things are put in the Trash.
folders-act-trashing = Inaweka { $what } kwenye Tupio

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = Imeweka { $what } kwenye Tupio.

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = kuwekwa kwenye Tupio

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = Tendua kuweka { $what } kwenye Tupio

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = Inafuta { $what }

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = Imefuta { $what } kabisa.

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = kufutwa

# Words on a job card while things are put back from the Trash.
folders-act-restoring = Inarudisha { $what }

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = Imerudisha { $what }.

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = Tendua kurudisha { $what }

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = kurudishwa

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = Inatendua

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = Imetendua.

# Words on a job card while things are packed into an archive.
folders-act-compressing = Inabana { $what }

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = Imetengeneza { $file } katika { $place }.

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = kubanwa

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = Tendua kubana { $what }

# Words on a job card while an archive is unpacked.
folders-act-extracting = Inatoa { $what }

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = Imetoa { $what } kwenye { $place }.

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = kutolewa

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = Tendua kutoa { $what }

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = Inafuta kabisa { $what }

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = Imefuta kabisa { $what }.

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = kufutwa kabisa

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = Inatoa { $what }

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = Imetoa { $what } kutoka kwenye jalada hadi kwenye { $place }.

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = kutolewa

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = Tendua kutoa { $what }

# Words on a job card while pictures are turned.
folders-act-turning = Inazungusha { $what }

# Words under the file lines when pictures have been turned.
folders-act-turned = Imezungusha { $what }.

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = kuzungushwa

# Words on a job card while pictures are resized.
folders-act-resizing = Inabadilisha ukubwa wa { $what }

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = Imetengeneza nakala yenye ukubwa uliobadilishwa ya { $what }.

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = kubadilishwa ukubwa

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = Tendua kubadilisha ukubwa wa { $what }

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = Folda mpya

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = Hati mpya

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders haikuweza kutengeneza { $base }: { $why }.

# Red words under a name being typed when it is empty.
folders-act-name-empty = Andika jina.

# Red words under a name being typed when it has a slash.
folders-act-name-slash = Jina haliwezi kuwa na mkwaju.

# Red words under a name being typed when it is . or ..
folders-act-name-kept = Mfumo unatumia jina hilo. Chagua jingine.

# Red words under a name being typed when it is too long.
folders-act-name-long = Jina hilo ni refu mno.

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders haikuweza kuanza kazi hii.

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = Fanya tena { $what }

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = Tendua { $what }

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = Kazi ilisimamishwa kabla ya kukamilika.

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [one] Folders imeruka kitu kimoja.
       *[other] Folders imeruka vitu { $n }.
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = Chagua kitu kwanza.

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = Hicho kiko kwenye Tupio tayari.

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = Faili za seva haziwezi kuwekwa kwenye Tupio. Tumia Futa kabisa badala yake.

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = Chagua kitu cha kunakili au kuhamisha kwanza.

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = Unaweza kutoa vitu kutoka kwenye jalada hadi kwenye folda tu.

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = Huwezi kunakili vitu kwenye Tupio.

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = Kidirisha kingine hakionyeshi folda.

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = Chagua kitu cha kunakili kwanza.

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = Imekata { $what }.

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = Imenakili { $what }.

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = Unaweza kubandika kwenye folda tu.

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = Hakuna faili kwenye ubao wa kunakili.

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = Hakuna kilichokatwa wala kunakiliwa.

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = Huwezi kufuta maeneo ya mtandao kutoka hapa.

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = Tupio liko tupu tayari.

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = Chagua kitu kwenye Tupio kwanza.

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = Inatengeneza { $name }

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = Unaweza kutengeneza folda au hati mpya ndani ya folda tu.

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = Tendua kutengeneza { $name }

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders haikuweza kuhifadhi rangi za folda kwenye vitu ilivyohamisha.

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = Hakuna cha kufanya tena.

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = Hakuna cha kutendua.

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = Inafanya tena { $what }

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = Imefanya tena { $what }.

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = kufanywa tena

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = Inatendua { $what }

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = Imetendua { $what }.

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = Nyumbani

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = Tupio

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = Unaweza kudondosha vitu kwenye folda au Tupio tu.

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] kiungo 1
       *[other] viungo { $n }
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = Imetengeneza { $what }.

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] Kiungo kimoja hakikuweza kutengenezwa: { $names }.
       *[other] Viungo { $n } havikuweza kutengenezwa: { $names }.
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = Kuna swali linalosubiri jibu lako. Lijibu, au simamisha kazi, kabla ya kufunga Folders.

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = Folders haiwezi kufungwa wakati kazi inaendelea. Subiri ikamilike, au isimamishe, kisha ufunge Folders.
