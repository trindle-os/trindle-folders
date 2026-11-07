# machine translated (Swahili), not yet reviewed by a person.

folders-progress-pace-known =
    { $files ->
        [yes] faili { $done } kati ya { $of }, { $rate } kwa sekunde, { $left }
       *[no] { $rate } kwa sekunde, { $left }
    }

folders-progress-pace-wait =
    { $files ->
        [yes] faili { $done } kati ya { $of }, inakadiria itachukua muda gani…
       *[no] inakadiria itachukua muda gani…
    }

folders-progress-line = { $doing }: { $how }

folders-progress-line-heavy = { $line } { $heavy }

folders-progress-speed-bytes = baiti { $n }

folders-progress-speed-kb = KB { $n }

folders-progress-speed-mb = MB { $n }

folders-progress-speed-gb = GB { $n }

folders-progress-left-few = zimebaki sekunde chache

folders-progress-left-seconds =
    { $n ->
        [one] imebaki takriban sekunde 1
       *[other] zimebaki takriban sekunde { $n }
    }

folders-progress-left-minute = imebaki takriban dakika moja

folders-progress-left-minutes =
    { $n ->
        [one] imebaki takriban dakika 1
       *[other] zimebaki takriban dakika { $n }
    }

folders-progress-left-hours =
    { $n ->
        [one] imebaki takriban saa 1
       *[other] zimebaki takriban saa { $n }
    }

folders-progress-more =
    { $n ->
        [one] Kazi nyingine 1 inaendelea
       *[other] Kazi nyingine { $n } zinaendelea
    }

folders-progress-heavy =
    { $verb ->
        [moving] Kuhamisha kunatumia diski sana, kwa hiyo programu nyingine zinaweza kuwa polepole hadi kuishe.
       *[copying] Kunakili kunatumia diski sana, kwa hiyo programu nyingine zinaweza kuwa polepole hadi kuishe.
    }

folders-progress-waiting = inasubiri jibu

folders-progress-paused = imesimamishwa kwa muda

folders-progress-carry-on = Endelea

folders-progress-pause = Simamisha kwa muda

folders-progress-stop = Acha

folders-progress-mark-newer-larger = mpya zaidi, kubwa zaidi

folders-progress-mark-newer = mpya zaidi

folders-progress-mark-larger = kubwa zaidi

folders-progress-changed = ilibadilishwa { $when }

folders-progress-taken =
    { $kind ->
        [folder] Folda iitwayo { $name } ipo tayari
        [file] Faili iitwayo { $name } ipo tayari
       *[unknown] { $name } ipo tayari
    }

folders-progress-coming = Inayoletwa

folders-progress-there = Iliyopo tayari

folders-progress-skip = Ruka

folders-progress-keep-both = Weka zote mbili

folders-progress-replace = Badilisha

folders-progress-replace-folder-spoken = Badilisha: folda iliyopo tayari huenda Tupioni

folders-progress-merge = Unganisha

folders-progress-all-folders = Jibu lilelile kwa kila folda iliyopo tayari.

folders-progress-all-files = Jibu lilelile kwa kila faili iliyopo tayari.

folders-progress-same-for-rest = Fanya vivyo hivyo kwa zilizobaki
