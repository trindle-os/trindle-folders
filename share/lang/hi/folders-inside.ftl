# machine translated (Hindi), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = पहले आर्काइव में से कुछ चुनें।

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = आर्काइव से चीज़ें केवल इस कंप्यूटर के किसी फ़ोल्डर में निकाली जा सकती हैं।

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = Folders { $name } पढ़ नहीं सका: { $why }।

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
        [one] Folders 1 चीज़ नहीं निकाल सका, क्योंकि उसका नाम फ़ोल्डर के बाहर तक जाता है या रास्ते में कुछ अड़ा है।
       *[other] Folders { $n } चीज़ें नहीं निकाल सका, क्योंकि उनके नाम फ़ोल्डर के बाहर तक जाते हैं या रास्ते में कुछ अड़ा है।
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = Folders इसे आर्काइव से निकाल नहीं सका: { $why }।
