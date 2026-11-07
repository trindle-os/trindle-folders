# Reasons and errors for secure FTP connections (FTPS) in Folders: certificate checks and setting up the secure link.

folders-remote-tls-no-roots = this computer has no trusted certificates to check it against
folders-remote-tls-unknown-issuer = it is not signed by anyone this computer trusts, as happens with one a server made for itself
folders-remote-tls-expired = it has expired
folders-remote-tls-not-yet = it is not valid yet
folders-remote-tls-wrong-name = it was made for a different name
folders-remote-tls-revoked = it has been withdrawn
folders-remote-tls-failed-checks = it did not pass the usual checks

folders-remote-tls-setup-failed = Secure connections could not be set up: { $why }
folders-remote-tls-bad-name = { $host } is not a name a certificate can carry
folders-remote-tls-start-failed = A secure connection could not be started: { $why }
folders-remote-tls-stopped = The server stopped answering while setting up a secure connection.
folders-remote-tls-closed = The server closed the connection while setting up a secure one.
folders-remote-tls-could-not-set-up = A secure connection could not be set up: { $why }
