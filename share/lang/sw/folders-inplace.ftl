
# machine translated (Swahili), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders haikuweza kubadilisha jina la { $name }: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] Folda iitwayo { $name } tayari ipo hapa.
       *[other] Faili iitwayo { $name } tayari ipo hapa.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = Huwezi kubadilisha jina la kitu chochote hapa.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = Chagua kitu cha kubadilisha jina kwanza.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = Tendua kutengeneza { $name }

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = Tendua kubadilisha { $old } kuwa { $name }

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = { $old } imebadilishwa kuwa { $name }.

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = Inabadilisha { $old } kuwa { $name }…
