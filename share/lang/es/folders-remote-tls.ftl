
# machine translated (Spanish), not yet reviewed by a person.

# Reason shown on the certificate card when this computer has no trusted certificates to check against; a sentence follows "could not be checked:".
folders-remote-tls-no-roots = este equipo no tiene certificados de confianza con los que comprobarlo

# Reason shown on the certificate card when nobody this computer trusts signed the certificate.
# SAFETY: needs a human review before it ships
folders-remote-tls-unknown-issuer = no está firmado por nadie en quien confíe este equipo. Esto ocurre cuando un servidor crea su propio certificado

# Reason shown on the certificate card when the certificate has run out.
# SAFETY: needs a human review before it ships
folders-remote-tls-expired = ha caducado

# Reason shown on the certificate card when the certificate is not valid yet.
folders-remote-tls-not-yet = todavía no es válido

# Reason shown on the certificate card when the certificate was made for another server name.
# SAFETY: needs a human review before it ships
folders-remote-tls-wrong-name = se creó para otro nombre de servidor

# Reason shown on the certificate card when the certificate has been withdrawn.
# SAFETY: needs a human review before it ships
folders-remote-tls-revoked = ha sido revocado

# Reason shown on the certificate card when the certificate failed the usual checks for another reason.
# SAFETY: needs a human review before it ships
folders-remote-tls-failed-checks = no superó las comprobaciones habituales

# Red words when the secure connection settings could not be made; the variable is the reason the system gave.
folders-remote-tls-setup-failed = Folders no pudo configurar las conexiones seguras: { $why }

# Red words when the server name cannot go in a certificate check; the variable is the server name.
folders-remote-tls-bad-name = { $host } no es un nombre de servidor válido para una conexión segura

# Red words when a secure connection could not be started; the variable is the reason the system gave.
folders-remote-tls-start-failed = Folders no pudo iniciar una conexión segura: { $why }

# Red words when the server stopped answering while the secure link was being set up.
folders-remote-tls-stopped = El servidor dejó de responder mientras Folders configuraba una conexión segura.

# Red words when the server closed the connection while the secure link was being set up.
folders-remote-tls-closed = El servidor cerró la conexión mientras Folders configuraba una conexión segura.

# Red words when the secure link could not be set up for another reason; the variable is the reason the system gave.
folders-remote-tls-could-not-set-up = Folders no pudo configurar una conexión segura: { $why }

