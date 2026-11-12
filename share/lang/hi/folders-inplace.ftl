# machine translated (Hindi), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = Folders { $name } का नाम नहीं बदल सका: { $why }।

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] { $name } नाम का फ़ोल्डर यहाँ पहले से है।
       *[other] { $name } नाम की फ़ाइल यहाँ पहले से है।
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = यहाँ आप किसी का भी नाम नहीं बदल सकते।

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = पहले नाम बदलने के लिए कुछ चुनें।

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = { $name } बनाना वापस लें

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = { $old } का नाम { $name } करना वापस लें

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = { $old } का नाम बदलकर { $name } कर दिया गया।

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = { $old } का नाम बदलकर { $name } किया जा रहा है…
