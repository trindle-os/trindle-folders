
# machine translated (Urdu), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders اس فولڈر کے اندر کا مواد نہیں پڑھ سکتا۔

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = آپ صرف خود فولڈر شیئر کر سکتے ہیں، اس کا لنک نہیں۔

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = آپ صرف وہ فولڈر شیئر کر سکتے ہیں جو آپ کے ہیں۔ یہ کسی اور کا ہے۔

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = آپ کا ہوم فولڈر

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = یہ فولڈر

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } اور { $second }

# Name of the switch that shares the folder.
# SAFETY: needs a human review before it ships
folders-sharing-share = یہ فولڈر شیئر کریں

# One line under the share switch when the folder is shared.
# SAFETY: needs a human review before it ships
folders-sharing-shared = گھر کے نیٹ ورک پر شیئر ہے۔


# Tip of the share switch.
# SAFETY: needs a human review before it ships
folders-sharing-share-tip = گھر کے نیٹ ورک پر شیئر کریں

# Label over the box for the name other computers see.
folders-sharing-name = نام

# Tip of the box for the name other computers see.
folders-sharing-name-tip = وہ نام جو دوسرے کمپیوٹر دیکھتے ہیں

# Name of the switch that lets others change files, and its tip.
# SAFETY: needs a human review before it ships
folders-sharing-write = دوسروں کو فائلیں بدلنے دیں

# One line under the switch that lets others change files.
# SAFETY: needs a human review before it ships
folders-sharing-write-sub = وہ اس میں فائلیں شامل کر سکتے ہیں، بدل سکتے ہیں اور مٹا سکتے ہیں۔

# Name of the switch that lets anyone in without a password.
# SAFETY: needs a human review before it ships
folders-sharing-guests = سب کو اندر آنے دیں

# One line under the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-sub = پاس ورڈ کی ضرورت نہیں۔

# Tip of the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-tip = پاس ورڈ کے بغیر سب کو اندر آنے دیں

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] شیئر ہونے کے بعد یہ گھر پر آپ کے دوسرے کمپیوٹرز پر، Windows والوں پر بھی، نیٹ ورک کے نیچے نظر آتا ہے۔ پہلی بار Trindle Samba ڈاؤن لوڈ کرتا ہے، جو فولڈر شیئر کرنے والا پروگرام ہے۔ اس میں ایک منٹ لگتا ہے۔
       *[no] شیئر ہونے کے بعد یہ گھر پر آپ کے دوسرے کمپیوٹرز پر، Windows والوں پر بھی، نیٹ ورک کے نیچے نظر آتا ہے۔
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] دوسرے کمپیوٹرز پر نیٹ ورک کے نیچے { $host } پر { $name } دیکھیں۔
       *[no] دوسرے کمپیوٹرز پر نیٹ ورک کے نیچے { $name } دیکھیں۔
    }

# Note when other computers are not allowed to connect yet.
# SAFETY: needs a human review before it ships
folders-sharing-cannot-reach = دوسرے کمپیوٹر ابھی اس فولڈر تک نہیں پہنچ سکتے۔ سیٹنگز میں نیٹ ورک کھولیں اور "دوسرے کمپیوٹرز کو جڑنے دیں" چالو کریں۔

# Tip of the button that lets other computers connect.
# SAFETY: needs a human review before it ships
folders-sharing-let-connect = دوسرے کمپیوٹرز کو جڑنے دیں

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
# SAFETY: needs a human review before it ships
folders-sharing-visitors =
    { $n ->
        [one] بغیر پاس ورڈ کے مہمان ابھی اسے نہیں کھول سکتے، کیونکہ { $names } دوسرے اکاؤنٹس کے لیے بند ہے۔ اگر آپ انہیں گزرنے دیں تو وہ اس فولڈر تک پہنچ سکتے ہیں، مگر راستے میں کچھ اور نہیں دیکھ سکتے۔
       *[other] بغیر پاس ورڈ کے مہمان ابھی اسے نہیں کھول سکتے، کیونکہ { $names } دوسرے اکاؤنٹس کے لیے بند ہیں۔ اگر آپ انہیں گزرنے دیں تو وہ اس فولڈر تک پہنچ سکتے ہیں، مگر راستے میں کچھ اور نہیں دیکھ سکتے۔
    }

# Tip of the button that lets visitors through to the folder.
# SAFETY: needs a human review before it ships
folders-sharing-let-through = مہمانوں کو اس فولڈر تک پہنچنے دیں

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
# SAFETY: needs a human review before it ships
folders-sharing-sign-in-set = دوسرے شیئرنگ پاس ورڈ کے ساتھ { $user } کے طور پر لاگ اِن کرتے ہیں۔

# Tip of the button that changes the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-change-password = شیئرنگ پاس ورڈ بدلیں

# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-set-password = دوسرے { $user } کے طور پر لاگ اِن کرتے ہیں۔ کم از کم { $n } حروف کا شیئرنگ پاس ورڈ مقرر کریں۔ یہ آپ کے لاگ اِن پاس ورڈ سے الگ ہے۔

# Label beside the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password = پاس ورڈ

# Tip of the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password-tip = شیئرنگ کا پاس ورڈ

# Tip of the button that keeps the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-keep-password = پاس ورڈ محفوظ کریں

# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = { $name } کو نیٹ ورک پر شیئر کریں

# Red words when the share's name is used by another share.
folders-sharing-name-taken = آپ کا پہلے سے اسی نام کا ایک شیئر کیا گیا فولڈر موجود ہے۔ دوسرا نام چنیں۔

# Calm words while sharing is being stopped.
folders-sharing-stopping = شیئرنگ روکی جا رہی ہے…

# Red words when the sharing password is too short; { $n } is the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-password-short = پاس ورڈ میں کم از کم { $n } حروف ہونے چاہییں۔

# Calm words while the sharing password is being kept.
# SAFETY: needs a human review before it ships
folders-sharing-keeping-password = پاس ورڈ محفوظ کیا جا رہا ہے۔ پہلے آپ سے منتظم کا پاس ورڈ مانگا جائے گا۔

# Calm words while other computers are being let connect.
# SAFETY: needs a human review before it ships
folders-sharing-letting-connect = دوسرے کمپیوٹرز کو جڑنے دیا جا رہا ہے۔ پہلے آپ سے منتظم کا پاس ورڈ مانگا جائے گا۔

# Calm words while Samba is fetched and set up the first time.
# SAFETY: needs a human review before it ships
folders-sharing-fetching = Samba، جو فولڈر شیئر کرنے والا پروگرام ہے، ڈاؤن لوڈ اور ترتیب دیا جا رہا ہے۔ پہلے آپ سے منتظم کا پاس ورڈ مانگا جائے گا۔ یہ صرف پہلی بار ہوتا ہے۔

# Calm words while sharing is set up the first time and Samba is already there.
# SAFETY: needs a human review before it ships
folders-sharing-setting-up = شیئرنگ ترتیب دی جا رہی ہے۔ پہلے آپ سے منتظم کا پاس ورڈ مانگا جائے گا۔ یہ صرف پہلی بار ہوتا ہے۔

# Calm words while an existing share is changed.
folders-sharing-changing = شیئر بدلا جا رہا ہے…

# Calm words while a share is made.
folders-sharing-sharing-it = فولڈر شیئر کیا جا رہا ہے…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = شیئر کیے گئے فولڈر کو ایک نام دیں۔

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = شیئر کیے گئے فولڈر کا نام { $most } حروف سے لمبا نہیں ہو سکتا۔

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = شیئر کیے گئے فولڈر کے نام میں % < > * ? | / \ + = ; : " یا کوما میں سے کچھ نہیں آ سکتا۔

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = $ پر ختم ہونے والا نام شیئر کو چھپا دے گا، اس لیے آپ اسے استعمال نہیں کر سکتے۔

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = شیئر کیے گئے فولڈر کا نام ڈیش سے شروع نہیں ہو سکتا۔

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba پہلے ہی { $name } نام استعمال کرتا ہے۔ دوسرا نام چنیں۔

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = مشترکہ

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = آپ صرف وہ فولڈرز شیئر کر سکتے ہیں جو آپ کے ہیں۔

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = یہ نام اس کمپیوٹر پر پہلے ہی کسی اکاؤنٹ کا نام ہے، اس لیے آپ اسے شیئر کے لیے استعمال نہیں کر سکتے۔ دوسرا نام چنیں۔

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = اس کمپیوٹر کی Samba ترتیبات پاس ورڈ کے بغیر شیئر کی اجازت نہیں دیتیں۔

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = یہ کمپیوٹر مزید فولڈرز شیئر نہیں کر سکتا۔

# Red words when the folder to share has gone.
folders-sharing-folder-gone = وہ فولڈر اب وہاں نہیں ہے۔

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba نے وجہ نہیں بتائی۔

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Samba کا پیغام: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } کسی اور کا ہے، اس لیے آپ یہاں سے مہمانوں کو اس تک رسائی نہیں دے سکتے۔

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders { $path } تک مہمانوں کو رسائی نہیں دے سکا: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle Samba ڈاؤن لوڈ نہیں کر سکا کیونکہ لگتا ہے یہ کمپیوٹر آف لائن ہے۔ انٹرنیٹ سے جڑیں اور دوبارہ کوشش کریں۔

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Trindle Samba نصب نہیں کر سکا کیونکہ کچھ اور نصب ہو رہا ہے۔ چند منٹ بعد دوبارہ کوشش کریں۔
