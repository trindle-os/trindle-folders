
# machine translated (Urdu), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = { $host } نام کا کوئی سرور نہیں ہے

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = یہ پوری نہیں پہنچی، اس لیے اصل اپنی جگہ سے ہٹی نہیں

# Reason a link was not sent to a server.
folders-far-not-followed = یہ ایک لنک ہے، اس لیے Folders نے اسے نہیں بھیجا

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = یہ فائل یا فولڈر نہیں ہے، اس لیے Folders نے اسے نہیں بھیجا

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders یہ نہیں جان سکا کہ یہ کس قسم کی چیز ہے، اس لیے اسے وہیں چھوڑ دیا

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = اس قسم کے سرور پر ایک فائل اسی نام کی دوسری فائل کی جگہ لے سکتی ہے۔

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = سرور کی فائلوں کے ساتھ آپ یہ نہیں کر سکتے

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders اسے پڑھ نہیں سکا

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders اسے کھول نہیں سکا۔

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (فولڈر اپنے اندر نہیں جا سکتا)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (اس نام کا فولڈر پہلے سے موجود ہے)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (اس نام کی فائل پہلے سے موجود ہے)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders سرور کا پتا نہیں جانتا۔

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = بھیجی گئی کاپی کا سائز غلط تھا

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = یہ صرف سرور کی فائلوں کے ساتھ کام کرتا ہے۔

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders نئی فائل اپنی جگہ پر نہیں رکھ سکا، اس لیے پرانی فائل ابھی وہیں ہے اور اس کا نام اب { $old } ہے، اور نئی فائل کا نام { $new } ہے: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders پرانی فائل ہٹا نہیں سکا۔ وہ ابھی وہیں ہے اور اس کا نام اب { $old } ہے

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders پرانی فائل ہٹا نہیں سکا، اس لیے وہ ابھی وہیں ہے اور اس کا نام اب { $old } ہے: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } خالی نہیں ہے۔

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders نے ایک فائل چھوڑ دی کیونکہ اس کا نام استعمال کے لیے محفوظ نہیں۔

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
        [one] Folders نے ایک چیز چھوڑ دی کیونکہ اس کا نام استعمال کے لیے محفوظ نہیں: { $names }۔
       *[other] Folders نے { $n } چیزیں چھوڑ دیں کیونکہ ان کے نام استعمال کے لیے محفوظ نہیں: { $names }۔
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = { $name } ڈاؤن لوڈ کیا جا رہا ہے…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders { $name } ڈاؤن لوڈ نہیں کر سکا: { $why }۔

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = { $name } کی ایک کاپی کھل گئی۔ محفوظ کرنے پر آپ کی تبدیلیاں سرور کو واپس چلی جاتی ہیں۔

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = { $name } سرور پر محفوظ ہو گئی۔

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders { $name } کو سرور پر محفوظ نہیں کر سکا: { $why }۔ آپ کی تبدیلیاں اب بھی اس کمپیوٹر پر موجود کاپی میں ہیں۔

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } کا نام بدل کر { $now } کر دیا گیا۔

# Heading of the Connect to a server card.
folders-far-connect-title = سرور سے جڑیں

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = بند کریں

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = پتا، جیسے sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
folders-far-connect-password = پاس ورڈ، اگر ضرورت ہو

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = کلید کی فائل، اگر آپ استعمال کرتے ہوں

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = کلید کی فائل چنیں


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders آپ کی ssh ترتیبات، ssh ایجنٹ، کلیدیں اور جانے پہچانے سرورز کی فہرست استعمال کرتا ہے۔

# Note on the Connect to a server card for a server not visited before; the server's name.
folders-far-connect-new-host = آپ پہلے { $host } سے نہیں جڑے، اس لیے Folders نہیں جانچ سکتا کہ یہ صحیح سرور ہے۔ اس پر صرف تب بھروسا کریں جب آپ اسی سے جڑنے کی توقع کر رہے تھے۔

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
folders-far-connect-host-changed = { $host } کی شناخت پچھلی بار سے مختلف ہے۔ ہو سکتا ہے کوئی آپ کا کنکشن روکنے کی کوشش کر رہا ہو، اس لیے Folders نے کنکشن نہیں بنایا۔ اگر مالک نے اسے بدلا ہے تو پہلے اپنے جانے پہچانے میزبانوں سے اس کا پرانا اندراج ہٹائیں۔

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
folders-far-connect-needs-password = اس سرور کو پاس ورڈ چاہیے۔ اسے اوپر لکھیں اور دوبارہ کوشش کریں۔

# Note on the Connect to a server card when a server asks for a key or password.
folders-far-connect-needs-key = اس سرور کو کلید یا پاس ورڈ چاہیے۔ اوپر اپنی کلید کی فائل چنیں، یا اسے اپنے ssh ایجنٹ میں شامل کریں، پھر دوبارہ کوشش کریں۔

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
folders-far-connect-cert-new = Folders { $host } کا سرٹیفکیٹ جانچ نہیں سکا: { $why }۔ ہو سکے تو اس کے فنگر پرنٹ کا موازنہ اس سے کریں جو مالک آپ کو دے۔ صرف تب بھروسا کریں جب دونوں ملیں۔

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
folders-far-connect-cert-changed = { $host } کا سرٹیفکیٹ اس کے بعد بدل گیا ہے جب آپ نے اس پر بھروسا کیا تھا۔ ہو سکتا ہے مالک نے اس کی تجدید کی ہو، یا کوئی آپ کا کنکشن روکنے کی کوشش کر رہا ہو۔ صرف تب بھروسا کریں جب مالک اس فنگر پرنٹ کی تصدیق کرے۔

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = جڑ رہا ہے

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
folders-far-connect-trust = بھروسا کریں اور جڑیں

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = جڑیں

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = فون کو پلگ ان اور ان لاک کرنے پر وہ سائیڈ بار میں نظر آ جاتے ہیں۔
