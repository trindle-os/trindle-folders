# machine translated (Swahili), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = Funga

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = Nyumbani

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = Fungua { $name } kwa

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = Programu ya sasa

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = Hakuna programu kwenye kompyuta hii iliyowekwa kufungua aina hii ya faili.

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = Tumia programu hii kila mara

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = Itumie kwa kila faili ya { $kind }.

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = Iweke kuwa programu ya kawaida

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = Onyesha programu zilizopendekezwa tu

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = Onyesha programu zote

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = Fungua

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = Chagua programu kwanza.

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = Folders haikuweza kufanya { $name } kuwa programu ya kawaida.

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = Folders haikuweza kuanzisha { $name }.

# Heading of the Properties window.
folders-dialogs-properties = Sifa

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = Kurasa za Sifa

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = Jumla

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = Maelezo

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = Kushiriki



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = Jina

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = Mahali

# Label in Properties of the kind of file.
folders-dialogs-row-kind = Aina

# Label in Properties of where a link points.
folders-dialogs-row-link-to = Kiungo cha

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = Ilibadilishwa

# Label in Properties of who owns the item.
folders-dialogs-row-owner = Mmiliki

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = Ruhusa

# Label in Properties of the size of the item.
folders-dialogs-row-size = Ukubwa

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = Haijulikani

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, unaweza kuifungua na kuibadilisha

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, unaweza kuifungua lakini huwezi kuibadilisha

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, unaweza kuibadilisha lakini huwezi kuifungua

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, huwezi kuifungua

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , baadhi hazikuweza kusomwa
       *[other]
    }{ $adding ->
        [yes] , bado inajumlisha
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = Inakokotoa ukubwa

# Label in Properties of the row about checksums.
folders-dialogs-checksums = Hesabu za uthibitisho

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = Kokotoa hesabu za uthibitisho

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = Inakokotoa hesabu za uthibitisho

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = Folders haikuweza kusoma faili.

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = Bandika hesabu ya uthibitisho ili kulinganisha

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } inalingana

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = Hailingani

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = Kagua


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [one] Tuma { $name } kwenda
       *[other] Tuma vitu { $n } kwenda
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = Hii inatengeneza nakala huko. Vya asili vinabaki vilipo.

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = Kidirisha kingine, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = Safu za mwonekano wa Orodha

# Note in the Columns window.
folders-dialogs-columns-note = Jina huonyeshwa kila mara. Safu zisizotosha hufichwa hadi kidirisha kiwe pana zaidi.

# Heading over the columns that suit every file.
folders-dialogs-columns-files = Faili

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = Muziki, picha, PDF

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = Onyesha { $name }

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = Folders haikuweza kuhifadhi safu.

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [one] Rangi ya { $name }
       *[other] Rangi ya folda { $n }
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = Bila rangi

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = Folders haikuweza kuhifadhi rangi ya folda.


folders-dialogs-no-match = Hakuna programu inayolingana na maneno hayo.


folders-dialogs-find-app = Tafuta programu


folders-dialogs-find-place = Tafuta mahali


folders-dialogs-no-match-place = Hakuna mahali panapolingana na maneno hayo.
