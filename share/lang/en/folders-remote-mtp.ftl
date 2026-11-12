# Names and errors for phones and tablets reached over USB (MTP) in Folders.

## Phones (src/files/remote/mtp.rs)

# Name of a phone in the sidebar when it did not tell its maker or model.
folders-remote-mtp-phone = Phone
# Red words when the phone sent something Folders could not read.
folders-remote-mtp-nonsense = Folders couldn't understand what the phone sent.
# Red words when the phone has no room left.
folders-remote-mtp-full = The phone is full.
# Name of what is in use by another program when the phone is already open elsewhere ("The phone is in use by another program.").
folders-remote-mtp-the-phone = The phone
# The thing asked of a phone that it cannot do, at the start of "... isn't supported."
folders-remote-mtp-that-here = Doing that on this phone
# Red words when the phone refused something for a reason Folders has no words for; the variable is the phone's answer code in hexadecimal.
folders-remote-mtp-refused = That didn't work on the phone (code { $code }).
# Reason in red words ("Could not reach it: ...") when the phone is no longer plugged in.
folders-remote-mtp-not-plugged = that phone isn't plugged in
# Name of the thing that failed, in red words such as "Permission was refused for ...": the phone.
folders-remote-mtp-what-phone = the phone
# Name of the thing that failed, in red words such as "Permission was refused for ...": the phone's storage.
folders-remote-mtp-what-storage = the phone's storage
# Red words when a file from the phone was shorter than the phone said.
folders-remote-mtp-shorter = The file was smaller than its listed size.
# Red words when something is put on a phone outside its storages.
folders-remote-mtp-only-storage = Files and folders can only go inside one of the phone's storage areas.
# Red words when a name on the phone was used as a folder but is not one; the variable is the name.
folders-remote-mtp-not-folder = { $name } is not a folder.
# Red words when a name on the phone was used as a file but is a folder; the variable is the name.
folders-remote-mtp-is-folder = { $name } is a folder.
# Red words when a folder on the phone to be removed still holds something; the variable is its name.
folders-remote-mtp-not-empty = { $name } is not empty.
# The thing that cannot be done, in "... isn't supported.": removing a whole storage of a phone.
folders-remote-mtp-remove-storage = Removing a whole storage area
# The thing that cannot be done, in "... isn't supported.": renaming a storage of a phone.
folders-remote-mtp-rename-storage = Renaming a storage area
# Words shown when the phone is plugged in but shows no storage.
folders-remote-mtp-no-storage = Folders can't see any storage on the phone. Unlock the phone and choose File transfer, then try again.
# Name shown for a phone's storage that has no name of its own.
folders-remote-mtp-storage-name = Storage
