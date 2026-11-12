
# machine translated (Hindi), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = इस कंप्यूटर को { $server } पर { $export } खोलने की अनुमति नहीं है।

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } पर { $export } नाम का कोई फ़ोल्डर साझा नहीं है।

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = अगली बार आपसे फिर पूछा जाएगा

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = एक दिन से कम बचा है

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] { $n } दिन बचा है
       *[other] { $n } दिन बचे हैं
    }

# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
# SAFETY: needs a human review before it ships
folders-shares-held =
    { $past ->
        [yes] { $server } अब { $now } पर है। इसके लिए आपका लॉगिन पहले किसी दूसरे पते पर इस्तेमाल हुआ था ({ $before })।
       *[no] { $server } { $now } पर है। इसके लिए आपका लॉगिन इस पते पर पहले कभी इस्तेमाल नहीं हुआ।
    }{ $full ->
        [yes] {" "}Folders केवल पिछले { $most } पते याद रखता है।
       *[no] {""}
    } नेटवर्क पर कोई भी उपकरण कोई नाम अपना बता सकता है, इसलिए Folders ने आपका सहेजा हुआ लॉगिन नहीं भेजा है। अगर यह आपका { $server } है, तो यहाँ इसे इस्तेमाल करने के लिए इसका उपयोगकर्ता नाम और पासवर्ड लिखें।

# Words on the sign-in card when no address could be found for the computer to show.
# SAFETY: needs a human review before it ships
folders-shares-nowhere = Folders को { $server } का पता नहीं मिला, इसलिए उसे कुछ नहीं भेजा गया है। यहाँ आप जो भी लिखेंगे, वह सुरक्षित रूप से भेजा नहीं जा सकता। इसके बजाय इसे इसके पते से खोलें, जैसे smb://192.168.1.20/।

# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
# SAFETY: needs a human review before it ships
folders-shares-lapsed = { $server } { $now } पर है। इसके लिए आपका लॉगिन वहाँ आख़िरी बार { $days } दिन से भी पहले इस्तेमाल हुआ था। तब से यह पता किसी दूसरे उपकरण को दिया गया हो सकता है, इसलिए Folders ने आपका सहेजा हुआ लॉगिन नहीं भेजा है। अगर यह आपका { $server } है, तो यहाँ इसे इस्तेमाल करने के लिए इसका उपयोगकर्ता नाम और पासवर्ड लिखें।

# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
# SAFETY: needs a human review before it ships
folders-shares-unknown = { $server } { $now } पर है। इसके लिए कोई लॉगिन वहाँ पहले इस्तेमाल हुआ था, पर Folders ने यह दर्ज नहीं किया कि कब, या कौन-सा लॉगिन। तब से यह पता किसी दूसरे उपकरण को दिया गया हो सकता है, इसलिए Folders ने आपका सहेजा हुआ लॉगिन नहीं भेजा है। अगर यह आपका { $server } है, तो यहाँ इसे इस्तेमाल करने के लिए इसका उपयोगकर्ता नाम और पासवर्ड लिखें।

# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
# SAFETY: needs a human review before it ships
folders-shares-unshown = { $server } { $now } पर है, जहाँ इसके लिए आपका लॉगिन पहले इस्तेमाल हुआ था। इस बार Folders ने इसे स्थानीय नेटवर्क पर पाया, जहाँ कोई भी उपकरण यह नाम इस्तेमाल कर सकता है। यह पता आपने पहले नहीं देखा है, इसलिए Folders ने आपका लॉगिन नहीं भेजा है। अगर यह आपका { $server } है, तो यहाँ इसे इस्तेमाल करने के लिए इसका उपयोगकर्ता नाम और पासवर्ड लिखें।

# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
# SAFETY: needs a human review before it ships
folders-shares-asking =
    { $server } पर किसी खाते का उपयोगकर्ता नाम और पासवर्ड लिखें।{ $heard ->
        [yes] {" "}यह { $at } पर है। नेटवर्क पर कोई भी उपकरण कोई नाम अपना बता सकता है, इसलिए लिखने से पहले जाँच लें कि यही इसका पता है।
       *[no] {""}
    }

# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
# SAFETY: needs a human review before it ships
folders-shares-where =
    आप जो लिखते हैं वह { $address } को जाता है{ $heard ->
        [yes] {" "}और कहीं नहीं। "याद रखें" चालू होने पर, Folders आपका लॉगिन केवल इसी पते के लिए सहेजता है।
       *[no] {"।"}
    }

# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
# SAFETY: needs a human review before it ships
folders-shares-again = Folders को स्थानीय नेटवर्क पर { $at } पर { $server } मिला, पोर्ट { $port }। वहाँ कोई भी उपकरण यह नाम इस्तेमाल कर सकता है, इसलिए Folders ने आपका लॉगिन नहीं भेजा है। अगर यही सही पता है, तो "खोलें" दबाएँ।

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders कीरिंग तक नहीं पहुँच सका: { $why }।

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = यह खुल गया है, पर Folders लॉगिन सहेज नहीं सका। { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } NFS संस्करण 4 नहीं देता, जो एकमात्र संस्करण है जिसे Folders खोल सकता है।

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = शेयर खुल गया, पर Folders यह नहीं ढूँढ सका कि वह इस कंप्यूटर पर कहाँ है।

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = शेयर खुल गया, पर Folders यह नहीं ढूँढ सका कि वह इस कंप्यूटर पर कहाँ है।

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = यह कोई पता नहीं है।

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = smb://{ $server } के लिए इस्तेमाल किए गए पते

# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
# SAFETY: needs a human review before it ships
folders-shares-password-label = { $what } पर { $user } का पासवर्ड

# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
# SAFETY: needs a human review before it ships
folders-shares-remember = 90 दिन तक याद रखें

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = हर बार इस्तेमाल करने पर 90 दिन फिर से शुरू हो जाते हैं।

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $server } पर { $share }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = { $it } खोलें

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = बंद करें

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = उपयोगकर्ता नाम, जैसे jo या WORKGROUP\jo

# Hint in the empty password box of the sign-in card.
# SAFETY: needs a human review before it ships
folders-shares-password = पासवर्ड

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
# SAFETY: needs a human review before it ships
folders-shares-forget = पहले के पते भूल जाएँ

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
# SAFETY: needs a human review before it ships
folders-shares-forget-spoken = इस लॉगिन के पहले के पते भूल जाएँ

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = खोल रहे हैं…

# Name of the main button of the sign-in card.
folders-shares-open = खोलें

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = यह कंप्यूटर का नाम नहीं है।

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = यह किसी साझा फ़ोल्डर का पथ नहीं है।

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = { $host } पर { $export } खोल रहे हैं…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = { $server } पर { $share } खोल रहे हैं…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = पहले { $it } में लॉगिन पूरा करें।

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = यह खुला है। { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } आपके साथ कोई फ़ोल्डर साझा नहीं करता।

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = { $name } बंद कर रहे हैं…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders { $name } बंद नहीं कर सका: { $why }।



# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = { $name } फिर से जुड़ रहा है…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders आपके शेयर अगली बार के लिए सहेज नहीं सका।

# Red words when a share's user name cannot be used.
folders-shares-bad-user = यह उपयोगकर्ता नाम शेयर के लिए इस्तेमाल नहीं हो सकता।

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = यह डोमेन शेयर के लिए इस्तेमाल नहीं हो सकता।

# Red words when a share's password holds a line break.
folders-shares-bad-password = शेयर के पासवर्ड में नई पंक्ति नहीं हो सकती।

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } को { $it } के लिए उपयोगकर्ता नाम और पासवर्ड चाहिए।

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = { $it } के लिए उपयोगकर्ता नाम या पासवर्ड गलत है।

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = { $server } पर आपका खाता बंद है या रोका गया है, या उसका पासवर्ड समाप्त हो गया है।

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } पर { $share } नाम का कोई साझा फ़ोल्डर नहीं है।

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = उस खाते को { $it } खोलने की अनुमति नहीं है।

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } ने समय पर जवाब नहीं दिया। वह सो रहा हो सकता है या बंद हो सकता है। उसे जगाएँ, फिर कोशिश करें।

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders को इस नेटवर्क पर { $server } नाम का कोई कंप्यूटर नहीं मिला।

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } केवल SMB1 इस्तेमाल करता है, जो साझा करने का पुराना तरीका है और अब सुरक्षित नहीं है। Folders उसे इस्तेमाल नहीं करता।

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = इसके लिए Debian का { $package } पैकेज चाहिए, जो इंस्टॉल नहीं है।

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders शेयर नहीं खोल सका।

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders साझा फ़ोल्डर नहीं खोल सका।

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders उसके साझा फ़ोल्डरों की सूची नहीं बना सका।
