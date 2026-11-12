
# machine translated (Turkish), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
# SAFETY: needs a human review before it ships
folders-shares-nfs-denied = Bu bilgisayarın { $server } üzerindeki { $export } klasörünü açmasına izin verilmiyor.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } sunucusu { $export } adlı bir klasör paylaşmıyor.

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = Bir sonraki sefer size yeniden sorulur

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = Bir günden az kaldı

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] { $n } gün kaldı
       *[other] { $n } gün kaldı
    }

# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
# SAFETY: needs a human review before it ships
folders-shares-held =
    { $past ->
        [yes] { $server } şu an { $now } adresinde. Girişiniz daha önce farklı bir adreste kullanılmıştı ({ $before }).
       *[no] { $server } { $now } adresinde. Girişiniz bu adreste daha önce kullanılmamış.
    }{ $full ->
        [yes] {" "}Folders yalnızca son { $most } adresi hatırlar.
       *[no] {""}
    } Ağdaki herhangi bir aygıt bir ada sahip çıkabilir, bu yüzden Folders kayıtlı girişinizi göndermedi. Bu sizin { $server } sunucunuzsa, burada kullanmak için kullanıcı adını ve parolayı yazın.

# Words on the sign-in card when no address could be found for the computer to show.
# SAFETY: needs a human review before it ships
folders-shares-nowhere = Folders { $server } sunucusunun adresini bulamadı, bu yüzden ona hiçbir şey gönderilmedi. Buraya yazdığınız hiçbir şey güvenle gönderilemez. Bunun yerine adresiyle açın, örneğin smb://192.168.1.20/.

# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
# SAFETY: needs a human review before it ships
folders-shares-lapsed = { $server } { $now } adresinde. Girişiniz burada en son { $days } günden daha uzun süre önce kullanılmıştı. O zamandan beri bu adres başka bir aygıta verilmiş olabilir, bu yüzden Folders kayıtlı girişinizi göndermedi. Bu sizin { $server } sunucunuzsa, burada kullanmak için kullanıcı adını ve parolayı yazın.

# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
# SAFETY: needs a human review before it ships
folders-shares-unknown = { $server } { $now } adresinde. Bunun için burada daha önce bir giriş kullanılmıştı, ama Folders ne zaman ya da hangi girişin kullanıldığını not etmedi. O zamandan beri bu adres başka bir aygıta verilmiş olabilir, bu yüzden Folders kayıtlı girişinizi göndermedi. Bu sizin { $server } sunucunuzsa, burada kullanmak için kullanıcı adını ve parolayı yazın.

# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
# SAFETY: needs a human review before it ships
folders-shares-unshown = { $server } { $now } adresinde ve girişiniz daha önce burada kullanılmıştı. Bu kez Folders onu yerel ağda buldu; orada herhangi bir aygıt bu adı kullanabilir. Bu adresi daha önce görmediniz, bu yüzden Folders girişinizi göndermedi. Bu sizin { $server } sunucunuzsa, burada kullanmak için kullanıcı adını ve parolayı yazın.

# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
# SAFETY: needs a human review before it ships
folders-shares-asking =
    { $server } sunucusundaki bir hesabın kullanıcı adını ve parolasını yazın.{ $heard ->
        [yes] {" "}Sunucu { $at } adresinde. Ağdaki herhangi bir aygıt bir ada sahip çıkabilir, bu yüzden yazmadan önce adresin bu olduğunu kontrol edin.
       *[no] {""}
    }

# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
# SAFETY: needs a human review before it ships
folders-shares-where =
    Yazdıklarınız { $address } adresine gider{ $heard ->
        [yes] {" "}ve başka hiçbir yere gitmez. Anımsa açıkken Folders girişinizi yalnızca bu adres için saklar.
       *[no] {"."}
    }

# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
# SAFETY: needs a human review before it ships
folders-shares-again = Folders, { $server } sunucusunu yerel ağda { $at } adresinde, { $port } bağlantı noktasında buldu. Ağdaki herhangi bir aygıt bu adı kullanabilir, bu yüzden Folders girişinizi göndermedi. Adres doğruysa Aç'a basın.

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders anahtar halkasına ulaşamadı: { $why }.

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = Açık, ama Folders girişi kaydedemedi. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } NFS sürüm 4'ü sunmuyor; Folders yalnızca bu sürümü açabilir.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = Paylaşım açıldı, ama Folders bu bilgisayarda nerede olduğunu bulamadı.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = Paylaşım açıldı, ama Folders bu bilgisayarda nerede olduğunu bulamadı.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = Bu bir adres değil.

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = smb://{ $server } için kullanılan adresler

# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
# SAFETY: needs a human review before it ships
folders-shares-password-label = { $what } için { $user } parolası

# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
# SAFETY: needs a human review before it ships
folders-shares-remember = 90 gün anımsa

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = 90 gün, her kullanışta yeniden başlar.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $server } üzerindeki { $share }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = Aç: { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Kapat

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = Kullanıcı adı, örneğin jo ya da WORKGROUP\jo

# Hint in the empty password box of the sign-in card.
# SAFETY: needs a human review before it ships
folders-shares-password = Parola

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
# SAFETY: needs a human review before it ships
folders-shares-forget = Önceki adresleri unut

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
# SAFETY: needs a human review before it ships
folders-shares-forget-spoken = Bu giriş için önceki adresleri unut

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Açılıyor…

# Name of the main button of the sign-in card.
folders-shares-open = Aç

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = Bu bir bilgisayar adı değil.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = Bu paylaşılan bir klasörün yolu değil.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = { $host } üzerindeki { $export } açılıyor…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = { $server } üzerindeki { $share } açılıyor…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Önce { $it } için girişi tamamlayın.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = Açık. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } sizinle hiçbir klasör paylaşmıyor.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = { $name } kapatılıyor…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders { $name } adlı öğeyi kapatamadı: { $why }.

# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = { $name } yeniden bağlanıyor…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders paylaşımlarınızı bir sonraki sefer için kaydedemedi.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = Bu kullanıcı adı bir paylaşım için kullanılamaz.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = Bu etki alanı bir paylaşım için kullanılamaz.

# Red words when a share's password holds a line break.
folders-shares-bad-password = Bir paylaşımın parolası satır sonu içeremez.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server }, { $it } için kullanıcı adı ve parola gerektiriyor.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = { $it } için kullanıcı adı veya parola yanlış.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = { $server } üzerindeki hesabınız kilitli ya da kapalı, veya parolasının süresi dolmuş.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } üzerinde { $share } adlı paylaşılan klasör yok.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = Bu hesabın { $it } öğesini açmasına izin verilmiyor.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } yanıt vermedi. Uyku modunda ya da kapalı olabilir. Uyandırın, sonra tekrar deneyin.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders bu ağda { $server } adlı bir bilgisayar bulamadı.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } yalnızca SMB1 kullanıyor. Bu, artık güvenli olmayan eski bir paylaşım yöntemi. Folders onu kullanmaz.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = Bunun için Debian'ın { $package } paketi gerekir, ama kurulu değil.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders paylaşımı açamadı.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders paylaşılan klasörü açamadı.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders paylaşılan klasörleri listeleyemedi.
