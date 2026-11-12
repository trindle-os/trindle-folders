
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = 正在查找此网络上共享文件夹的电脑…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = 在此网络上没有找到共享文件夹的电脑。

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] Folders 查找了 Windows 电脑，以及 NAS、Mac 和 Linux 电脑。
        [windows] Folders 查找了 Windows 电脑。
       *[other] Folders 查找了 NAS、Mac 和 Linux 电脑。
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = Folders 无法查找 Windows 电脑，因为没有安装 trindle-wsdd。

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = Folders 无法查找 NAS、Mac 和 Linux 电脑，因为这台电脑没有连接到网络。只有 VPN 是不够的。

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = Folders 无法查找 NAS、Mac 和 Linux 电脑（{ $why }）。

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = 如果某台电脑没有出现，或在防火墙后面，你仍可通过它的名称或地址打开它。

