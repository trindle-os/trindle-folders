

folders-shares-nfs-denied = { $server } sunucusu bu bilgisayarın { $export } klasörünü açmasına izin vermiyor.

folders-shares-nfs-no-share = { $server } sunucusunda { $export } adlı paylaşım yok.

folders-shares-left-none = Bir sonraki açılışta yeniden sorulur

folders-shares-left-under-day = Bir günden az kaldı

folders-shares-left-days =
    { $n ->
        [one] { $n } gün kaldı
       *[other] { $n } gün kaldı
    }

folders-shares-held =
    { $past ->
        [yes] { $server } şu an { $now } adresinden yanıt veriyor; bu, girişinizin daha önce kullanıldığı adresten ({ $before }) farklı.
       *[no] { $server } { $now } adresinden yanıt veriyor; girişiniz bu adreste daha önce kullanılmamış.
    }{ $full ->
        [yes] {" "}Yalnızca kullanıldığı son { $most } adres hatırlanır.
       *[no] {""}
    } Ağdaki her aygıt bir ada yanıt verebilir, bu yüzden bunun için saklanan giriş gönderilmedi. Bu sizin { $server } sunucunuzsa, burada kullanmak için kullanıcı adını ve parolayı yazın.

folders-shares-nowhere = { $server } sunucusunun yanıt verdiği adres önceden bulunamadı, bu yüzden ona hiçbir şey gönderilmedi ve buraya yazılanların güvenle gidebileceği bir yer yok. Bunun yerine adresiyle açın, örneğin smb://192.168.1.20/.

folders-shares-lapsed = { $server } { $now } adresinden yanıt veriyor; girişiniz burada en son { $days } günden daha uzun süre önce kullanılmıştı. O zamandan beri bu adres başka bir aygıta verilmiş olabilir, bu yüzden saklanan giriş gönderilmedi. Bu sizin { $server } sunucunuzsa, burada kullanmak için kullanıcı adını ve parolayı yazın.

folders-shares-unknown = { $server } { $now } adresinden yanıt veriyor; burada daha önce bir giriş kullanılmıştı, ama ne zaman ya da hangi giriş olduğu kaydedilmemiş. O zamandan beri bu adres başka bir aygıta verilmiş olabilir, bu yüzden saklanan giriş gönderilmedi. Bu sizin { $server } sunucunuzsa, burada kullanmak için kullanıcı adını ve parolayı yazın.

folders-shares-unshown = { $server } { $now } adresinden yanıt veriyor; girişiniz daha önce bu adrese gitmişti. Bu kez adres yalnızca ağda duyuldu, ağdaki her aygıt bir ada yanıt verebilir ve bu adres size burada daha önce gösterilmedi, bu yüzden giriş gönderilmedi. Bu sizin { $server } sunucunuzsa, burada kullanmak için kullanıcı adını ve parolayı yazın.

folders-shares-asking =
    { $server } bir kullanıcı adı ve parola istiyor. Bunlar o bilgisayardaki bir hesabın bilgileridir.{ $heard ->
        [yes] {" "}{ $at } adresinden yanıt veriyor. Ağdaki her aygıt bir ada yanıt verebilir, bu yüzden yazmadan önce adresin bu olduğundan emin olun.
       *[no] {""}
    }

folders-shares-where =
    Yazdıklarınız { $address } adresine gider{ $heard ->
        [yes] {" "}ve başka hiçbir yere gitmez. Anımsa açıkken girişiniz yalnızca bu adres için saklanır.
       *[no] {"."}
    }

folders-shares-again = { $server } { $at } adresinden, { $port } numaralı bağlantı noktasından yanıt veriyor. Bu adres yalnızca ağda duyuldu, ağdaki her aygıt bir ada yanıt verebilir, bu yüzden girişiniz gönderilmedi. Adres buysa Aç'a basın.

folders-shares-keyring-unreachable = Anahtar halkasına ulaşılamadı: { $why }.

folders-shares-unkept = Açık, ama giriş saklanmadı. { $why }

folders-shares-nfs-version = { $server } NFS sürüm 4'ü sunmuyor; Trindle yalnızca bu sürümü açar.

folders-shares-nfs-opened-unknown = Açıldı, ama nerede olduğu bilinmiyor.

folders-shares-opened-unknown = Paylaşım açıldı, ama nerede olduğu bilinmiyor.

folders-shares-not-address = Bu bir adres değil.

folders-shares-seen-label = smb://{ $server } sunucusunun açıldığı adresler

folders-shares-password-label = { $what } için { $user } parolası

folders-shares-remember = 90 gün anımsa

folders-shares-remember-line = Her kullanışta yeniden başlar.

folders-shares-share-on = { $server } üzerindeki { $share }

folders-shares-card-title = Aç: { $it }

folders-shares-close = Kapat

folders-shares-user-placeholder = Kullanıcı adı, örneğin jo ya da WORKGROUP\jo

folders-shares-password = Parola

folders-shares-forget = Önceki adresleri unut

folders-shares-forget-spoken = Bu girişin daha önce kullanıldığı adresleri unut

folders-shares-opening = Açılıyor

folders-shares-open = Aç

folders-shares-bad-computer = Bu bir bilgisayar adı değil.

folders-shares-bad-path = Bu paylaşılan bir klasörün yolu değil.

folders-shares-opening-export = { $host } üzerindeki { $export } açılıyor…

folders-shares-opening-share = { $server } üzerindeki { $share } açılıyor…

folders-shares-answer-card = Önce { $it } için olan kartı yanıtlayın.

folders-shares-open-waiting = Açık. { $words }

folders-shares-none = { $server } sizinle hiçbir klasör paylaşmıyor.

folders-shares-closing = { $name } kapatılıyor…

folders-shares-close-failed = { $name } kapatılamadı: { $why }.

folders-shares-reconnecting = { $name } yeniden bağlanıyor…

folders-shares-not-remembered = Paylaşımlar bir sonraki sefer için hatırlanamadı.
