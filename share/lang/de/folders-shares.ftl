

folders-shares-nfs-denied = { $server } erlaubt diesem Computer nicht, { $export } zu öffnen.

folders-shares-nfs-no-share = { $server } gibt nichts mit dem Namen { $export } frei.

folders-shares-left-none = Fragt beim nächsten Öffnen erneut

folders-shares-left-under-day = Weniger als ein Tag übrig

folders-shares-left-days =
    { $n ->
        [one] { $n } Tag übrig
       *[other] { $n } Tage übrig
    }

folders-shares-held =
    { $past ->
        [yes] { $server } antwortet jetzt unter { $now }, einer anderen Adresse als der, unter der dein Login dafür zuvor verwendet wurde ({ $before }).
       *[no] { $server } antwortet unter { $now }, einer Adresse, unter der dein Login dafür noch nie verwendet wurde.
    }{ $full ->
        [yes] {" "}Es werden nur die letzten { $most } Adressen gemerkt, unter denen er verwendet wurde.
       *[no] {""}
    } Jedes Gerät in einem Netzwerk kann auf einen Namen antworten, darum wurde der dafür gespeicherte Login nicht gesendet. Wenn dies dein { $server } ist, gib Benutzername und Passwort ein, um ihn hier zu verwenden.

folders-shares-nowhere = Die Adresse, unter der { $server } antwortet, konnte vorher nicht gefunden werden. Darum wurde nichts dorthin gesendet, und was hier eingegeben wird, kann nirgends sicher hin. Öffne ihn stattdessen über seine Adresse, zum Beispiel smb://192.168.1.20/.

folders-shares-lapsed = { $server } antwortet unter { $now }, wo dein Login dafür zuletzt vor mehr als { $days } Tagen verwendet wurde. Seitdem kann ein anderes Gerät diese Adresse bekommen haben, darum wurde der dafür gespeicherte Login nicht gesendet. Wenn dies dein { $server } ist, gib Benutzername und Passwort ein, um ihn hier zu verwenden.

folders-shares-unknown = { $server } antwortet unter { $now }, wo früher ein Login dafür verwendet wurde, aber wann oder welcher Login, wurde nicht notiert. Seitdem kann ein anderes Gerät diese Adresse bekommen haben, darum wurde der dafür gespeicherte Login nicht gesendet. Wenn dies dein { $server } ist, gib Benutzername und Passwort ein, um ihn hier zu verwenden.

folders-shares-unshown = { $server } antwortet unter { $now }, der Adresse, an die dein Login dafür früher ging. Diesmal wurde diese Adresse nur im Netzwerk gehört, wo jedes Gerät auf einen Namen antworten kann, und sie wurde dir hier noch nie gezeigt, darum wurde der Login nicht gesendet. Wenn dies dein { $server } ist, gib Benutzername und Passwort ein, um ihn hier zu verwenden.

folders-shares-asking =
    { $server } möchte einen Benutzernamen und ein Passwort. Es sind die für ein Konto auf diesem Computer.{ $heard ->
        [yes] {" "}Er antwortet unter { $at }. Jedes Gerät in einem Netzwerk kann auf einen Namen antworten, prüfe also, ob das seine Adresse ist, bevor du etwas eingibst.
       *[no] {""}
    }

folders-shares-where =
    Was du eingibst, geht an { $address }{ $heard ->
        [yes] {" "}und an niemanden sonst. Mit „Merken“ wird dein Login nur für diese Adresse gespeichert.
       *[no] {"."}
    }

folders-shares-again = { $server } antwortet unter { $at }, Port { $port }. Diese Adresse wurde nur im Netzwerk gehört, wo jedes Gerät auf einen Namen antworten kann, darum wurde dein Login dafür nicht gesendet. Wenn das seine Adresse ist, drücke Öffnen.

folders-shares-keyring-unreachable = Der Schlüsselbund war nicht erreichbar: { $why }.

folders-shares-unkept = Er ist geöffnet, aber der Login wurde nicht gespeichert. { $why }

folders-shares-nfs-version = { $server } bietet kein NFS Version 4 an, die einzige Art, die Trindle öffnet.

folders-shares-nfs-opened-unknown = Es wurde geöffnet, aber wo, ist nicht bekannt.

folders-shares-opened-unknown = Die Freigabe wurde geöffnet, aber wo, ist nicht bekannt.

folders-shares-not-address = Das ist keine Adresse.

folders-shares-seen-label = Adressen, unter denen smb://{ $server } geöffnet wurde

folders-shares-password-label = Passwort für { $user } auf { $what }

folders-shares-remember = 90 Tage merken

folders-shares-remember-line = Beginnt bei jeder Nutzung neu.

folders-shares-share-on = { $share } auf { $server }

folders-shares-card-title = { $it } öffnen

folders-shares-close = Schließen

folders-shares-user-placeholder = Benutzername, zum Beispiel jo, oder WORKGROUP\jo

folders-shares-password = Passwort

folders-shares-forget = Die früheren Adressen vergessen

folders-shares-forget-spoken = Die Adressen vergessen, unter denen dieser Login früher verwendet wurde

folders-shares-opening = Wird geöffnet

folders-shares-open = Öffnen

folders-shares-bad-computer = Das ist nicht der Name eines Computers.

folders-shares-bad-path = Das ist nicht der Pfad eines freigegebenen Ordners.

folders-shares-opening-export = { $export } auf { $host } wird geöffnet …

folders-shares-opening-share = { $share } auf { $server } wird geöffnet …

folders-shares-answer-card = Beantworte zuerst die Karte für { $it }.

folders-shares-open-waiting = Es ist geöffnet. { $words }

folders-shares-none = { $server } gibt keine Ordner für dich frei.

folders-shares-closing = { $name } wird geschlossen …

folders-shares-close-failed = { $name } konnte nicht geschlossen werden: { $why }.

folders-shares-reconnecting = { $name } wird erneut verbunden …

folders-shares-not-remembered = Die Freigaben konnten für das nächste Mal nicht gemerkt werden.
