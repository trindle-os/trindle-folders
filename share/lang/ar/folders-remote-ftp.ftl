
# machine translated (Arabic), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = لا يوجد خادم باسم { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } لا يقبل الاتصالات

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = لم يردّ { $host } في الوقت المناسب

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = لا توجد طريقة للوصول إلى { $host } على الشبكة

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
# SAFETY: needs a human review before it ships
folders-remote-ftp-no-protect = لا يستطيع هذا الخادم تأمين نقل الملفات، لذلك لم يرسل Folders أي شيء: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = لم يردّ الخادم بالشكل المتوقع عندما اتصل به Folders: { $said }

# Red words when the server does not offer secure FTP.
# SAFETY: needs a human review before it ships
folders-remote-ftp-no-secure = هذا الخادم لا يقدّم FTP آمناً. جرّب العنوان مع ftp:// في بدايته.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = اتصال آمن

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
# SAFETY: needs a human review before it ships
folders-remote-ftp-what-login = الدخول

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = اتصال البيانات

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = وضع النقل الثنائي

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = المجلد الذي تبدأ منه

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = الدخول بحساب FTP

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = تعذّر على Folders فتح اتصال بيانات مع الخادم.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = FTP الآمن

# Red words when the server stopped answering.
folders-remote-ftp-stopped = توقف الخادم عن الاستجابة.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = انقطع الاتصال بالخادم.

# Red words when the server closed the connection.
folders-remote-ftp-closed = أغلق الخادم الاتصال.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = تنفيذ ذلك على هذا الخادم

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = حدث خطأ في { $what } على الخادم: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = أرسل الخادم شيئاً ليس ردّ FTP

