# Reasons and errors for secure FTP connections (FTPS) in Folders: certificate checks and setting up the secure link.

## Certificate checks (src/files/remote/tls.rs)

# Reason shown on the certificate card when this computer has no trusted certificates to check against; a sentence follows "could not be checked:".
folders-remote-tls-no-roots = this computer has no trusted certificates to check it against
# Reason shown on the certificate card when nobody this computer trusts signed the certificate.
folders-remote-tls-unknown-issuer = it isn't signed by anyone this computer trusts. This happens when a server makes its own certificate
# Reason shown on the certificate card when the certificate has run out.
folders-remote-tls-expired = it has expired
# Reason shown on the certificate card when the certificate is not valid yet.
folders-remote-tls-not-yet = it isn't valid yet
# Reason shown on the certificate card when the certificate was made for another server name.
folders-remote-tls-wrong-name = it was made for a different server name
# Reason shown on the certificate card when the certificate has been withdrawn.
folders-remote-tls-revoked = it has been cancelled
# Reason shown on the certificate card when the certificate failed the usual checks for another reason.
folders-remote-tls-failed-checks = it didn't pass the usual checks

## Setting up the secure link (src/files/remote/tls.rs)

# Red words when the secure connection settings could not be made; the variable is the reason the system gave.
folders-remote-tls-setup-failed = Folders couldn't set up secure connections: { $why }
# Red words when the server name cannot go in a certificate check; the variable is the server name.
folders-remote-tls-bad-name = { $host } isn't a valid server name for a secure connection
# Red words when a secure connection could not be started; the variable is the reason the system gave.
folders-remote-tls-start-failed = Folders couldn't start a secure connection: { $why }
# Red words when the server stopped answering while the secure link was being set up.
folders-remote-tls-stopped = The server stopped responding while Folders was setting up a secure connection.
# Red words when the server closed the connection while the secure link was being set up.
folders-remote-tls-closed = The server closed the connection while Folders was setting up a secure connection.
# Red words when the secure link could not be set up for another reason; the variable is the reason the system gave.
folders-remote-tls-could-not-set-up = Folders couldn't set up a secure connection: { $why }
