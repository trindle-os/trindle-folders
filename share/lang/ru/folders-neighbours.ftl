
# machine translated (Russian), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = Идёт поиск компьютеров в этой сети, которые открывают доступ к папкам…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = В этой сети не найдено компьютеров с общими папками.

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] Folders искал компьютеры Windows, а также NAS, Mac и компьютеры Linux.
        [windows] Folders искал компьютеры Windows.
       *[other] Folders искал NAS, Mac и компьютеры Linux.
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = Folders не смог искать компьютеры Windows, потому что trindle-wsdd не установлен.

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = Folders не смог искать NAS, Mac и компьютеры Linux, потому что этот компьютер не подключён к сети. Одного VPN недостаточно.

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = Folders не смог искать NAS, Mac и компьютеры Linux ({ $why }).

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = Если компьютер не появился в списке или стоит за брандмауэром, его всё равно можно открыть по имени или адресу.

