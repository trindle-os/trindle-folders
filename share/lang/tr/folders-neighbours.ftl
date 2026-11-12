
# machine translated (Turkish), not yet reviewed by a person.

# Line on the empty Network page while Folders is still looking for computers.
folders-neighbours-looking = Bu ağda klasör paylaşan bilgisayarlar aranıyor…

# First line on the empty Network page when nothing was found.
folders-neighbours-none = Bu ağda klasör paylaşan bilgisayar bulunamadı.

# Line on the empty Network page saying which ways of looking were used; the variable is both, windows or other.
folders-neighbours-asked =
    { $ways ->
        [both] Folders Windows bilgisayarlarını, ayrıca NAS sürücülerini, Mac'leri ve Linux bilgisayarlarını aradı.
        [windows] Folders Windows bilgisayarlarını aradı.
       *[other] Folders NAS sürücülerini, Mac'leri ve Linux bilgisayarlarını aradı.
    }

# Line on the empty Network page saying the Windows way could not be asked because Trindle's finder program is missing.
folders-neighbours-no-finder = Folders Windows bilgisayarlarını arayamadı, çünkü trindle-wsdd kurulu değil.

# Line on the empty Network page saying the other way could not be asked because no network card is connected.
folders-neighbours-no-card = Folders NAS sürücülerini, Mac'leri ve Linux bilgisayarlarını arayamadı, çünkü bu bilgisayar bir ağa bağlı değil. Tek başına bir VPN yeterli değil.

# Line on the empty Network page saying the network refused the question; the variable is the reason the system gave.
folders-neighbours-refused = Folders NAS sürücülerini, Mac'leri ve Linux bilgisayarlarını arayamadı ({ $why }).

# Last line on the empty Network page: a quiet computer can still be opened by hand.
folders-neighbours-quiet = Bir bilgisayar görünmüyorsa ya da güvenlik duvarının arkasındaysa, yine de adıyla veya adresiyle açabilirsiniz.

