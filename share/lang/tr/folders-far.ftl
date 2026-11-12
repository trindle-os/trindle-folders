
# machine translated (Turkish), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = { $host } adlı bir sunucu yok

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = tamamı ulaşmadı, bu yüzden asıl dosya taşınmadı

# Reason a link was not sent to a server.
folders-far-not-followed = bu bir bağlantı, bu yüzden Folders göndermedi

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = dosya ya da klasör değil, bu yüzden Folders göndermedi

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = Folders ne tür bir öğe olduğunu anlayamadı, bu yüzden olduğu yerde bıraktı

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = Bu tür bir sunucuda bir dosya, aynı adlı başka bir dosyanın yerini alabilir.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = Sunucudaki dosyalara bunu yapamazsınız

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = Folders okuyamadı

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = Folders açamadı.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (klasör kendi içine konamaz)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (bu adda zaten bir klasör var)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (bu adda zaten bir dosya var)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = Folders sunucunun adresini bilmiyor.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = gönderilen kopyanın boyutu yanlıştı

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = Bu yalnızca sunucudaki dosyalarla çalışır.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = Folders yeni dosyayı yerine koyamadı, bu yüzden eskisi hâlâ orada, artık adı { $old }, yenisinin adı ise { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = Folders eski dosyayı kaldıramadı. Hâlâ orada, artık adı { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = Folders eski dosyayı kaldıramadı, bu yüzden hâlâ orada, artık adı { $old }: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } boş değil.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = Folders, adı kullanmak için güvenli olmadığından bir dosyayı atladı.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
        [one] Folders, adı kullanmak için güvenli olmadığından bir öğeyi atladı: { $names }.
       *[other] Folders, adları kullanmak için güvenli olmadığından { $n } öğeyi atladı: { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = { $name } indiriliyor…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = Folders { $name } dosyasını indiremedi: { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = { $name } dosyasının bir kopyası açıldı. Kaydettiğinizde değişiklikleriniz sunucuya geri gider.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = { $name } sunucuya kaydedildi.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = Folders { $name } dosyasını sunucuya kaydedemedi: { $why }. Değişiklikleriniz bu bilgisayardaki kopyada duruyor.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } adı { $now } olarak değiştirildi.

# Heading of the Connect to a server card.
folders-far-connect-title = Bir sunucuya bağlan

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Kapat

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Adres, örneğin sftp://jo@example.com

# SAFETY: needs a human review before it ships
# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
folders-far-connect-password = Parola, gerekiyorsa

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = Anahtar dosyası, kullanıyorsanız

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = Anahtar dosyasını seçin


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = Folders ssh ayarlarınızı, ssh aracınızı, anahtarlarınızı ve bilinen sunucu listenizi kullanır.

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card for a server not visited before; the server's name.
folders-far-connect-new-host = { $host } sunucusuna daha önce bağlanmadınız, bu yüzden Folders doğru sunucu olduğunu denetleyemez. Yalnızca bağlanmayı bekliyorsanız güvenin.

# SAFETY: needs a human review before it ships
# Red note on the Connect to a server card when a known server has a different identity; the server's name.
folders-far-connect-host-changed = { $host } sunucusunun kimliği son seferkinden farklı. Biri bağlantınızı araya girip dinlemeye çalışıyor olabilir, bu yüzden Folders bağlanmadı. Sahibi değiştirdiyse önce eski kaydını bilinen sunucularınızdan kaldırın.

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
folders-far-connect-needs-password = Bu sunucu için parola gerekiyor. Yukarıya yazıp yeniden deneyin.

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card when a server asks for a key or password.
folders-far-connect-needs-key = Bu sunucu için anahtar ya da parola gerekiyor. Yukarıdan anahtar dosyanızı seçin ya da ssh aracınıza ekleyin, sonra yeniden deneyin.

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
folders-far-connect-cert-new = Folders { $host } sunucusunun sertifikasını denetleyemedi: { $why }. Mümkünse parmak izini sahibinin verdiğiyle karşılaştırın. Yalnızca eşleşiyorsa güvenin.

# SAFETY: needs a human review before it ships
# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
folders-far-connect-cert-changed = { $host } sunucusunun sertifikası, güvendiğinizden bu yana değişti. Sahibi yenilemiş olabilir ya da biri bağlantınızı araya girip dinlemeye çalışıyor olabilir. Yalnızca sahibi bu parmak izini doğrularsa güvenin.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Bağlanılıyor

# SAFETY: needs a human review before it ships
# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
folders-far-connect-trust = Güven ve bağlan

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Bağlan

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Telefonlar, takıp kilidini açtığınızda kenar çubuğunda görünür.
