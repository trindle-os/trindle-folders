
# machine translated (Turkish), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = Kapat

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = Ev

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = { $name } dosyasını şununla aç

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = Geçerli uygulama

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = Bu bilgisayarda bu tür dosyayı açacak şekilde ayarlanmış uygulama yok.

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = Her zaman bu uygulamayı kullan

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = Her { $kind } dosyası için kullan.

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = Varsayılan uygulama yap

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = Yalnızca önerilen uygulamaları göster

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = Tüm uygulamaları göster

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = Aç

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = Önce bir uygulama seçin.

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders { $name } uygulamasını varsayılan uygulama yapamadı.

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders { $name } uygulamasını başlatamadı.

# Heading of the Properties window.
folders-dialogs-properties = Özellikler

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = Özellikler sayfaları

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = Genel

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = Ayrıntılar

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = Paylaşım



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = Ad

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = Konum

# Label in Properties of the kind of file.
folders-dialogs-row-kind = Tür

# Label in Properties of where a link points.
folders-dialogs-row-link-to = Bağlantı hedefi

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = Değiştirilme

# Label in Properties of who owns the item.
folders-dialogs-row-owner = Sahip

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = İzinler

# Label in Properties of the size of the item.
folders-dialogs-row-size = Boyut

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = Bilinmiyor

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, açabilir ve değiştirebilirsiniz

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, açabilirsiniz ama değiştiremezsiniz

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, değiştirebilirsiniz ama açamazsınız

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, açamazsınız

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , bazıları okunamadı
       *[other]
    }{ $adding ->
        [yes] , hâlâ hesaplanıyor
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = Boyut hesaplanıyor

# Label in Properties of the row about checksums.
folders-dialogs-checksums = Sağlama toplamları

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = Sağlama toplamlarını hesapla

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = Sağlama toplamları hesaplanıyor

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders dosyayı okuyamadı.

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = Karşılaştırmak için bir sağlama toplamı yapıştırın

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } eşleşiyor

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = Eşleşme yok

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = Denetle


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [one] { $name } öğesini şuraya gönder
       *[other] { $n } öğeyi şuraya gönder
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = Bu, oraya bir kopya yapar. Asıllar olduğu yerde kalır.

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = Diğer bölme, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = Liste görünümü sütunları

# Note in the Columns window.
folders-dialogs-columns-note = Ad her zaman gösterilir. Sığmayan sütunlar, bölme genişleyene kadar gizlenir.

# Heading over the columns that suit every file.
folders-dialogs-columns-files = Dosyalar

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = Müzik, fotoğraflar, PDF'ler

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = { $name } göster

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders sütunları kaydedemedi.

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [one] { $name } için renk
       *[other] { $n } klasör için renk
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = Renk yok

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders klasör rengini kaydedemedi.


folders-dialogs-no-match = Bu sözcüklerle eşleşen uygulama yok.


folders-dialogs-find-app = Uygulama bul


folders-dialogs-find-place = Yer bul


folders-dialogs-no-match-place = Bu sözcüklerle eşleşen yer yok.
