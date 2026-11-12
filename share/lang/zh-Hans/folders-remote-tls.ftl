
# machine translated (Simplified Chinese), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when this computer has no trusted certificates to check against; a sentence follows "could not be checked:".
folders-remote-tls-no-roots = 这台电脑没有可用于核对的受信任证书

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when nobody this computer trusts signed the certificate.
folders-remote-tls-unknown-issuer = 它不是由这台电脑信任的任何一方签发的。服务器自己制作证书时就会出现这种情况

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate has run out.
folders-remote-tls-expired = 它已过期

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate is not valid yet.
folders-remote-tls-not-yet = 它尚未生效

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate was made for another server name.
folders-remote-tls-wrong-name = 它是为另一个服务器名称签发的

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate has been withdrawn.
folders-remote-tls-revoked = 它已被撤销

# SAFETY: needs a human review before it ships
# Reason shown on the certificate card when the certificate failed the usual checks for another reason.
folders-remote-tls-failed-checks = 它没有通过常规检查

# Red words when the secure connection settings could not be made; the variable is the reason the system gave.
folders-remote-tls-setup-failed = Folders 无法设置安全连接：{ $why }

# Red words when the server name cannot go in a certificate check; the variable is the server name.
folders-remote-tls-bad-name = { $host } 不是可用于安全连接的有效服务器名称

# Red words when a secure connection could not be started; the variable is the reason the system gave.
folders-remote-tls-start-failed = Folders 无法开始安全连接：{ $why }

# Red words when the server stopped answering while the secure link was being set up.
folders-remote-tls-stopped = Folders 建立安全连接时，服务器不再响应了。

# Red words when the server closed the connection while the secure link was being set up.
folders-remote-tls-closed = Folders 建立安全连接时，服务器关闭了连接。

# Red words when the secure link could not be set up for another reason; the variable is the reason the system gave.
folders-remote-tls-could-not-set-up = Folders 无法建立安全连接：{ $why }

