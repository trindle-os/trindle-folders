
# machine translated (Spanish), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = Buscando en esta red equipos que compartan carpetas…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = No se encontró ningún equipo que comparta carpetas en esta red.

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] Folders buscó equipos con Windows, y también unidades NAS, Macs y equipos con Linux.
        [windows] Folders buscó equipos con Windows.
       *[other] Folders buscó unidades NAS, Macs y equipos con Linux.
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = Folders no pudo buscar equipos con Windows, porque trindle-wsdd no está instalado.

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = Folders no pudo buscar unidades NAS, Macs ni equipos con Linux, porque este equipo no está conectado a una red. Una VPN por sí sola no basta.

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = Folders no pudo buscar unidades NAS, Macs ni equipos con Linux ({ $why }).

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = Si un equipo no aparece, o está tras un cortafuegos, aún puedes abrirlo con su nombre o dirección.

