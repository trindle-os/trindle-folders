

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = { $host } adlı bir sunucu bulunamadı

folders-far-not-all-arrived = tamamı ulaşmadı, bu yüzden olduğu yerde duruyor

folders-far-not-followed = bir bağlantı; izlenmediği için gönderilmedi

folders-far-not-sent = dosya ya da klasör değil, bu yüzden gönderilmedi

folders-far-not-told = sunucu ne olduğunu söylemiyor, bu yüzden atlandı

folders-far-ftp-cannot-refuse = Bu sunucu iki dosyaya aynı adı verebilir.

folders-far-cannot-do = Bu işlem bir sunucudaki dosyalara uygulanamaz

folders-far-not-looked-at = incelenemedi

folders-far-not-opened = Açılamadı.

folders-far-inside-itself = { $name } (klasör kendi içine konamaz)

folders-far-taken-by-folder = { $name } (bu adda bir klasör var)

folders-far-taken-by-file = { $name } (bu adda bir dosya var)

folders-far-address-unknown = Sunucunun adresi bilinmiyor.

folders-far-copy-wrong-length = gönderilen kopya olması gereken uzunlukta değildi

folders-far-only-server-file = Yalnızca sunucudaki bir dosya bu şekilde yeniden adlandırılabilir.

folders-far-aside-both = eskisi { $old } olarak tam durumda, yenisi { $new } olarak duruyor: { $why }

folders-far-aside-kept = eskisi kaldırılamadı ve { $old } olarak tam durumda duruyor

folders-far-aside-kept-why = eskisi kaldırılamadı ve { $old } olarak tam durumda duruyor: { $why }

folders-far-not-empty = { $name } boş değil.

folders-far-unsafe-name = Atlandı: sunucu güvenli olmayan bir ad gönderdi.

folders-far-unsafe-names =
    { $n ->
        [one] Atlandı: sunucu güvenli olmayan bir ad gönderdi: { $names }.
       *[other] { $n } öğe atlandı: sunucu güvenli olmayan adlar gönderdi: { $names }.
    }

folders-far-fetching = { $name } getiriliyor…

folders-far-cannot-fetch = { $name } getirilemiyor: { $why }.

folders-far-opened-copy = { $name } dosyasının bir kopyası açıldı. Kaydederseniz değişiklikler sunucuya geri gönderilir.

folders-far-saved = { $name } sunucuya kaydedildi.

folders-far-not-saved = { $name } sunucuya kaydedilemedi: { $why }. Buradaki kopya değişiklikleri koruyor.

folders-far-renamed = { $was } adı { $now } olarak değiştirildi.

folders-far-connect-title = Bir sunucuya bağlan

folders-far-connect-close = Kapat

folders-far-connect-address = Adres, örneğin sftp://jo@example.com

folders-far-connect-password = Parola, gerekiyorsa

folders-far-connect-key = Anahtar dosyası, seçerseniz

folders-far-connect-pick-key = Anahtar dosyasını seçin

folders-far-connect-choose = Seç

folders-far-connect-ssh-setup = Kendi ssh kurulumunuz kullanılır: yapılandırması, aracısı, anahtarları ve bilinen sunucuları.

folders-far-connect-new-host = { $host } sunucusuna daha önce bağlanılmadı, bu yüzden Trindle doğru sunucu olduğunu bilemez. Ulaşmayı bekliyorsanız güvenin.

folders-far-connect-host-changed = { $host } artık önceki kimliğine sahip değil. Bu, araya birinin girdiği anlamına gelebilir, bu yüzden açılmadı. Sahibi değiştirdiyse önce eskisini bilinen sunucularınızdan kaldırın.

folders-far-connect-needs-password = Bu sunucu bir parola istiyor. Yukarıya yazın ve yeniden deneyin.

folders-far-connect-needs-key = Bu sunucu bir anahtar ya da parola istiyor. Anahtar dosyanızı yukarıda belirtin ya da ssh aracınıza ekleyin ve yeniden deneyin.

folders-far-connect-cert-new = { $host } sunucusunun sertifikası denetlenemedi: { $why }. Mümkünse parmak izini sunucu sahibinin verdiğiyle karşılaştırın ve yalnızca eşleşiyorsa güvenin.

folders-far-connect-cert-changed = { $host } sunucusunun sertifikası daha önce güvendiğiniz sertifika değil. Sahibi yenilemiş olabilir ya da araya biri girmiş olabilir. Yenisine yalnızca sahibi bu parmak izini doğrularsa güvenin.

folders-far-connect-connecting = Bağlanılıyor

folders-far-connect-trust = Güven ve bağlan

folders-far-connect-go = Bağlan

folders-far-phones = Telefonlar, takılıp kilitleri açıldığında kenar çubuğunda kendiliğinden görünür.
