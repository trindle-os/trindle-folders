# Errors and notes for FTP, FTPS and FTPES servers in Folders.

## FTP servers (src/files/remote/ftp.rs)

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = there's no server called { $host }
# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } isn't accepting connections
# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } didn't answer in time
# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = there's no way to reach { $host } on the network
# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
folders-remote-ftp-no-protect = This server can't keep file transfers secure, so Folders didn't send anything: { $said }
# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = The server didn't reply as expected when Folders connected: { $said }
# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = This server doesn't offer secure FTP. Try the address with ftp:// at the start.
# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = a secure connection
# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = the login
# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = a data connection
# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = binary transfer mode
# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = the folder you start in
# The thing the server asks for that Folders cannot do, in "... isn't supported.": FTP accounts.
folders-remote-ftp-accounts = Logging in with an FTP account
# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders couldn't open a data connection to the server.
# The thing a server wants first, in "... isn't supported.": secure FTP.
folders-remote-ftp-secure-ftp = Secure FTP
# Red words when the server stopped answering.
folders-remote-ftp-stopped = The server stopped answering.
# Red words when the connection to the server was lost.
folders-remote-ftp-lost = The connection to the server was lost.
# Red words when the server closed the connection.
folders-remote-ftp-closed = The server closed the connection.
# The thing asked of a server that it cannot do, at the start of "... isn't supported."
folders-remote-ftp-that-here = Doing that on this server
# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Something went wrong with { $what } on the server: { $said }
# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = the server sent something that isn't an FTP reply
