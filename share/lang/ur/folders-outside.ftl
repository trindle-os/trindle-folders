
# machine translated (Urdu), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = چھپا ہوا ہے، یا چھپے فولڈر میں ہے

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = اس میں چھپی چیزیں ہیں

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = اس میں جانچنے کے لیے بہت زیادہ چیزیں ہیں، اس لیے Folders اس کی چھپی چیزیں چھوڑ دے گا

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = آپ کے ہوم فولڈر اور ڈرائیوز سے باہر ہے

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = کسی چھپی چیز کا لنک ہے، یا آپ کے ہوم فولڈر اور ڈرائیوز سے باہر ہے

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = اس میں کسی چھپی چیز کا لنک ہے، یا آپ کے ہوم فولڈر اور ڈرائیوز سے باہر ہے

# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [one] { $name } کو یہاں منتقل کریں؟
       *[other] یہ { $n } چیزیں یہاں منتقل کریں؟
    }

# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [one] { $name } کو یہاں لائیں؟
       *[other] یہ { $n } چیزیں یہاں لائیں؟
    }

# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [one] { $name } کی نقل یہاں بنائیں؟
       *[other] ان { $n } چیزوں کی نقل یہاں بنائیں؟
    }

# Words on the question card when the drop would be moved: why it is asked.
folders-outside-body-move =
    { $n ->
        [one] کسی اور پروگرام نے اسے Folders پر گرایا ہے۔ کوئی بھی پروگرام کوئی بھی فائل گرا سکتا ہے، اور یہ آپ کی عام فائلوں میں سے نہیں ہے۔ جب تک آپ نہ کہیں Folders اسے منتقل نہیں کرے گا۔
       *[other] کسی اور پروگرام نے انہیں Folders پر گرایا ہے۔ کوئی بھی پروگرام کوئی بھی فائل گرا سکتا ہے، اور ان میں سے کچھ آپ کی عام فائلیں نہیں ہیں۔ جب تک آپ نہ کہیں Folders انہیں منتقل نہیں کرے گا۔
    }

# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
        [one] کسی اور پروگرام نے اسے Folders پر گرایا ہے۔ کوئی بھی پروگرام کوئی بھی فائل گرا سکتا ہے، اور یہ آپ کی عام فائلوں میں سے نہیں ہے۔ جب تک آپ نہ کہیں Folders اسے یہاں نہیں لائے گا۔
       *[other] کسی اور پروگرام نے انہیں Folders پر گرایا ہے۔ کوئی بھی پروگرام کوئی بھی فائل گرا سکتا ہے، اور ان میں سے کچھ آپ کی عام فائلیں نہیں ہیں۔ جب تک آپ نہ کہیں Folders انہیں یہاں نہیں لائے گا۔
    }

# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
        [one] کسی اور پروگرام نے اسے Folders پر گرایا ہے۔ کوئی بھی پروگرام کوئی بھی فائل گرا سکتا ہے، اور یہ آپ کی عام فائلوں میں سے نہیں ہے۔ جب تک آپ نہ کہیں Folders اس کی نقل نہیں بنائے گا۔
       *[other] کسی اور پروگرام نے انہیں Folders پر گرایا ہے۔ کوئی بھی پروگرام کوئی بھی فائل گرا سکتا ہے، اور ان میں سے کچھ آپ کی عام فائلیں نہیں ہیں۔ جب تک آپ نہ کہیں Folders ان کی نقل نہیں بنائے گا۔
    }

# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
        [one] اسے چھوڑ دیں
       *[other] انہیں چھوڑ دیں
    }

# Button on the question card that moves what was dropped.
folders-outside-do-move =
    { $n ->
        [one] اسے منتقل کریں
       *[other] انہیں منتقل کریں
    }

# Button on the question card that brings what was dropped.
folders-outside-do-bring =
    { $n ->
        [one] اسے لائیں
       *[other] انہیں لائیں
    }

# Button on the question card that copies what was dropped.
folders-outside-do-copy =
    { $n ->
        [one] اسے کاپی کریں
       *[other] انہیں کاپی کریں
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }، { $from } سے: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }، { $from } سے

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = اور { $n } مزید۔

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = { $into } میں۔

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = پہلے اس سوال کا جواب دیں جو آپ کی گرائی گئی چیزوں کے بارے میں ہے۔

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders ابھی آپ کی گرائی ہوئی آخری چیز کو جانچ رہا ہے۔

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = یہاں صرف فائلیں اور فولڈر گرائے جا سکتے ہیں۔

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = سائیڈ بار میں صرف فولڈر پن کیے جا سکتے ہیں۔

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = چیزیں صرف فولڈر پر گرائی جا سکتی ہیں۔

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } اب وہاں نہیں ہے، اس لیے کچھ نقل یا منتقل نہیں ہوا۔

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = آپ فولڈر کو اسی کے اندر نہیں رکھ سکتے۔

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = وہ پہلے سے اسی فولڈر میں ہیں۔

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders وقت پر اس ڈرائیو تک نہیں پہنچ سکا جس پر { $name } ہے، اس لیے کچھ نقل یا منتقل نہیں ہوا۔

# Words when the question card is answered with no.
folders-outside-nothing-brought = کچھ نقل یا منتقل نہیں ہوا۔

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] 1 چیز
       *[other] { $n } چیزیں
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = اور { $n } مزید

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }۔

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] { $n } چیز گھسیٹنا شروع کرنے کے بعد بدل گئی، اس لیے Folders نے اسے وہیں چھوڑ دیا جہاں وہ تھی: { $names }۔
       *[other] { $n } چیزیں گھسیٹنا شروع کرنے کے بعد بدل گئیں، اس لیے Folders نے انہیں وہیں چھوڑ دیا جہاں وہ تھیں: { $names }۔
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] { $n } چیز اب بھی وہیں ہے جہاں تھی: { $each }۔
       *[other] { $n } چیزیں اب بھی وہیں ہیں جہاں تھیں: { $each }۔
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders نہیں جان سکتا کہ دوسرے پروگرام کو آپ کی گھسیٹی ہوئی سب چیزیں ملیں یا نہیں، اس لیے اس نے اصل چیزیں یہیں رہنے دیں۔ دوسرے پروگرام کے پاس نقل ہے۔

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = دوسرے پروگرام نے آپ کی گھسیٹی ہوئی سب چیزیں نہیں لیں، اس لیے Folders نے کچھ منتقل نہیں کیا۔

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = دوسرے پروگرام نے آپ کی گھسیٹی ہوئی سب چیزیں نہیں لیں، اس لیے اس کی نقل ادھوری ہو سکتی ہے۔


