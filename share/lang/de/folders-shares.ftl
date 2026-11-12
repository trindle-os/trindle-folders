
# machine translated (German), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = Dieser Computer darf { $export } auf { $server } nicht öffnen.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } gibt keinen Ordner namens { $export } frei.

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = Du wirst beim nächsten Mal wieder gefragt

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = Weniger als ein Tag übrig

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] { $n } Tag übrig
       *[other] { $n } Tage übrig
    }

# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
# SAFETY: needs a human review before it ships
folders-shares-held =
    { $past ->
        [yes] { $server } ist jetzt unter { $now } zu finden. Dein Login dafür wurde zuvor unter einer anderen Adresse verwendet ({ $before }).
       *[no] { $server } ist unter { $now } zu finden. Dein Login dafür wurde unter dieser Adresse noch nie verwendet.
    }{ $full ->
        [yes] {" "}Folders merkt sich nur die letzten { $most } Adressen.
       *[no] {""}
    } Jedes Gerät in einem Netzwerk kann einen Namen für sich beanspruchen, darum hat Folders deinen gespeicherten Login nicht gesendet. Wenn dies dein { $server } ist, gib Benutzername und Passwort ein, um ihn hier zu verwenden.

# Words on the sign-in card when no address could be found for the computer to show.
# SAFETY: needs a human review before it ships
folders-shares-nowhere = Folders konnte die Adresse von { $server } nicht finden, darum wurde nichts dorthin gesendet. Was du hier eingibst, kann nicht sicher gesendet werden. Öffne ihn stattdessen über seine Adresse, zum Beispiel smb://192.168.1.20/.

# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
# SAFETY: needs a human review before it ships
folders-shares-lapsed = { $server } ist unter { $now } zu finden. Dein Login dafür wurde dort zuletzt vor mehr als { $days } Tagen verwendet. Seitdem kann ein anderes Gerät diese Adresse bekommen haben, darum hat Folders deinen gespeicherten Login nicht gesendet. Wenn dies dein { $server } ist, gib Benutzername und Passwort ein, um ihn hier zu verwenden.

# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
# SAFETY: needs a human review before it ships
folders-shares-unknown = { $server } ist unter { $now } zu finden. Dort wurde früher ein Login dafür verwendet, aber Folders hat nicht notiert, wann oder welcher. Seitdem kann ein anderes Gerät diese Adresse bekommen haben, darum hat Folders deinen gespeicherten Login nicht gesendet. Wenn dies dein { $server } ist, gib Benutzername und Passwort ein, um ihn hier zu verwenden.

# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
# SAFETY: needs a human review before it ships
folders-shares-unshown = { $server } ist unter { $now } zu finden, wo dein Login dafür früher verwendet wurde. Diesmal hat Folders die Adresse im lokalen Netzwerk gefunden, wo jedes Gerät diesen Namen benutzen kann. Du hast diese Adresse noch nie gesehen, darum hat Folders deinen Login nicht gesendet. Wenn dies dein { $server } ist, gib Benutzername und Passwort ein, um ihn hier zu verwenden.

# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
# SAFETY: needs a human review before it ships
folders-shares-asking =
    Gib Benutzername und Passwort für ein Konto auf { $server } ein.{ $heard ->
        [yes] {" "}Er ist unter { $at } zu finden. Jedes Gerät in einem Netzwerk kann einen Namen für sich beanspruchen, prüfe also, ob das seine Adresse ist, bevor du etwas eingibst.
       *[no] {""}
    }

# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
# SAFETY: needs a human review before it ships
folders-shares-where =
    Was du eingibst, geht an { $address }{ $heard ->
        [yes] {" "}und an niemanden sonst. Mit „Merken“ speichert Folders deinen Login nur für diese Adresse.
       *[no] {"."}
    }

# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
# SAFETY: needs a human review before it ships
folders-shares-again = Folders hat { $server } im lokalen Netzwerk unter { $at }, Port { $port }, gefunden. Dort kann jedes Gerät diesen Namen benutzen, deshalb hat Folders deinen Login nicht gesendet. Wenn das die richtige Adresse ist, drücke auf Öffnen.

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders konnte den Schlüsselbund nicht erreichen: { $why }.

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = Er ist geöffnet, aber Folders konnte den Login nicht speichern. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } bietet kein NFS Version 4 an, die einzige Version, die Folders öffnen kann.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = Die Freigabe wurde geöffnet, aber Folders konnte nicht herausfinden, wo sie sich auf diesem Computer befindet.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = Die Freigabe wurde geöffnet, aber Folders konnte nicht herausfinden, wo sie sich auf diesem Computer befindet.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = Das ist keine Adresse.

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = Adressen, die für smb://{ $server } verwendet wurden

# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
# SAFETY: needs a human review before it ships
folders-shares-password-label = Passwort für { $user } auf { $what }

# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
# SAFETY: needs a human review before it ships
folders-shares-remember = 90 Tage merken

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = Die 90 Tage beginnen bei jeder Nutzung neu.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } auf { $server }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = { $it } öffnen

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Schließen

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = Benutzername, zum Beispiel jo oder WORKGROUP\jo

# Hint in the empty password box of the sign-in card.
# SAFETY: needs a human review before it ships
folders-shares-password = Passwort

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
# SAFETY: needs a human review before it ships
folders-shares-forget = Frühere Adressen vergessen

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
# SAFETY: needs a human review before it ships
folders-shares-forget-spoken = Frühere Adressen für diesen Login vergessen

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Wird geöffnet …

# Name of the main button of the sign-in card.
folders-shares-open = Öffnen

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = Das ist kein Computername.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = Das ist kein Pfad zu einem freigegebenen Ordner.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = { $export } auf { $host } wird geöffnet …

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = { $share } auf { $server } wird geöffnet …

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Schließe zuerst die Anmeldung bei { $it } ab.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = Es ist geöffnet. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } gibt keine Ordner für dich frei.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = { $name } wird geschlossen …

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders konnte { $name } nicht schließen: { $why }.


# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = { $name } wird erneut verbunden …

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders konnte deine Freigaben nicht für das nächste Mal speichern.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = Dieser Benutzername kann für eine Freigabe nicht verwendet werden.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = Diese Domain kann für eine Freigabe nicht verwendet werden.

# Red words when a share's password holds a line break.
folders-shares-bad-password = Das Passwort einer Freigabe darf keinen Zeilenumbruch enthalten.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } braucht einen Benutzernamen und ein Passwort für { $it }.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = Benutzername oder Passwort für { $it } sind falsch.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = Dein Konto auf { $server } ist gesperrt oder abgeschaltet, oder sein Passwort ist abgelaufen.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } hat keinen freigegebenen Ordner namens { $share }.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = Dieses Konto darf { $it } nicht öffnen.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } hat nicht geantwortet. Vielleicht schläft er oder ist ausgeschaltet. Wecke ihn auf und versuche es dann erneut.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders konnte in diesem Netzwerk keinen Computer namens { $server } finden.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } verwendet nur SMB1, eine alte Art der Freigabe, die nicht mehr sicher ist. Folders verwendet sie nicht.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = Dafür wird das Debian-Paket { $package } gebraucht, das nicht installiert ist.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders konnte die Freigabe nicht öffnen.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders konnte den freigegebenen Ordner nicht öffnen.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders konnte die freigegebenen Ordner nicht auflisten.
