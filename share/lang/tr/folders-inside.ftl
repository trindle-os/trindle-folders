
# machine translated (Turkish), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = Önce arşivden bir şey seçin.

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = Öğeleri bir arşivden yalnızca bu bilgisayardaki bir klasöre çıkarabilirsiniz.

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = Folders { $name } dosyasını okuyamadı: { $why }.

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
        [one] Folders, adı klasörün dışına çıktığı ya da yolda bir engel olduğu için 1 öğeyi çıkaramadı.
       *[other] Folders, adları klasörün dışına çıktığı ya da yolda bir engel olduğu için { $n } öğeyi çıkaramadı.
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = Folders arşivden çıkaramadı: { $why }.
