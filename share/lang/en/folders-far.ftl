# Words of Folders working with servers and other computers: the Connect to a server card, and the notes and failure reasons of copying, moving, deleting, renaming and opening things on a server.

folders-far-failed-why = { $name } ({ $why })
folders-far-no-server = no server called { $host } could be found
folders-far-not-all-arrived = not all of it arrived, so it is still where it was
folders-far-not-followed = a link, which is not followed, so it was not sent
folders-far-not-sent = not a file or a folder, so it was not sent
folders-far-not-told = the server will not say what it is, so it was left
folders-far-ftp-cannot-refuse = This server may give two files one name.
folders-far-cannot-do = That cannot be done to a server's files
folders-far-not-looked-at = it could not be looked at
folders-far-not-opened = It could not be opened.
folders-far-inside-itself = { $name } (a folder cannot go inside itself)
folders-far-taken-by-folder = { $name } (a folder of that name is there)
folders-far-taken-by-file = { $name } (a file of that name is there)
folders-far-address-unknown = The server's address is not known.
folders-far-copy-wrong-length = the copy sent was not the length it should be
folders-far-only-server-file = Only a file on a server is named this way.
folders-far-aside-both = the old one is there whole as { $old }, and the new one as { $new }: { $why }
folders-far-aside-kept = the old one could not be removed, and is there whole as { $old }
folders-far-aside-kept-why = the old one could not be removed, and is there whole as { $old }: { $why }
folders-far-not-empty = { $name } is not empty.
folders-far-unsafe-name = Skipped: the server sent an unsafe name.
folders-far-unsafe-names =
    { $n ->
        [one] Skipped: the server sent an unsafe name: { $names }.
       *[other] Skipped { $n }: the server sent an unsafe name: { $names }.
    }

folders-far-fetching = Fetching { $name }…
folders-far-cannot-fetch = { $name } cannot be fetched: { $why }.
folders-far-opened-copy = Opened a copy of { $name }. Saving it sends the changes back.
folders-far-saved = Saved { $name } on the server.
folders-far-not-saved = { $name } could not be saved on the server: { $why }. The copy here keeps the changes.
folders-far-renamed = Renamed { $was } to { $now }.

folders-far-connect-title = Connect to a server
folders-far-connect-close = Close
folders-far-connect-address = Address, such as sftp://jo@example.com
folders-far-connect-password = Password, if it needs one
folders-far-connect-key = Key file, if you pick one
folders-far-connect-pick-key = Choose the key file
folders-far-connect-choose = Choose
folders-far-connect-ssh-setup = Your own ssh set-up is used: its config, agent, keys and known servers.
folders-far-connect-new-host = { $host } has not been visited before, so Trindle cannot tell it is the right one. Trust it if you expected to reach it.
folders-far-connect-host-changed = { $host } no longer has the identity it had before. This can mean someone is in the way, so it is not opened. If its owner changed it, remove the old one from your known hosts first.
folders-far-connect-needs-password = This server wants a password. Type it above, and try again.
folders-far-connect-needs-key = This server wants a key or password. Name your key file above, or add it to your ssh agent, and try again.
folders-far-connect-cert-new = { $host }'s certificate could not be checked: { $why }. If you can, compare its fingerprint with the one the server's owner gives, and trust it only if they match.
folders-far-connect-cert-changed = { $host }'s certificate is not the one you trusted before. Its owner may have renewed it, or someone may be in the way. Trust the new one only if its owner confirms this fingerprint.
folders-far-connect-connecting = Connecting
folders-far-connect-trust = Trust it and connect
folders-far-connect-go = Connect
folders-far-phones = Phones show in the sidebar by themselves once they are plugged in and unlocked.
