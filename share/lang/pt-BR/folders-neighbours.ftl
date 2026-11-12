
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = Procurando computadores nesta rede que compartilham pastas…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = Nenhum computador que compartilhe pastas foi encontrado nesta rede.

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] O Folders procurou computadores Windows, e também NAS, Macs e computadores Linux.
        [windows] O Folders procurou computadores Windows.
       *[other] O Folders procurou NAS, Macs e computadores Linux.
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = O Folders não conseguiu procurar computadores Windows, porque o trindle-wsdd não está instalado.

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = O Folders não conseguiu procurar NAS, Macs e computadores Linux, porque este computador não está conectado a uma rede. Uma VPN sozinha não basta.

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = O Folders não conseguiu procurar NAS, Macs e computadores Linux ({ $why }).

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = Se um computador não aparecer, ou estiver atrás de um firewall, você ainda pode abri-lo pelo nome ou endereço.

