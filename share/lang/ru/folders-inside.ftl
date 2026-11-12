# machine translated (Russian), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = Сначала выберите что-нибудь в архиве.

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = Файлы из архива можно извлечь только в папку на этом компьютере.

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = Не удалось прочитать { $name }: { $why }.

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
        [one] Folders не удалось извлечь { $n } элемент: его имя указывает за пределы папки, или что-то мешает.
        [few] Folders не удалось извлечь { $n } элемента: их имена указывают за пределы папки, или что-то мешает.
        [many] Folders не удалось извлечь { $n } элементов: их имена указывают за пределы папки, или что-то мешает.
       *[other] Folders не удалось извлечь { $n } элемента: их имена указывают за пределы папки, или что-то мешает.
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = Не удалось извлечь из архива: { $why }.

