# machine translated (Swahili), not yet reviewed by a person.

folders-shares-nfs-denied = { $server } hairuhusu kompyuta hii kufungua { $export }.

folders-shares-nfs-no-share = { $server } haishiriki kitu kinachoitwa { $export }.

folders-shares-left-none = Itaomba tena wakati wa kufungua mara ijayo

folders-shares-left-under-day = Imebaki chini ya siku moja

folders-shares-left-days =
    { $n ->
        [one] Imebaki siku { $n }
       *[other] Zimebaki siku { $n }
    }

folders-shares-held =
    { $past ->
        [yes] { $server } sasa inajibu kwenye { $now }, anwani tofauti na ile ambayo kuingia kwako kwake kulitumika hapo awali ({ $before }).
       *[no] { $server } inajibu kwenye { $now }, anwani ambayo kuingia kwako kwake hakujawahi kutumika hapo awali.
    }{ $full ->
        [yes] {" "}Anwani { $most } za mwisho tu zilizotumika ndizo zinazokumbukwa.
       *[no] {""}
    } Kifaa chochote kwenye mtandao kinaweza kujibu kwa jina, kwa hiyo kuingia kulikohifadhiwa kwake hakujatumwa. Ikiwa hii ni { $server } yako, andika jina la mtumiaji na nenosiri ili kuitumia hapa.

folders-shares-nowhere = Anwani ambayo { $server } inajibu kwenye haikuweza kupatikana mapema, kwa hiyo hakuna kilichotumwa kwake na kile kinachoandikwa hapa hakina mahali salama pa kwenda. Ifungue kwa anwani yake badala yake, kama smb://192.168.1.20/.

folders-shares-lapsed = { $server } inajibu kwenye { $now }, ambapo kuingia kwako kwake kulitumika mara ya mwisho zaidi ya siku { $days } zilizopita. Kifaa kingine huenda kimepewa anwani hiyo tangu hapo, kwa hiyo kuingia kulikohifadhiwa kwake hakujatumwa. Ikiwa hii ni { $server } yako, andika jina la mtumiaji na nenosiri ili kuitumia hapa.

folders-shares-unknown = { $server } inajibu kwenye { $now }, ambapo kuingia kwake kulitumika hapo awali, lakini lini, au ni kuingia kupi, hakukuandikwa. Kifaa kingine huenda kimepewa anwani hiyo tangu hapo, kwa hiyo kuingia kulikohifadhiwa kwake hakujatumwa. Ikiwa hii ni { $server } yako, andika jina la mtumiaji na nenosiri ili kuitumia hapa.

folders-shares-unshown = { $server } inajibu kwenye { $now }, anwani ambayo kuingia kwako kwake kulitumwa hapo awali. Mara hii anwani hiyo ilisikika tu kwenye mtandao, ambapo kifaa chochote kinaweza kujibu kwa jina, na haijaonyeshwa kwako hapa hapo awali, kwa hiyo kuingia hakujatumwa. Ikiwa hii ni { $server } yako, andika jina la mtumiaji na nenosiri ili kuitumia hapa.

folders-shares-asking =
    { $server } inahitaji jina la mtumiaji na nenosiri. Ni vile vya akaunti kwenye kompyuta hiyo.{ $heard ->
        [yes] {" "}Inajibu kwenye { $at }. Kifaa chochote kwenye mtandao kinaweza kujibu kwa jina, kwa hiyo hakikisha hiyo ndiyo anwani yake kabla ya kuandika.
       *[no] {""}
    }

folders-shares-where =
    Unachoandika kinaenda kwenye { $address }{ $heard ->
        [yes] {" "}na hakuna mahali pengine. Kukiwa na Kumbuka, kuingia kwako kunahifadhiwa kwa anwani hiyo tu.
       *[no] {"."}
    }

folders-shares-again = { $server } inajibu kwenye { $at }, mlango { $port }. Anwani hiyo ilisikika tu kwenye mtandao, ambapo kifaa chochote kinaweza kujibu kwa jina, kwa hiyo kuingia kwako kwake hakujatumwa. Ikiwa hiyo ndiyo anwani yake, bonyeza Fungua.

folders-shares-keyring-unreachable = Hifadhi ya funguo haikuweza kufikiwa: { $why }.

folders-shares-unkept = Imefunguliwa, lakini kuingia hakukuhifadhiwa. { $why }

folders-shares-nfs-version = { $server } haitoi NFS toleo la 4, ambalo ndilo pekee Trindle inalofungua.

folders-shares-nfs-opened-unknown = Imefunguka, lakini mahali ilipo hapajulikani.

folders-shares-opened-unknown = Kishiriki kimefunguka, lakini mahali kilipo hapajulikani.

folders-shares-not-address = Hiyo si anwani.

folders-shares-seen-label = Anwani ambazo smb://{ $server } ilifunguliwa

folders-shares-password-label = Nenosiri la { $user } kwenye { $what }

folders-shares-remember = Kumbuka kwa siku 90

folders-shares-remember-line = Huanza tena kila unapoitumia.

folders-shares-share-on = { $share } kwenye { $server }

folders-shares-card-title = Fungua { $it }

folders-shares-close = Funga

folders-shares-user-placeholder = Jina la mtumiaji, kama jo, au WORKGROUP\jo

folders-shares-password = Nenosiri

folders-shares-forget = Sahau anwani za awali

folders-shares-forget-spoken = Sahau anwani ambazo kuingia huku kulitumika hapo awali

folders-shares-opening = Inafungua

folders-shares-open = Fungua

folders-shares-bad-computer = Hilo si jina la kompyuta.

folders-shares-bad-path = Hiyo si njia ya folda iliyoshirikiwa.

folders-shares-opening-export = Inafungua { $export } kwenye { $host }…

folders-shares-opening-share = Inafungua { $share } kwenye { $server }…

folders-shares-answer-card = Jibu kadi ya { $it } kwanza.

folders-shares-open-waiting = Imefunguliwa. { $words }

folders-shares-none = { $server } haishiriki folda zozote nawe.

folders-shares-closing = Inafunga { $name }…

folders-shares-close-failed = { $name } haikuweza kufungwa: { $why }.

folders-shares-reconnecting = Inaunganisha tena { $name }…

folders-shares-not-remembered = Hazikuweza kukumbukwa kwa mara ijayo.
