# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = आपने प्रशासक का पासवर्ड नहीं दिया
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = प्रशासक का पासवर्ड स्वीकार नहीं किया गया
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = आपके खाते को यह करने की अनुमति नहीं है, प्रशासक का पासवर्ड देने पर भी
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = आपसे प्रशासक का पासवर्ड माँगने का कोई तरीका नहीं था
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = प्रशासक का पासवर्ड स्वीकार नहीं किया गया, या आपके खाते को यह करने की अनुमति नहीं है
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = आपने प्रशासक का पासवर्ड नहीं दिया
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = प्रशासक का पासवर्ड स्वीकार नहीं किया गया
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = आपके खाते को यह करने की अनुमति नहीं है, प्रशासक का पासवर्ड देने पर भी
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = आपसे प्रशासक का पासवर्ड माँगने का कोई तरीका नहीं था
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = प्रशासक का पासवर्ड स्वीकार नहीं किया गया, या आपके खाते को यह करने की अनुमति नहीं है
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = यह इसलिए नहीं हुआ क्योंकि आपने प्रशासक का पासवर्ड नहीं दिया।
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = यह इसलिए नहीं हुआ क्योंकि प्रशासक का पासवर्ड स्वीकार नहीं किया गया।
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = यह नहीं हुआ। आपके खाते को यह करने की अनुमति नहीं है, प्रशासक का पासवर्ड देने पर भी। लॉग आउट करके फिर लॉग इन करें, फिर दोबारा कोशिश करें।
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = यह नहीं हुआ। आपसे प्रशासक का पासवर्ड माँगने का कोई तरीका नहीं था। लॉग आउट करके फिर लॉग इन करें, फिर दोबारा कोशिश करें।
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = यह नहीं हुआ। प्रशासक का पासवर्ड स्वीकार नहीं किया गया, या आपके खाते को यह करने की अनुमति नहीं है। अगर आपसे पासवर्ड नहीं माँगा गया, तो लॉग आउट करके फिर लॉग इन करें, फिर दोबारा कोशिश करें।
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = यह नहीं हुआ, और कोई कारण नहीं बताया गया।
# Red words when the keyring will not open without the person's password.
common-keyring-locked = कीरिंग बंद है। उसे खोलें, या लॉग आउट करके फिर लॉग इन करें, फिर दोबारा कोशिश करें।
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = इस कंप्यूटर पर इसे सहेजने के लिए कोई कीरिंग नहीं है।
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = Folders समय पर कीरिंग तक नहीं पहुँच सका।
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = Folders ने कीरिंग का इंतज़ार करना छोड़ दिया।
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Folders कीरिंग तक पहुँच नहीं पा सका ({ $why })।
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] Folders अभी कीरिंग का इंतज़ार कर रहा है, इसलिए { $what } अभी हटाया नहीं गया है।
       *[other] Folders अभी कीरिंग का इंतज़ार कर रहा है, इसलिए { $what } अभी सहेजा नहीं गया है।
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] देरी के बाद, { $what } कीरिंग से हटा दिया गया।
       *[other] देरी के बाद, { $what } कीरिंग में सहेज लिया गया।
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] Folders ने कीरिंग का इंतज़ार करना छोड़ दिया, इसलिए { $what } हटाया नहीं गया।
       *[other] Folders ने कीरिंग का इंतज़ार करना छोड़ दिया, इसलिए { $what } सहेजा नहीं गया।
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] Folders { $what } को कीरिंग से हटा नहीं सका। { $why }
       *[other] Folders { $what } को कीरिंग में सहेज नहीं सका। { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = और
common-more-spoken = इसके बारे में सहायता खोलें
# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
folders-volumes-no-password = आपने पासवर्ड नहीं दिया, इसलिए कुछ नहीं बदला गया।
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, इसलिए कुछ नहीं बदला गया।
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = अब भी कोई चीज़ इसे इस्तेमाल कर रही है। इसकी फ़ाइलें दिखाने वाली विंडो या प्रोग्राम बंद करें, फिर कोशिश करें।
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = इस आकार पर इसमें जो कुछ है उसके लिए पर्याप्त जगह नहीं है।
# SAFETY: needs a human review before it ships
# Heading over the password box, and the tip shown in the box while it is empty.
folders-prompt-password = पासवर्ड
# SAFETY: needs a human review before it ships
# Main button at the bottom of the password window that checks the password.
folders-prompt-unlock = अनलॉक करें
# SAFETY: needs a human review before it ships
# Button at the bottom of the password window that closes it without unlocking.
folders-prompt-cancel = रद्द करें
# SAFETY: needs a human review before it ships
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = पहले पासवर्ड लिखें।
# Shown under the password box while the password is being checked.
folders-prompt-checking = जाँच हो रही है…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = 4K से बड़ी
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = आपकी स्क्रीन से बड़ी
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = 20 MB से ऊपर (एनिमेटेड हो तो 200 MB से ऊपर)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = 20 MB से ऊपर
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = 200 MB से ऊपर
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } या { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = बंद: { $name }

# Name given to a drive that has none.
folders-volumes-drive = ड्राइव

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = उस पासफ़्रेज़ से ड्राइव अनलॉक नहीं हुई।

# Said when a drive that is open already is opened again.
folders-volumes-already-open = यह पहले से खुली है।

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = इसे खोलने के लिए आपको पासफ़्रेज़ चाहिए।

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = { $name } का पासफ़्रेज़

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = ड्राइव खोलें

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = Folders पासफ़्रेज़ वाली विंडो नहीं खोल सका।
