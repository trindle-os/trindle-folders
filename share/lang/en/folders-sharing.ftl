# Folders: sharing one of the person's own folders with other computers on the home network (the Share part of Properties and the Share on the network window).

## Sharing (src/files/sharing.rs)

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders can't read what's in this folder.
# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = You can only share a folder itself, not a link to one.
# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = You can only share folders that belong to you. This one belongs to someone else.
# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = your home folder
# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = this folder
# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } and { $second }
# Name of the switch that shares the folder.
folders-sharing-share = Share this folder
# One line under the share switch when the folder is shared.
folders-sharing-shared = Shared on the home network.
# Tip of the share switch.
folders-sharing-share-tip = Share on the home network
# Label over the box for the name other computers see.
folders-sharing-name = Name
# Tip of the box for the name other computers see.
folders-sharing-name-tip = The name other computers see
# Name of the switch that lets others change files, and its tip.
folders-sharing-write = Let others change files
# One line under the switch that lets others change files.
folders-sharing-write-sub = They can add, change and delete files in it.
# Name of the switch that lets anyone in without a password.
folders-sharing-guests = Let anyone in
# One line under the let in anyone switch.
folders-sharing-guests-sub = No password needed.
# Tip of the let in anyone switch.
folders-sharing-guests-tip = Let anyone in without a password
# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Once it's shared, it shows under Network on your other computers at home, Windows ones too. The first time, Trindle downloads Samba, the program that shares folders. This takes a minute.
       *[no] Once it's shared, it shows under Network on your other computers at home, Windows ones too.
    }
# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] On other computers, look under Network for { $name } on { $host }.
       *[no] On other computers, look under Network for { $name }.
    }
# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = Other computers can't reach this folder yet. In Settings, open Network and switch on "Let other computers connect".
# Tip of the button that lets other computers connect.
folders-sharing-let-connect = Let other computers connect
# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
folders-sharing-visitors =
    { $n ->
        [one] Guests without a password can't open it yet, because { $names } is closed to other accounts. If you let them through, they can reach this folder but can't see anything else on the way.
       *[other] Guests without a password can't open it yet, because { $names } are closed to other accounts. If you let them through, they can reach this folder but can't see anything else on the way.
    }
# Tip of the button that lets visitors through to the folder.
folders-sharing-let-through = Let guests reach this folder
# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
folders-sharing-sign-in-set = Others log in as { $user } with the sharing password.
# Tip of the button that changes the sharing password.
folders-sharing-change-password = Change the sharing password
# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
folders-sharing-set-password = Others log in as { $user }. Set a sharing password of at least { $n } characters. It's separate from your login password.
# Label beside the sharing password box.
folders-sharing-password = Password
# Tip of the sharing password box.
folders-sharing-password-tip = Sharing password
# Tip of the button that keeps the sharing password.
folders-sharing-keep-password = Save password
# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = Share { $name } on the network
# Red words when the share's name is used by another share.
folders-sharing-name-taken = You already have a shared folder with that name. Choose another.
# Calm words while sharing is being stopped.
folders-sharing-stopping = Stopping sharing…
# Red words when the sharing password is too short; { $n } is the fewest letters.
folders-sharing-password-short = The password needs at least { $n } characters.
# Calm words while the sharing password is being kept.
folders-sharing-keeping-password = Saving the password. You'll be asked for the administrator's password first.
# Calm words while other computers are being let connect.
folders-sharing-letting-connect = Letting other computers connect. You'll be asked for the administrator's password first.
# Calm words while Samba is fetched and set up the first time.
folders-sharing-fetching = Downloading and setting up Samba, the program that shares folders. You'll be asked for the administrator's password first. This only happens the first time.
# Calm words while sharing is set up the first time and Samba is already there.
folders-sharing-setting-up = Setting up sharing. You'll be asked for the administrator's password first. This only happens the first time.
# Calm words while an existing share is changed.
folders-sharing-changing = Changing the share…
# Calm words while a share is made.
folders-sharing-sharing-it = Sharing the folder…
# Red words under the name box when a share has no name.
folders-sharing-name-empty = Give the shared folder a name.
# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = A shared folder's name can't be longer than { $most } characters.
# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = A shared folder's name can't contain any of % < > * ? | / \ + = ; : " or a comma.
# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = A name ending in $ would hide the share, so you can't use it.
# Red words when a share's name starts with a dash.
folders-sharing-name-dash = A shared folder's name can't start with a dash.
# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba already uses the name { $name }. Choose another.
# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Shared
# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = You can only share folders that belong to you.
# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = That name is already an account name on this computer, so you can't use it for a share. Choose another.
# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = This computer's Samba settings don't allow shares without a password.
# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = This computer can't share any more folders.
# Red words when the folder to share has gone.
folders-sharing-folder-gone = That folder isn't there any more.
# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba didn't give a reason.
# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Samba's message: { $said }
# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } belongs to someone else, so you can't give guests access to it from here.
# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders couldn't give guests access to { $path }: { $why }
# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle couldn't download Samba because this computer seems to be offline. Connect to the internet and try again.
# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Trindle couldn't install Samba while something else is installing. Try again in a few minutes.
