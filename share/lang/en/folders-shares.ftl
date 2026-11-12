# Folders' words about network shares: the sign-in card, the lines said about opening and closing shares, and the lines about kept logins.

## Shares (src/files/shares.rs)

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = This computer isn't allowed to open { $export } on { $server }.
# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } doesn't share a folder called { $export }.
# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = You'll be asked again next time
# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = Less than a day left
# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] { $n } day left
       *[other] { $n } days left
    }
# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
folders-shares-held =
    { $past ->
        [yes] { $server } is now at { $now }. Your login for it was used at a different address before ({ $before }).
       *[no] { $server } is at { $now }. Your login for it hasn't been used at that address before.
    }{ $full ->
        [yes] {" "}Folders only remembers the last { $most } addresses.
       *[no] {""}
    } Any device on a network can claim a name, so Folders hasn't sent your saved login. If this is your { $server }, type its user name and password to use it here.
# Words on the sign-in card when no address could be found for the computer to show.
folders-shares-nowhere = Folders couldn't find the address of { $server }, so nothing has been sent to it. Anything you type here can't be sent safely. Open it by its address instead, such as smb://192.168.1.20/.
# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
folders-shares-lapsed = { $server } is at { $now }. Your login for it was last used there more than { $days } days ago. Another device may have been given that address since, so Folders hasn't sent your saved login. If this is your { $server }, type its user name and password to use it here.
# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
folders-shares-unknown = { $server } is at { $now }. A login for it was used there before, but Folders didn't note when, or which login. Another device may have been given that address since, so Folders hasn't sent your saved login. If this is your { $server }, type its user name and password to use it here.
# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
folders-shares-unshown = { $server } is at { $now }, where your login for it was used before. This time Folders found it on the local network, where any device can use that name. You haven't seen this address before, so Folders hasn't sent your login. If this is your { $server }, type its user name and password to use it here.
# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
folders-shares-asking =
    Type the user name and password for an account on { $server }.{ $heard ->
        [yes] {" "}It's at { $at }. Any device on a network can claim a name, so check this is its address before you type.
       *[no] {""}
    }
# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
folders-shares-where =
    What you type goes to { $address }{ $heard ->
        [yes] {" "}and nowhere else. With Remember on, Folders keeps your login for this address only.
       *[no] {"."}
    }
# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
folders-shares-again = Folders found { $server } on the local network at { $at }, port { $port }. Any device can use that name there, so Folders hasn't sent your login. If this is the right address, press Open.
# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders couldn't reach the keyring: { $why }.
# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = It's open, but Folders couldn't save the login. { $why }
# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } doesn't offer NFS version 4, the only version Folders can open.
# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = The share opened, but Folders couldn't find where it is on this computer.
# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = The share opened, but Folders couldn't find where it is on this computer.
# Red words when the address to remember for a login is not an address.
folders-shares-not-address = That isn't an address.
# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = Addresses used for smb://{ $server }
# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
folders-shares-password-label = Password for { $user } on { $what }
# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
folders-shares-remember = Remember for 90 days
# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = The 90 days start again each time you use it.
# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } on { $server }
# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = Open { $it }
# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Close
# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = User name, such as jo or WORKGROUP\jo
# Hint in the empty password box of the sign-in card.
folders-shares-password = Password
# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
folders-shares-forget = Forget earlier addresses
# Name a screen reader gives the switch that forgets the earlier addresses of a login.
folders-shares-forget-spoken = Forget earlier addresses for this login
# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Opening…
# Name of the main button of the sign-in card.
folders-shares-open = Open
# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = That isn't a computer name.
# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = That isn't a path to a shared folder.
# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = Opening { $export } on { $host }…
# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = Opening { $share } on { $server }…
# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Finish logging in to { $it } first.
# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = It's open. { $words }
# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } doesn't share any folders with you.
# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = Closing { $name }…
# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders couldn't close { $name }: { $why }.
# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = Reconnecting { $name }…
# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders couldn't save your shares for next time.
# Red words when a share's user name cannot be used.
folders-shares-bad-user = That user name can't be used for a share.
# Red words when a share's domain cannot be used.
folders-shares-bad-domain = That domain can't be used for a share.
# Red words when a share's password holds a line break.
folders-shares-bad-password = A share's password can't contain a line break.
# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } needs a user name and password for { $it }.
# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = The user name or password for { $it } is wrong.
# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = Your account on { $server } is locked or switched off, or its password has expired.
# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } doesn't have a shared folder called { $share }.
# Red words when the account may not open a share; what is being opened.
folders-shares-denied = That account isn't allowed to open { $it }.
# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } didn't respond. It may be asleep or switched off. Wake it up, then try again.
# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders couldn't find a computer called { $server } on this network.
# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } only uses SMB1, an old way of sharing that isn't safe any more. Folders doesn't use it.
# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = This needs Debian's { $package } package, which isn't installed.
# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders couldn't open the share.
# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders couldn't open the shared folder.
# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders couldn't list its shared folders.
