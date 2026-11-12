
# machine translated (German), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = Suche nach Computern in diesem Netzwerk, die Ordner freigeben…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = In diesem Netzwerk wurden keine Computer gefunden, die Ordner freigeben.

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] Folders hat nach Windows-Computern gesucht sowie nach NAS-Laufwerken, Macs und Linux-Computern.
        [windows] Folders hat nach Windows-Computern gesucht.
       *[other] Folders hat nach NAS-Laufwerken, Macs und Linux-Computern gesucht.
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = Folders konnte nicht nach Windows-Computern suchen, weil trindle-wsdd nicht installiert ist.

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = Folders konnte nicht nach NAS-Laufwerken, Macs und Linux-Computern suchen, weil dieser Computer mit keinem Netzwerk verbunden ist. Ein VPN allein reicht nicht.

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = Folders konnte nicht nach NAS-Laufwerken, Macs und Linux-Computern suchen ({ $why }).

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = Wenn ein Computer nicht erscheint oder hinter einer Firewall steht, kannst du ihn trotzdem über seinen Namen oder seine Adresse öffnen.

