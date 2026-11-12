# Folders' sidebar list of drives plugged in: names, the eject button, and the words said while a drive is opened or put away (src/files/drives.rs).

## Drives (src/files/drives.rs)

# Name of a drive in the sidebar when two drives share a name: its name and the drive it is on.
folders-drives-name-on = { $name } on { $drive }
# Tip and spoken name of the round eject button on a drive's row in the sidebar.
folders-drives-eject = Eject { $name }
# Red words shown when a drive is asked for while it is being opened or put away.
folders-drives-busy = Folders is still opening or ejecting this drive. Try again in a moment.
# Name used in the notice about a drive when the drive has no name.
folders-drives-the-drive = the drive
# Notice that a drive was put away and can be unplugged.
folders-drives-taken-out = You can unplug { $name } now
# Red words shown when a drive could not be opened; the reason follows.
folders-drives-open-failed = Folders couldn't open { $name }: { $why }.
# Red words shown when a drive could not be put away; the reason follows.
folders-drives-away-failed = Folders couldn't eject { $name }: { $why }.
# Red words shown when the drive service (UDisks) cannot be reached.
folders-drives-unreachable = Folders can't reach your drives right now.
# Words shown while a drive is being opened.
folders-drives-opening = Opening { $name }…
# Words shown while a drive is being put away.
folders-drives-putting-away = Ejecting { $name }…
