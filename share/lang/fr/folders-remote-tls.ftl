
# machine translated (French), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when this computer has no trusted certificates to check against; a sentence follows "could not be checked:".
folders-remote-tls-no-roots = cet ordinateur n’a aucun certificat de confiance pour le vérifier

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when nobody this computer trusts signed the certificate.
folders-remote-tls-unknown-issuer = il n’est signé par personne en qui cet ordinateur a confiance. Cela arrive quand un serveur crée son propre certificat

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate has run out.
folders-remote-tls-expired = il a expiré

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate is not valid yet.
folders-remote-tls-not-yet = il n’est pas encore valide

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate was made for another server name.
folders-remote-tls-wrong-name = il a été fait pour un autre nom de serveur

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate has been withdrawn.
folders-remote-tls-revoked = il a été annulé

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate failed the usual checks for another reason.
folders-remote-tls-failed-checks = il n’a pas passé les vérifications habituelles

# Red words when the secure connection settings could not be made; the variable is the reason the system gave.
folders-remote-tls-setup-failed = Folders n’a pas pu préparer les connexions sécurisées : { $why }

# Red words when the server name cannot go in a certificate check; the variable is the server name.
folders-remote-tls-bad-name = { $host } n’est pas un nom de serveur valide pour une connexion sécurisée

# Red words when a secure connection could not be started; the variable is the reason the system gave.
folders-remote-tls-start-failed = Folders n’a pas pu démarrer une connexion sécurisée : { $why }

# Red words when the server stopped answering while the secure link was being set up.
folders-remote-tls-stopped = Le serveur a cessé de répondre pendant que Folders mettait en place une connexion sécurisée.

# Red words when the server closed the connection while the secure link was being set up.
folders-remote-tls-closed = Le serveur a fermé la connexion pendant que Folders mettait en place une connexion sécurisée.

# Red words when the secure link could not be set up for another reason; the variable is the reason the system gave.
folders-remote-tls-could-not-set-up = Folders n’a pas pu mettre en place une connexion sécurisée : { $why }
