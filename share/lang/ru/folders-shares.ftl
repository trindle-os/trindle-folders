# machine translated (Russian), not yet reviewed by a person.

folders-shares-nfs-denied = Сервер { $server } не разрешает этому компьютеру открыть { $export }.

folders-shares-nfs-no-share = На сервере { $server } нет общего ресурса с именем { $export }.

folders-shares-left-none = Будет запрошен снова при следующем открытии

folders-shares-left-under-day = Осталось меньше суток

folders-shares-left-days =
    { $n ->
        [one] Остался { $n } день
        [few] Осталось { $n } дня
        [many] Осталось { $n } дней
       *[other] Осталось { $n } дня
    }

folders-shares-held =
    { $past ->
        [yes] Сейчас { $server } отвечает по адресу { $now }, который отличается от адреса, по которому ваш вход для него использовался раньше ({ $before }).
       *[no] { $server } отвечает по адресу { $now }, по которому ваш вход для него раньше не использовался.
    }{ $full ->
        [yes] {" "}Запоминаются только последние адреса, по которым он использовался: { $most }.
       *[no] {""}
    } Любое устройство в сети может откликаться на имя, поэтому сохранённый вход не отправлен. Если это ваш { $server }, введите имя пользователя и пароль, чтобы использовать его здесь.

folders-shares-nowhere = Заранее не удалось найти адрес, по которому отвечает { $server }, поэтому ему ничего не отправлено, а введённым здесь данным некуда безопасно уйти. Откройте его по адресу, например smb://192.168.1.20/.

folders-shares-lapsed = { $server } отвечает по адресу { $now }, где ваш вход для него последний раз использовался более { $days } дней назад. С тех пор этот адрес мог достаться другому устройству, поэтому сохранённый вход не отправлен. Если это ваш { $server }, введите имя пользователя и пароль, чтобы использовать его здесь.

folders-shares-unknown = { $server } отвечает по адресу { $now }, где вход для него использовался раньше, но когда и какой именно, не записано. С тех пор этот адрес мог достаться другому устройству, поэтому сохранённый вход не отправлен. Если это ваш { $server }, введите имя пользователя и пароль, чтобы использовать его здесь.

folders-shares-unshown = { $server } отвечает по адресу { $now }, куда раньше уходил ваш вход для него. На этот раз адрес был лишь услышан в сети, где любое устройство может откликаться на имя, и вам он здесь раньше не показывался, поэтому вход не отправлен. Если это ваш { $server }, введите имя пользователя и пароль, чтобы использовать его здесь.

folders-shares-asking =
    { $server } запрашивает имя пользователя и пароль. Это данные учётной записи на том компьютере.{ $heard ->
        [yes] {" "}Он отвечает по адресу { $at }. Любое устройство в сети может откликаться на имя, поэтому перед вводом убедитесь, что это его адрес.
       *[no] {""}
    }

folders-shares-where =
    Введённое уходит на адрес { $address }{ $heard ->
        [yes] {" "}и больше никуда. Если включено «Запомнить», ваш вход сохраняется только для этого адреса.
       *[no] {"."}
    }

folders-shares-again = { $server } отвечает по адресу { $at }, порт { $port }. Этот адрес был лишь услышан в сети, где любое устройство может откликаться на имя, поэтому ваш вход для него не отправлен. Если это его адрес, нажмите «Открыть».

folders-shares-keyring-unreachable = Не удалось связаться со связкой ключей: { $why }.

folders-shares-unkept = Ресурс открыт, но вход не сохранён. { $why }

folders-shares-nfs-version = Сервер { $server } не поддерживает NFS версии 4, а Trindle открывает только её.

folders-shares-nfs-opened-unknown = Папка открыта, но её расположение неизвестно.

folders-shares-opened-unknown = Общий ресурс открыт, но его расположение неизвестно.

folders-shares-not-address = Это не адрес.

folders-shares-seen-label = Адреса, по которым открывался smb://{ $server }

folders-shares-password-label = Пароль для { $user } на { $what }

folders-shares-remember = Запомнить на 90 дней

folders-shares-remember-line = Срок начинается заново при каждом использовании.

folders-shares-share-on = { $share } на { $server }

folders-shares-card-title = Открыть { $it }

folders-shares-close = Закрыть

folders-shares-user-placeholder = Имя пользователя, например jo или WORKGROUP\jo

folders-shares-password = Пароль

folders-shares-forget = Забыть прежние адреса

folders-shares-forget-spoken = Забыть адреса, по которым этот вход использовался раньше

folders-shares-opening = Открывается

folders-shares-open = Открыть

folders-shares-bad-computer = Это не имя компьютера.

folders-shares-bad-path = Это не путь к общей папке.

folders-shares-opening-export = Открывается { $export } на { $host }…

folders-shares-opening-share = Открывается { $share } на { $server }…

folders-shares-answer-card = Сначала ответьте на запрос для { $it }.

folders-shares-open-waiting = Ресурс открыт. { $words }

folders-shares-none = { $server } не предоставляет вам общих папок.

folders-shares-closing = Закрывается { $name }…

folders-shares-close-failed = Не удалось закрыть { $name }: { $why }.

folders-shares-reconnecting = Повторное подключение { $name }…

folders-shares-not-remembered = Не удалось запомнить сетевые папки до следующего раза.
