

folders-sharing-cannot-read = Dieser Ordner kann nicht gelesen werden.

folders-sharing-only-folder = Nur ein Ordner selbst kann freigegeben werden, kein Link auf einen Ordner.

folders-sharing-only-yours = Nur Ordner, die dir gehören, können freigegeben werden. Dieser gehört jemand anderem.

folders-sharing-your-home = dein Home-Ordner

folders-sharing-this-folder = dieser Ordner

folders-sharing-and = { $first } und { $second }

folders-sharing-share = Diesen Ordner freigeben

folders-sharing-shared = Im Heimnetzwerk freigegeben.

folders-sharing-not-shared = Nicht freigegeben.

folders-sharing-share-tip = Diesen Ordner für andere Computer im Heimnetzwerk freigeben

folders-sharing-name = Name

folders-sharing-name-tip = Der Name, den andere Computer sehen

folders-sharing-write = Andere können Dateien ändern

folders-sharing-write-sub = Sie dürfen Dateien darin hinzufügen, ändern und löschen.

folders-sharing-guests = Jeden hereinlassen

folders-sharing-guests-sub = Es wird kein Passwort verlangt.

folders-sharing-guests-tip = Jeden ohne Passwort hereinlassen

folders-sharing-explain =
    { $fetch ->
        [yes] Freigegeben erscheint er unter Netzwerk auf den anderen Computern zu Hause, auch auf Windows-Computern. Beim ersten Mal holt Trindle Samba, das Programm, das Ordner freigibt, was eine Minute dauert.
       *[no] Freigegeben erscheint er unter Netzwerk auf den anderen Computern zu Hause, auch auf Windows-Computern.
    }

folders-sharing-find-it =
    { $host_known ->
        [yes] Andere Computer finden ihn unter Netzwerk als { $name } auf { $host }.
       *[no] Andere Computer finden ihn unter Netzwerk als { $name }.
    }

folders-sharing-cannot-reach = Andere Computer können ihn noch nicht erreichen: Andere Computer verbinden lassen ist in Einstellungen, Netzwerk ausgeschaltet.

folders-sharing-let-connect = Andere Computer verbinden lassen

folders-sharing-visitors =
    { $n ->
        [one] Besucher ohne Passwort können ihn noch nicht öffnen: { $names } ist für andere Konten gesperrt. Beim Durchlassen wird nur der Weg zu diesem Ordner geöffnet; sie können nicht sehen, was sonst dort liegt.
       *[other] Besucher ohne Passwort können ihn noch nicht öffnen: { $names } sind für andere Konten gesperrt. Beim Durchlassen wird nur der Weg zu diesem Ordner geöffnet; sie können nicht sehen, was sonst dort liegt.
    }

folders-sharing-let-through = Besucher zu diesem Ordner durchlassen

folders-sharing-sign-in-set = Andere melden sich als { $user } an, mit dem für die Freigabe festgelegten Passwort.

folders-sharing-change-password = Das Passwort ändern, das andere Computer verwenden

folders-sharing-set-password = Andere melden sich als { $user } an. Lege das Passwort fest, mit dem andere Computer deine freigegebenen Ordner öffnen, mindestens { $n } Zeichen. Es ist von deinem Anmeldepasswort getrennt.

folders-sharing-password = Passwort

folders-sharing-password-tip = Freigabepasswort

folders-sharing-keep-password = Dieses Passwort behalten

folders-sharing-card-title = { $name } im Netzwerk freigeben

folders-sharing-name-taken = Ein anderer deiner freigegebenen Ordner hat diesen Namen schon. Wähle einen anderen.

folders-sharing-stopping = Die Freigabe wird beendet…

folders-sharing-password-short = Das Passwort braucht mindestens { $n } Zeichen.

folders-sharing-keeping-password = Das Passwort wird gespeichert; zuerst wird nach dem Passwort des Administrators gefragt.

folders-sharing-letting-connect = Andere Computer dürfen sich verbinden; zuerst wird nach dem Passwort des Administrators gefragt.

folders-sharing-fetching = Samba, das Programm, das Ordner freigibt, wird geholt und eingerichtet. Das geschieht nur beim ersten Mal, und zuerst wird nach dem Passwort des Administrators gefragt.

folders-sharing-setting-up = Die Freigabe wird eingerichtet. Das geschieht nur beim ersten Mal, und zuerst wird nach dem Passwort des Administrators gefragt.

folders-sharing-changing = Die Freigabe wird geändert…

folders-sharing-sharing-it = Wird freigegeben…
