# machine translated (Swahili), not yet reviewed by a person.

# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-ftp-no-server = hakuna seva iitwayo { $host }

# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-ftp-turned-away = { $host } haikubali miunganisho

# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-ftp-no-answer = { $host } haikujibu kwa wakati

# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-ftp-no-way = hakuna njia ya kufika { $host } kwenye mtandao

# Red words when the server will not protect file transfers on a secure connection; the variable is the server's own last line.
folders-remote-ftp-no-protect = Seva hii haiwezi kulinda uhamishaji wa faili, kwa hiyo Folders haikutuma chochote: { $said }

# Red words when the server did not greet Folders as expected; the variable is the server's own words.
folders-remote-ftp-no-welcome = Seva haikujibu kama ilivyotarajiwa Folders ilipounganisha: { $said }

# Red words when the server does not offer secure FTP.
folders-remote-ftp-no-secure = Seva hii haitoi FTP salama. Jaribu anwani yenye ftp:// mwanzoni.

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: setting up a secure connection.
folders-remote-ftp-what-secure = muunganisho salama

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the login.
folders-remote-ftp-what-login = kuingia

# Name of the thing that failed, in "Permission was refused for ..." and similar red words: the connection that carries the data.
folders-remote-ftp-what-data = muunganisho wa data

# Name of the thing that failed, in red words: switching the server to binary mode.
folders-remote-ftp-what-binary = hali ya uhamishaji wa binary

# Name of the thing that failed, in red words: finding the folder the login opens in.
folders-remote-ftp-what-home = folda unayoanzia

# The thing the server asks for that Folders cannot do, in "... is not supported yet.": FTP accounts.
folders-remote-ftp-accounts = Kuingia kwa akaunti ya FTP

# Red words when the data connection to the server could not be opened.
folders-remote-ftp-data-failed = Folders haikuweza kufungua muunganisho wa data na seva.

# The thing a server wants first, in "... is not supported yet.": secure FTP.
folders-remote-ftp-secure-ftp = FTP salama

# Red words when the server stopped answering.
folders-remote-ftp-stopped = Seva iliacha kujibu.

# Red words when the connection to the server was lost.
folders-remote-ftp-lost = Muunganisho na seva umepotea.

# Red words when the server closed the connection.
folders-remote-ftp-closed = Seva ilifunga muunganisho.

# The thing asked of a server that it cannot do, at the start of "... is not supported yet."
folders-remote-ftp-that-here = Kufanya hilo kwenye seva hii

# Red words when the server refused something; the variables are what it was about and the server's own words.
folders-remote-ftp-could-not = Kuna tatizo na { $what } kwenye seva: { $said }

# Reason in red words when the server sent something that is not an FTP reply.
folders-remote-ftp-not-reply = seva ilituma kitu ambacho si jibu la FTP
