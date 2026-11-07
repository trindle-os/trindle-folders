

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = Es wurde kein Server namens { $host } gefunden

folders-far-not-all-arrived = es ist nicht alles angekommen, deshalb liegt die Datei noch dort, wo sie war

folders-far-not-followed = ein Link, dem nicht gefolgt wird, deshalb wurde er nicht gesendet

folders-far-not-sent = weder Datei noch Ordner, deshalb wurde es nicht gesendet

folders-far-not-told = der Server sagt nicht, was es ist, deshalb wurde es übergangen

folders-far-ftp-cannot-refuse = Dieser Server kann zwei Dateien denselben Namen geben.

folders-far-cannot-do = Das lässt sich mit Dateien auf einem Server nicht machen

folders-far-not-looked-at = es konnte nicht angesehen werden

folders-far-not-opened = Es konnte nicht geöffnet werden.

folders-far-inside-itself = { $name } (ein Ordner kann nicht in sich selbst liegen)

folders-far-taken-by-folder = { $name } (dort liegt ein Ordner mit diesem Namen)

folders-far-taken-by-file = { $name } (dort liegt eine Datei mit diesem Namen)

folders-far-address-unknown = Die Adresse des Servers ist nicht bekannt.

folders-far-copy-wrong-length = die gesendete Kopie hatte nicht die richtige Länge

folders-far-only-server-file = Nur eine Datei auf einem Server kann so benannt werden.

folders-far-aside-both = die alte liegt unversehrt als { $old } dort, die neue als { $new }: { $why }

folders-far-aside-kept = die alte konnte nicht entfernt werden und liegt unversehrt als { $old } dort

folders-far-aside-kept-why = die alte konnte nicht entfernt werden und liegt unversehrt als { $old } dort: { $why }

folders-far-not-empty = { $name } ist nicht leer.

folders-far-unsafe-name = Übersprungen: Der Server hat einen unsicheren Namen gesendet.

folders-far-unsafe-names =
    { $n ->
        [one] Übersprungen: Der Server hat einen unsicheren Namen gesendet: { $names }.
       *[other] { $n } übersprungen: Der Server hat unsichere Namen gesendet: { $names }.
    }

folders-far-fetching = { $name } wird abgerufen…

folders-far-cannot-fetch = { $name } kann nicht abgerufen werden: { $why }.

folders-far-opened-copy = Eine Kopie von { $name } wurde geöffnet. Beim Speichern werden die Änderungen zurückgesendet.

folders-far-saved = { $name } auf dem Server gespeichert.

folders-far-not-saved = { $name } konnte nicht auf dem Server gespeichert werden: { $why }. Die Kopie hier behält die Änderungen.

folders-far-renamed = { $was } wurde in { $now } umbenannt.

folders-far-connect-title = Mit einem Server verbinden

folders-far-connect-close = Schließen

folders-far-connect-address = Adresse, zum Beispiel sftp://jo@example.com

folders-far-connect-password = Passwort, falls nötig

folders-far-connect-key = Schlüsseldatei, falls du eine wählst

folders-far-connect-pick-key = Schlüsseldatei wählen

folders-far-connect-choose = Wählen

folders-far-connect-ssh-setup = Dein eigenes ssh-Setup wird verwendet: Konfiguration, Agent, Schlüssel und bekannte Server.

folders-far-connect-new-host = { $host } wurde noch nie besucht, daher kann Trindle nicht erkennen, ob es der richtige ist. Vertraue ihm nur, wenn du ihn erwartet hast.

folders-far-connect-host-changed = { $host } hat nicht mehr die Identität von früher. Das kann bedeuten, dass sich jemand dazwischenschaltet, deshalb wird er nicht geöffnet. Wenn der Besitzer sie geändert hat, entferne zuerst den alten Eintrag aus deinen bekannten Hosts.

folders-far-connect-needs-password = Dieser Server verlangt ein Passwort. Gib es oben ein und versuche es erneut.

folders-far-connect-needs-key = Dieser Server verlangt einen Schlüssel oder ein Passwort. Nenne oben deine Schlüsseldatei oder füge sie deinem ssh-Agenten hinzu und versuche es erneut.

folders-far-connect-cert-new = Das Zertifikat von { $host } konnte nicht geprüft werden: { $why }. Vergleiche, wenn möglich, den Fingerabdruck mit dem, den der Besitzer des Servers nennt, und vertraue ihm nur, wenn beide übereinstimmen.

folders-far-connect-cert-changed = Das Zertifikat von { $host } ist nicht das, dem du früher vertraut hast. Der Besitzer hat es vielleicht erneuert, oder jemand schaltet sich dazwischen. Vertraue dem neuen nur, wenn der Besitzer diesen Fingerabdruck bestätigt.

folders-far-connect-connecting = Verbindung wird hergestellt

folders-far-connect-trust = Vertrauen und verbinden

folders-far-connect-go = Verbinden

folders-far-phones = Telefone erscheinen von selbst in der Seitenleiste, sobald sie angeschlossen und entsperrt sind.
