
# machine translated (Swahili), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = hakuna seva iitwayo { $host }

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = haikufika yote, kwa hiyo ya asili haijahamishwa

# Reason a link was not sent to a server.
folders-far-not-followed = ni kiungo, kwa hiyo Folders haikukituma

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = si faili wala folda, kwa hiyo Folders haikukituma

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders haikuweza kujua ni aina gani ya kitu, kwa hiyo ilikiacha hapo

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = Kwenye aina hii ya seva, faili inaweza kuchukua nafasi ya faili nyingine yenye jina lilelile.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = Huwezi kufanya hilo kwa faili zilizo kwenye seva

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders haikuweza kuisoma

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders haikuweza kuifungua.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (folda haiwezi kuingia ndani yake yenyewe)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (tayari kuna folda yenye jina hilo)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (tayari kuna faili yenye jina hilo)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders haijui anwani ya seva.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = nakala iliyotumwa ilikuwa na ukubwa usio sahihi

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = Hili linafanya kazi na faili zilizo kwenye seva tu.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders haikuweza kuweka faili mpya mahali pake, kwa hiyo ya zamani bado iko, sasa inaitwa { $old }, na mpya inaitwa { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders haikuweza kuondoa faili ya zamani. Bado iko, sasa inaitwa { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders haikuweza kuondoa faili ya zamani, kwa hiyo bado iko, sasa inaitwa { $old }: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
# SAFETY: needs a human review before it ships
folders-far-not-empty = { $name } si tupu.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders iliruka faili kwa sababu jina lake si salama kutumia.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
        [one] Folders iliruka kitu kimoja kwa sababu jina lake si salama kutumia: { $names }.
       *[other] Folders iliruka vitu { $n } kwa sababu majina yake si salama kutumia: { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = Inapakua { $name }…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders haikuweza kupakua { $name }: { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = Nakala ya { $name } imefunguliwa. Ukiihifadhi, mabadiliko yako yatarudi kwenye seva.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = { $name } imehifadhiwa kwenye seva.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders haikuweza kuhifadhi { $name } kwenye seva: { $why }. Mabadiliko yako bado yako kwenye nakala iliyo kwenye kompyuta hii.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } imepewa jina { $now }.

# Heading of the Connect to a server card.
folders-far-connect-title = Unganisha kwenye seva

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Funga

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Anwani, kama sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
# SAFETY: needs a human review before it ships
folders-far-connect-password = Nenosiri, kama linahitajika

# Placeholder of the box on the Connect to a server card where the key file is named.
# SAFETY: needs a human review before it ships
folders-far-connect-key = Faili ya ufunguo, kama unatumia

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
# SAFETY: needs a human review before it ships
folders-far-connect-pick-key = Chagua faili ya ufunguo


# Note on the Connect to a server card saying where the connection settings come from.
# SAFETY: needs a human review before it ships
folders-far-connect-ssh-setup = Folders inatumia mipangilio yako ya ssh, wakala wa ssh, funguo na orodha ya seva zinazojulikana.

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = Hujawahi kuunganisha kwenye { $host } hapo awali, kwa hiyo Folders haiwezi kuthibitisha kama ndiyo sahihi. Iamini tu ikiwa ndiyo uliyokusudia kuunganisha.

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = { $host } ina utambulisho tofauti na mara ya mwisho. Huenda mtu anajaribu kuingilia muunganisho wako, kwa hiyo Folders haikuunganisha. Ikiwa mmiliki aliubadilisha, ondoa ingizo lake la zamani kwenye seva zako zinazojulikana kwanza.

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-password = Seva hii inahitaji nenosiri. Liandike hapo juu kisha ujaribu tena.

# Note on the Connect to a server card when a server asks for a key or password.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-key = Seva hii inahitaji ufunguo au nenosiri. Chagua faili yako ya ufunguo hapo juu, au iongeze kwenye wakala wako wa ssh, kisha ujaribu tena.

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = Folders haikuweza kukagua cheti cha { $host }: { $why }. Ikiwezekana, linganisha alama yake ya vidole na ile anayokupa mmiliki. Kiamini tu ikiwa zinalingana.

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = Cheti cha { $host } kimebadilika tangu ulipokiamini. Mmiliki anaweza kuwa alikifanya upya, au mtu anaweza kuwa anajaribu kuingilia muunganisho wako. Kiamini tu ikiwa mmiliki anathibitisha alama hii ya vidole.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Inaunganisha

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = Iamini kisha uunganishe

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Unganisha

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Simu huonekana kwenye utepe wa pembeni unapozichomeka na kuzifungua.
