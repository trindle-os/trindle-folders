
# machine translated (Turkish), not yet reviewed by a person.

# Words on the Connect card for a server heard only on the network; the variables are its name, its address and its port.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-card = Folders, { $host } sunucusunu yerel ağda { $at } adresinde, { $port } bağlantı noktasında buldu. Ağdaki herhangi bir aygıt bu adı kullanabilir, bu yüzden Folders parolanızı göndermedi. Adres doğruysa Bağlan düğmesine basın.

# Red words where work with a server stopped because its address was only heard on the network; the variables are its name and address.
# SAFETY: needs a human review before it ships
folders-remote-mod-heard-stopped = Folders, { $host } sunucusunu yerel ağda { $at } adresinde buldu. Ağdaki herhangi bir aygıt bu adı kullanabilir, bu yüzden Folders parolanızı göndermedi. Adresi denetlemek için { $host } sunucusunu yeniden açın.

# Red words when an address typed could not be read; the variable is the reason, a short phrase.
folders-remote-mod-bad-address = Folders bu adresi okuyamadı: { $why }.

# Red words when a server needs a password or key and none could be asked for.
# SAFETY: needs a human review before it ships
folders-remote-mod-needs-password = Bu sunucu bir parola ya da anahtar istiyor.

# Red words when a server turned down the login.
# SAFETY: needs a human review before it ships
folders-remote-mod-login-refused = Sunucu bu girişi kabul etmedi.

# Words on the card that asks whether to trust a server seen for the first time; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-unknown = { $host } sunucusuna daha önce bağlanmadınız, bu yüzden Folders kimliğini henüz bilmiyor.

# Red words when a server's identity differs from the one remembered; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-host-key-changed = { $host } sunucusunun kimliği son seferkinden farklı. Yeniden kurulmuş olabilir ya da biri bağlantıya müdahale ediyor olabilir.

# Words on the card that asks whether to trust a certificate that failed the usual checks; the variables are the server's name and the reason.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-unknown = Folders { $host } sertifikasını denetleyemedi: { $why }.

# Red words when a server's certificate differs from the one trusted before; the variable is the server's name.
# SAFETY: needs a human review before it ships
folders-remote-mod-cert-changed = { $host } sertifikası, ona güvendiğinizden beri değişmiş.

# Red words when a server or phone could not be reached; the variable is the reason, a short phrase.
folders-remote-mod-unreachable = Folders bağlanamadı: { $why }.

# Red words when something on a server or phone is not there; the variable is its name.
folders-remote-mod-not-found = Folders şunu bulamıyor: { $what }.

# Red words when a name on a server or phone is already taken; the variable is the name.
folders-remote-mod-exists = { $what } zaten var.

# Red words when a server or phone refused permission; the variable is what it was refused for.
# SAFETY: needs a human review before it ships
folders-remote-mod-denied = { $what } için izniniz yok.

# Red words when a phone is open in another program; the variable is what is in use ("The phone").
folders-remote-mod-busy = { $what } başka bir program tarafından kullanılıyor.

# Red words when a server or phone cannot do something; the variable is the thing it cannot do, a short phrase.
folders-remote-mod-not-supported = { $what } desteklenmiyor.

# Red words when a copy or other work with a server was cancelled.
# SAFETY: needs a human review before it ships
folders-remote-mod-cancelled = İptal edildi.

# The thing a server cannot do, in "Links is not supported yet.": reading where a link points.
folders-remote-mod-links = Bağlantıları izleme

# The thing that cannot be done, in "... is not supported yet.": copying with a Windows share that is not opened yet.
folders-remote-mod-copy-share = Bir Windows paylaşımı açılmadan önce ona veya ondan kopyalama

# The thing that cannot be done, in "... is not supported yet.": copying with an NFS export that is not opened yet.
folders-remote-mod-copy-nfs = Bir NFS paylaşımı açılmadan önce ona veya ondan kopyalama

# Reason inside "That address could not be read": the address does not start with a kind of server.
folders-remote-address-no-scheme = sftp://, ftp:// ya da mtp:// ile başlamıyor

# Reason inside "That address could not be read": the kind of server is not one Folders opens.
folders-remote-address-bad-scheme = sftp://, ftp://, ftps://, ftpes://, dav://, davs://, smb://, nfs:// ya da mtp:// ile başlamalı

# Reason inside "That address could not be read": a square bracket was opened and never closed.
folders-remote-address-no-bracket = [ işaretinin kapanışı olan ] eksik

# Reason inside "That address could not be read": no server is named.
folders-remote-address-no-server = bir sunucu adı belirtilmemiş

# Reason inside "That address could not be read": the server's name is not a valid name.
folders-remote-address-bad-name = sunucu adı geçerli değil

# Reason inside "That address could not be read": the path has a line break in it.
folders-remote-address-line-break = yolun içinde satır sonu var

# Reason inside "That address could not be read": something odd follows the server's name.
folders-remote-address-after-name = sunucu adından sonra fazladan bir şey var

# Reason inside "That address could not be read": the port is not a number; the variable is what was written.
folders-remote-address-not-port = { $port } geçerli bir bağlantı noktası numarası değil

# Note beside a thing in a server's folder that was skipped because its name is unsafe.
folders-remote-screen-not-a-name = sunucu güvenli olmayan bir ad gönderdiği için atlandı

# Stand-in for the name of a thing in a server's folder that has no name.
folders-remote-screen-no-name = adı olmayan bir şey

# Red words when a server or phone sent far more than the answer to a question could hold, and was stopped.
folders-remote-mod-too-much = Yanıt çok büyüktü, bu yüzden Folders onu durdurdu.

# Why a thing on a WebDAV server was not removed as a file: it is a folder now.
folders-remote-dav-not-a-file = { $name } artık bir klasör, bu yüzden Folders onu kaldırmadı.
