# Folders: sharing one of the person's own folders with other computers on the home network (the Share part of Properties and the Share on the network window).

folders-sharing-cannot-read = This folder cannot be read.
folders-sharing-only-folder = Only a folder itself can be shared, not a link to one.
folders-sharing-only-yours = Only folders that belong to you can be shared. This one belongs to someone else.
folders-sharing-your-home = your home folder
folders-sharing-this-folder = this folder
folders-sharing-and = { $first } and { $second }
folders-sharing-share = Share this folder
folders-sharing-shared = Shared on the home network.
folders-sharing-not-shared = Not shared.
folders-sharing-share-tip = Share this folder with other computers on the home network
folders-sharing-name = Name
folders-sharing-name-tip = The name other computers see
folders-sharing-write = Others can change files
folders-sharing-write-sub = They may add, change and delete files in it.
folders-sharing-guests = Let in anyone
folders-sharing-guests-sub = No password is asked for.
folders-sharing-guests-tip = Let in anyone, without a password
folders-sharing-explain =
    { $fetch ->
        [yes] Shared, it shows under Network on the other computers at home, Windows ones too. The first time, Trindle fetches Samba, the program that shares folders, which takes a minute.
       *[no] Shared, it shows under Network on the other computers at home, Windows ones too.
    }
folders-sharing-find-it =
    { $host_known ->
        [yes] Other computers find it under Network, as { $name } on { $host }.
       *[no] Other computers find it under Network, as { $name }.
    }
folders-sharing-cannot-reach = Other computers cannot reach it yet: Let other computers connect is off in Settings, Network.
folders-sharing-let-connect = Let other computers connect
folders-sharing-visitors =
    { $n ->
        [one] Visitors without a password cannot open it yet: { $names } is closed to other accounts. Letting them through opens only the way to this folder; they cannot see what else is there.
       *[other] Visitors without a password cannot open it yet: { $names } are closed to other accounts. Letting them through opens only the way to this folder; they cannot see what else is there.
    }
folders-sharing-let-through = Let visitors through to this folder
folders-sharing-sign-in-set = Others sign in as { $user }, with the password set for sharing.
folders-sharing-change-password = Change the password other computers use
folders-sharing-set-password = Others sign in as { $user }. Set the password other computers use to open your shared folders, at least { $n } letters. It is kept apart from your login password.
folders-sharing-password = Password
folders-sharing-password-tip = Sharing password
folders-sharing-keep-password = Keep this password
folders-sharing-card-title = Share { $name } on the network
folders-sharing-name-taken = Another of your shared folders already has that name. Choose another.
folders-sharing-stopping = Stopping sharing…
folders-sharing-password-short = The password needs at least { $n } letters.
folders-sharing-keeping-password = Keeping the password; the administrator's password is asked for first.
folders-sharing-letting-connect = Letting other computers connect; the administrator's password is asked for first.
folders-sharing-fetching = Fetching Samba, the program that shares folders, and setting it up. This happens only the first time, and the administrator's password is asked for first.
folders-sharing-setting-up = Setting up sharing. This happens only the first time, and the administrator's password is asked for first.
folders-sharing-changing = Changing the share…
folders-sharing-sharing-it = Sharing it…
