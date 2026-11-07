# Folders' words about network shares: the sign-in card, the lines said about opening and closing shares, and the lines about kept logins.

folders-shares-nfs-denied = { $server } does not let this computer open { $export }.
folders-shares-nfs-no-share = { $server } shares nothing called { $export }.
folders-shares-left-none = Asks again at the next opening
folders-shares-left-under-day = Less than a day left
folders-shares-left-days =
    { $n ->
        [one] { $n } day left
       *[other] { $n } days left
    }
folders-shares-held =
    { $past ->
        [yes] { $server } answers at { $now } now, a different address from the one your login for it was used at before ({ $before }).
       *[no] { $server } answers at { $now }, an address your login for it has not been used at before.
    }{ $full ->
        [yes] {" "}Only the last { $most } addresses it was used at are remembered.
       *[no] {""}
    } Any device on a network can answer to a name, so the login kept for it has not been sent. If this is your { $server }, type its user name and password to use it here.
folders-shares-nowhere = The address { $server } answers at could not be found beforehand, so nothing has been sent to it and what is typed here has nowhere it can safely go. Open it by its address instead, such as smb://192.168.1.20/.
folders-shares-lapsed = { $server } answers at { $now }, where your login for it was last used more than { $days } days ago. Another device may have been given that address since, so the login kept for it has not been sent. If this is your { $server }, type its user name and password to use it here.
folders-shares-unknown = { $server } answers at { $now }, where a login for it was used before, but when, or which login, was not written down. Another device may have been given that address since, so the login kept for it has not been sent. If this is your { $server }, type its user name and password to use it here.
folders-shares-unshown = { $server } answers at { $now }, the address your login for it went to earlier. This time that address was only heard on the network, where any device can answer to a name, and it has not been shown to you here before, so the login has not been sent. If this is your { $server }, type its user name and password to use it here.
folders-shares-asking =
    { $server } wants a user name and password. They are the ones for an account on that computer.{ $heard ->
        [yes] {" "}It answers at { $at }. Any device on a network can answer to a name, so check that is its address before you type.
       *[no] {""}
    }
folders-shares-where =
    What you type goes to { $address }{ $heard ->
        [yes] {" "}and nowhere else. With Remember on, your login is kept for that address only.
       *[no] {"."}
    }
folders-shares-again = { $server } answers at { $at }, port { $port }. That address was only heard on the network, where any device can answer to a name, so your login for it has not been sent. If that is its address, press Open.
folders-shares-keyring-unreachable = The keyring could not be reached: { $why }.
folders-shares-unkept = It is open, but the login was not kept. { $why }
folders-shares-nfs-version = { $server } does not offer NFS version 4, the only kind Trindle opens.
folders-shares-nfs-opened-unknown = It opened, but where is not known.
folders-shares-opened-unknown = The share opened, but where is not known.
folders-shares-not-address = That is not an address.
folders-shares-seen-label = Addresses smb://{ $server } was opened at
folders-shares-password-label = Password for { $user } on { $what }
folders-shares-remember = Remember for 90 days
folders-shares-remember-line = Starts again each time you use it.
folders-shares-share-on = { $share } on { $server }
folders-shares-card-title = Open { $it }
folders-shares-close = Close
folders-shares-user-placeholder = User name, such as jo, or WORKGROUP\jo
folders-shares-password = Password
folders-shares-forget = Forget the earlier addresses
folders-shares-forget-spoken = Forget the addresses this login was used at before
folders-shares-opening = Opening
folders-shares-open = Open
folders-shares-bad-computer = That is not the name of a computer.
folders-shares-bad-path = That is not the path of a shared folder.
folders-shares-opening-export = Opening { $export } on { $host }…
folders-shares-opening-share = Opening { $share } on { $server }…
folders-shares-answer-card = Answer the card for { $it } first.
folders-shares-open-waiting = It is open. { $words }
folders-shares-none = { $server } shares no folders with you.
folders-shares-closing = Closing { $name }…
folders-shares-close-failed = { $name } could not be closed: { $why }.
folders-shares-reconnecting = Reconnecting { $name }…
folders-shares-not-remembered = The shares could not be remembered for next time.
