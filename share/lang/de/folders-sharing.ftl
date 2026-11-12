
# machine translated (German), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders kann nicht lesen, was in diesem Ordner liegt.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = Du kannst nur einen Ordner selbst freigeben, keinen Link auf einen Ordner.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = Du kannst nur Ordner freigeben, die dir gehören. Dieser gehört jemand anderem.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = dein Home-Ordner

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = dieser Ordner

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } und { $second }

# Name of the switch that shares the folder.
# SAFETY: needs a human review before it ships
folders-sharing-share = Diesen Ordner freigeben

# One line under the share switch when the folder is shared.
folders-sharing-shared = Im Heimnetzwerk freigegeben.


# Tip of the share switch.
# SAFETY: needs a human review before it ships
folders-sharing-share-tip = Im Heimnetzwerk freigeben

# Label over the box for the name other computers see.
folders-sharing-name = Name

# Tip of the box for the name other computers see.
folders-sharing-name-tip = Der Name, den andere Computer sehen

# Name of the switch that lets others change files, and its tip.
# SAFETY: needs a human review before it ships
folders-sharing-write = Anderen erlauben, Dateien zu ändern

# One line under the switch that lets others change files.
# SAFETY: needs a human review before it ships
folders-sharing-write-sub = Sie können Dateien darin hinzufügen, ändern und löschen.

# Name of the switch that lets anyone in without a password.
# SAFETY: needs a human review before it ships
folders-sharing-guests = Jeden hereinlassen

# One line under the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-sub = Kein Passwort nötig.

# Tip of the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-tip = Jeden ohne Passwort hereinlassen

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Sobald er freigegeben ist, erscheint er unter Netzwerk auf deinen anderen Computern zu Hause, auch auf Windows-Computern. Beim ersten Mal lädt Trindle Samba herunter, das Programm, das Ordner freigibt. Das dauert eine Minute.
       *[no] Sobald er freigegeben ist, erscheint er unter Netzwerk auf deinen anderen Computern zu Hause, auch auf Windows-Computern.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] Suche auf anderen Computern unter Netzwerk nach { $name } auf { $host }.
       *[no] Suche auf anderen Computern unter Netzwerk nach { $name }.
    }

# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = Andere Computer können diesen Ordner noch nicht erreichen. Öffne in den Einstellungen „Netzwerk“ und schalte „Andere Computer verbinden lassen“ ein.

# Tip of the button that lets other computers connect.
# SAFETY: needs a human review before it ships
folders-sharing-let-connect = Andere Computer verbinden lassen

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
folders-sharing-visitors =
    { $n ->
        [one] Gäste ohne Passwort können ihn noch nicht öffnen, weil { $names } für andere Konten gesperrt ist. Wenn du sie durchlässt, können sie diesen Ordner erreichen, aber nichts anderes auf dem Weg dorthin sehen.
       *[other] Gäste ohne Passwort können ihn noch nicht öffnen, weil { $names } für andere Konten gesperrt sind. Wenn du sie durchlässt, können sie diesen Ordner erreichen, aber nichts anderes auf dem Weg dorthin sehen.
    }

# Tip of the button that lets visitors through to the folder.
# SAFETY: needs a human review before it ships
folders-sharing-let-through = Gäste zu diesem Ordner durchlassen

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
# SAFETY: needs a human review before it ships
folders-sharing-sign-in-set = Andere melden sich als { $user } mit dem Freigabepasswort an.

# Tip of the button that changes the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-change-password = Freigabepasswort ändern

# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-set-password = Andere melden sich als { $user } an. Lege ein Freigabepasswort mit mindestens { $n } Zeichen fest. Es ist von deinem Anmeldepasswort getrennt.

# Label beside the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password = Passwort

# Tip of the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password-tip = Freigabepasswort

# Tip of the button that keeps the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-keep-password = Passwort speichern

# Title of the Share on the network window; { $name } is the folder's name.
# SAFETY: needs a human review before it ships
folders-sharing-card-title = { $name } im Netzwerk freigeben

# Red words when the share's name is used by another share.
folders-sharing-name-taken = Du hast schon einen freigegebenen Ordner mit diesem Namen. Wähle einen anderen.

# Calm words while sharing is being stopped.
folders-sharing-stopping = Die Freigabe wird beendet…

# Red words when the sharing password is too short; { $n } is the fewest letters.
folders-sharing-password-short = Das Passwort braucht mindestens { $n } Zeichen.

# Calm words while the sharing password is being kept.
# SAFETY: needs a human review before it ships
folders-sharing-keeping-password = Das Passwort wird gespeichert. Zuerst wirst du nach dem Passwort des Administrators gefragt.

# Calm words while other computers are being let connect.
# SAFETY: needs a human review before it ships
folders-sharing-letting-connect = Andere Computer dürfen sich verbinden. Zuerst wirst du nach dem Passwort des Administrators gefragt.

# Calm words while Samba is fetched and set up the first time.
# SAFETY: needs a human review before it ships
folders-sharing-fetching = Samba, das Programm, das Ordner freigibt, wird heruntergeladen und eingerichtet. Zuerst wirst du nach dem Passwort des Administrators gefragt. Das passiert nur beim ersten Mal.

# Calm words while sharing is set up the first time and Samba is already there.
# SAFETY: needs a human review before it ships
folders-sharing-setting-up = Die Freigabe wird eingerichtet. Zuerst wirst du nach dem Passwort des Administrators gefragt. Das passiert nur beim ersten Mal.

# Calm words while an existing share is changed.
folders-sharing-changing = Die Freigabe wird geändert…

# Calm words while a share is made.
folders-sharing-sharing-it = Der Ordner wird freigegeben …

# Red words under the name box when a share has no name.
folders-sharing-name-empty = Gib dem freigegebenen Ordner einen Namen.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = Der Name eines freigegebenen Ordners darf höchstens { $most } Zeichen lang sein.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = Der Name eines freigegebenen Ordners darf keines der Zeichen % < > * ? | / \ + = ; : " oder ein Komma enthalten.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = Ein Name, der auf $ endet, würde die Freigabe verbergen, deshalb kannst du ihn nicht verwenden.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = Der Name eines freigegebenen Ordners darf nicht mit einem Bindestrich beginnen.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba verwendet den Namen { $name } bereits. Wähle einen anderen.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Freigegeben

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = Du kannst nur Ordner freigeben, die dir gehören.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = Dieser Name ist auf diesem Computer schon ein Kontoname, deshalb kannst du ihn nicht für eine Freigabe verwenden. Wähle einen anderen.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = Die Samba-Einstellungen dieses Computers erlauben keine Freigaben ohne Passwort.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = Dieser Computer kann keine weiteren Ordner freigeben.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = Dieser Ordner ist nicht mehr da.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba hat keinen Grund genannt.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Meldung von Samba: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } gehört jemand anderem, deshalb kannst du von hier aus keinen Gästen Zugriff darauf geben.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders konnte Gästen keinen Zugriff auf { $path } geben: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle konnte Samba nicht herunterladen, weil dieser Computer offenbar offline ist. Verbinde dich mit dem Internet und versuche es erneut.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Trindle konnte Samba nicht installieren, solange etwas anderes installiert wird. Versuche es in ein paar Minuten erneut.
