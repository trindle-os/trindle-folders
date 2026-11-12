
# machine translated (Russian), not yet reviewed by a person.

# Under a card's name on the Storage page: the disk the system is on.
folders-computer-kind-system = Системный диск

# Under a card's name on the Storage page: another fixed disk inside the computer.
folders-computer-kind-inside = Внутренний диск

# Under a card's name on the Storage page: a USB drive that is plugged in.
folders-computer-kind-usb = USB-накопитель

# Under a card's name on the Storage page: a memory card.
folders-computer-kind-card = Карта памяти

# Under a card's name on the Storage page: an optical disc.
folders-computer-kind-disc = Диск

# Under a card's name on the Storage page: a removable drive of no known kind.
folders-computer-kind-other = Накопитель

# Under a card's name on the Storage page: a phone or tablet.
folders-computer-kind-phone = Телефон или планшет

# Under a card's name on the Storage page: a folder shared from another computer.
folders-computer-kind-share = Сетевая папка

# Name of a Storage card when another card has the same name; the drive it is on tells them apart.
folders-computer-name-on = { $name } на { $drive }

# Under a card's bar on the Storage page: how much room is left of the whole, as sizes such as 12.0 GB.
folders-computer-free-of = Свободно { $free } из { $all }

# Under a card's bar on the Storage page when the drive is almost full; the line that follows says how much is free.
folders-computer-almost-full = Почти полностью заполнен: { $said }

# Under a card's bar on the Storage page when the drive is nearly full; the line that follows says how much is free.
folders-computer-nearly-full = Место заканчивается: { $said }

# Under a card's bar on the Storage page: a locked drive whose size is not known.
folders-computer-locked = Заблокирован

# Under a card's bar on the Storage page: a locked drive and its size.
folders-computer-locked-size = Заблокирован, { $size }

# Under a card's bar on the Storage page: what a phone or tablet card invites the person to do.
folders-computer-phone-hint = Откройте, чтобы увидеть файлы

# Under a card's bar on the Storage page: a drive that is not mounted and whose size is not known.
folders-computer-not-mounted = Ещё не открыт

# Under a card's bar on the Storage page: a drive that is not mounted and its size.
folders-computer-not-mounted-size = Ещё не открыт, { $size }

# Line at the foot of the Storage page: how many drives it holds.
folders-computer-drive-count =
    { $n ->
        [one] { $n } накопитель
        [few] { $n } накопителя
        [many] { $n } накопителей
       *[other] { $n } накопителя
    }

# Tip and name of the round button on a Storage card that opens the drive.
folders-computer-open = Открыть { $name }

# Tip and name of the round button on a Storage card that unlocks a locked drive and opens it.
# SAFETY: needs a human review before it ships
folders-computer-unlock = Разблокировать { $name } и открыть

# Tip and name of the round button on a Storage card that mounts a drive and opens it.
folders-computer-mount = Открыть { $name }

# Tip and name of the round button on a Storage card for a network share that closes the share.
folders-computer-close = Закрыть { $name }

# Tip and name of the round button on a Storage card that ejects a drive.
# SAFETY: needs a human review before it ships
folders-computer-eject = Извлечь { $name }

# Tip of the bar on a Storage card: how full the drive is, as a number such as 73.
folders-computer-percent-full = Заполнено на { $pct }%
