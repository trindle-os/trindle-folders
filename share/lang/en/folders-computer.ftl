# The Storage page of Folders: a card for every drive, phone and network share (src/files/computer.rs).

## Storage page (src/files/computer.rs)

# Under a card's name on the Storage page: the disk the system is on.
folders-computer-kind-system = System disk
# Under a card's name on the Storage page: another fixed disk inside the computer.
folders-computer-kind-inside = Internal disk
# Under a card's name on the Storage page: a USB drive that is plugged in.
folders-computer-kind-usb = USB drive
# Under a card's name on the Storage page: a memory card.
folders-computer-kind-card = Memory card
# Under a card's name on the Storage page: an optical disc.
folders-computer-kind-disc = Disc
# Under a card's name on the Storage page: a removable drive of no known kind.
folders-computer-kind-other = Drive
# Under a card's name on the Storage page: a phone or tablet.
folders-computer-kind-phone = Phone or tablet
# Under a card's name on the Storage page: a folder shared from another computer.
folders-computer-kind-share = Network share
# Name of a Storage card when another card has the same name; the drive it is on tells them apart.
folders-computer-name-on = { $name } on { $drive }
# Under a card's bar on the Storage page: how much room is left of the whole, as sizes such as 12.0 GB.
folders-computer-free-of = { $free } free of { $all }
# Under a card's bar on the Storage page when the drive is almost full; the line that follows says how much is free.
folders-computer-almost-full = Very nearly full: { $said }
# Under a card's bar on the Storage page when the drive is nearly full; the line that follows says how much is free.
folders-computer-nearly-full = Getting full: { $said }
# Under a card's bar on the Storage page: a locked drive whose size is not known.
folders-computer-locked = Locked
# Under a card's bar on the Storage page: a locked drive and its size.
folders-computer-locked-size = Locked, { $size }
# Under a card's bar on the Storage page: a phone or tablet card, which has no size until it is opened.
folders-computer-phone-hint = Open it to see its files
# Under a card's bar on the Storage page: a drive that is not mounted and whose size is not known.
folders-computer-not-mounted = Not opened yet
# Under a card's bar on the Storage page: a drive that is not mounted and its size.
folders-computer-not-mounted-size = Not opened yet, { $size }
# Line at the foot of the Storage page: how many drives it holds.
folders-computer-drive-count =
    { $n ->
        [one] 1 drive
       *[other] { $n } drives
    }
# Tip and name of the round button on a Storage card that opens the drive.
folders-computer-open = Open { $name }
# Tip and name of the round button on a Storage card that unlocks a locked drive and opens it.
folders-computer-unlock = Unlock { $name } and open it
# Tip and name of the round button on a Storage card that mounts a drive (gets it ready to use) and opens it.
folders-computer-mount = Open { $name }
# Tip and name of the round button on a Storage card for a network share that closes the share.
folders-computer-close = Close { $name }
# Tip and name of the round button on a Storage card that ejects a drive.
folders-computer-eject = Eject { $name }
# Tip of the bar on a Storage card: how full the drive is, as a number such as 73.
folders-computer-percent-full = { $pct }% full
