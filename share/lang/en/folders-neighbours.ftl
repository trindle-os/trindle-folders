# What the Network page in Folders says while it looks for computers that share folders, and when it finds none.

## Network page (src/files/neighbours.rs)

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = Looking for computers on this network that share folders…
# First line on the empty Network page when nothing was found.
folders-neighbours-none = No computers sharing folders were found on this network.
# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] Folders looked for Windows computers, and for NAS drives, Macs and Linux computers.
        [windows] Folders looked for Windows computers.
       *[other] Folders looked for NAS drives, Macs and Linux computers.
    }
# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = Folders couldn't look for Windows computers, because trindle-wsdd isn't installed.
# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = Folders couldn't look for NAS drives, Macs and Linux computers, because this computer isn't connected to a network. A VPN on its own isn't enough.
# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = Folders couldn't look for NAS drives, Macs and Linux computers ({ $why }).
# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = If a computer doesn't show up, or is behind a firewall, you can still open it by its name or address.
