# Words of Folders working with servers and other computers: the Connect to a server card, and the notes and failure reasons of copying, moving, deleting, renaming and opening things on a server.

## Failure lists (src/files/far.rs)

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })
# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = there's no server called { $host }
# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = not all of it arrived, so the original hasn't moved
# Reason a link was not sent to a server.
folders-far-not-followed = it's a link, so Folders didn't send it
# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = it isn't a file or folder, so Folders didn't send it
# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders couldn't tell what kind of item it is, so it left it there
# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = On this kind of server, a file could replace another with the same name.
# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = You can't do that to files on a server
# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders couldn't read it
# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders couldn't open it.
# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (a folder can't go inside itself)
# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (there's already a folder with that name)
# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (there's already a file with that name)
# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders doesn't know the server's address.
# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = the copy that was sent was the wrong size
# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = This only works with files on a server.
# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders couldn't put the new file in place, so the old one is still there, now called { $old }, and the new one is called { $new }: { $why }
# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders couldn't remove the old file. It's still there, now called { $old }
# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders couldn't remove the old file, so it's still there, now called { $old }: { $why }
# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } is not empty.
# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders skipped a file because its name isn't safe to use.
# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
        [one] Folders skipped an item because its name isn't safe to use: { $names }.
       *[other] Folders skipped { $n } items because their names aren't safe to use: { $names }.
    }

## Opening and saving files from a server (src/files/far.rs)

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = Downloading { $name }…
# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders couldn't download { $name }: { $why }.
# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = Opened a copy of { $name }. When you save it, your changes go back to the server.
# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = Saved { $name } on the server.
# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders couldn't save { $name } on the server: { $why }. Your changes are still in the copy on this computer.
# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = Renamed { $was } to { $now }.

## Connect to a server card (src/files/far.rs)

# Heading of the Connect to a server card.
folders-far-connect-title = Connect to a server
# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Close
# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Address, such as sftp://jo@example.com
# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
folders-far-connect-password = Password, if needed
# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = Key file, if you use one
# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = Choose the key file
# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders uses your ssh settings, ssh agent, keys and list of known servers.
# Note on the Connect to a server card for a server not visited before; the server's name.
folders-far-connect-new-host = You haven't connected to { $host } before, so Folders can't check it's the right one. Only trust it if you were expecting to connect to it.
# Red note on the Connect to a server card when a known server has a different identity; the server's name.
folders-far-connect-host-changed = { $host } has a different identity from last time. Someone may be trying to intercept your connection, so Folders didn't connect. If the owner changed it, remove its old entry from your known hosts first.
# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
folders-far-connect-needs-password = This server needs a password. Type it above and try again.
# Note on the Connect to a server card when a server asks for a key or password.
folders-far-connect-needs-key = This server needs a key or password. Choose your key file above, or add it to your ssh agent, then try again.
# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
folders-far-connect-cert-new = Folders couldn't check the certificate for { $host }: { $why }. If you can, compare its fingerprint with the one the owner gives you. Only trust it if they match.
# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
folders-far-connect-cert-changed = The certificate for { $host } has changed since you trusted it. The owner may have renewed it, or someone may be trying to intercept your connection. Only trust it if the owner confirms this fingerprint.
# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Connecting
# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
folders-far-connect-trust = Trust it and connect
# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Connect
# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Phones appear in the sidebar when you plug them in and unlock them.
