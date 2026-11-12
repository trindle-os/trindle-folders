# The card in Folders that asks for the password of a locked zip or 7z archive.

## Password card (src/files/locked.rs)

# Name of the main button on the password card when the archive is being extracted.
folders-locked-extract = Extract
# Name of the main button on the password card when the archive is being opened.
folders-locked-open = Open
# Placeholder and spoken name of the password box on the password card.
folders-locked-password = Password
# Line under the password box before anything has been tried: says the password is not saved.
folders-locked-kept-note = Folders forgets it when the archive closes.
# Words beside the spinning ring on the password card while a password is being tried.
folders-locked-trying = Trying the password
# Title of the password card; the variable is the archive's name.
folders-locked-title = Password for { $name }
# Tip of the round button that closes the password card.
folders-locked-close = Close
# First line of the password card, saying which archive is locked; the variable is the archive's name.
folders-locked-is-locked = { $name } is locked with a password.
# Notice shown when another card is already open and an archive needs its password; the variable is the archive's name.
folders-locked-open-again = { $name } is locked with a password. Open it again to type the password.
# Notice shown when a locked archive cannot be read whatever the password; the variables are the archive's name and the reason.
folders-locked-could-not-read = Folders couldn't read { $name }: { $why }.
