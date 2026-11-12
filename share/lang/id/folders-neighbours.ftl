
# machine translated (Indonesian), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = Mencari komputer di jaringan ini yang membagikan folder…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = Tidak ditemukan komputer yang membagikan folder di jaringan ini.

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] Folders mencari komputer Windows, serta drive NAS, Mac, dan komputer Linux.
        [windows] Folders mencari komputer Windows.
       *[other] Folders mencari drive NAS, Mac, dan komputer Linux.
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = Folders tidak dapat mencari komputer Windows, karena trindle-wsdd tidak terpasang.

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = Folders tidak dapat mencari drive NAS, Mac, dan komputer Linux, karena komputer ini tidak terhubung ke jaringan. VPN saja tidak cukup.

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = Folders tidak dapat mencari drive NAS, Mac, dan komputer Linux ({ $why }).

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = Jika sebuah komputer tidak muncul, atau berada di balik firewall, Anda tetap dapat membukanya lewat nama atau alamatnya.

