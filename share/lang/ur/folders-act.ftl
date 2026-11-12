
# machine translated (Urdu), not yet reviewed by a person.

# Words under the file lines when copy, cut or paste is tried on a server's files.
folders-act-far-clip = سرور کی فائلوں کے ساتھ کاٹنا، نقل کرنا اور چسپاں کرنا کام نہیں کرتے۔ اس کی بجائے حصوں کے درمیان نقل اور منتقل کے بٹن استعمال کریں۔

# Words under the file lines when something is tried that would change what is inside an archive.
folders-act-shut = آرکائیو کے اندر کچھ نہیں بدلا جا سکتا۔ پہلے جو چاہیے وہ نکال لیں۔

# Words on a finished job when files that another program cut were pasted: they are copied.
folders-act-cut-elsewhere = یہ دوسری ایپ میں کاٹے گئے تھے، اس لیے Folders نے انہیں نقل کر دیا۔

# Words under the file lines when something is picked that is already being copied or moved.
folders-act-being-moved = پہلے ہی نقل یا منتقل ہو رہا ہے۔

# Words under the file lines when a copy that is not whole yet is picked.
folders-act-arriving = یہ ابھی یہاں نقل ہو رہا ہے۔

# Part of the question about a name already taken: a folder that holds this many things or more.
folders-act-holds-most = 10,000 یا اس سے زیادہ چیزیں

# Names a number of things picked, inside the words about copying, moving and the like.
folders-act-items =
    { $n ->
        [one] 1 چیز
       *[other] { $n } چیزیں
    }

# Red words on a finished job: how many things could not be done and the first few names; the verb is the word for what was tried, like copied.
folders-act-failed =
    { $n ->
        [one] ایک چیز کے ساتھ یہ کام ({ $verb }) نہیں ہو سکا: { $names }۔
       *[other] { $n } چیزوں کے ساتھ یہ کام ({ $verb }) نہیں ہو سکا: { $names }۔
    }

# Red words on a finished job when a drive would not say it kept what was written: the drives, why, then what else failed.
folders-act-not-kept-and-failed = { $kept }: { $why }۔ { $rest }

# The first few names in a list of things that failed, then how many more there are.
folders-act-names-more = { $names } اور { $more } مزید

# Words on a job card while things are copied: what, and where to.
folders-act-copying = { $what } کو { $place } میں نقل کیا جا رہا ہے

# Words under the file lines when a copy has finished: what, and where to.
folders-act-copied = { $what } کو { $place } میں نقل کر دیا گیا۔

# The word for what was tried, put in 'could not be ...' when a copy failed.
folders-act-verb-copied = نقل کرنا

# Tip of the Undo button after a copy; what was copied.
folders-act-undo-copying = { $what } کی نقل کالعدم کریں

# Words on a job card while things are moved: what, and where to.
folders-act-moving = { $what } کو { $place } میں منتقل کیا جا رہا ہے

# Words under the file lines when a move has finished: what, and where to.
folders-act-moved = { $what } کو { $place } میں منتقل کر دیا گیا۔

# The word for what was tried, put in 'could not be ...' when a move failed.
folders-act-verb-moved = منتقل کرنا

# Tip of the Undo button after a move; what was moved.
folders-act-undo-moving = { $what } کی منتقلی کالعدم کریں

# Words on a job card while things are put in the Trash.
folders-act-trashing = { $what } کو ردی دان میں ڈالا جا رہا ہے

# Words under the file lines when things have been put in the Trash.
folders-act-trashed = { $what } کو ردی دان میں ڈال دیا گیا۔

# The word for what was tried, put in 'could not be ...' when putting things in the Trash failed.
folders-act-verb-trashed = ردی دان میں ڈالنا

# Tip of the Undo button after things were put in the Trash.
folders-act-undo-trashing = { $what } کو ردی دان میں ڈالنا کالعدم کریں

# Words on a job card while things are deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleting = { $what } ہمیشہ کے لیے مٹائی جا رہی ہے

# Words under the file lines when things have been deleted for good.
# SAFETY: needs a human review before it ships
folders-act-deleted = { $what } ہمیشہ کے لیے مٹا دی گئی۔

# The word for what was tried, put in 'could not be ...' when deleting failed.
# SAFETY: needs a human review before it ships
folders-act-verb-deleted = مٹانا

# Words on a job card while things are put back from the Trash.
folders-act-restoring = { $what } کو واپس رکھا جا رہا ہے

# Words under the file lines when things have been put back from the Trash.
folders-act-restored = { $what } کو واپس رکھ دیا گیا۔

# Tip of the Undo button after things were put back from the Trash.
folders-act-undo-restoring = { $what } کو واپس رکھنا کالعدم کریں

# The word for what was tried, put in 'could not be ...' when putting back or undoing failed.
folders-act-verb-put-back = واپس رکھنا

# Words on a job card while a step is undone, when nothing more is said of it.
folders-act-undoing-plain = کالعدم ہو رہا ہے

# Words under the file lines when a step has been undone, when nothing more is said of it.
folders-act-undone = کالعدم ہو گیا۔

# Words on a job card while things are packed into an archive.
folders-act-compressing = { $what } کو سکیڑا جا رہا ہے

# Words under the file lines when an archive was made: its file name (with ending), and where.
folders-act-compressed = { $place } میں { $file } بنا دی گئی۔

# The word for what was tried, put in 'could not be ...' when packing failed.
folders-act-verb-packed = کمپریس کرنا

# Tip of the Undo button after things were packed into an archive.
folders-act-undo-compressing = { $what } کو سکیڑنا کالعدم کریں

# Words on a job card while an archive is unpacked.
folders-act-extracting = { $what } نکالی جا رہی ہے

# Words under the file lines when an archive was unpacked: what, and where.
folders-act-extracted = { $what } کو { $place } میں نکال دیا گیا۔

# The word for what was tried, put in 'could not be ...' when unpacking failed.
folders-act-verb-extracted = نکالنا

# Tip of the Undo button after an archive was unpacked.
folders-act-undo-extracting = { $what } نکالنا کالعدم کریں

# Words on a job card while files are wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiping = { $what } محفوظ طریقے سے مٹائی جا رہی ہے

# Words under the file lines when files have been wiped securely.
# SAFETY: needs a human review before it ships
folders-act-wiped = { $what } محفوظ طریقے سے مٹا دی گئی۔

# The word for what was tried, put in 'could not be ...' when wiping failed.
# SAFETY: needs a human review before it ships
folders-act-verb-wiped = محفوظ طریقے سے مٹانا

# Words on a job card while things are taken out of an archive.
folders-act-taking-out = { $what } کو آرکائیو سے نکالا جا رہا ہے

# Words under the file lines when things were taken out of an archive: what, and where to.
folders-act-took-out = { $what } کو آرکائیو سے نکال کر { $place } میں رکھ دیا گیا۔

# The word for what was tried, put in 'could not be ...' when taking things out of an archive failed.
folders-act-verb-taken-out = آرکائیو سے نکالنا

# Tip of the Undo button after things were taken out of an archive.
folders-act-undo-taking-out = { $what } کو نکالنا کالعدم کریں

# Words on a job card while pictures are turned.
folders-act-turning = { $what } کو گھمایا جا رہا ہے

# Words under the file lines when pictures have been turned.
folders-act-turned = { $what } کو گھما دیا گیا۔

# The word for what was tried, put in 'could not be ...' when turning failed.
folders-act-verb-turned = گھمانا

# Words on a job card while pictures are resized.
folders-act-resizing = { $what } کا سائز بدلا جا رہا ہے

# Words under the file lines when a resized copy of a picture was made.
folders-act-resized = { $what } کی نئے سائز کی نقل بنا دی گئی۔

# The word for what was tried, put in 'could not be ...' when resizing failed.
folders-act-verb-resized = سائز بدلنا

# Tip of the Undo button after a picture was resized.
folders-act-undo-resizing = { $what } کا سائز بدلنا کالعدم کریں

# Name given to a folder made with the New folder button; a number follows if the name is taken.
folders-act-new-folder = نیا فولڈر

# Name given to an empty document made with the New document button; a number follows if the name is taken.
folders-act-new-document = نئی دستاویز

# Red words when a new folder or document could not be made; its name, then the reason.
folders-act-make-failed = Folders { $base } نہیں بنا سکا: { $why }۔

# Red words under a name being typed when it is empty.
folders-act-name-empty = نام لکھیں۔

# Red words under a name being typed when it has a slash.
folders-act-name-slash = نام میں سلیش نہیں ہو سکتا۔

# Red words under a name being typed when it is . or ..
folders-act-name-kept = یہ نام نظام استعمال کرتا ہے۔ کوئی اور نام چنیں۔

# Red words under a name being typed when it is too long.
folders-act-name-long = یہ نام بہت لمبا ہے۔

# Red words under the file lines when a piece of work could not be started.
folders-act-no-start = Folders یہ کام شروع نہیں کر سکا۔

# Tip of the Redo button after a step was undone; what the step was. Every undo and redo tip begins the same way.
folders-act-redo-label = { $what } دوبارہ کریں

# Tip of the Undo button after a step was redone; what the step was. Every undo and redo tip begins the same way.
folders-act-undo-label = { $what } کالعدم کریں

# Words under the file lines when work was stopped before it was done.
folders-act-stopped = کام ختم ہونے سے پہلے روک دیا گیا۔

# Words under the file lines when work finished but some things were skipped; what was done, then how many were skipped.
folders-act-skipped = { $done } { $n ->
        [one] Folders نے ایک چیز چھوڑ دی۔
       *[other] Folders نے { $n } چیزیں چھوڑ دیں۔
    }

# Words under the file lines when an action needs something picked and nothing is.
folders-act-pick-first = پہلے کچھ منتخب کریں۔

# Words under the file lines when something already in the Trash is put there again.
folders-act-in-trash = یہ پہلے ہی ردی دان میں ہے۔

# Words under the file lines when a server's files are put in the Trash.
# SAFETY: needs a human review before it ships
folders-act-far-trash = سرور کی فائلیں ردی دان میں نہیں جا سکتیں۔ اس کی بجائے ہمیشہ کے لیے مٹانا استعمال کریں۔

# Words under the file lines when the copy or move button is pressed with nothing picked.
folders-act-pick-copy-move = پہلے نقل یا منتقل کرنے کے لیے کچھ منتخب کریں۔

# Words under the file lines when things from an archive are sent anywhere but a folder.
folders-act-archive-out = آرکائیو سے چیزیں صرف کسی فولڈر میں نکالی جا سکتی ہیں۔

# Words under the file lines when a copy is sent to the Trash.
folders-act-copy-trash = آپ چیزوں کی نقل ردی دان میں نہیں بنا سکتے۔

# Words under the file lines when the copy or move button finds the other pane showing no folder.
folders-act-other-pane = دوسرا حصہ کوئی فولڈر نہیں دکھا رہا۔

# Words under the file lines when Copy is used with nothing picked.
folders-act-pick-copy = پہلے نقل کرنے کے لیے کچھ منتخب کریں۔

# Words under the file lines after Cut: what was cut.
folders-act-cut-said = { $what } کاٹ لی گئی۔

# Words under the file lines after Copy: what was copied.
folders-act-copied-said = { $what } نقل کر لی گئی۔

# Words under the file lines when Paste is used where there is no folder.
folders-act-paste-folder = چیزیں صرف فولڈر میں چسپاں کی جا سکتی ہیں۔

# Words under the file lines when Paste finds no files on the clipboard.
folders-act-no-clip = کلپ بورڈ پر کوئی فائل نہیں ہے۔

# Words under the file lines when Paste is used and nothing was cut or copied.
folders-act-nothing-cut = کچھ بھی کاٹا یا نقل نہیں کیا گیا۔

# Words under the file lines when Delete is used on the Network list.
# SAFETY: needs a human review before it ships
folders-act-network-delete = نیٹ ورک کے مقامات یہاں سے مٹائے نہیں جا سکتے۔

# Words under the file lines when Empty the Trash is used on an empty Trash.
folders-act-trash-empty = ردی دان پہلے ہی خالی ہے۔

# Words under the file lines when Put back is used with nothing picked in the Trash.
folders-act-pick-trash = پہلے ردی دان میں کچھ منتخب کریں۔

# Name given to a second, third and later new folder or document on a server: the usual name, then a number.
folders-act-new-numbered = { $base } { $n }

# Words under the file lines while a new folder or document is made on a server.
folders-act-making = { $name } بنایا جا رہا ہے

# Words under the file lines when New folder or New document is used where there is no folder.
folders-act-make-folder-only = نیا فولڈر یا دستاویز صرف کسی فولڈر کے اندر بنایا جا سکتا ہے۔

# Tip of the Undo button after a folder, document or link was made.
folders-act-undo-making = { $name } بنانا کالعدم کریں

# Red words under the file lines when folder colours could not follow things that were moved.
folders-act-colours-lost = Folders منتقل کی گئی چیزوں پر فولڈر کے رنگ برقرار نہیں رکھ سکا۔

# Words under the file lines when Redo is used and there is nothing to redo.
folders-act-nothing-redo = دوبارہ کرنے کے لیے کچھ نہیں۔

# Words under the file lines when Undo is used and there is nothing to undo.
folders-act-nothing-undo = کالعدم کرنے کے لیے کچھ نہیں۔

# Words on a job card while a step is done again; what the step was.
folders-act-redoing = { $what } دوبارہ ہو رہا ہے

# Words under the file lines when a step was done again; what the step was.
folders-act-redid = { $what } دوبارہ کر دیا گیا۔

# The word for what was tried, put in 'could not be ...' when redoing failed.
folders-act-verb-done-again = دوبارہ کرنا

# Words on a job card while a step is undone; what the step was.
folders-act-undoing = { $what } کالعدم ہو رہا ہے

# Words under the file lines when a step was undone; what the step was.
folders-act-undid = { $what } کالعدم کر دیا گیا۔

# Name of the home folder among the places offered when things are dropped on the sidebar.
folders-act-place-home = ہوم

# Name of the Trash among the places offered when things are dropped on the sidebar.
folders-act-place-trash = ردی دان

# Words under the file lines when things are dropped somewhere that takes nothing.
folders-act-drop-only = چیزیں صرف فولڈر یا ردی دان پر چھوڑی جا سکتی ہیں۔

# Names how many links were made, inside the words about making them.
folders-act-links =
    { $n ->
        [one] 1 لنک
       *[other] { $n } لنک
    }

# Words under the file lines after links were made; the link's name, or how many.
folders-act-made = { $what } بنا دیا گیا۔

# Red words under the file lines when links could not be made: how many, and the first few names.
folders-act-links-failed =
    { $n ->
        [one] ایک لنک بنایا نہیں جا سکا: { $names }۔
       *[other] { $n } لنک بنائے نہیں جا سکے: { $names }۔
    }

# Words under the file lines when the window is closed while work waits for an answer.
folders-act-closing-waits = ایک سوال آپ کے جواب کا انتظار کر رہا ہے۔ Folders بند کرنے سے پہلے جواب دیں، یا کام روک دیں۔

# Words under the file lines when the window is closed while work goes on.
folders-act-closing-busy = جب تک کوئی کام چل رہا ہو Folders بند نہیں ہو سکتا۔ اس کے ختم ہونے کا انتظار کریں، یا اسے روک دیں، پھر Folders بند کریں۔
