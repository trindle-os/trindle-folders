
# machine translated (German), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders konnte { $name } nicht umbenennen: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] Hier gibt es schon einen Ordner namens { $name }.
       *[other] Hier gibt es schon eine Datei namens { $name }.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = Hier kannst du nichts umbenennen.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Wähle zuerst etwas zum Umbenennen aus.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = Erstellen von { $name } rückgängig machen

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = Umbenennen von { $old } in { $name } rückgängig machen

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = { $old } wurde in { $name } umbenannt.

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = { $old } wird in { $name } umbenannt…
