# machine translated (French), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = Vous ne pouvez rien modifier dans une copie antérieure. Copiez-en les éléments pour les conserver.

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] aujourd'hui à { $time }
        [yesterday] hier à { $time }
       *[other] le { $date } à { $time }
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] Aujourd'hui à { $time }
        [yesterday] Hier à { $time }
       *[other] Le { $date } à { $time }
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = Voici une copie antérieure, telle qu'elle était { $when }. Vous ne pouvez pas la modifier, mais vous pouvez en copier des éléments.

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = Voici une copie antérieure. Vous ne pouvez pas la modifier, mais vous pouvez en copier des éléments.

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = Antérieur : { $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = Annuler la restauration de { $name }

# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = Folders l'a remis en place. La version actuelle est à côté, sous un nouveau nom.

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = Folders l'a remis à sa place.

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = Son contenu est identique à celui du fichier actuel.

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = Il diffère du fichier actuel.

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = Folders n'a pas pu les comparer : { $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = Versions antérieures

# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = Éléments supprimés d'ici

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = Versions antérieures de { $name }

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = Aucune copie antérieure ne diffère de la version actuelle.

# Note at the foot of the earlier versions card about what putting one back does.
# SAFETY: needs a human review before it ships
folders-earlier-versions-foot = Si vous en remettez une en place, la version actuelle est conservée à côté. Vous pouvez annuler cette action.

# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = Éléments supprimés de { $name }

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = Aucun élément des copies antérieures n'a été supprimé de ce dossier.

# Note at the foot of the deleted things card about what putting one back does.
# SAFETY: needs a human review before it ships
folders-earlier-deleted-foot = Si vous en remettez un en place, il revient dans ce dossier. Vous pouvez annuler cette action.

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = Le dossier tel qu'il était alors

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }, de la copie du { $date }

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = Un dossier, vu pour la dernière fois { $when }

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }, vu pour la dernière fois { $when }

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = En cours…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = Ouvrir le dossier de ce jour dans un nouvel onglet

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = Ouvrir

# Tip and name of the round button on a row of the earlier versions card that puts it back.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-tip = Restaurer

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = Comparer avec la version actuelle

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = Ce n'est pas une version antérieure.

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = Vous ne pouvez remettre une version antérieure qu'à l'intérieur de votre dossier personnel.

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = Folders ne peut pas remettre un fichier à cet endroit.

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = Folders n'a pas pu recréer le dossier où il se trouvait : { $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = Folders n'a pas pu copier la version antérieure : { $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = Folders n'a pas pu déplacer la version actuelle pour libérer la place : { $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = Folders n'a pas pu remettre la version antérieure en place : { $why }. La version actuelle s'appelle maintenant { $name }.

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = Folders n'a pas pu remettre la version antérieure en place : { $why }
