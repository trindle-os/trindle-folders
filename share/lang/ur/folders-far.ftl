

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = { $host } نام کا کوئی سرور نہیں ملا

folders-far-not-all-arrived = یہ پوری نہیں پہنچی، اس لیے یہ اپنی جگہ پر ہی ہے

folders-far-not-followed = یہ ایک لنک ہے جس کے پیچھے نہیں جایا جاتا، اس لیے نہیں بھیجا گیا

folders-far-not-sent = یہ فائل یا فولڈر نہیں ہے، اس لیے نہیں بھیجا گیا

folders-far-not-told = سرور نہیں بتاتا کہ یہ کیا ہے، اس لیے اسے چھوڑ دیا گیا

folders-far-ftp-cannot-refuse = یہ سرور دو فائلوں کو ایک ہی نام دے سکتا ہے۔

folders-far-cannot-do = سرور کی فائلوں کے ساتھ یہ نہیں کیا جا سکتا

folders-far-not-looked-at = اسے دیکھا نہیں جا سکا

folders-far-not-opened = اسے کھولا نہیں جا سکا۔

folders-far-inside-itself = { $name } (فولڈر اپنے اندر نہیں جا سکتا)

folders-far-taken-by-folder = { $name } (اس نام کا ایک فولڈر موجود ہے)

folders-far-taken-by-file = { $name } (اس نام کی ایک فائل موجود ہے)

folders-far-address-unknown = سرور کا پتا معلوم نہیں۔

folders-far-copy-wrong-length = بھیجی گئی کاپی کی لمبائی وہ نہیں تھی جو ہونی چاہیے تھی

folders-far-only-server-file = نام صرف سرور کی فائل کا بدلا جا سکتا ہے۔

folders-far-aside-both = پرانی فائل پوری { $old } کے طور پر موجود ہے، اور نئی { $new } کے طور پر: { $why }

folders-far-aside-kept = پرانی فائل ہٹائی نہیں جا سکی، اور پوری { $old } کے طور پر موجود ہے

folders-far-aside-kept-why = پرانی فائل ہٹائی نہیں جا سکی، اور پوری { $old } کے طور پر موجود ہے: { $why }

folders-far-not-empty = { $name } خالی نہیں ہے۔

folders-far-unsafe-name = چھوڑ دیا گیا: سرور نے غیر محفوظ نام بھیجا۔

folders-far-unsafe-names =
    { $n ->
        [one] چھوڑ دیا گیا: سرور نے غیر محفوظ نام بھیجا: { $names }۔
       *[other] { $n } چھوڑ دیے گئے: سرور نے غیر محفوظ نام بھیجے: { $names }۔
    }

folders-far-fetching = { $name } لایا جا رہا ہے…

folders-far-cannot-fetch = { $name } لایا نہیں جا سکا: { $why }۔

folders-far-opened-copy = { $name } کی ایک کاپی کھل گئی۔ محفوظ کرنے پر تبدیلیاں سرور کو واپس بھیج دی جاتی ہیں۔

folders-far-saved = { $name } سرور پر محفوظ ہو گئی۔

folders-far-not-saved = { $name } سرور پر محفوظ نہیں ہو سکی: { $why }۔ یہاں کی کاپی میں تبدیلیاں موجود ہیں۔

folders-far-renamed = { $was } کا نام بدل کر { $now } کر دیا گیا۔

folders-far-connect-title = سرور سے جڑیں

folders-far-connect-close = بند کریں

folders-far-connect-address = پتا، جیسے sftp://jo@example.com

folders-far-connect-password = پاس ورڈ، اگر ضرورت ہو

folders-far-connect-key = کلید کی فائل، اگر آپ چنیں

folders-far-connect-pick-key = کلید کی فائل چنیں

folders-far-connect-choose = چنیں

folders-far-connect-ssh-setup = آپ کا اپنا ssh انتظام استعمال ہوتا ہے: اس کی ترتیب، ایجنٹ، کلیدیں اور جانے پہچانے سرور۔

folders-far-connect-new-host = { $host } پر پہلے کبھی نہیں گئے، اس لیے Trindle نہیں بتا سکتا کہ یہ صحیح سرور ہے۔ اگر آپ اسی تک پہنچنا چاہتے تھے تو اس پر بھروسا کریں۔

folders-far-connect-host-changed = { $host } کی شناخت وہ نہیں رہی جو پہلے تھی۔ اس کا مطلب ہو سکتا ہے کہ کوئی بیچ میں آ گیا ہے، اس لیے اسے نہیں کھولا گیا۔ اگر اس کے مالک نے اسے بدلا ہے تو پہلے اپنے جانے پہچانے میزبانوں سے پرانی اندراج ہٹائیں۔

folders-far-connect-needs-password = اس سرور کو پاس ورڈ چاہیے۔ اسے اوپر لکھیں اور دوبارہ کوشش کریں۔

folders-far-connect-needs-key = اس سرور کو کلید یا پاس ورڈ چاہیے۔ اوپر اپنی کلید کی فائل کا نام لکھیں، یا اسے اپنے ssh ایجنٹ میں شامل کریں، اور دوبارہ کوشش کریں۔

folders-far-connect-cert-new = { $host } کا سرٹیفکیٹ جانچا نہیں جا سکا: { $why }۔ ہو سکے تو اس کے فنگر پرنٹ کا موازنہ اس سے کریں جو سرور کا مالک بتائے، اور صرف تب بھروسا کریں جب دونوں ملیں۔

folders-far-connect-cert-changed = { $host } کا سرٹیفکیٹ وہ نہیں جس پر آپ نے پہلے بھروسا کیا تھا۔ ہو سکتا ہے اس کے مالک نے اسے تجدید کیا ہو، یا کوئی بیچ میں آ گیا ہو۔ نئے پر صرف تب بھروسا کریں جب اس کا مالک اس فنگر پرنٹ کی تصدیق کرے۔

folders-far-connect-connecting = جڑ رہا ہے

folders-far-connect-trust = بھروسا کریں اور جڑیں

folders-far-connect-go = جڑیں

folders-far-phones = فون پلگ ان اور ان لاک ہوتے ہی سائیڈ بار میں خود بخود نظر آ جاتے ہیں۔
