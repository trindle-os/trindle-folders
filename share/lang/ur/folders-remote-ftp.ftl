
# machine translated (Urdu), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = { $host } نام کا کوئی سرور موجود نہیں

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } کنکشن قبول نہیں کر رہا

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } نے وقت پر جواب نہیں دیا

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = نیٹ ورک پر { $host } تک پہنچنے کا کوئی راستہ نہیں

# SAFETY: needs a human review before it ships
# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
folders-remote-ftp-no-protect = یہ سرور فائلوں کی منتقلی کو محفوظ نہیں رکھ سکتا، اس لیے Folders نے کچھ نہیں بھیجا: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Folders کے جڑنے پر سرور نے توقع کے مطابق جواب نہیں دیا: { $said }

# SAFETY: needs a human review before it ships
# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = یہ سرور محفوظ FTP پیش نہیں کرتا۔ پتے کے شروع میں ftp:// لگا کر آزمائیں۔

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = محفوظ کنکشن

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = لاگ اِن

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = ڈیٹا کنکشن

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = بائنری ٹرانسفر موڈ

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = وہ فولڈر جس میں آپ شروع کرتے ہیں

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = FTP اکاؤنٹ سے لاگ ان کرنا

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders سرور کے ساتھ ڈیٹا کنکشن نہیں کھول سکا۔

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = محفوظ FTP

# Red words when the server stopped answering.
folders-remote-ftp-stopped = سرور نے جواب دینا بند کر دیا۔

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = سرور سے کنکشن ٹوٹ گیا۔

# Red words when the server closed the connection.
folders-remote-ftp-closed = سرور نے کنکشن بند کر دیا۔

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = اس سرور پر یہ کرنا

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = سرور پر { $what } کے ساتھ کچھ غلط ہو گیا: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = سرور نے ایسی چیز بھیجی جو FTP کا جواب نہیں
