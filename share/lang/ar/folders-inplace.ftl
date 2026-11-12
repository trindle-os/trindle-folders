
# machine translated (Arabic), not yet reviewed by a person.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
folders-inplace-not-renamed = تعذّر على Folders إعادة تسمية { $name }: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
folders-inplace-taken =
    { $kind ->
        [folder] يوجد هنا مجلد اسمه { $name } بالفعل.
       *[other] يوجد هنا ملف اسمه { $name } بالفعل.
    }

# Red notice at the foot of Folders when Rename is asked for in a place where nothing can be renamed.
folders-inplace-only-folder = لا يمكنك إعادة تسمية أي شيء هنا.

# Red notice at the foot of Folders when Rename is asked for and nothing is picked.
folders-inplace-pick-first = حدّد شيئاً لتعيد تسميته أولاً.

# Label of the undo for a new folder or document that was named straight away; the name it was given.
folders-inplace-undo-making = التراجع عن إنشاء { $name }

# Label of the undo for a rename: the old name, then the new one.
folders-inplace-undo-renaming = التراجع عن إعادة تسمية { $old } إلى { $name }

# Notice at the foot of Folders once a rename is done: the old name, then the new one.
folders-inplace-renamed = أُعيدت تسمية { $old } إلى { $name }.

# Notice at the foot of Folders while a rename on a server is under way: the old name, then the new one.
folders-inplace-renaming = جارٍ إعادة تسمية { $old } إلى { $name }…

