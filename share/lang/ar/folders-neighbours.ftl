
# machine translated (Arabic), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = جارٍ البحث عن حواسيب على هذه الشبكة تشارك مجلدات…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = لم يُعثر على حواسيب تشارك مجلدات على هذه الشبكة.

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] بحث Folders عن حواسيب Windows، وعن أجهزة NAS وحواسيب Mac وLinux.
        [windows] بحث Folders عن حواسيب Windows.
       *[other] بحث Folders عن أجهزة NAS وحواسيب Mac وLinux.
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = تعذّر على Folders البحث عن حواسيب Windows لأن trindle-wsdd غير مثبّت.

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = تعذّر على Folders البحث عن أجهزة NAS وحواسيب Mac وLinux لأن هذا الحاسوب غير متصل بشبكة. الشبكة الافتراضية الخاصة (VPN) وحدها لا تكفي.

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = تعذّر على Folders البحث عن أجهزة NAS وحواسيب Mac وLinux ({ $why }).

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = إذا لم يظهر حاسوب، أو كان خلف جدار حماية، فلا يزال بإمكانك فتحه باسمه أو عنوانه.

