
# machine translated (French), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = Recherche des ordinateurs de ce réseau qui partagent des dossiers…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = Aucun ordinateur partageant des dossiers n’a été trouvé sur ce réseau.

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] Folders a cherché des ordinateurs Windows, ainsi que des NAS, des Mac et des ordinateurs Linux.
        [windows] Folders a cherché des ordinateurs Windows.
       *[other] Folders a cherché des NAS, des Mac et des ordinateurs Linux.
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = Folders n’a pas pu chercher d’ordinateurs Windows, car trindle-wsdd n’est pas installé.

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = Folders n’a pas pu chercher de NAS, de Mac ni d’ordinateurs Linux, car cet ordinateur n’est pas connecté à un réseau. Un VPN seul ne suffit pas.

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = Folders n’a pas pu chercher de NAS, de Mac ni d’ordinateurs Linux ({ $why }).

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = Si un ordinateur n’apparaît pas, ou s’il est derrière un pare-feu, vous pouvez quand même l’ouvrir par son nom ou son adresse.
