
# machine translated (French), not yet reviewed by a person.

# Reason an archive would not open: it is locked with a password and none was given.
# SAFETY: needs a human review before it ships
folders-archive-trouble-locked = elle est verrouillée par un mot de passe

# Reason an archive would not open: the password given is not the one.
# SAFETY: needs a human review before it ships
folders-archive-trouble-wrong-key = ce mot de passe ne l’ouvre pas

# Reason an archive would not open: what came out with the password makes no sense, so the password is wrong or the archive is damaged.
# SAFETY: needs a human review before it ships
folders-archive-trouble-wrong-or-bad = ce mot de passe ne l’ouvre pas, ou l’archive est endommagée

# Reason an archive would not open: it is a RAR locked with a password.
# SAFETY: needs a human review before it ships
folders-archive-trouble-locked-rar = c’est un fichier RAR protégé par un mot de passe que Folders ne peut pas déverrouiller, mais le fichier lui-même n’est pas endommagé

# Reason an archive would not open: it is locked and packed a way Folders does not know.
# SAFETY: needs a human review before it ships
folders-archive-trouble-locked-foreign = elle utilise un type de protection par mot de passe que Folders ne peut pas déverrouiller

# Reason a file would not open as an archive: it is not a kind Folders reads.
folders-archive-trouble-unknown = ce n’est pas un type d’archive que Folders peut ouvrir

# Reason an archive would not unpack: it holds too many things.
folders-archive-trouble-too-many = elle contient plus d’un million d’éléments, trop pour que Folders les extraie

# Reason an archive would not unpack: the place it was to go has too little room.
folders-archive-trouble-no-room = il n’y a pas assez de place ici pour son contenu

# Reason an archive would not open: reading it takes more memory or time than is allowed.
folders-archive-trouble-gave-up = son ouverture demande plus de mémoire ou de temps que Folders n’en permet

# Reason an archive would not open: the program that reads this kind is not on the computer.
folders-archive-trouble-no-reader = la partie de Trindle qui ouvre ce type d’archive est absente ; installez les mises à jour pour la remettre

# Reason an archive would not open: it is packed a way Folders does not know.
folders-archive-trouble-foreign = elle utilise un type de compression que Folders ne peut pas lire

# Reason a split archive would not open: one of its parts is not there.
folders-archive-trouble-parts = une partie est manquante, et toutes ses parties doivent se trouver dans le même dossier

# Reason given for an archive that cannot be read when the file is damaged or ends too soon.
folders-archive-why-damaged = elle est endommagée ou incomplète

# Reason given for an archive that cannot be written when the drive is full.
folders-archive-why-no-room = il n'y a plus de place

# Reason given for an archive that cannot be unpacked when the folder cannot be written in.
folders-archive-why-read-only = vous n'avez pas l'autorisation, ou le disque est en lecture seule

# Reason given for an archive that cannot be read when the file has gone.
folders-archive-why-gone = elle n'existe plus

# Reason given for an archive that cannot be read: it needs more memory than is allowed.
folders-archive-why-memory = sa lecture demande plus de mémoire que Folders n'en permet

# Reason given for an archive that cannot be read when no better reason is known.
folders-archive-why-other = un problème inattendu s'est produit

# Reason a file inside an archive cannot be opened: it is too big.
folders-archive-inside-too-big = il est trop gros pour être ouvert depuis l'archive

# Reason a file cannot be taken from an archive: it is not in it.
folders-archive-inside-missing = il n'est pas dans l'archive

# Reason a 7z archive would not unpack: it needs more memory than is allowed.
folders-archive-7z-memory = cette archive 7z demande plus de mémoire pour être extraite que Folders n'en autorise

# Reason making an archive failed: a file grew while it was being packed.
folders-archive-pack-grew = un fichier est devenu trop gros pendant que Folders le compressait

# Reason making an archive failed: something with the archive's name is already there.
folders-archive-pack-exists = il existe déjà quelque chose portant ce nom

# Reason making an archive failed: an item has no name to put in it.
folders-archive-pack-no-name = un élément n'a pas de nom que Folders peut mettre dans l'archive

# Reason making an archive failed: Folders reads this kind but cannot write it.
folders-archive-pack-cannot-make = Folders peut ouvrir ce type d'archive mais ne peut pas en créer

# Reason a 7z archive cannot be read; the cause follows, already written out.
folders-archive-damaged-7z = l'archive 7z est endommagée : { $why }

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock too slow to open.
folders-archive-7z-a-lock-too-slow-to-open = son déverrouillage prendrait beaucoup trop de temps

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock with no settings.
folders-archive-7z-a-lock-with-no-settings = les réglages de sa protection par mot de passe sont absents

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock with short settings.
folders-archive-7z-a-lock-with-short-settings = les réglages de sa protection par mot de passe sont incomplets

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a lock with no key.
folders-archive-7z-a-lock-with-no-key = sa protection par mot de passe n'a pas de clé utilisable

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a locked block is not whole.
folders-archive-7z-a-locked-block-is-not-whole = une partie verrouillée est incomplète

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header ends too soon.
folders-archive-7z-the-header-ends-too-soon = sa liste de contenu se termine trop tôt

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a count is far too big.
folders-archive-7z-a-count-is-far-too-big = un nombre qu'elle contient est beaucoup trop grand

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block has no packed bytes.
folders-archive-7z-a-block-has-no-packed-bytes = une partie n'a aucune donnée compressée

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block is out of reach.
folders-archive-7z-a-block-is-out-of-reach = une partie pointe au-delà de la fin du fichier

# Cause given after "damaged 7z:" when a 7z archive cannot be read: too short.
folders-archive-7z-too-short = le fichier est trop court

# Cause given after "damaged 7z:" when a 7z archive cannot be read: no 7z mark.
folders-archive-7z-no-7z-mark = il ne commence pas comme un fichier 7z

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the start header's check does not match.
folders-archive-7z-the-start-header-s-check-does-not-match = le début du fichier échoue à sa vérification

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header is out of reach.
folders-archive-7z-the-header-is-out-of-reach = sa liste de contenu pointe au-delà de la fin du fichier

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header's check does not match.
folders-archive-7z-the-header-s-check-does-not-match = sa liste de contenu échoue à sa vérification

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a packed header with no block.
folders-archive-7z-a-packed-header-with-no-block = sa liste de contenu compressée ne contient aucune donnée

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the header is far too big.
folders-archive-7z-the-header-is-far-too-big = sa liste de contenu est beaucoup trop grande

# Cause given after "damaged 7z:" when a 7z archive cannot be read: the packed header's check does not match.
folders-archive-7z-the-packed-header-s-check-does-not-match = sa liste de contenu compressée échoue à sa vérification

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown header.
folders-archive-7z-an-unknown-header = sa liste de contenu est dans un format que Folders ne reconnaît pas

# Cause given after "damaged 7z:" when a 7z archive cannot be read: headers packed too many times over.
folders-archive-7z-headers-packed-too-many-times-over = sa liste de contenu est compressée trop de fois

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the header.
folders-archive-7z-an-unknown-part-in-the-header = il y a quelque chose d'inconnu dans sa liste de contenu

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the streams.
folders-archive-7z-an-unknown-part-in-the-streams = il y a quelque chose d'inconnu dans ses données

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the packed streams.
folders-archive-7z-an-unknown-part-in-the-packed-streams = il y a quelque chose d'inconnu dans ses données compressées

# Cause given after "damaged 7z:" when a 7z archive cannot be read: packed streams with no sizes.
folders-archive-7z-packed-streams-with-no-sizes = les tailles de ses données compressées sont absentes

# Cause given after "damaged 7z:" when a 7z archive cannot be read: no list of blocks.
folders-archive-7z-no-list-of-blocks = la liste de ses parties est absente

# Cause given after "damaged 7z:" when a 7z archive cannot be read: blocks kept elsewhere.
folders-archive-7z-blocks-kept-elsewhere = la liste de ses parties est stockée à un endroit que Folders ne peut pas lire

# Cause given after "damaged 7z:" when a 7z archive cannot be read: blocks with no sizes.
folders-archive-7z-blocks-with-no-sizes = les tailles de ses parties sont absentes

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the blocks.
folders-archive-7z-an-unknown-part-in-the-blocks = il y a quelque chose d'inconnu dans la liste de ses parties

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block with no sensible coders.
folders-archive-7z-a-block-with-no-sensible-coders = une de ses parties n'a pas d'étapes de compression valables

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a coder with too many ends.
folders-archive-7z-a-coder-with-too-many-ends = une de ses étapes de compression a trop d'entrées ou de sorties

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block's coders do not join up.
folders-archive-7z-a-block-s-coders-do-not-join-up = les étapes de compression d'une de ses parties ne s'enchaînent pas

# Cause given after "damaged 7z:" when a 7z archive cannot be read: far too many files.
folders-archive-7z-far-too-many-files = elle liste beaucoup trop de fichiers

# Cause given after "damaged 7z:" when a 7z archive cannot be read: files in a block with no sizes.
folders-archive-7z-files-in-a-block-with-no-sizes = certains fichiers d'une de ses parties n'ont pas de taille enregistrée

# Cause given after "damaged 7z:" when a 7z archive cannot be read: files bigger than their block.
folders-archive-7z-files-bigger-than-their-block = certains fichiers sont plus gros que la partie qui les contient

# Cause given after "damaged 7z:" when a 7z archive cannot be read: an unknown part in the files' sizes.
folders-archive-7z-an-unknown-part-in-the-files-sizes = il y a quelque chose d'inconnu dans les tailles de ses fichiers

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a part is far too big.
folders-archive-7z-a-part-is-far-too-big = une de ses parties est beaucoup trop grande

# Cause given after "damaged 7z:" when a 7z archive cannot be read: names kept elsewhere.
folders-archive-7z-names-kept-elsewhere = les noms de ses fichiers sont stockés à un endroit que Folders ne peut pas lire

# Cause given after "damaged 7z:" when a 7z archive cannot be read: too few names.
folders-archive-7z-too-few-names = certains noms de fichiers sont absents

# Cause given after "damaged 7z:" when a 7z archive cannot be read: times kept elsewhere.
folders-archive-7z-times-kept-elsewhere = les dates de ses fichiers sont stockées à un endroit que Folders ne peut pas lire

# Cause given after "damaged 7z:" when a 7z archive cannot be read: attributes kept elsewhere.
folders-archive-7z-attributes-kept-elsewhere = les détails de ses fichiers sont stockés à un endroit que Folders ne peut pas lire

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block runs past the end.
folders-archive-7z-a-block-runs-past-the-end = une de ses parties dépasse la fin du fichier

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block's coders go round in a loop.
folders-archive-7z-a-block-s-coders-go-round-in-a-loop = les étapes de compression d'une de ses parties tournent en boucle

# Cause given after "damaged 7z:" when a 7z archive cannot be read: LZMA with no settings.
folders-archive-7z-lzma-with-no-settings = sa compression LZMA n'a pas de réglages

# Cause given after "damaged 7z:" when a 7z archive cannot be read: LZMA2 with no settings.
folders-archive-7z-lzma2-with-no-settings = sa compression LZMA2 n'a pas de réglages

# Cause given after "damaged 7z:" when a 7z archive cannot be read: LZMA2 with impossible settings.
folders-archive-7z-lzma2-with-impossible-settings = sa compression LZMA2 a des réglages non valides

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block would not open.
folders-archive-7z-a-block-would-not-open = Folders n'a pas pu ouvrir une de ses parties

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a block ends too soon.
folders-archive-7z-a-block-ends-too-soon = une de ses parties se termine trop tôt

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a file ends too soon.
folders-archive-7z-a-file-ends-too-soon = un fichier qu'elle contient se termine trop tôt

# Cause given after "damaged 7z:" when a 7z archive cannot be read: a file's check does not match.
folders-archive-7z-a-file-s-check-does-not-match = un fichier qu'elle contient échoue à sa vérification

# Cause given after "damaged 7z:" when a 7z archive cannot be read: files' sizes and checks do not match up.
folders-archive-7z-files-sizes-and-checks-do-not-match-up = les tailles et les vérifications de ses fichiers ne concordent pas

# Cause given after "damaged 7z:" when a 7z archive cannot be read: more files than blocks hold.
folders-archive-7z-more-files-than-blocks-hold = elle liste plus de fichiers que ses parties n'en contiennent

# Cause given after "damaged 7z:" when a 7z archive cannot be read: more files than sizes.
folders-archive-7z-more-files-than-sizes = certains de ses fichiers n'ont pas de taille enregistrée

# Reason a zip archive cannot be read; the cause follows, already written out.
folders-archive-damaged-zip = l'archive zip est endommagée : { $why }

# Cause given after "damaged zip:" when a zip archive cannot be read: no end record.
folders-archive-zip-no-end-record = l'enregistrement à sa fin est absent

# Cause given after "damaged zip:" when a zip archive cannot be read: no ZIP64 end record.
folders-archive-zip-no-zip64-end-record = l'enregistrement ZIP64 à sa fin est absent

# Cause given after "damaged zip:" when a zip archive cannot be read: directory out of place.
folders-archive-zip-directory-out-of-place = sa liste de contenu est au mauvais endroit

# Cause given after "damaged zip:" when a zip archive cannot be read: directory too big.
folders-archive-zip-directory-too-big = sa liste de contenu est trop grande

# Cause given after "damaged zip:" when a zip archive cannot be read: record runs past the directory.
folders-archive-zip-record-runs-past-the-directory = une entrée dépasse la fin de sa liste de contenu

# Cause given after "damaged zip:" when a zip archive cannot be read: no local header.
folders-archive-zip-no-local-header = les détails placés avant un de ses fichiers sont absents

# Cause given after "damaged zip:" when a zip archive cannot be read: entry runs past the end.
folders-archive-zip-entry-runs-past-the-end = un fichier qu'elle contient dépasse la fin de l'archive

# Cause given after "damaged zip:" when a zip archive cannot be read: an entry's check does not match.
folders-archive-zip-an-entry-s-check-does-not-match = un de ses fichiers échoue à sa vérification

# Cause given after "damaged zip:" when a zip archive cannot be read: an entry is longer than it says.
folders-archive-zip-an-entry-is-longer-than-it-says = un fichier qu'elle contient est plus long que sa taille enregistrée

# Cause given after "damaged zip:" when a zip archive cannot be read: a locked entry is cut short.
folders-archive-zip-a-locked-entry-is-cut-short = un fichier protégé par mot de passe qu'elle contient est incomplet

# Cause given after "damaged zip:" when a zip archive cannot be read: a lock with a key of no known length.
folders-archive-zip-a-lock-with-a-key-of-no-known-length = sa protection par mot de passe utilise une longueur de clé que Folders ne reconnaît pas

# Cause given after "damaged zip:" when a zip archive cannot be read: a locked entry's check does not match.
folders-archive-zip-a-locked-entry-s-check-does-not-match = un fichier protégé par mot de passe qu'elle contient échoue à sa vérification

