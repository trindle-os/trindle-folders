# Errors and notes for WebDAV servers (dav:// and davs://) in Folders.

## WebDAV servers (src/files/remote/dav.rs)

# Red words when a thing on a server changed just before it was to be removed and was left alone; the variable is its name.
folders-remote-dav-changed-at-removal = { $name } (not removed because it had just changed)
# Reason in red words when the server's reply is cut into pieces and one has no size.
folders-remote-dav-chunk = part of the server's reply had no size
# Reason in red words ("Could not reach it: ...") when a server's name has no address; the variables are the name and the system's reason.
folders-remote-dav-not-found = { $host } couldn't be found ({ $why })
# Reason in red words ("Could not reach it: ...") when a server's name gives no address; the variable is the name.
folders-remote-dav-no-address = there's no address for { $host } on the network
# Red words when the address opened names a file, not a folder; the variable is the path.
folders-remote-dav-is-file = { $path } is a file, not a folder.
# Red words when a file being sent ended before all of it was sent.
folders-remote-dav-file-ended = The file ended before it was fully sent.
# Red words when the server wants a kind of login Folders cannot do.
folders-remote-dav-unknown-login = The server uses a kind of login Folders doesn't support.
# Red words when the server does not list folders over WebDAV.
folders-remote-dav-no-listing = This server doesn't let Folders list folders over WebDAV.
# Red words when the server did not answer like a web server.
folders-remote-dav-not-web = The server didn't reply the way a web server should.
# Red words when the server has no room left.
folders-remote-dav-full = The server is full.
# Red words when the server answered with a number Folders has no words for; the variables are the number and the name of the thing.
folders-remote-dav-status = The server replied with code { $status } for { $name }.
# Red words when the server's folder listing could not be read; the variable is the reason.
folders-remote-dav-listing-unreadable = Folders couldn't read the folder listing: { $why }
# Red words when a file being fetched ended before all of it came.
folders-remote-dav-cut-short = Only part of the file arrived.
# Red words when a folder to be removed still holds something; the variable is its name.
folders-remote-dav-not-empty = { $name } is not empty.
