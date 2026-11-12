# machine translated (Swahili), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
folders-shares-nfs-denied = Kompyuta hii hairuhusiwi kufungua { $export } kwenye { $server }.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } haishiriki folda iitwayo { $export }.

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = Utaombwa tena mara ijayo

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = Imebaki chini ya siku moja

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] Imebaki siku { $n }
       *[other] Zimebaki siku { $n }
    }

# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
# SAFETY: needs a human review before it ships
folders-shares-held =
    { $past ->
        [yes] { $server } sasa iko { $now }. Kuingia kwako kwake kulitumika kwenye anwani tofauti hapo awali ({ $before }).
       *[no] { $server } iko { $now }. Kuingia kwako kwake hakujawahi kutumika kwenye anwani hiyo.
    }{ $full ->
        [yes] {" "}Folders hukumbuka tu anwani { $most } za mwisho.
       *[no] {""}
    } Kifaa chochote kwenye mtandao kinaweza kutumia jina hilo, kwa hiyo Folders haijatuma kuingia kwako kulikohifadhiwa. Ikiwa hii ni { $server } yako, andika jina la mtumiaji na nenosiri ili kuitumia hapa.

# Words on the sign-in card when no address could be found for the computer to show.
# SAFETY: needs a human review before it ships
folders-shares-nowhere = Folders haikuweza kupata anwani ya { $server }, kwa hiyo hakuna kilichotumwa kwake. Chochote unachoandika hapa hakiwezi kutumwa kwa usalama. Ifungue kwa anwani yake badala yake, kama smb://192.168.1.20/.

# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
# SAFETY: needs a human review before it ships
folders-shares-lapsed = { $server } iko { $now }. Kuingia kwako kwake kulitumika huko mara ya mwisho zaidi ya siku { $days } zilizopita. Kifaa kingine huenda kimepewa anwani hiyo tangu hapo, kwa hiyo Folders haijatuma kuingia kwako kulikohifadhiwa. Ikiwa hii ni { $server } yako, andika jina la mtumiaji na nenosiri ili kuitumia hapa.

# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
# SAFETY: needs a human review before it ships
folders-shares-unknown = { $server } iko { $now }. Kuingia kwake kulitumika huko hapo awali, lakini Folders haikuandika lini, wala ni kuingia kupi. Kifaa kingine huenda kimepewa anwani hiyo tangu hapo, kwa hiyo Folders haijatuma kuingia kwako kulikohifadhiwa. Ikiwa hii ni { $server } yako, andika jina la mtumiaji na nenosiri ili kuitumia hapa.

# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
# SAFETY: needs a human review before it ships
folders-shares-unshown = { $server } iko { $now }, ambapo kuingia kwako kwake kulitumika hapo awali. Mara hii Folders iliipata kwenye mtandao wa ndani, ambapo kifaa chochote kinaweza kutumia jina hilo. Hujawahi kuona anwani hii hapo awali, kwa hiyo Folders haijatuma kuingia kwako. Ikiwa hii ni { $server } yako, andika jina la mtumiaji na nenosiri ili kuitumia hapa.

# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
# SAFETY: needs a human review before it ships
folders-shares-asking =
    Andika jina la mtumiaji na nenosiri la akaunti kwenye { $server }.{ $heard ->
        [yes] {" "}Iko { $at }. Kifaa chochote kwenye mtandao kinaweza kutumia jina hilo, kwa hiyo hakikisha hii ndiyo anwani yake kabla ya kuandika.
       *[no] {""}
    }

# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
# SAFETY: needs a human review before it ships
folders-shares-where =
    Unachoandika kinaenda kwenye { $address }{ $heard ->
        [yes] {" "}na hakuna mahali pengine. Kukiwa na Kumbuka, Folders huhifadhi kuingia kwako kwa anwani hii tu.
       *[no] {"."}
    }

# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
# SAFETY: needs a human review before it ships
folders-shares-again = Folders imepata { $server } kwenye mtandao wa ndani, anwani { $at }, mlango { $port }. Kifaa chochote kinaweza kutumia jina hilo huko, kwa hiyo Folders haijatuma kuingia kwako. Ikiwa hii ni anwani sahihi, bonyeza Fungua.

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = Folders haikuweza kufikia hifadhi ya funguo: { $why }.

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = Imefunguliwa, lakini Folders haikuweza kuhifadhi kuingia. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } haitoi NFS toleo la 4, ambalo ndilo pekee Folders inaweza kufungua.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = Shea imefunguka, lakini Folders haikuweza kupata mahali ilipo kwenye kompyuta hii.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = Shea imefunguka, lakini Folders haikuweza kupata mahali ilipo kwenye kompyuta hii.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = Hiyo si anwani.

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = Anwani zilizotumika kwa smb://{ $server }

# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
# SAFETY: needs a human review before it ships
folders-shares-password-label = Nenosiri la { $user } kwenye { $what }

# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
# SAFETY: needs a human review before it ships
folders-shares-remember = Kumbuka kwa siku 90

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = Siku 90 huanza tena kila unapoitumia.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } kwenye { $server }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = Fungua { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Funga

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = Jina la mtumiaji, kama jo au WORKGROUP\jo

# Hint in the empty password box of the sign-in card.
# SAFETY: needs a human review before it ships
folders-shares-password = Nenosiri

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
# SAFETY: needs a human review before it ships
folders-shares-forget = Sahau anwani za awali

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
# SAFETY: needs a human review before it ships
folders-shares-forget-spoken = Sahau anwani za awali za kuingia huku

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Inafungua…

# Name of the main button of the sign-in card.
folders-shares-open = Fungua

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = Hilo si jina la kompyuta.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = Hiyo si njia ya folda iliyoshirikiwa.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = Inafungua { $export } kwenye { $host }…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = Inafungua { $share } kwenye { $server }…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Maliza kuingia kwenye { $it } kwanza.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = Imefunguliwa. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } haishiriki folda zozote nawe.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = Inafunga { $name }…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = Folders haikuweza kufunga { $name }: { $why }.

# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = Inaunganisha tena { $name }…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = Folders haikuweza kuhifadhi shea zako kwa mara ijayo.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = Jina hilo la mtumiaji haliwezi kutumika kwa shea.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = Domeni hiyo haiwezi kutumika kwa shea.

# Red words when a share's password holds a line break.
folders-shares-bad-password = Nenosiri la shea haliwezi kuwa na mstari mpya.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } inahitaji jina la mtumiaji na nenosiri kwa { $it }.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = Jina la mtumiaji au nenosiri la { $it } si sahihi.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = Akaunti yako kwenye { $server } imefungwa au imezimwa, au nenosiri lake limeisha muda.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } haina folda iliyoshirikiwa iitwayo { $share }.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = Akaunti hiyo hairuhusiwi kufungua { $it }.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } haikujibu. Huenda imelala au imezimwa. Iamshe kisha ujaribu tena.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = Folders haikuweza kupata kompyuta iitwayo { $server } kwenye mtandao huu.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } inatumia SMB1 tu, njia ya zamani ya kushiriki ambayo si salama tena. Folders haiitumii.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = Hii inahitaji kifurushi cha { $package } cha Debian, ambacho hakijasakinishwa.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = Folders haikuweza kufungua shea.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = Folders haikuweza kufungua folda iliyoshirikiwa.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = Folders haikuweza kuorodhesha folda zake zilizoshirikiwa.
