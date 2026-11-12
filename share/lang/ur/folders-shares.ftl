
# machine translated (Urdu), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = اس کمپیوٹر کو { $server } پر { $export } کھولنے کی اجازت نہیں۔

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } پر { $export } نام کا کوئی فولڈر شیئر نہیں ہے۔

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = اگلی بار آپ سے دوبارہ پوچھا جائے گا

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = ایک دن سے کم باقی

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] { $n } دن باقی
       *[other] { $n } دن باقی
    }

# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
# SAFETY: needs a human review before it ships
folders-shares-held =
    { $past ->
        [yes] { $server } اب { $now } پر ہے۔ اس کے لیے آپ کا لاگ ان پہلے ایک مختلف پتے پر استعمال ہوا تھا ({ $before })۔
       *[no] { $server } { $now } پر ہے۔ اس کے لیے آپ کا لاگ ان اس پتے پر پہلے کبھی استعمال نہیں ہوا۔
    }{ $full ->
        [yes] {" "}Folders صرف آخری { $most } پتے یاد رکھتا ہے۔
       *[no] {""}
    } نیٹ ورک پر کوئی بھی آلہ کوئی نام اپنا بتا سکتا ہے، اس لیے Folders نے آپ کا محفوظ لاگ ان نہیں بھیجا۔ اگر یہ آپ کا { $server } ہے تو اسے یہاں استعمال کرنے کے لیے اس کا صارف نام اور پاس ورڈ لکھیں۔

# Words on the sign-in card when no address could be found for the computer to show.
# SAFETY: needs a human review before it ships
folders-shares-nowhere = Folders کو { $server } کا پتا نہیں مل سکا، اس لیے اسے کچھ نہیں بھیجا گیا۔ یہاں آپ جو بھی لکھیں گے اسے محفوظ طریقے سے بھیجا نہیں جا سکتا۔ اسے اس کے پتے سے کھولیں، جیسے smb://192.168.1.20/۔

# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
# SAFETY: needs a human review before it ships
folders-shares-lapsed = { $server } { $now } پر ہے۔ اس کے لیے آپ کا لاگ ان آخری بار وہاں { $days } دن سے زیادہ پہلے استعمال ہوا تھا۔ ہو سکتا ہے اس کے بعد وہ پتا کسی اور آلے کو دے دیا گیا ہو، اس لیے Folders نے آپ کا محفوظ لاگ ان نہیں بھیجا۔ اگر یہ آپ کا { $server } ہے تو اسے یہاں استعمال کرنے کے لیے اس کا صارف نام اور پاس ورڈ لکھیں۔

# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
# SAFETY: needs a human review before it ships
folders-shares-unknown = { $server } { $now } پر ہے۔ اس کے لیے ایک لاگ ان وہاں پہلے استعمال ہوا تھا، مگر Folders نے یہ نہیں لکھا کہ کب، یا کون سا لاگ ان۔ ہو سکتا ہے اس کے بعد وہ پتا کسی اور آلے کو دے دیا گیا ہو، اس لیے Folders نے آپ کا محفوظ لاگ ان نہیں بھیجا۔ اگر یہ آپ کا { $server } ہے تو اسے یہاں استعمال کرنے کے لیے اس کا صارف نام اور پاس ورڈ لکھیں۔

# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
# SAFETY: needs a human review before it ships
folders-shares-unshown = { $server } { $now } پر ہے، جہاں اس کے لیے آپ کا لاگ ان پہلے استعمال ہوا تھا۔ اس بار Folders کو یہ مقامی نیٹ ورک پر ملا، جہاں کوئی بھی آلہ یہ نام استعمال کر سکتا ہے۔ آپ نے یہ پتا پہلے نہیں دیکھا، اس لیے Folders نے آپ کا لاگ ان نہیں بھیجا۔ اگر یہ آپ کا { $server } ہے تو اسے یہاں استعمال کرنے کے لیے اس کا صارف نام اور پاس ورڈ لکھیں۔

# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
# SAFETY: needs a human review before it ships
folders-shares-asking =
    { $server } پر کسی اکاؤنٹ کا صارف نام اور پاس ورڈ لکھیں۔{ $heard ->
        [yes] {" "}یہ { $at } پر ہے۔ نیٹ ورک پر کوئی بھی آلہ کوئی نام اپنا بتا سکتا ہے، اس لیے لکھنے سے پہلے دیکھ لیں کہ یہی اس کا پتا ہے۔
       *[no] {""}
    }

# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
# SAFETY: needs a human review before it ships
folders-shares-where =
    جو آپ لکھیں گے وہ { $address } کو جاتا ہے{ $heard ->
        [yes] {" "}اور کہیں اور نہیں۔ یاد رکھیں چالو ہو تو Folders آپ کا لاگ ان صرف اسی پتے کے لیے محفوظ رکھتا ہے۔
       *[no] {"۔"}
    }

# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
# SAFETY: needs a human review before it ships
folders-shares-again = Folders کو مقامی نیٹ ورک پر { $server } ملا، پتا { $at }، پورٹ { $port }۔ وہاں کوئی بھی آلہ یہ نام استعمال کر سکتا ہے، اس لیے Folders نے آپ کا لاگ ان نہیں بھیجا۔ اگر یہی صحیح پتا ہے تو کھولیں دبائیں۔

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders کی رنگ تک نہیں پہنچ سکا: { $why }۔

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
# SAFETY: needs a human review before it ships
folders-shares-unkept = یہ کھلا ہے، مگر Folders لاگ ان محفوظ نہیں کر سکا۔ { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } NFS ورژن 4 پیش نہیں کرتا، جو واحد ورژن ہے جسے Folders کھول سکتا ہے۔

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = شیئر کھل گیا، مگر Folders یہ نہیں ڈھونڈ سکا کہ وہ اس کمپیوٹر پر کہاں ہے۔

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = شیئر کھل گیا، مگر Folders یہ نہیں ڈھونڈ سکا کہ وہ اس کمپیوٹر پر کہاں ہے۔

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = یہ پتا نہیں ہے۔

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = smb://{ $server } کے لیے استعمال ہونے والے پتے

# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
# SAFETY: needs a human review before it ships
folders-shares-password-label = { $what } پر { $user } کا پاس ورڈ

# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
# SAFETY: needs a human review before it ships
folders-shares-remember = 90 دن کے لیے یاد رکھیں

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = 90 دن ہر بار استعمال کرنے پر دوبارہ شروع ہوتے ہیں۔

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $server } پر { $share }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = { $it } کھولیں

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = بند کریں

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = صارف نام، جیسے jo یا WORKGROUP\jo

# Hint in the empty password box of the sign-in card.
# SAFETY: needs a human review before it ships
folders-shares-password = پاس ورڈ

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
# SAFETY: needs a human review before it ships
folders-shares-forget = پہلے کے پتے بھول جائیں

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
# SAFETY: needs a human review before it ships
folders-shares-forget-spoken = اس لاگ ان کے پہلے کے پتے بھول جائیں

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = کھل رہا ہے…

# Name of the main button of the sign-in card.
folders-shares-open = کھولیں

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = یہ کمپیوٹر کا نام نہیں ہے۔

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = یہ مشترکہ فولڈر کا راستہ نہیں ہے۔

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = { $host } پر { $export } کھل رہا ہے…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = { $server } پر { $share } کھل رہا ہے…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = پہلے { $it } میں لاگ ان مکمل کریں۔

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = یہ کھلا ہے۔ { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } آپ کے ساتھ کوئی فولڈر شیئر نہیں کرتا۔

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = { $name } بند ہو رہا ہے…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders { $name } بند نہیں کر سکا: { $why }۔

# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = { $name } دوبارہ جوڑا جا رہا ہے…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders آپ کے شیئرز اگلی بار کے لیے محفوظ نہیں کر سکا۔

# Red words when a share's user name cannot be used.
folders-shares-bad-user = یہ صارف نام شیئر کے لیے استعمال نہیں ہو سکتا۔

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = یہ ڈومین شیئر کے لیے استعمال نہیں ہو سکتا۔

# Red words when a share's password holds a line break.
folders-shares-bad-password = شیئر کے پاس ورڈ میں نئی لائن نہیں آ سکتی۔

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } کو { $it } کے لیے صارف نام اور پاس ورڈ چاہیے۔

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = { $it } کے لیے صارف نام یا پاس ورڈ غلط ہے۔

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = { $server } پر آپ کا اکاؤنٹ مقفل یا بند ہے، یا اس کے پاس ورڈ کی مدت ختم ہو گئی ہے۔

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } پر { $share } نام کا کوئی مشترکہ فولڈر نہیں ہے۔

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = وہ اکاؤنٹ { $it } کھولنے کی اجازت نہیں رکھتا۔

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } نے جواب نہیں دیا۔ ہو سکتا ہے وہ سویا ہوا یا بند ہو۔ اسے جگائیں، پھر دوبارہ کوشش کریں۔

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders کو اس نیٹ ورک پر { $server } نام کا کوئی کمپیوٹر نہیں ملا۔

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } صرف SMB1 استعمال کرتا ہے، جو شیئرنگ کا پرانا طریقہ ہے اور اب محفوظ نہیں۔ Folders اسے استعمال نہیں کرتا۔

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = اس کے لیے Debian کا { $package } پیکیج چاہیے، جو نصب نہیں ہے۔

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders شیئر نہیں کھول سکا۔

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders مشترکہ فولڈر نہیں کھول سکا۔

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders اس کے مشترکہ فولڈرز کی فہرست نہیں بنا سکا۔
