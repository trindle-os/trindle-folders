
# machine translated (German), not yet reviewed by a person.

# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-card = Folders hat { $host } im lokalen Netzwerk unter { $at }, Port { $port }, gefunden. Dort kann jedes Gerät diesen Namen benutzen, deshalb hat Folders dein Passwort nicht gesendet. Wenn das die richtige Adresse ist, drücke auf Verbinden.

# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-stopped = Folders hat { $host } im lokalen Netzwerk unter { $at } gefunden. Dort kann jedes Gerät diesen Namen benutzen, deshalb hat Folders dein Passwort nicht gesendet. Öffne { $host } erneut, um die Adresse zu prüfen.

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders konnte diese Adresse nicht lesen: { $why }.

# Red words when a server needs a password or key and none could be asked for.
# SAFETY: needs a human review before it ships
folders-remote-mod-needs-password = Dieser Server braucht ein Passwort oder einen Schlüssel.

# Red words when a server turned down the login.
# SAFETY: needs a human review before it ships
folders-remote-mod-login-refused = Der Server hat diese Anmeldung nicht akzeptiert.

# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-unknown = Du hast dich noch nie mit { $host } verbunden, deshalb kennt Folders seine Identität noch nicht.

# Red words when a server's identity differs from the one remembered; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-changed = { $host } hat eine andere Identität als beim letzten Mal. Er wurde vielleicht neu aufgebaut, oder jemand fängt die Verbindung ab.

# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-unknown = Folders konnte das Zertifikat von { $host } nicht prüfen: { $why }.

# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-changed = Das Zertifikat von { $host } hat sich geändert, seit du ihm vertraut hast.

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders konnte keine Verbindung herstellen: { $why }.

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders findet { $what } nicht.

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } ist schon da.

# Red words when a server or phone refused permission; the variable is what it was refused for.
# SAFETY: needs a human review before it ships
folders-remote-mod-denied = Für { $what } fehlt dir die Erlaubnis.

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } wird von einem anderen Programm benutzt.

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } wird nicht unterstützt.

# Red words when a copy or other work with a server was cancelled.
folders-remote-mod-cancelled = Abgebrochen.

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = Das Folgen von Verknüpfungen

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = Das Kopieren zu oder von einer Windows-Freigabe, bevor sie geöffnet ist,

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = Das Kopieren zu oder von einer NFS-Freigabe, bevor sie geöffnet ist,

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = am Anfang steht nicht sftp://, ftp:// oder mtp://

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = am Anfang muss sftp://, ftp://, ftps://, ftpes://, dav://, davs://, smb://, nfs:// oder mtp:// stehen

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = zu einer [ fehlt die ]

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = es nennt keinen Server

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = der Name des Servers ist ungültig

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = der Pfad enthält einen Zeilenumbruch

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = nach dem Namen des Servers steht noch etwas

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } ist keine gültige Portnummer

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = übersprungen, weil der Server einen unsicheren Namen gesendet hat

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = etwas ohne Namen

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = Die Antwort war viel zu groß, deshalb hat Folders sie gestoppt.

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } ist jetzt ein Ordner und wurde deshalb von Folders nicht entfernt.
