# machine translated (Hindi), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = पुरानी प्रति में कुछ भी बदला नहीं जा सकता। रखने के लिए चीज़ें उसमें से बाहर कॉपी करें।

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] आज { $time } बजे
        [yesterday] कल { $time } बजे
       *[other] { $date } को { $time } बजे
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] आज { $time } बजे
        [yesterday] कल { $time } बजे
       *[other] { $date } को { $time } बजे
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = यह { $when } की एक पुरानी प्रति है। आप इसे बदल नहीं सकते, पर इसमें से चीज़ें कॉपी कर सकते हैं।

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = यह एक पुरानी प्रति है। आप इसे बदल नहीं सकते, पर इसमें से चीज़ें कॉपी कर सकते हैं।

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = पुरानी: { $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = { $name } को वापस रखना पूर्ववत करें

# Said under a version's row after it is put back and the file that was there is kept beside it.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-kept = Folders ने इसे वापस रख दिया। मौजूदा वाली प्रति नए नाम से उसके बगल में है।

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = Folders ने इसे उसी जगह वापस रख दिया।

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = यह मौजूदा प्रति जैसी ही है।

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = यह मौजूदा प्रति से अलग है।

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = Folders उनकी तुलना नहीं कर सका: { $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = पुराने संस्करण

# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = यहाँ से हटाई गई चीज़ें

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = { $name } के पुराने संस्करण

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = कोई भी पुरानी प्रति मौजूदा प्रति से अलग नहीं है।

# Note at the foot of the earlier versions card about what putting one back does.
# SAFETY: needs a human review before it ships
folders-earlier-versions-foot = किसी को वापस रखने पर मौजूदा प्रति उसके बगल में रहती है। आप इसे पूर्ववत कर सकते हैं।

# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = { $name } से हटाई गई चीज़ें

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = पुरानी प्रतियों में से कुछ भी इस फ़ोल्डर से मिटाया नहीं गया है।

# Note at the foot of the deleted things card about what putting one back does.
# SAFETY: needs a human review before it ships
folders-earlier-deleted-foot = किसी को वापस रखने पर वह इस फ़ोल्डर में लौट आता है। आप इसे पूर्ववत कर सकते हैं।

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = फ़ोल्डर जैसा वह तब था

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }, { $date } को बनाई गई प्रति से

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = एक फ़ोल्डर, आखिरी बार { $when } देखा गया

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }, आखिरी बार { $when } देखा गया

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = काम जारी है…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = उस दिन का फ़ोल्डर नए टैब में खोलें

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = खोलें

# Tip and name of the round button on a row of the earlier versions card that puts it back.
folders-earlier-put-back-tip = वापस रखें

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = मौजूदा प्रति से तुलना करें

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = यह पहले का संस्करण नहीं है।

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = आप पहले का संस्करण केवल अपने होम फ़ोल्डर के भीतर वापस रख सकते हैं।

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = Folders फ़ाइल को वहाँ वापस नहीं रख सकता।

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = Folders वह फ़ोल्डर दोबारा नहीं बना सका जिसमें वह था: { $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = Folders पहले का संस्करण कॉपी नहीं कर सका: { $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = Folders मौजूदा प्रति को अलग नहीं रख सका: { $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = Folders पहले का संस्करण वापस नहीं रख सका: { $why }। मौजूदा प्रति का नाम अब { $name } है।

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = Folders पहले का संस्करण वापस नहीं रख सका: { $why }
