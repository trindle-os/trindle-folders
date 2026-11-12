# machine translated (Swahili), not yet reviewed by a person.

# Reason beside a dropped thing in the question card: it is hidden or inside a hidden folder.
folders-outside-wary-hidden = imefichwa, au iko kwenye folda iliyofichwa

# Reason beside a dropped folder in the question card: it holds hidden things.
folders-outside-wary-holds = ina vitu vilivyofichwa ndani yake

# Reason beside a dropped folder in the question card: it holds too much to look through beforehand.
folders-outside-wary-deep = ina vitu vingi mno kuvikagua, kwa hiyo Folders itaacha vitu vyake vilivyofichwa

# Reason beside a dropped thing in the question card: it is outside the person's home folder and drives.
folders-outside-wary-system = iko nje ya folda yako ya nyumbani na diski zako

# Reason beside a dropped link in the question card: it leads to something hidden or outside the home folder and drives.
folders-outside-wary-leads = kiungo cha kitu kilichofichwa, au kilicho nje ya folda yako ya nyumbani na diski zako

# Reason beside a dropped folder in the question card: it holds a link to something hidden or outside the home folder and drives.
folders-outside-wary-holds-link = ina kiungo ndani yake cha kitu kilichofichwa, au kilicho nje ya folda yako ya nyumbani na diski zako

# Heading of the card that asks whether to move what another program dropped; one name or a count of items.
folders-outside-title-move =
    { $n ->
        [one] Kuhamishia { $name } hapa?
       *[other] Kuhamishia vitu hivi { $n } hapa?
    }

# Heading of the card that asks whether to bring what another program dropped (copy or move as the program asks); one name or a count of items.
folders-outside-title-bring =
    { $n ->
        [one] Kuleta { $name } hapa?
       *[other] Kuleta vitu hivi { $n } hapa?
    }

# Heading of the card that asks whether to copy what another program dropped; one name or a count of items.
folders-outside-title-copy =
    { $n ->
        [one] Kunakili { $name } hapa?
       *[other] Kunakili vitu hivi { $n } hapa?
    }

# Words on the question card when the drop would be moved: why it is asked.
folders-outside-body-move =
    { $n ->
        [one] Programu nyingine imekiangusha kwenye Folders. Programu yoyote inaweza kuangusha faili yoyote, na hiki si mojawapo ya faili zako za kawaida. Folders haitakihamisha hadi useme.
       *[other] Programu nyingine imeviangusha kwenye Folders. Programu yoyote inaweza kuangusha faili yoyote, na baadhi ya hivi si faili zako za kawaida. Folders haitavihamisha hadi useme.
    }

# Words on the question card when the drop would be brought: why it is asked.
folders-outside-body-bring =
    { $n ->
        [one] Programu nyingine imekiangusha kwenye Folders. Programu yoyote inaweza kuangusha faili yoyote, na hiki si mojawapo ya faili zako za kawaida. Folders haitakileta hadi useme.
       *[other] Programu nyingine imeviangusha kwenye Folders. Programu yoyote inaweza kuangusha faili yoyote, na baadhi ya hivi si faili zako za kawaida. Folders haitavileta hadi useme.
    }

# Words on the question card when the drop would be copied: why it is asked.
folders-outside-body-copy =
    { $n ->
        [one] Programu nyingine imekiangusha kwenye Folders. Programu yoyote inaweza kuangusha faili yoyote, na hiki si mojawapo ya faili zako za kawaida. Folders haitakinakili hadi useme.
       *[other] Programu nyingine imeviangusha kwenye Folders. Programu yoyote inaweza kuangusha faili yoyote, na baadhi ya hivi si faili zako za kawaida. Folders haitavinakili hadi useme.
    }

# Tip of the round close button, and name of the main button, on the question card: leave what was dropped alone.
folders-outside-leave =
    { $n ->
        [one] Kiache
       *[other] Viache
    }

# Button on the question card that moves what was dropped.
folders-outside-do-move =
    { $n ->
        [one] Kihamishe
       *[other] Vihamishe
    }

# Button on the question card that brings what was dropped.
folders-outside-do-bring =
    { $n ->
        [one] Kilete
       *[other] Vilete
    }

# Button on the question card that copies what was dropped.
folders-outside-do-copy =
    { $n ->
        [one] Kinakili
       *[other] Vinakili
    }

# Line on the question card for a thing with a reason it is asked about: its name, the folder it came from, the reason.
folders-outside-named-why = { $name }, kutoka { $from }: { $why }

# Line on the question card for a thing with no reason of its own: its name and the folder it came from.
folders-outside-named = { $name }, kutoka { $from }

# Line on the question card saying how many more things there are than are listed.
folders-outside-more = Na { $n } zaidi.

# Line on the question card saying which folder or place the things would go into.
folders-outside-into = Ndani ya { $into }.

# Red words when something is dropped while the question card is still up.
folders-outside-answer-first = Jibu swali kuhusu ulichoangusha kwanza.

# Red words when something is dropped while the last drop is still being looked at.
folders-outside-still-looking = Folders bado inakagua kitu ulichoangusha mwisho.

# Red words when text, not files, is dropped on Folders.
folders-outside-only-files = Faili na folda pekee ndizo zinazoweza kuangushwa hapa.

# Red words when something other than folders is dropped between pinned folders in the sidebar.
folders-outside-only-folders-pin = Folda pekee ndizo zinazoweza kubandikwa kwenye utepe wa pembeni.

# Red words when something is dropped where it cannot go, such as on a list that is not a folder.
folders-outside-only-on-folder = Vitu vinaweza kuangushwa kwenye folda tu.

# Red words when a dropped thing is no longer there; its name.
folders-outside-gone = { $name } haipo tena, kwa hiyo hakuna kilichonakiliwa au kuhamishwa.

# Red words when a folder is dropped into itself.
folders-outside-inside-itself = Huwezi kuweka folda ndani yake yenyewe.

# Words when things are dragged into the folder they are already in.
folders-outside-already-there = Tayari viko kwenye folda hiyo.

# Red words when the place a drop is in does not answer in time; the name of the dropped thing.
folders-outside-not-answering = Folders haikuweza kufikia diski ambayo { $name } iko kwa wakati, kwa hiyo hakuna kilichonakiliwa au kuhamishwa.

# Words when the question card is answered with no.
folders-outside-nothing-brought = Hakuna kilichonakiliwa au kuhamishwa.

# Name on the picture dragged out of Folders when several items are dragged to another program.
folders-outside-items-dragged =
    { $n ->
        [one] Kitu 1
       *[other] Vitu { $n }
    }

# Line for the round i beside the red words: how many more files there are than are listed; the count written out.
folders-outside-lines-more = na { $n } zaidi

# Red words when one dragged thing stayed because of a reason: its name, then the reason.
folders-outside-stayed-one = { $name } { $why }.

# Red words when several dragged things changed after they were picked up, so they stayed; the count and their names.
folders-outside-stayed-changed =
    { $n ->
        [one] Kitu { $n } kilibadilika baada ya kuanza kukiburuta, kwa hiyo Folders ilikiacha mahali kilipokuwa: { $names }.
       *[other] Vitu { $n } vilibadilika baada ya kuanza kuviburuta, kwa hiyo Folders iliviacha mahali vilipokuwa: { $names }.
    }

# Red words when several dragged things stayed for different reasons; the count and each name with its reason.
folders-outside-stayed-many =
    { $n ->
        [one] Kitu { $n } bado kiko mahali pake: { $each }.
       *[other] Vitu { $n } bado viko mahali pake: { $each }.
    }

# One dragged thing that stayed, in a list: its name and the reason in brackets.
folders-outside-stayed-each = { $name } ({ $why })

# Red words with two sentences: why some things stayed, then the words about what was not moved.
folders-outside-both = { $first } { $second }

# Red words when the other program took a move in a way that cannot show it read all of it.
folders-outside-unshown = Folders haiwezi kujua kama programu nyingine ilipata kila kitu ulichoburuta, kwa hiyo iliacha vya asili hapa. Programu nyingine ina nakala.

# Red words when the other program did not take all of a drag that was a move.
folders-outside-short-move = Programu nyingine haikuchukua kila kitu ulichoburuta, kwa hiyo Folders haikuhamisha chochote.

# Red words when the other program did not take all of a drag that was a copy.
folders-outside-short-copy = Programu nyingine haikuchukua kila kitu ulichoburuta, kwa hiyo nakala yake huenda haijakamilika.

