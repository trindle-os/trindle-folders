

folders-shares-nfs-denied = { $server } اس کمپیوٹر کو { $export } کھولنے نہیں دیتا۔

folders-shares-nfs-no-share = { $server } پر { $export } نام کی کوئی شیئر نہیں ہے۔

folders-shares-left-none = اگلی بار کھولنے پر دوبارہ پوچھے گا

folders-shares-left-under-day = ایک دن سے کم باقی

folders-shares-left-days =
    { $n ->
        [one] { $n } دن باقی
       *[other] { $n } دن باقی
    }

folders-shares-held =
    { $past ->
        [yes] { $server } اب { $now } پر جواب دیتا ہے، جو اس پتے سے مختلف ہے جس پر پہلے اس کے لیے آپ کا لاگ ان استعمال ہوا تھا ({ $before })۔
       *[no] { $server } { $now } پر جواب دیتا ہے، ایسا پتا جس پر اس کے لیے آپ کا لاگ ان پہلے کبھی استعمال نہیں ہوا۔
    }{ $full ->
        [yes] {" "}صرف آخری { $most } پتے یاد رکھے جاتے ہیں جن پر یہ استعمال ہوا۔
       *[no] {""}
    } نیٹ ورک پر کوئی بھی آلہ کسی نام پر جواب دے سکتا ہے، اس لیے اس کے لیے رکھا گیا لاگ ان بھیجا نہیں گیا۔ اگر یہ آپ کا { $server } ہے تو اسے یہاں استعمال کرنے کے لیے اس کا صارف نام اور پاس ورڈ لکھیں۔

folders-shares-nowhere = وہ پتا جس پر { $server } جواب دیتا ہے پہلے سے نہیں مل سکا، اس لیے اسے کچھ نہیں بھیجا گیا اور یہاں جو لکھا جائے اس کے لیے کوئی محفوظ جگہ نہیں۔ اسے اس کے پتے سے کھولیں، جیسے smb://192.168.1.20/۔

folders-shares-lapsed = { $server } { $now } پر جواب دیتا ہے، جہاں اس کے لیے آپ کا لاگ ان آخری بار { $days } دن سے زیادہ پہلے استعمال ہوا تھا۔ ہو سکتا ہے اس کے بعد وہ پتا کسی اور آلے کو دے دیا گیا ہو، اس لیے اس کے لیے رکھا گیا لاگ ان بھیجا نہیں گیا۔ اگر یہ آپ کا { $server } ہے تو اسے یہاں استعمال کرنے کے لیے اس کا صارف نام اور پاس ورڈ لکھیں۔

folders-shares-unknown = { $server } { $now } پر جواب دیتا ہے، جہاں اس کے لیے پہلے لاگ ان استعمال ہوا تھا، مگر کب، یا کون سا لاگ ان، لکھا نہیں گیا۔ ہو سکتا ہے اس کے بعد وہ پتا کسی اور آلے کو دے دیا گیا ہو، اس لیے اس کے لیے رکھا گیا لاگ ان بھیجا نہیں گیا۔ اگر یہ آپ کا { $server } ہے تو اسے یہاں استعمال کرنے کے لیے اس کا صارف نام اور پاس ورڈ لکھیں۔

folders-shares-unshown = { $server } { $now } پر جواب دیتا ہے، یہ وہی پتا ہے جہاں پہلے اس کے لیے آپ کا لاگ ان گیا تھا۔ اس بار وہ پتا صرف نیٹ ورک پر سنا گیا، جہاں کوئی بھی آلہ کسی نام پر جواب دے سکتا ہے، اور وہ آپ کو یہاں پہلے دکھایا نہیں گیا، اس لیے لاگ ان بھیجا نہیں گیا۔ اگر یہ آپ کا { $server } ہے تو اسے یہاں استعمال کرنے کے لیے اس کا صارف نام اور پاس ورڈ لکھیں۔

folders-shares-asking =
    { $server } کو صارف نام اور پاس ورڈ چاہیے۔ یہ اس کمپیوٹر پر کسی اکاؤنٹ کے ہیں۔{ $heard ->
        [yes] {" "}یہ { $at } پر جواب دیتا ہے۔ نیٹ ورک پر کوئی بھی آلہ کسی نام پر جواب دے سکتا ہے، اس لیے لکھنے سے پہلے دیکھ لیں کہ یہی اس کا پتا ہے۔
       *[no] {""}
    }

folders-shares-where =
    جو آپ لکھیں گے وہ { $address } کو جاتا ہے{ $heard ->
        [yes] {" "}اور کہیں اور نہیں۔ یاد رکھیں چالو ہو تو آپ کا لاگ ان صرف اسی پتے کے لیے رکھا جاتا ہے۔
       *[no] {"۔"}
    }

folders-shares-again = { $server } { $at } پر، پورٹ { $port } سے جواب دیتا ہے۔ وہ پتا صرف نیٹ ورک پر سنا گیا، جہاں کوئی بھی آلہ کسی نام پر جواب دے سکتا ہے، اس لیے اس کے لیے آپ کا لاگ ان بھیجا نہیں گیا۔ اگر وہی اس کا پتا ہے تو کھولیں دبائیں۔

folders-shares-keyring-unreachable = کی رنگ تک رسائی نہیں ہو سکی: { $why }۔

folders-shares-unkept = یہ کھلا ہے، مگر لاگ ان محفوظ نہیں ہوا۔ { $why }

folders-shares-nfs-version = { $server } NFS ورژن 4 پیش نہیں کرتا، جو واحد قسم ہے جسے Trindle کھولتا ہے۔

folders-shares-nfs-opened-unknown = یہ کھل گیا، مگر معلوم نہیں کہاں۔

folders-shares-opened-unknown = شیئر کھل گئی، مگر معلوم نہیں کہاں۔

folders-shares-not-address = یہ پتا نہیں ہے۔

folders-shares-seen-label = وہ پتے جن پر smb://{ $server } کھولا گیا

folders-shares-password-label = { $what } پر { $user } کا پاس ورڈ

folders-shares-remember = 90 دن کے لیے یاد رکھیں

folders-shares-remember-line = ہر بار استعمال کرنے پر دوبارہ شروع ہوتا ہے۔

folders-shares-share-on = { $server } پر { $share }

folders-shares-card-title = { $it } کھولیں

folders-shares-close = بند کریں

folders-shares-user-placeholder = صارف نام، جیسے jo، یا WORKGROUP\jo

folders-shares-password = پاس ورڈ

folders-shares-forget = پہلے کے پتے بھول جائیں

folders-shares-forget-spoken = وہ پتے بھول جائیں جن پر پہلے یہ لاگ ان استعمال ہوا تھا

folders-shares-opening = کھل رہا ہے

folders-shares-open = کھولیں

folders-shares-bad-computer = یہ کسی کمپیوٹر کا نام نہیں ہے۔

folders-shares-bad-path = یہ کسی مشترکہ فولڈر کا راستہ نہیں ہے۔

folders-shares-opening-export = { $host } پر { $export } کھل رہا ہے…

folders-shares-opening-share = { $server } پر { $share } کھل رہا ہے…

folders-shares-answer-card = پہلے { $it } کے کارڈ کا جواب دیں۔

folders-shares-open-waiting = یہ کھلا ہے۔ { $words }

folders-shares-none = { $server } آپ کے ساتھ کوئی فولڈر شیئر نہیں کرتا۔

folders-shares-closing = { $name } بند ہو رہا ہے…

folders-shares-close-failed = { $name } بند نہیں ہو سکا: { $why }۔

folders-shares-reconnecting = { $name } دوبارہ جوڑا جا رہا ہے…

folders-shares-not-remembered = شیئرز اگلی بار کے لیے یاد نہیں رکھے جا سکے۔
