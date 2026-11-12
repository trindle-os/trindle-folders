
# machine translated (Urdu), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
folders-remote-mod-heard-card = Folders کو مقامی نیٹ ورک پر { $host } ملا، پتہ { $at }، پورٹ { $port }۔ وہاں کوئی بھی آلہ یہ نام استعمال کر سکتا ہے، اس لیے Folders نے آپ کا پاس ورڈ نہیں بھیجا۔ اگر یہی صحیح پتہ ہے تو جوڑیں دبائیں۔

# SAFETY: needs a human review before it ships
# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
folders-remote-mod-heard-stopped = Folders کو مقامی نیٹ ورک پر { $host } ملا، پتہ { $at }۔ وہاں کوئی بھی آلہ یہ نام استعمال کر سکتا ہے، اس لیے Folders نے آپ کا پاس ورڈ نہیں بھیجا۔ اس کا پتہ جانچنے کے لیے { $host } کو دوبارہ کھولیں۔

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders یہ پتہ پڑھ نہیں سکا: { $why }۔

# SAFETY: needs a human review before it ships
# Red words when a server needs a password or key and none could be asked for.
folders-remote-mod-needs-password = اس سرور کو پاس ورڈ یا کلید چاہیے۔

# SAFETY: needs a human review before it ships
# Red words when a server turned down the login.
folders-remote-mod-login-refused = سرور نے یہ لاگ اِن قبول نہیں کیا۔

# SAFETY: needs a human review before it ships
# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
folders-remote-mod-host-key-unknown = آپ پہلے { $host } سے نہیں جڑے، اس لیے Folders ابھی اس کی شناخت نہیں جانتا۔

# SAFETY: needs a human review before it ships
# Red words when a server's identity differs from the one remembered; the variable is the server's name.
folders-remote-mod-host-key-changed = { $host } کی شناخت پچھلی بار سے مختلف ہے۔ ہو سکتا ہے اسے نئے سرے سے بنایا گیا ہو، یا کوئی کنکشن کے بیچ میں مداخلت کر رہا ہو۔

# SAFETY: needs a human review before it ships
# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
folders-remote-mod-cert-unknown = Folders { $host } کی سند کی جانچ نہیں کر سکا: { $why }۔

# SAFETY: needs a human review before it ships
# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
folders-remote-mod-cert-changed = جب سے آپ نے { $host } کی سند پر اعتماد کیا، وہ بدل گئی ہے۔

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders جڑ نہیں سکا: { $why }۔

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders { $what } تلاش نہیں کر سکا۔

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } پہلے سے موجود ہے۔

# SAFETY: needs a human review before it ships
# Red words when a server or phone refused permission; the variable is what it was refused for.
folders-remote-mod-denied = آپ کو { $what } استعمال کرنے کی اجازت نہیں ہے۔

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } کسی اور پروگرام میں استعمال ہو رہا ہے۔

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } تعاون یافتہ نہیں ہے۔

# Red words when a copy or other work with a server was cancelled.
folders-remote-mod-cancelled = منسوخ کر دیا گیا۔

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = لنکس پر جانا

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = کھولنے سے پہلے Windows شیئر پر یا اس سے کاپی کرنا

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = کھولنے سے پہلے NFS شیئر پر یا اس سے کاپی کرنا

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = شروع میں sftp://، ftp:// یا mtp:// نہیں ہے

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = اسے sftp://، ftp://، ftps://، ftpes://، dav://، davs://، smb://، nfs:// یا mtp:// سے شروع ہونا چاہیے

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = [ کے بعد ] غائب ہے

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = اس میں سرور کا نام نہیں ہے

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = سرور کا نام درست نہیں

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = راستے میں سطر کی تبدیلی ہے

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = سرور کے نام کے بعد کوئی فالتو چیز ہے

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } درست پورٹ نمبر نہیں ہے

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = چھوڑ دیا گیا کیونکہ سرور نے غیر محفوظ نام بھیجا

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = بے نام چیز

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = جواب بہت بڑا تھا، اس لیے Folders نے اسے روک دیا۔

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } اب فولڈر ہے، اس لیے Folders نے اسے ہٹایا نہیں۔
