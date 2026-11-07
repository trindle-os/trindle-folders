

folders-shares-nfs-denied = { $server } इस कंप्यूटर को { $export } खोलने नहीं देता।

folders-shares-nfs-no-share = { $server } पर { $export } नाम का कुछ साझा नहीं है।

folders-shares-left-none = अगली बार खोलने पर फिर पूछेगा

folders-shares-left-under-day = एक दिन से कम बचा है

folders-shares-left-days =
    { $n ->
        [one] { $n } दिन बचा है
       *[other] { $n } दिन बचे हैं
    }

folders-shares-held =
    { $past ->
        [yes] { $server } अब { $now } पर जवाब देता है, जो उस पते से अलग है जिस पर इसके लिए आपका लॉगिन पहले इस्तेमाल हुआ था ({ $before })।
       *[no] { $server } { $now } पर जवाब देता है, ऐसा पता जिस पर इसके लिए आपका लॉगिन पहले कभी इस्तेमाल नहीं हुआ।
    }{ $full ->
        [yes] {" "}इसके पिछले { $most } पते ही याद रखे जाते हैं।
       *[no] {""}
    } नेटवर्क पर कोई भी उपकरण किसी नाम पर जवाब दे सकता है, इसलिए इसके लिए सहेजा गया लॉगिन भेजा नहीं गया है। अगर यह आपका { $server } है, तो यहाँ इसे इस्तेमाल करने के लिए इसका उपयोगकर्ता नाम और पासवर्ड लिखें।

folders-shares-nowhere = { $server } जिस पते पर जवाब देता है वह पहले से नहीं मिल सका, इसलिए उसे कुछ भेजा नहीं गया है और यहाँ लिखी जानकारी के सुरक्षित रूप से जाने की कोई जगह नहीं है। इसे इसके पते से खोलें, जैसे smb://192.168.1.20/।

folders-shares-lapsed = { $server } { $now } पर जवाब देता है, जहाँ इसके लिए आपका लॉगिन आख़िरी बार { $days } दिन से भी पहले इस्तेमाल हुआ था। तब से यह पता किसी दूसरे उपकरण को दिया गया हो सकता है, इसलिए इसके लिए सहेजा गया लॉगिन भेजा नहीं गया है। अगर यह आपका { $server } है, तो यहाँ इसे इस्तेमाल करने के लिए इसका उपयोगकर्ता नाम और पासवर्ड लिखें।

folders-shares-unknown = { $server } { $now } पर जवाब देता है, जहाँ इसके लिए कोई लॉगिन पहले इस्तेमाल हुआ था, पर कब या कौन-सा लॉगिन, यह दर्ज नहीं किया गया था। तब से यह पता किसी दूसरे उपकरण को दिया गया हो सकता है, इसलिए इसके लिए सहेजा गया लॉगिन भेजा नहीं गया है। अगर यह आपका { $server } है, तो यहाँ इसे इस्तेमाल करने के लिए इसका उपयोगकर्ता नाम और पासवर्ड लिखें।

folders-shares-unshown = { $server } { $now } पर जवाब देता है, वही पता जिस पर इसके लिए आपका लॉगिन पहले गया था। इस बार यह पता केवल नेटवर्क पर सुना गया, जहाँ कोई भी उपकरण किसी नाम पर जवाब दे सकता है, और यह पता आपको यहाँ पहले कभी दिखाया नहीं गया, इसलिए लॉगिन भेजा नहीं गया है। अगर यह आपका { $server } है, तो यहाँ इसे इस्तेमाल करने के लिए इसका उपयोगकर्ता नाम और पासवर्ड लिखें।

folders-shares-asking =
    { $server } को उपयोगकर्ता नाम और पासवर्ड चाहिए। ये उस कंप्यूटर पर किसी खाते के होने चाहिए।{ $heard ->
        [yes] {" "}यह { $at } पर जवाब देता है। नेटवर्क पर कोई भी उपकरण किसी नाम पर जवाब दे सकता है, इसलिए लिखने से पहले जाँच लें कि यही इसका पता है।
       *[no] {""}
    }

folders-shares-where =
    आप जो लिखते हैं वह { $address } को जाता है{ $heard ->
        [yes] {" "}और कहीं नहीं। "याद रखें" चालू होने पर, आपका लॉगिन केवल उसी पते के लिए सहेजा जाता है।
       *[no] {"।"}
    }

folders-shares-again = { $server } { $at } पर, पोर्ट { $port } पर जवाब देता है। यह पता केवल नेटवर्क पर सुना गया था, जहाँ कोई भी उपकरण किसी नाम पर जवाब दे सकता है, इसलिए इसके लिए आपका लॉगिन भेजा नहीं गया है। अगर यही इसका पता है, तो "खोलें" दबाएँ।

folders-shares-keyring-unreachable = कीरिंग तक नहीं पहुँचा जा सका: { $why }।

folders-shares-unkept = यह खुल गया है, पर लॉगिन सहेजा नहीं गया। { $why }

folders-shares-nfs-version = { $server } NFS संस्करण 4 नहीं देता, जो Trindle द्वारा खोला जाने वाला एकमात्र प्रकार है।

folders-shares-nfs-opened-unknown = यह खुल गया, पर यह कहाँ है, पता नहीं।

folders-shares-opened-unknown = साझा फ़ोल्डर खुल गया, पर यह कहाँ है, पता नहीं।

folders-shares-not-address = यह कोई पता नहीं है।

folders-shares-seen-label = वे पते जिन पर smb://{ $server } खोला गया

folders-shares-password-label = { $what } पर { $user } का पासवर्ड

folders-shares-remember = 90 दिन तक याद रखें

folders-shares-remember-line = हर बार इस्तेमाल करने पर फिर से शुरू होता है।

folders-shares-share-on = { $server } पर { $share }

folders-shares-card-title = { $it } खोलें

folders-shares-close = बंद करें

folders-shares-user-placeholder = उपयोगकर्ता नाम, जैसे jo, या WORKGROUP\jo

folders-shares-password = पासवर्ड

folders-shares-forget = पहले के पते भूल जाएँ

folders-shares-forget-spoken = वे पते भूल जाएँ जिन पर यह लॉगिन पहले इस्तेमाल हुआ था

folders-shares-opening = खोल रहे हैं

folders-shares-open = खोलें

folders-shares-bad-computer = यह किसी कंप्यूटर का नाम नहीं है।

folders-shares-bad-path = यह किसी साझा फ़ोल्डर का पथ नहीं है।

folders-shares-opening-export = { $host } पर { $export } खोल रहे हैं…

folders-shares-opening-share = { $server } पर { $share } खोल रहे हैं…

folders-shares-answer-card = पहले { $it } का कार्ड भरें।

folders-shares-open-waiting = यह खुला है। { $words }

folders-shares-none = { $server } आपके साथ कोई फ़ोल्डर साझा नहीं करता।

folders-shares-closing = { $name } बंद कर रहे हैं…

folders-shares-close-failed = { $name } बंद नहीं हो सका: { $why }।

folders-shares-reconnecting = { $name } फिर से जुड़ रहा है…

folders-shares-not-remembered = शेयर अगली बार के लिए याद नहीं रखे जा सके।

