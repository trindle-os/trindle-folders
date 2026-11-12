
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Said under the pointer when the person tries to change something inside an earlier copy of a folder.
folders-earlier-shut = Você não pode alterar nada em uma cópia anterior. Copie itens dela para mantê-los.

# When an earlier copy was made, inside a sentence. The day is today, yesterday or other; the date is written out for other days.
folders-earlier-day-words =
    { $day ->
        [today] hoje às { $time }
        [yesterday] ontem às { $time }
       *[other] { $date } às { $time }
    }

# First line of a version's row on the earlier versions card: when the copy was made, starting a line with a capital.
folders-earlier-day-line =
    { $day ->
        [today] Hoje às { $time }
        [yesterday] Ontem às { $time }
       *[other] { $date } às { $time }
    }

# Line over the list of an earlier copy, saying what it is and when it was made.
folders-earlier-note-dated = Esta é uma cópia anterior de { $when }. Você não pode alterá-la, mas pode copiar itens dela.

# Line over the list of an earlier copy when its time is not known.
folders-earlier-note = Esta é uma cópia anterior. Você não pode alterá-la, mas pode copiar itens dela.

# Name on the tab of a folder shown as it was in an earlier copy.
folders-earlier-tab-name = Anterior: { $name }

# Name of the Undo step after an earlier version is put back.
folders-earlier-undo-put-back = Desfazer a recolocação de { $name }

# Said under a version's row after it is put back and the file that was there is kept beside it.
folders-earlier-put-back-kept = O Folders recolocou. O que estava lá está ao lado, com outro nome.

# Said under a version's row after it is put back where it was.
folders-earlier-put-back = O Folders recolocou onde estava.

# Said under a version's row after comparing it with the one there now, when they hold the same.
folders-earlier-same = É igual ao que está lá agora.

# Said under a version's row after comparing it with the one there now, when they differ.
folders-earlier-differs = É diferente do que está lá agora.

# Red words under a version's row when the two could not be compared; the reason follows.
folders-earlier-compare-failed = O Folders não conseguiu compará-los: { $why }

# Menu row on a file's menu that shows its earlier versions.
folders-earlier-menu-versions = Versões anteriores

# Menu row on the folder's menu that shows what was deleted from the folder.
folders-earlier-menu-deleted = Coisas excluídas daqui

# Title of the card that lists the earlier versions of one file or folder.
folders-earlier-versions-title = Versões anteriores de { $name }

# Empty-state line on the earlier versions card when no copy differs from the current one.
folders-earlier-versions-none = Não há versões anteriores diferentes da atual.

# Note at the foot of the earlier versions card about what putting one back does.
folders-earlier-versions-foot = Recolocar uma versão mantém a atual ao lado dela. Você pode desfazer isso.

# Title of the card that lists what was deleted from a folder.
folders-earlier-deleted-title = Coisas excluídas de { $name }

# Empty-state line on the deleted things card when nothing is missing from the folder.
folders-earlier-deleted-none = Nada das cópias anteriores foi excluído desta pasta.

# Note at the foot of the deleted things card about what putting one back does.
folders-earlier-deleted-foot = Recolocar um item o devolve a esta pasta. Você pode desfazer isso.

# Second line of a row for a folder on the earlier versions card.
folders-earlier-folder-then = A pasta como era naquele momento

# Second line of a row for a file version: its size and the date of the copy it comes from.
folders-earlier-version-line = { $size }, da cópia de { $date }

# Second line of a row for a deleted folder: when it was last kept.
folders-earlier-deleted-folder-line = Uma pasta, vista pela última vez { $when }

# Second line of a row for a deleted file: its size and when it was last kept.
folders-earlier-deleted-line = { $size }, visto pela última vez { $when }

# Second line of a version's row while it is being put back or compared.
folders-earlier-working = Trabalhando…

# Tip and name of the round button on an earlier folder's row that opens that day's folder in a new tab.
folders-earlier-open-folder-tip = Abrir a pasta daquele dia em uma nova aba

# Tip and name of the round button on an earlier file's row that opens it to look at.
folders-earlier-open-tip = Abrir

# Tip and name of the round button on a row of the earlier versions card that puts it back.
folders-earlier-put-back-tip = Recolocar

# Tip and name of the round button on an earlier version's row that compares it with the one there now.
folders-earlier-compare-tip = Comparar com o atual

# Red words when what is given to put back is not one of the earlier versions.
folders-earlier-put-back-not-version = Essa não é uma versão anterior.

# Red words when an earlier version is asked to be put back outside the home folder.
folders-earlier-put-back-home-only = Você só pode recolocar uma versão anterior dentro da sua pasta pessoal.

# Red words when the place to put an earlier version back has no usable name.
folders-earlier-put-back-no-place = O Folders não pode recolocar um arquivo lá.

# Red words when the folder an earlier version was in cannot be made again; why it failed.
folders-earlier-put-back-folder-lost = O Folders não conseguiu recriar a pasta em que ele estava: { $why }

# Red words when copying an earlier version back fails; why it failed.
folders-earlier-put-back-copy-failed = O Folders não conseguiu copiar a versão anterior: { $why }

# Red words when what is there now cannot be set aside before putting an earlier version back; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-aside-failed = O Folders não conseguiu tirar o atual do caminho: { $why }

# Red words when the earlier version could not be put in place and what was there has been left under another name; why it failed, then that name.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place-named = O Folders não conseguiu recolocar a versão anterior: { $why }. O atual agora se chama { $name }.

# Red words when the earlier version could not be put in place; why it failed.
# SAFETY: needs a human review before it ships
folders-earlier-put-back-in-place = O Folders não conseguiu recolocar a versão anterior: { $why }
