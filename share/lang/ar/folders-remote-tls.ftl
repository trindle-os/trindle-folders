
# machine translated (Arabic), not yet reviewed by a person.

# Reason shown on the certificate card when this computer has no trusted certificates to check against; a sentence follows "could not be checked:".
# SAFETY: needs a human review before it ships
folders-remote-tls-no-roots = لا توجد على هذا الحاسوب شهادات موثوقة يُفحص بها

# Reason shown on the certificate card when nobody this computer trusts signed the certificate.
# SAFETY: needs a human review before it ships
folders-remote-tls-unknown-issuer = لم يوقّعها أي طرف يثق به هذا الحاسوب. يحدث هذا عندما ينشئ الخادم شهادته بنفسه

# Reason shown on the certificate card when the certificate has run out.
# SAFETY: needs a human review before it ships
folders-remote-tls-expired = انتهت صلاحيتها

# Reason shown on the certificate card when the certificate is not valid yet.
# SAFETY: needs a human review before it ships
folders-remote-tls-not-yet = لم تبدأ صلاحيتها بعد

# Reason shown on the certificate card when the certificate was made for another server name.
# SAFETY: needs a human review before it ships
folders-remote-tls-wrong-name = صُنعت لاسم خادم مختلف

# Reason shown on the certificate card when the certificate has been withdrawn.
# SAFETY: needs a human review before it ships
folders-remote-tls-revoked = أُلغيت

# Reason shown on the certificate card when the certificate failed the usual checks for another reason.
# SAFETY: needs a human review before it ships
folders-remote-tls-failed-checks = لم تجتز الفحوص المعتادة

# Red words when the secure connection settings could not be made; the variable is the reason the system gave.
folders-remote-tls-setup-failed = تعذّر على Folders إعداد الاتصالات الآمنة: { $why }

# Red words when the server name cannot go in a certificate check; the variable is the server name.
folders-remote-tls-bad-name = { $host } ليس اسم خادم صالحاً لاتصال آمن

# Red words when a secure connection could not be started; the variable is the reason the system gave.
folders-remote-tls-start-failed = تعذّر على Folders بدء اتصال آمن: { $why }

# Red words when the server stopped answering while the secure link was being set up.
folders-remote-tls-stopped = توقف الخادم عن الاستجابة بينما كان Folders يعدّ اتصالاً آمناً.

# Red words when the server closed the connection while the secure link was being set up.
folders-remote-tls-closed = أغلق الخادم الاتصال بينما كان Folders يعدّ اتصالاً آمناً.

# Red words when the secure link could not be set up for another reason; the variable is the reason the system gave.
folders-remote-tls-could-not-set-up = تعذّر على Folders إعداد اتصال آمن: { $why }

