# machine translated (Turkish), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders bu klasörün içindekileri okuyamıyor.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = Yalnızca klasörün kendisini paylaşabilirsiniz, bir klasöre giden bağlantıyı değil.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = Yalnızca size ait klasörleri paylaşabilirsiniz. Bu klasör başkasına ait.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = ana klasörünüz

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = bu klasör

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } ve { $second }

# SAFETY: needs a human review before it ships
# Name of the switch that shares the folder.
folders-sharing-share = Bu klasörü paylaş

# One line under the share switch when the folder is shared.
folders-sharing-shared = Ev ağında paylaşılıyor.


# SAFETY: needs a human review before it ships
# Tip of the share switch.
folders-sharing-share-tip = Ev ağında paylaş

# Label over the box for the name other computers see.
folders-sharing-name = Ad

# Tip of the box for the name other computers see.
folders-sharing-name-tip = Diğer bilgisayarların gördüğü ad

# SAFETY: needs a human review before it ships
# Name of the switch that lets others change files, and its tip.
folders-sharing-write = Başkaları dosyaları değiştirebilsin

# SAFETY: needs a human review before it ships
# One line under the switch that lets others change files.
folders-sharing-write-sub = İçine dosya ekleyebilir, dosyaları değiştirebilir ve silebilirler.

# SAFETY: needs a human review before it ships
# Name of the switch that lets anyone in without a password.
folders-sharing-guests = Herkes girebilsin

# SAFETY: needs a human review before it ships
# One line under the let in anyone switch.
folders-sharing-guests-sub = Parola gerekmez.

# SAFETY: needs a human review before it ships
# Tip of the let in anyone switch.
folders-sharing-guests-tip = Parolasız herkes girebilsin

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Paylaşıldığında, evdeki diğer bilgisayarlarınızda (Windows olanlar dahil) Ağ bölümünde görünür. İlk seferde Trindle, klasörleri paylaşan program olan Samba'yı indirir. Bu bir dakika sürer.
       *[no] Paylaşıldığında, evdeki diğer bilgisayarlarınızda (Windows olanlar dahil) Ağ bölümünde görünür.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] Diğer bilgisayarlarda Ağ bölümünde { $host } üzerindeki { $name } öğesine bakın.
       *[no] Diğer bilgisayarlarda Ağ bölümünde { $name } öğesine bakın.
    }

# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = Diğer bilgisayarlar bu klasöre henüz ulaşamıyor. Ayarlar'da Ağ'ı açın ve "Diğer bilgisayarların bağlanmasına izin ver" seçeneğini açın.

# SAFETY: needs a human review before it ships
# Tip of the button that lets other computers connect.
folders-sharing-let-connect = Diğer bilgisayarların bağlanmasına izin ver

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
folders-sharing-visitors =
    { $n ->
        [one] Parolasız konuklar henüz açamaz, çünkü { $names } diğer hesaplara kapalı. İzin verirseniz bu klasöre ulaşabilirler ama yolda başka hiçbir şeyi göremezler.
       *[other] Parolasız konuklar henüz açamaz, çünkü { $names } diğer hesaplara kapalı. İzin verirseniz bu klasöre ulaşabilirler ama yolda başka hiçbir şeyi göremezler.
    }

# SAFETY: needs a human review before it ships
# Tip of the button that lets visitors through to the folder.
folders-sharing-let-through = Konukların bu klasöre ulaşmasına izin ver

# SAFETY: needs a human review before it ships
# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
folders-sharing-sign-in-set = Başkaları, paylaşım parolasıyla { $user } olarak giriş yapar.

# SAFETY: needs a human review before it ships
# Tip of the button that changes the sharing password.
folders-sharing-change-password = Paylaşım parolasını değiştir

# SAFETY: needs a human review before it ships
# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
folders-sharing-set-password = Başkaları { $user } olarak giriş yapar. En az { $n } karakterlik bir paylaşım parolası belirleyin. Oturum açma parolanızdan ayrıdır.

# SAFETY: needs a human review before it ships
# Label beside the sharing password box.
folders-sharing-password = Parola

# SAFETY: needs a human review before it ships
# Tip of the sharing password box.
folders-sharing-password-tip = Paylaşım parolası

# SAFETY: needs a human review before it ships
# Tip of the button that keeps the sharing password.
folders-sharing-keep-password = Parolayı kaydet

# Title of the Share on the network window; { $name } is the folder's name.
folders-sharing-card-title = { $name } klasörünü ağda paylaş

# Red words when the share's name is used by another share.
folders-sharing-name-taken = Bu adda paylaşılan bir klasörünüz zaten var. Başka bir ad seçin.

# Calm words while sharing is being stopped.
folders-sharing-stopping = Paylaşım durduruluyor…

# Red words when the sharing password is too short; { $n } is the fewest letters.
folders-sharing-password-short = Parola en az { $n } karakter olmalı.

# Calm words while the sharing password is being kept.
folders-sharing-keeping-password = Parola kaydediliyor. Önce sizden yöneticinin parolası istenecek.

# Calm words while other computers are being let connect.
folders-sharing-letting-connect = Diğer bilgisayarların bağlanmasına izin veriliyor. Önce sizden yöneticinin parolası istenecek.

# Calm words while Samba is fetched and set up the first time.
folders-sharing-fetching = Klasörleri paylaşan program olan Samba indirilip kuruluyor. Önce sizden yöneticinin parolası istenecek. Bu yalnızca ilk seferde olur.

# Calm words while sharing is set up the first time and Samba is already there.
folders-sharing-setting-up = Paylaşım kuruluyor. Önce sizden yöneticinin parolası istenecek. Bu yalnızca ilk seferde olur.

# Calm words while an existing share is changed.
folders-sharing-changing = Paylaşım değiştiriliyor…

# Calm words while a share is made.
folders-sharing-sharing-it = Klasör paylaşılıyor…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = Paylaşılan klasöre bir ad verin.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = Paylaşılan bir klasörün adı en fazla { $most } karakter olabilir.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = Paylaşılan bir klasörün adında % < > * ? | / \ + = ; : " ve virgül bulunamaz.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = $ ile biten bir ad paylaşımı gizler, bu yüzden kullanamazsınız.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = Paylaşılan bir klasörün adı tire ile başlayamaz.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba { $name } adını zaten kullanıyor. Başka bir ad seçin.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Paylaşılan

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = Yalnızca size ait klasörleri paylaşabilirsiniz.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = Bu ad bu bilgisayarda zaten bir hesap adı, bu yüzden paylaşım için kullanamazsınız. Başka bir ad seçin.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = Bu bilgisayarın Samba ayarları parolasız paylaşıma izin vermiyor.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = Bu bilgisayar daha fazla klasör paylaşamaz.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = Bu klasör artık yok.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba bir neden vermedi.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Samba'nın iletisi: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } başkasına ait, bu yüzden buradan konuklara erişim veremezsiniz.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders { $path } için konuklara erişim veremedi: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle, bu bilgisayar çevrimdışı göründüğü için Samba'yı indiremedi. İnternete bağlanıp yeniden deneyin.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Başka bir kurulum sürerken Trindle Samba'yı kuramadı. Birkaç dakika sonra yeniden deneyin.
