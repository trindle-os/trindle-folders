# machine translated (Hindi), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = इस नेटवर्क पर फ़ोल्डर साझा करने वाले कंप्यूटर खोजे जा रहे हैं…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = इस नेटवर्क पर फ़ोल्डर साझा करने वाला कोई कंप्यूटर नहीं मिला।

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] Folders ने Windows कंप्यूटर खोजे, और NAS ड्राइव, Mac और Linux कंप्यूटर भी।
        [windows] Folders ने Windows कंप्यूटर खोजे।
       *[other] Folders ने NAS ड्राइव, Mac और Linux कंप्यूटर खोजे।
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = Folders Windows कंप्यूटर नहीं खोज सका, क्योंकि trindle-wsdd इंस्टॉल नहीं है।

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = Folders NAS ड्राइव, Mac और Linux कंप्यूटर नहीं खोज सका, क्योंकि यह कंप्यूटर किसी नेटवर्क से जुड़ा नहीं है। अकेला VPN काफ़ी नहीं है।

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = Folders NAS ड्राइव, Mac और Linux कंप्यूटर नहीं खोज सका ({ $why })।

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = अगर कोई कंप्यूटर नहीं दिखता, या फ़ायरवॉल के पीछे है, तो भी आप उसे उसके नाम या पते से खोल सकते हैं।
