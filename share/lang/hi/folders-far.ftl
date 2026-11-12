# machine translated (Hindi), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = { $host } नाम का कोई सर्वर नहीं है

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = पूरी फ़ाइल नहीं पहुँची, इसलिए मूल फ़ाइल अपनी जगह से हटी नहीं

# Reason a link was not sent to a server.
folders-far-not-followed = यह एक लिंक है, इसलिए Folders ने इसे नहीं भेजा

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = यह फ़ाइल या फ़ोल्डर नहीं है, इसलिए Folders ने इसे नहीं भेजा

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders पता नहीं लगा सका कि यह किस तरह की चीज़ है, इसलिए उसने इसे वहीं छोड़ दिया

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = इस तरह के सर्वर पर एक फ़ाइल उसी नाम की दूसरी फ़ाइल की जगह ले सकती है।

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = सर्वर की फ़ाइलों के साथ आप यह नहीं कर सकते

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders इसे पढ़ नहीं सका

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders इसे खोल नहीं सका।

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (फ़ोल्डर अपने भीतर नहीं जा सकता)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (इस नाम का फ़ोल्डर पहले से है)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (इस नाम की फ़ाइल पहले से है)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders को सर्वर का पता मालूम नहीं है।

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = भेजी गई प्रति का आकार ग़लत था

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = यह सिर्फ़ सर्वर की फ़ाइलों के साथ काम करता है।

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
# SAFETY: needs a human review before it ships
folders-far-aside-both = Folders नई फ़ाइल जगह पर नहीं रख सका, इसलिए पुरानी फ़ाइल अभी भी वहीं है, जिसका नाम अब { $old } है, और नई फ़ाइल का नाम { $new } है: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
# SAFETY: needs a human review before it ships
folders-far-aside-kept = Folders पुरानी फ़ाइल हटा नहीं सका। वह अभी भी वहीं है, जिसका नाम अब { $old } है

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
# SAFETY: needs a human review before it ships
folders-far-aside-kept-why = Folders पुरानी फ़ाइल हटा नहीं सका, इसलिए वह अभी भी वहीं है, जिसका नाम अब { $old } है: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } खाली नहीं है।

# Red note when a name sent by a server is unsafe to use and the file is skipped.
# SAFETY: needs a human review before it ships
folders-far-unsafe-name = Folders ने एक फ़ाइल छोड़ दी, क्योंकि उसका नाम इस्तेमाल करने के लिए सुरक्षित नहीं है।

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
# SAFETY: needs a human review before it ships
folders-far-unsafe-names =
    { $n ->
        [one] Folders ने एक चीज़ छोड़ दी, क्योंकि उसका नाम इस्तेमाल करने के लिए सुरक्षित नहीं है: { $names }।
       *[other] Folders ने { $n } चीज़ें छोड़ दीं, क्योंकि उनके नाम इस्तेमाल करने के लिए सुरक्षित नहीं हैं: { $names }।
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = { $name } डाउनलोड किया जा रहा है…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders { $name } डाउनलोड नहीं कर सका: { $why }।

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = { $name } की एक प्रति खोली गई। इसे सहेजने पर आपके बदलाव सर्वर पर वापस चले जाते हैं।

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = { $name } सर्वर पर सहेजी गई।

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders { $name } को सर्वर पर सहेज नहीं सका: { $why }। आपके बदलाव अभी भी इस कंप्यूटर की प्रति में हैं।

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } का नाम बदलकर { $now } किया गया।

# Heading of the Connect to a server card.
folders-far-connect-title = सर्वर से जुड़ें

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = बंद करें

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = पता, जैसे sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
# SAFETY: needs a human review before it ships
folders-far-connect-password = पासवर्ड, अगर ज़रूरी हो

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = कुंजी फ़ाइल, अगर आप इस्तेमाल करते हैं

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = कुंजी फ़ाइल चुनें


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders आपकी ssh सेटिंग्स, ssh एजेंट, कुंजियाँ और जाने-पहचाने सर्वरों की सूची इस्तेमाल करता है।

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = आप पहले कभी { $host } से नहीं जुड़े, इसलिए Folders जाँच नहीं सकता कि यही सही सर्वर है। इस पर तभी भरोसा करें जब आप इसी से जुड़ने की उम्मीद कर रहे थे।

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = { $host } की पहचान पिछली बार से अलग है। हो सकता है कोई आपके कनेक्शन को बीच में पकड़ने की कोशिश कर रहा हो, इसलिए Folders ने कनेक्ट नहीं किया। अगर स्वामी ने इसे बदला है, तो पहले अपने जाने-पहचाने होस्ट से इसकी पुरानी प्रविष्टि हटाएँ।

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-password = इस सर्वर को पासवर्ड चाहिए। इसे ऊपर लिखें और फिर कोशिश करें।

# Note on the Connect to a server card when a server asks for a key or password.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-key = इस सर्वर को कुंजी या पासवर्ड चाहिए। ऊपर अपनी कुंजी फ़ाइल चुनें, या उसे अपने ssh एजेंट में जोड़ें, फिर कोशिश करें।

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = Folders { $host } का प्रमाणपत्र जाँच नहीं सका: { $why }। हो सके तो उसकी फ़िंगरप्रिंट को स्वामी की दी हुई फ़िंगरप्रिंट से मिलाएँ। उस पर तभी भरोसा करें जब दोनों मेल खाएँ।

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = { $host } का प्रमाणपत्र उस समय से बदल गया है जब आपने उस पर भरोसा किया था। हो सकता है स्वामी ने उसे नया किया हो, या कोई आपके कनेक्शन को बीच में पकड़ने की कोशिश कर रहा हो। उस पर तभी भरोसा करें जब स्वामी इस फ़िंगरप्रिंट की पुष्टि करे।

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = जुड़ रहा है

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = भरोसा करें और जुड़ें

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = जुड़ें

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = फ़ोन प्लग इन और अनलॉक होते ही साइडबार में अपने आप दिख जाते हैं।

