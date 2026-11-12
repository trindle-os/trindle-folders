# Errors and notices of Folders' connections to servers and phones: the shared error words, address reading, and names a server sent that cannot be shown.

## Errors (src/files/remote/mod.rs)

# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
folders-remote-mod-heard-card = Folders found { $host } on the local network at { $at }, port { $port }. Any device can use that name there, so Folders hasn't sent your password. If this is the right address, press Connect.
# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
folders-remote-mod-heard-stopped = Folders found { $host } on the local network at { $at }. Any device can use that name there, so Folders hasn't sent your password. Open { $host } again to check its address.
# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders couldn't read that address: { $why }.
# Red words when a server needs a password or key and none could be asked for.
folders-remote-mod-needs-password = This server needs a password or a key.
# Red words when a server turned down the login.
folders-remote-mod-login-refused = The server didn't accept that login.
# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
folders-remote-mod-host-key-unknown = You haven't connected to { $host } before, so Folders doesn't know its identity yet.
# Red words when a server's identity differs from the one remembered; the variable is the server's name.
folders-remote-mod-host-key-changed = { $host } has a different identity from last time. It may have been rebuilt, or someone may be intercepting the connection.
# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
folders-remote-mod-cert-unknown = Folders couldn't check the certificate for { $host }: { $why }.
# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
folders-remote-mod-cert-changed = The certificate for { $host } has changed since you trusted it.
# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders couldn't connect: { $why }.
# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders can't find { $what }.
# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } is already there.
# Red words when a server or phone refused permission; the variable is what it was refused for.
folders-remote-mod-denied = You don't have permission to use { $what }.
# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } is in use by another program.
# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } isn't supported.
# Red words when a copy or other work with a server was cancelled.
folders-remote-mod-cancelled = Cancelled.
# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = The reply was far too big, so Folders stopped it.
# The thing a server cannot do, in "... isn't supported.": reading where a link points.
folders-remote-mod-links = Following links
# The thing that cannot be done, in "... isn't supported.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = Copying to or from a Windows share before it's opened
# The thing that cannot be done, in "... isn't supported.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = Copying to or from an NFS share before it's opened

## Reading an address (src/files/remote/address.rs)

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = it doesn't start with sftp://, ftp:// or mtp://
# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = it must start with sftp://, ftp://, ftps://, ftpes://, dav://, davs://, smb://, nfs:// or mtp://
# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = a [ is missing its ]
# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = it doesn't name a server
# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = the server's name isn't valid
# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = the path has a line break in it
# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = there's something extra after the server's name
# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } isn't a valid port number

## Names a server sent (src/files/remote/screen.rs)

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = skipped because the server sent an unsafe name
# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = something with no name
# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } is now a folder, so Folders didn't remove it.
