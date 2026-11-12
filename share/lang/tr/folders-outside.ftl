
# machine translated (Turkish), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = gizli ya da gizli bir klasörün içinde

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = içinde gizli öğeler var

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = içinde denetlenemeyecek kadar çok öğe var, bu yüzden Folders gizli öğelerini dışarıda bırakır

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = ev klasörünüzün ve sürücülerinizin dışında

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = gizli bir şeye ya da ev klasörünüzün ve sürücülerinizin dışındaki bir şeye giden bağlantı

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = içinde gizli bir şeye ya da ev klasörünüzün ve sürücülerinizin dışındaki bir şeye giden bir bağlantı var

# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
# SAFETY: needs a human review before it ships
folders-outside-title-move =
    { $n ->
        [one] { $name } buraya taşınsın mı?
       *[other] Bu { $n } öge buraya taşınsın mı?
    }

# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
# SAFETY: needs a human review before it ships
folders-outside-title-bring =
    { $n ->
        [one] { $name } buraya getirilsin mi?
       *[other] Bu { $n } öge buraya getirilsin mi?
    }

# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
# SAFETY: needs a human review before it ships
folders-outside-title-copy =
    { $n ->
        [one] { $name } buraya kopyalansın mı?
       *[other] Bu { $n } öge buraya kopyalansın mı?
    }

# Words on the question card when the drop would be moved: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-move =
    { $n ->
        [one] Başka bir program bunu Folders'a bıraktı. Her program herhangi bir dosyayı bırakabilir ve bu, sıradan dosyalarınızdan biri değil. Siz söyleyene kadar Folders bunu taşımaz.
       *[other] Başka bir program bunları Folders'a bıraktı. Her program herhangi bir dosyayı bırakabilir ve bunların bazıları sıradan dosyalarınız değil. Siz söyleyene kadar Folders bunları taşımaz.
    }

# Words on the question card when the drop would be brought: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-bring =
    { $n ->
        [one] Başka bir program bunu Folders'a bıraktı. Her program herhangi bir dosyayı bırakabilir ve bu, sıradan dosyalarınızdan biri değil. Siz söyleyene kadar Folders bunu getirmez.
       *[other] Başka bir program bunları Folders'a bıraktı. Her program herhangi bir dosyayı bırakabilir ve bunların bazıları sıradan dosyalarınız değil. Siz söyleyene kadar Folders bunları getirmez.
    }

# Words on the question card when the drop would be copied: why it is asked.
# SAFETY: needs a human review before it ships
folders-outside-body-copy =
    { $n ->
        [one] Başka bir program bunu Folders'a bıraktı. Her program herhangi bir dosyayı bırakabilir ve bu, sıradan dosyalarınızdan biri değil. Siz söyleyene kadar Folders bunu kopyalamaz.
       *[other] Başka bir program bunları Folders'a bıraktı. Her program herhangi bir dosyayı bırakabilir ve bunların bazıları sıradan dosyalarınız değil. Siz söyleyene kadar Folders bunları kopyalamaz.
    }

# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
# SAFETY: needs a human review before it ships
folders-outside-leave =
    { $n ->
        [one] Dokunma
       *[other] Dokunma
    }

# Button on the question card that moves what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-move =
    { $n ->
        [one] Taşı
       *[other] Taşı
    }

# Button on the question card that brings what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-bring =
    { $n ->
        [one] Getir
       *[other] Getir
    }

# Button on the question card that copies what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-copy =
    { $n ->
        [one] Kopyala
       *[other] Kopyala
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, { $from } klasöründen: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, { $from } klasöründen

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = Ve { $n } tane daha.

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = Hedef: { $into }.

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Önce bıraktığınız şeyle ilgili soruyu yanıtlayın.

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders en son bıraktığınız şeyi hâlâ denetliyor.

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Buraya yalnızca dosya ve klasör bırakılabilir.

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = Kenar çubuğuna yalnızca klasörler sabitlenebilir.

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = Yalnızca bir klasörün üzerine bir şey bırakabilirsiniz.

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } artık orada değil, bu yüzden hiçbir şey kopyalanmadı veya taşınmadı.

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = Bir klasörü kendi içine koyamazsınız.

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = Zaten o klasörde.

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders { $name } öğesinin bulunduğu sürücüye zamanında ulaşamadı, bu yüzden hiçbir şey kopyalanmadı veya taşınmadı.

# Words when the question card is answered with no.
folders-outside-nothing-brought = Hiçbir şey kopyalanmadı veya taşınmadı.

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] 1 öge
       *[other] { $n } öge
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = ve { $n } tane daha

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] { $n } öğe sürüklemeye başladıktan sonra değişti, bu yüzden Folders onu olduğu yerde bıraktı: { $names }.
       *[other] { $n } öğe sürüklemeye başladıktan sonra değişti, bu yüzden Folders onları olduğu yerlerde bıraktı: { $names }.
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] { $n } öğe hâlâ eski yerinde: { $each }.
       *[other] { $n } öğe hâlâ eski yerlerinde: { $each }.
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders diğer programın sürüklediğiniz her şeyi alıp almadığını anlayamıyor, bu yüzden asılları burada bıraktı. Diğer programda bir kopya var.

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = Diğer program sürüklediğiniz her şeyi almadı, bu yüzden Folders hiçbir şeyi taşımadı.

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = Diğer program sürüklediğiniz her şeyi almadı, bu yüzden elindeki kopya eksik olabilir.
