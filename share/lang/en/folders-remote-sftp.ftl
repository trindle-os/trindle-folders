# Errors for SFTP servers (sftp://, through the ssh program) in Folders.

## SFTP servers (src/files/remote/sftp.rs)

# Red words when the server sent a message Folders could not read.
folders-remote-sftp-nonsense = The server sent a message Folders couldn't read.
# Red words when the ssh program is not on this computer.
folders-remote-sftp-no-ssh = The ssh program isn't installed.
# Red words when the ssh program could not be started.
folders-remote-sftp-ssh-failed = Folders couldn't start ssh.
# The thing that cannot be done, in "... isn't supported.": a server's old version of SFTP.
folders-remote-sftp-old = This server's older version of SFTP
# Red words when the connection to the server was lost.
folders-remote-sftp-lost = The connection to the server was lost.
# The thing asked of a server that it cannot do, at the start of "... isn't supported."
folders-remote-sftp-that-here = Doing that on this server
# Red words when the server refused something; the variable is the path it was about.
folders-remote-sftp-could-not = The server couldn't do that with { $path }.
# Red words when the server refused something and gave a reason; the variables are the path and the server's own words.
folders-remote-sftp-could-not-said = The server couldn't do that with { $path }: { $said }.
# Red words when the key file picked is missing; the variable is its path.
folders-remote-sftp-no-key = Folders can't find the key file { $key }.
# Reason in red words ("Could not reach it: ...") when a server's name is not known; the variable is the name.
folders-remote-sftp-no-server = there's no server called { $host }
# Reason in red words ("Could not reach it: ...") when the server refused the connection; the variable is its name.
folders-remote-sftp-turned-away = the connection to { $host } was refused
# Reason in red words ("Could not reach it: ...") when the server did not answer in time; the variable is its name.
folders-remote-sftp-no-answer = { $host } didn't answer in time
# Reason in red words ("Could not reach it: ...") when no route to the server exists; the variable is its name.
folders-remote-sftp-no-way = there's no way to reach { $host } on the network
# Reason in red words when the server does not offer SFTP at all.
folders-remote-sftp-none-here = This server doesn't offer SFTP.
# Red words passing on what the ssh program said; the variable is its last line.
folders-remote-sftp-ssh-said = Message from ssh: { $said }
