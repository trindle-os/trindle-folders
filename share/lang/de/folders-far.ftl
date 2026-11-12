
# machine translated (German), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = Es gibt keinen Server namens { $host }

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = es ist nicht alles angekommen, deshalb wurde das Original nicht verschoben

# Reason a link was not sent to a server.
folders-far-not-followed = es ist ein Link, deshalb hat Folders es nicht gesendet

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = es ist weder Datei noch Ordner, deshalb hat Folders es nicht gesendet

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders konnte nicht erkennen, was es für ein Objekt ist, und hat es dort gelassen

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = Auf dieser Art von Server könnte eine Datei eine andere mit demselben Namen ersetzen.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = Das lässt sich mit Dateien auf einem Server nicht machen

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders konnte es nicht lesen

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders konnte es nicht öffnen.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (ein Ordner kann nicht in sich selbst liegen)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (es gibt schon einen Ordner mit diesem Namen)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (es gibt schon eine Datei mit diesem Namen)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders kennt die Adresse des Servers nicht.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = die gesendete Kopie hatte die falsche Größe

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = Das funktioniert nur mit Dateien auf einem Server.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders konnte die neue Datei nicht an ihren Platz legen. Die alte liegt noch dort und heißt jetzt { $old }, die neue heißt { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders konnte die alte Datei nicht entfernen. Sie liegt noch dort und heißt jetzt { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders konnte die alte Datei nicht entfernen, deshalb liegt sie noch dort und heißt jetzt { $old }: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } ist nicht leer.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders hat eine Datei übersprungen, weil ihr Name nicht sicher verwendbar ist.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
        [one] Folders hat einen Eintrag übersprungen, weil sein Name nicht sicher verwendbar ist: { $names }.
       *[other] Folders hat { $n } Einträge übersprungen, weil ihre Namen nicht sicher verwendbar sind: { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = { $name } wird heruntergeladen…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders konnte { $name } nicht herunterladen: { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = Eine Kopie von { $name } wurde geöffnet. Wenn du speicherst, gehen deine Änderungen zurück an den Server.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = { $name } auf dem Server gespeichert.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders konnte { $name } nicht auf dem Server speichern: { $why }. Deine Änderungen sind noch in der Kopie auf diesem Computer.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } wurde in { $now } umbenannt.

# Heading of the Connect to a server card.
folders-far-connect-title = Mit einem Server verbinden

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Schließen

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Adresse, zum Beispiel sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
# SAFETY: needs a human review before it ships
folders-far-connect-password = Passwort, falls nötig

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = Schlüsseldatei, falls du eine verwendest

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = Schlüsseldatei wählen


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders verwendet deine ssh-Einstellungen, deinen ssh-Agenten, deine Schlüssel und deine Liste bekannter Server.

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = Du hast dich noch nie mit { $host } verbunden, deshalb kann Folders nicht prüfen, ob es der richtige Server ist. Vertraue ihm nur, wenn du diese Verbindung erwartet hast.

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = { $host } hat eine andere Identität als beim letzten Mal. Möglicherweise versucht jemand, deine Verbindung abzufangen, deshalb hat Folders sich nicht verbunden. Wenn der Besitzer sie geändert hat, entferne zuerst den alten Eintrag aus deinen bekannten Hosts.

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-password = Dieser Server braucht ein Passwort. Gib es oben ein und versuche es erneut.

# Note on the Connect to a server card when a server asks for a key or password.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-key = Dieser Server braucht einen Schlüssel oder ein Passwort. Wähle oben deine Schlüsseldatei aus oder füge sie deinem ssh-Agenten hinzu, und versuche es dann erneut.

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = Folders konnte das Zertifikat von { $host } nicht prüfen: { $why }. Vergleiche, wenn möglich, den Fingerabdruck mit dem, den dir der Besitzer nennt. Vertraue ihm nur, wenn beide übereinstimmen.

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = Das Zertifikat von { $host } hat sich geändert, seit du ihm vertraut hast. Der Besitzer hat es vielleicht erneuert, oder jemand versucht, deine Verbindung abzufangen. Vertraue ihm nur, wenn der Besitzer diesen Fingerabdruck bestätigt.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Verbindung wird hergestellt

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = Vertrauen und verbinden

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Verbinden

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Telefone erscheinen in der Seitenleiste, wenn du sie anschließt und entsperrst.
