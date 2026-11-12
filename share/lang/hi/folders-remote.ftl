
# machine translated (Hindi), not yet reviewed by a person.

# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-card = Folders को स्थानीय नेटवर्क पर { $at } पर { $host } मिला, पोर्ट { $port }। वहाँ कोई भी उपकरण यह नाम इस्तेमाल कर सकता है, इसलिए Folders ने आपका पासवर्ड नहीं भेजा है। अगर यही सही पता है, तो जोड़ें दबाएँ।

# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-stopped = Folders को स्थानीय नेटवर्क पर { $at } पर { $host } मिला। वहाँ कोई भी उपकरण यह नाम इस्तेमाल कर सकता है, इसलिए Folders ने आपका पासवर्ड नहीं भेजा है। उसका पता जाँचने के लिए { $host } को फिर खोलें।

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders वह पता पढ़ नहीं सका: { $why }।

# Red words when a server needs a password or key and none could be asked for.
# SAFETY: needs a human review before it ships
folders-remote-mod-needs-password = इस सर्वर को पासवर्ड या कुंजी चाहिए।

# Red words when a server turned down the login.
# SAFETY: needs a human review before it ships
folders-remote-mod-login-refused = सर्वर ने वह लॉगिन स्वीकार नहीं किया।

# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-unknown = आप पहले कभी { $host } से नहीं जुड़े, इसलिए Folders अभी उसकी पहचान नहीं जानता।

# Red words when a server's identity differs from the one remembered; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-changed = { $host } की पहचान पिछली बार से अलग है। हो सकता है उसे दोबारा बनाया गया हो, या कोई कनेक्शन के बीच में दखल दे रहा हो।

# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-unknown = Folders { $host } का प्रमाणपत्र जाँच नहीं सका: { $why }।

# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-changed = जब से आपने { $host } के प्रमाणपत्र पर भरोसा किया, तब से वह बदल गया है।

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders जुड़ नहीं सका: { $why }।

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders को { $what } नहीं मिल रहा।

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } पहले से मौजूद है।

# Red words when a server or phone refused permission; the variable is what it was refused for.
# SAFETY: needs a human review before it ships
folders-remote-mod-denied = आपको { $what } इस्तेमाल करने की अनुमति नहीं है।

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } किसी दूसरे प्रोग्राम के इस्तेमाल में है।

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } समर्थित नहीं है।

# Red words when a copy or other work with a server was cancelled.
# SAFETY: needs a human review before it ships
folders-remote-mod-cancelled = रद्द किया गया।

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = कड़ियाँ खोलना

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = खोलने से पहले Windows शेयर पर या शेयर से कॉपी करना

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = खोलने से पहले NFS शेयर पर या शेयर से कॉपी करना

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = यह sftp://, ftp:// या mtp:// से शुरू नहीं होता

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = यह sftp://, ftp://, ftps://, ftpes://, dav://, davs://, smb://, nfs:// या mtp:// से शुरू होना चाहिए

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = [ के साथ ] नहीं है

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = इसमें सर्वर का नाम नहीं है

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = सर्वर का नाम मान्य नहीं है

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = पथ में पंक्ति विराम है

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = सर्वर के नाम के बाद कुछ अतिरिक्त है

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } मान्य पोर्ट नंबर नहीं है

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = छोड़ा गया, क्योंकि सर्वर ने असुरक्षित नाम भेजा

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = बिना नाम की चीज़

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = जवाब बहुत ही बड़ा था, इसलिए Folders ने उसे रोक दिया।

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } अब फ़ोल्डर है, इसलिए Folders ने उसे नहीं हटाया।
