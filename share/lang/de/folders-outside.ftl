
# machine translated (German), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = versteckt oder in einem versteckten Ordner

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = enthält versteckte Dinge

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = enthält zu viele Objekte zum Prüfen, deshalb lässt Folders die versteckten Objekte darin weg

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = liegt außerhalb deines Home-Ordners und deiner Laufwerke

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = ein Link auf etwas Verstecktes oder außerhalb deines Home-Ordners und deiner Laufwerke

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = enthält einen Link auf etwas Verstecktes oder außerhalb deines Home-Ordners und deiner Laufwerke

# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [one] { $name } hierher verschieben?
       *[other] Diese { $n } Objekte hierher verschieben?
    }

# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [one] { $name } hierher holen?
       *[other] Diese { $n } Objekte hierher holen?
    }

# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [one] { $name } hierher kopieren?
       *[other] Diese { $n } Objekte hierher kopieren?
    }

# Words on the question card when the drop would be moved: why it is asked.
folders-outside-body-move =
    { $n ->
        [one] Ein anderes Programm hat dies auf Folders abgelegt. Jedes Programm kann jede Datei ablegen, und dies gehört nicht zu deinen gewöhnlichen Dateien. Folders verschiebt es erst, wenn du es sagst.
       *[other] Ein anderes Programm hat diese auf Folders abgelegt. Jedes Programm kann jede Datei ablegen, und manche davon gehören nicht zu deinen gewöhnlichen Dateien. Folders verschiebt sie erst, wenn du es sagst.
    }

# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
        [one] Ein anderes Programm hat dies auf Folders abgelegt. Jedes Programm kann jede Datei ablegen, und dies gehört nicht zu deinen gewöhnlichen Dateien. Folders holt es erst, wenn du es sagst.
       *[other] Ein anderes Programm hat diese auf Folders abgelegt. Jedes Programm kann jede Datei ablegen, und manche davon gehören nicht zu deinen gewöhnlichen Dateien. Folders holt sie erst, wenn du es sagst.
    }

# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
        [one] Ein anderes Programm hat dies auf Folders abgelegt. Jedes Programm kann jede Datei ablegen, und dies gehört nicht zu deinen gewöhnlichen Dateien. Folders kopiert es erst, wenn du es sagst.
       *[other] Ein anderes Programm hat diese auf Folders abgelegt. Jedes Programm kann jede Datei ablegen, und manche davon gehören nicht zu deinen gewöhnlichen Dateien. Folders kopiert sie erst, wenn du es sagst.
    }

# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
        [one] Lassen
       *[other] Lassen
    }

# Button on the question card that moves what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-move =
    { $n ->
        [one] Verschieben
       *[other] Verschieben
    }

# Button on the question card that brings what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-bring =
    { $n ->
        [one] Holen
       *[other] Holen
    }

# Button on the question card that copies what was dropped.
# SAFETY: needs a human review before it ships
folders-outside-do-copy =
    { $n ->
        [one] Kopieren
       *[other] Kopieren
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, aus { $from }: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, aus { $from }

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = Und { $n } weitere.

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = In { $into }.

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Beantworte zuerst die Frage zu dem, was du abgelegt hast.

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders prüft noch das Letzte, was du abgelegt hast.

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Hier können nur Dateien und Ordner abgelegt werden.

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = Nur Ordner können an die Seitenleiste angeheftet werden.

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = Du kannst Dinge nur auf einem Ordner ablegen.

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } ist nicht mehr da, deshalb wurde nichts kopiert oder verschoben.

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = Du kannst einen Ordner nicht in sich selbst legen.

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = Sie sind schon in diesem Ordner.

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders konnte das Laufwerk, auf dem { $name } liegt, nicht rechtzeitig erreichen, deshalb wurde nichts kopiert oder verschoben.

# Words when the question card is answered with no.
folders-outside-nothing-brought = Es wurde nichts kopiert oder verschoben.

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] 1 Objekt
       *[other] { $n } Objekte
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = und { $n } weitere

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] { $n } Objekt hat sich geändert, nachdem du begonnen hast, es zu ziehen, deshalb hat Folders es dort gelassen, wo es war: { $names }.
       *[other] { $n } Objekte haben sich geändert, nachdem du begonnen hast, sie zu ziehen, deshalb hat Folders sie dort gelassen, wo sie waren: { $names }.
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] { $n } Objekt liegt noch dort, wo es war: { $each }.
       *[other] { $n } Objekte liegen noch dort, wo sie waren: { $each }.
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders kann nicht erkennen, ob das andere Programm alles bekommen hat, was du gezogen hast, deshalb hat es die Originale hier gelassen. Das andere Programm hat eine Kopie.

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = Das andere Programm hat nicht alles übernommen, was du gezogen hast, deshalb hat Folders nichts verschoben.

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = Das andere Programm hat nicht alles übernommen, was du gezogen hast, deshalb ist seine Kopie vielleicht nicht vollständig.

