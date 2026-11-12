# machine translated (Hindi), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = { $host } नाम का कोई सर्वर नहीं है

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } कनेक्शन स्वीकार नहीं कर रहा

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } ने समय पर जवाब नहीं दिया

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = नेटवर्क पर { $host } तक पहुँचने का कोई रास्ता नहीं है

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
folders-remote-ftp-no-protect = यह सर्वर फ़ाइल ट्रांसफ़र को सुरक्षित नहीं रख सकता, इसलिए Folders ने कुछ नहीं भेजा: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Folders के जुड़ने पर सर्वर ने उम्मीद के मुताबिक जवाब नहीं दिया: { $said }

# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = यह सर्वर सुरक्षित FTP नहीं देता। पते की शुरुआत में ftp:// लगाकर देखें।

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = एक सुरक्षित कनेक्शन

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = लॉगिन

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = एक डेटा कनेक्शन

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = बाइनरी ट्रांसफ़र मोड

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = वह फ़ोल्डर जिसमें आप शुरू होते हैं

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = FTP खाते से लॉगिन करना

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders सर्वर से डेटा कनेक्शन नहीं खोल सका।

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = सुरक्षित FTP

# Red words when the server stopped answering.
folders-remote-ftp-stopped = सर्वर ने जवाब देना बंद कर दिया।

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = सर्वर से कनेक्शन टूट गया।

# Red words when the server closed the connection.
folders-remote-ftp-closed = सर्वर ने कनेक्शन बंद कर दिया।

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = इस सर्वर पर ऐसा करना

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = सर्वर पर { $what } में कुछ गड़बड़ हुई: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = सर्वर ने ऐसा कुछ भेजा जो FTP का जवाब नहीं है
