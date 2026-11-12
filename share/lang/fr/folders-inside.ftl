
# machine translated (French), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = Sélectionnez d’abord un élément dans l’archive.

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = Les éléments d’une archive ne peuvent être extraits que dans un dossier de cet ordinateur.

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = Folders n’a pas pu lire { $name } : { $why }.

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
        [one] Folders n’a pas pu extraire 1 élément, car son nom sort du dossier ou parce que quelque chose gêne.
        [many] Folders n’a pas pu extraire { $n } d’éléments, car leurs noms sortent du dossier ou parce que quelque chose gêne.
       *[other] Folders n’a pas pu extraire { $n } éléments, car leurs noms sortent du dossier ou parce que quelque chose gêne.
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = Folders n’a pas pu l’extraire de l’archive : { $why }.
