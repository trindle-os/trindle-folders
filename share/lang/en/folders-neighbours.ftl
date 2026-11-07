# What the Network page in Folders says while it looks for computers that share folders, and when it finds none.

folders-neighbours-looking = Looking for computers on this network that share folders.
folders-neighbours-none = No computers sharing folders were found on this network.
folders-neighbours-asked =
    { $ways ->
        [both] Folders asked the Windows way, and the way a NAS, a Mac or Linux answers.
        [windows] Folders asked the Windows way.
       *[other] Folders asked the way a NAS, a Mac or Linux answers.
    }
folders-neighbours-no-finder = The Windows way could not be asked: Trindle's finder for it (trindle-wsdd) is not on this computer.
folders-neighbours-no-card = No network card that can be asked is connected; a VPN alone carries no such questions.
folders-neighbours-refused = The network would not take the question ({ $why }).
folders-neighbours-quiet = One that stays quiet, or is behind a firewall, can still be opened by its name or address.
