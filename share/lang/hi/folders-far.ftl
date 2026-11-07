# machine translated (Hindi), not yet reviewed by a person.

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = { $host } नाम का कोई सर्वर नहीं मिला

folders-far-not-all-arrived = पूरी फ़ाइल नहीं पहुँची, इसलिए वह अपनी जगह पर ही है

folders-far-not-followed = एक लिंक, जिसका पीछा नहीं किया जाता, इसलिए नहीं भेजा गया

folders-far-not-sent = फ़ाइल या फ़ोल्डर नहीं है, इसलिए नहीं भेजा गया

folders-far-not-told = सर्वर नहीं बताता कि यह क्या है, इसलिए इसे छोड़ दिया गया

folders-far-ftp-cannot-refuse = यह सर्वर दो फ़ाइलों को एक ही नाम दे सकता है।

folders-far-cannot-do = सर्वर की फ़ाइलों के साथ यह नहीं किया जा सकता

folders-far-not-looked-at = इसे देखा नहीं जा सका

folders-far-not-opened = इसे खोला नहीं जा सका।

folders-far-inside-itself = { $name } (फ़ोल्डर अपने भीतर नहीं जा सकता)

folders-far-taken-by-folder = { $name } (इस नाम का एक फ़ोल्डर वहाँ है)

folders-far-taken-by-file = { $name } (इस नाम की एक फ़ाइल वहाँ है)

folders-far-address-unknown = सर्वर का पता मालूम नहीं है।

folders-far-copy-wrong-length = भेजी गई प्रति सही लंबाई की नहीं थी

folders-far-only-server-file = इस तरह केवल सर्वर की फ़ाइल का नाम बदला जाता है।

folders-far-aside-both = पुरानी फ़ाइल पूरी तरह { $old } में है, और नई { $new } में: { $why }

folders-far-aside-kept = पुरानी फ़ाइल हटाई नहीं जा सकी, और वह पूरी तरह { $old } में है

folders-far-aside-kept-why = पुरानी फ़ाइल हटाई नहीं जा सकी, और वह पूरी तरह { $old } में है: { $why }

folders-far-not-empty = { $name } खाली नहीं है।

folders-far-unsafe-name = छोड़ा गया: सर्वर ने असुरक्षित नाम भेजा।

folders-far-unsafe-names =
    { $n ->
        [one] छोड़ा गया: सर्वर ने असुरक्षित नाम भेजा: { $names }।
       *[other] { $n } छोड़े गए: सर्वर ने असुरक्षित नाम भेजे: { $names }।
    }

folders-far-fetching = { $name } लाया जा रहा है…

folders-far-cannot-fetch = { $name } लाया नहीं जा सकता: { $why }।

folders-far-opened-copy = { $name } की एक प्रति खोली गई। इसे सहेजने पर बदलाव वापस सर्वर को भेजे जाते हैं।

folders-far-saved = { $name } सर्वर पर सहेजी गई।

folders-far-not-saved = { $name } सर्वर पर सहेजी नहीं जा सकी: { $why }। यहाँ की प्रति में बदलाव बने रहते हैं।

folders-far-renamed = { $was } का नाम बदलकर { $now } किया गया।

folders-far-connect-title = सर्वर से जुड़ें

folders-far-connect-close = बंद करें

folders-far-connect-address = पता, जैसे sftp://jo@example.com

folders-far-connect-password = पासवर्ड, अगर ज़रूरी हो

folders-far-connect-key = कुंजी फ़ाइल, अगर आप चुनें

folders-far-connect-pick-key = कुंजी फ़ाइल चुनें

folders-far-connect-choose = चुनें

folders-far-connect-ssh-setup = आपका अपना ssh सेट-अप इस्तेमाल होता है: उसका config, एजेंट, कुंजियाँ और जाने-पहचाने सर्वर।

folders-far-connect-new-host = { $host } पर पहले कभी नहीं गए, इसलिए Trindle यह नहीं बता सकता कि यही सही सर्वर है। अगर आप इसी तक पहुँचना चाहते थे, तभी इस पर भरोसा करें।

folders-far-connect-host-changed = { $host } की पहचान अब वह नहीं है जो पहले थी। इसका मतलब हो सकता है कि बीच में कोई है, इसलिए इसे खोला नहीं गया। अगर इसके स्वामी ने इसे बदला है, तो पहले पुरानी प्रविष्टि को अपने जाने-पहचाने होस्ट से हटाएँ।

folders-far-connect-needs-password = इस सर्वर को पासवर्ड चाहिए। इसे ऊपर लिखें और फिर कोशिश करें।

folders-far-connect-needs-key = इस सर्वर को कुंजी या पासवर्ड चाहिए। ऊपर अपनी कुंजी फ़ाइल का नाम दें, या उसे अपने ssh एजेंट में जोड़ें, और फिर कोशिश करें।

folders-far-connect-cert-new = { $host } का प्रमाणपत्र जाँचा नहीं जा सका: { $why }। हो सके तो उसकी फ़िंगरप्रिंट को सर्वर के स्वामी की दी हुई फ़िंगरप्रिंट से मिलाएँ, और तभी भरोसा करें जब दोनों मेल खाएँ।

folders-far-connect-cert-changed = { $host } का प्रमाणपत्र वह नहीं है जिस पर आपने पहले भरोसा किया था। हो सकता है स्वामी ने उसे नया किया हो, या बीच में कोई हो। नए पर तभी भरोसा करें जब स्वामी इस फ़िंगरप्रिंट की पुष्टि करे।

folders-far-connect-connecting = जुड़ रहा है

folders-far-connect-trust = भरोसा करें और जुड़ें

folders-far-connect-go = जुड़ें

folders-far-phones = फ़ोन प्लग इन और अनलॉक होते ही साइडबार में अपने आप दिख जाते हैं।

