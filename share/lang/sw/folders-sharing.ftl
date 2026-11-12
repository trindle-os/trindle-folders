
# machine translated (Swahili), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders haiwezi kusoma kilichomo kwenye folda hii.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = Unaweza kushiriki folda yenyewe tu, si kiungo cha folda.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = Unaweza kushiriki folda zako mwenyewe tu. Hii ni ya mtu mwingine.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = folda yako ya nyumbani

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = folda hii

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } na { $second }

# Name of the switch that shares the folder.
# SAFETY: needs a human review before it ships
folders-sharing-share = Shiriki folda hii

# One line under the share switch when the folder is shared.
# SAFETY: needs a human review before it ships
folders-sharing-shared = Imeshirikiwa kwenye mtandao wa nyumbani.


# Tip of the share switch.
# SAFETY: needs a human review before it ships
folders-sharing-share-tip = Shiriki kwenye mtandao wa nyumbani

# Label over the box for the name other computers see.
folders-sharing-name = Jina

# Tip of the box for the name other computers see.
folders-sharing-name-tip = Jina ambalo kompyuta nyingine zinaona

# Name of the switch that lets others change files, and its tip.
# SAFETY: needs a human review before it ships
folders-sharing-write = Ruhusu wengine kubadilisha faili

# One line under the switch that lets others change files.
# SAFETY: needs a human review before it ships
folders-sharing-write-sub = Wanaweza kuongeza, kubadilisha na kufuta faili ndani yake.

# Name of the switch that lets anyone in without a password.
# SAFETY: needs a human review before it ships
folders-sharing-guests = Mruhusu yeyote aingie

# One line under the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-sub = Hakuna nenosiri linalohitajika.

# Tip of the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-tip = Mruhusu yeyote aingie bila nenosiri

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Ikishirikiwa, inaonekana chini ya Mtandao kwenye kompyuta zako nyingine nyumbani, pamoja na zile za Windows. Mara ya kwanza, Trindle hupakua Samba, programu inayoshiriki folda. Hili huchukua dakika moja.
       *[no] Ikishirikiwa, inaonekana chini ya Mtandao kwenye kompyuta zako nyingine nyumbani, pamoja na zile za Windows.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] Kwenye kompyuta nyingine, tafuta chini ya Mtandao { $name } kwenye { $host }.
       *[no] Kwenye kompyuta nyingine, tafuta chini ya Mtandao { $name }.
    }

# Note when other computers are not allowed to connect yet.
# SAFETY: needs a human review before it ships
folders-sharing-cannot-reach = Kompyuta nyingine bado haziwezi kuifikia folda hii. Kwenye Mipangilio, fungua Mtandao kisha uwashe "Ruhusu kompyuta nyingine ziunganishwe".

# Tip of the button that lets other computers connect.
# SAFETY: needs a human review before it ships
folders-sharing-let-connect = Ruhusu kompyuta nyingine ziunganishwe

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
# SAFETY: needs a human review before it ships
folders-sharing-visitors =
    { $n ->
        [one] Wageni wasio na nenosiri bado hawawezi kuifungua, kwa sababu { $names } imefungwa kwa akaunti nyingine. Ukiwaruhusu, wanaweza kufika kwenye folda hii lakini hawawezi kuona kingine chochote njiani.
       *[other] Wageni wasio na nenosiri bado hawawezi kuifungua, kwa sababu { $names } zimefungwa kwa akaunti nyingine. Ukiwaruhusu, wanaweza kufika kwenye folda hii lakini hawawezi kuona kingine chochote njiani.
    }

# Tip of the button that lets visitors through to the folder.
# SAFETY: needs a human review before it ships
folders-sharing-let-through = Waruhusu wageni kufika kwenye folda hii

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
# SAFETY: needs a human review before it ships
folders-sharing-sign-in-set = Wengine huingia kama { $user } kwa nenosiri la kushiriki.

# Tip of the button that changes the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-change-password = Badilisha nenosiri la kushiriki

# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-set-password = Wengine huingia kama { $user }. Weka nenosiri la kushiriki lenye angalau herufi { $n }. Ni tofauti na nenosiri lako la kuingia.

# Label beside the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password = Nenosiri

# Tip of the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password-tip = Nenosiri la kushiriki

# Tip of the button that keeps the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-keep-password = Hifadhi nenosiri

# Title of the Share on the network window; { $name } is the folder's name.
# SAFETY: needs a human review before it ships
folders-sharing-card-title = Shiriki { $name } kwenye mtandao

# Red words when the share's name is used by another share.
folders-sharing-name-taken = Tayari una folda iliyoshirikiwa yenye jina hilo. Chagua jina lingine.

# Calm words while sharing is being stopped.
folders-sharing-stopping = Inasimamisha kushiriki…

# Red words when the sharing password is too short; { $n } is the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-password-short = Nenosiri linahitaji angalau herufi { $n }.

# Calm words while the sharing password is being kept.
# SAFETY: needs a human review before it ships
folders-sharing-keeping-password = Inahifadhi nenosiri. Utaombwa nenosiri la msimamizi kwanza.

# Calm words while other computers are being let connect.
# SAFETY: needs a human review before it ships
folders-sharing-letting-connect = Inaruhusu kompyuta nyingine ziunganishwe. Utaombwa nenosiri la msimamizi kwanza.

# Calm words while Samba is fetched and set up the first time.
# SAFETY: needs a human review before it ships
folders-sharing-fetching = Inapakua na kuandaa Samba, programu inayoshiriki folda. Utaombwa nenosiri la msimamizi kwanza. Hili hutokea mara ya kwanza tu.

# Calm words while sharing is set up the first time and Samba is already there.
# SAFETY: needs a human review before it ships
folders-sharing-setting-up = Inaandaa kushiriki. Utaombwa nenosiri la msimamizi kwanza. Hili hutokea mara ya kwanza tu.

# Calm words while an existing share is changed.
folders-sharing-changing = Inabadilisha kushiriki…

# Calm words while a share is made.
folders-sharing-sharing-it = Inashiriki folda…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = Ipe folda iliyoshirikiwa jina.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = Jina la folda iliyoshirikiwa haliwezi kuwa refu kuliko herufi { $most }.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = Jina la folda iliyoshirikiwa haliwezi kuwa na lolote kati ya % < > * ? | / \ + = ; : " au koma.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = Jina linaloishia na $ lingeficha shea, kwa hiyo huwezi kulitumia.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = Jina la folda iliyoshirikiwa haliwezi kuanza na kistari.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba tayari inatumia jina { $name }. Chagua jingine.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Imeshirikiwa

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = Unaweza kushiriki folda zako tu.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = Jina hilo tayari ni jina la akaunti kwenye kompyuta hii, kwa hiyo huwezi kulitumia kwa shea. Chagua jingine.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = Mipangilio ya Samba ya kompyuta hii hairuhusu shea bila nenosiri.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = Kompyuta hii haiwezi kushiriki folda zaidi.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = Folda hiyo haipo tena.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba haikutoa sababu.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Ujumbe wa Samba: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } ni ya mtu mwingine, kwa hiyo huwezi kuwapa wageni ufikiaji wake kutoka hapa.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders haikuweza kuwapa wageni ufikiaji wa { $path }: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle haikuweza kupakua Samba kwa sababu kompyuta hii inaonekana haina mtandao. Unganisha kwenye intaneti kisha ujaribu tena.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Trindle haikuweza kusakinisha Samba wakati kitu kingine kinasakinishwa. Jaribu tena baada ya dakika chache.
