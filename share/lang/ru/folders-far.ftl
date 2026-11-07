# machine translated (Russian), not yet reviewed by a person.

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = сервер «{ $host }» не найден

folders-far-not-all-arrived = файл дошёл не весь, поэтому он остался на прежнем месте

folders-far-not-followed = это ссылка, по ссылкам не переходят, поэтому она не отправлена

folders-far-not-sent = это не файл и не папка, поэтому оно не отправлено

folders-far-not-told = сервер не говорит, что это, поэтому оно пропущено

folders-far-ftp-cannot-refuse = Этот сервер может допустить два файла с одним именем.

folders-far-cannot-do = Так нельзя поступать с файлами на сервере

folders-far-not-looked-at = его не удалось просмотреть

folders-far-not-opened = Не удалось открыть.

folders-far-inside-itself = { $name } (папку нельзя поместить внутрь самой себя)

folders-far-taken-by-folder = { $name } (там уже есть папка с таким именем)

folders-far-taken-by-file = { $name } (там уже есть файл с таким именем)

folders-far-address-unknown = Адрес сервера неизвестен.

folders-far-copy-wrong-length = отправленная копия оказалась не той длины

folders-far-only-server-file = Так переименовывают только файл на сервере.

folders-far-aside-both = старый файл цел и лежит здесь: { $old }, а новый здесь: { $new }: { $why }

folders-far-aside-kept = старый файл не удалось убрать, он цел и лежит здесь: { $old }

folders-far-aside-kept-why = старый файл не удалось убрать, он цел и лежит здесь: { $old }: { $why }

folders-far-not-empty = Папка { $name } не пуста.

folders-far-unsafe-name = Пропущено: сервер прислал небезопасное имя.

folders-far-unsafe-names =
    { $n ->
        [one] Пропущено: сервер прислал небезопасное имя: { $names }.
        [few] Пропущено: { $n }. Сервер прислал небезопасные имена: { $names }.
        [many] Пропущено: { $n }. Сервер прислал небезопасные имена: { $names }.
       *[other] Пропущено: { $n }. Сервер прислал небезопасные имена: { $names }.
    }

folders-far-fetching = Загружается { $name }…

folders-far-cannot-fetch = Не удалось загрузить { $name }: { $why }.

folders-far-opened-copy = Открыта копия файла { $name }. Если сохранить её, изменения отправятся обратно.

folders-far-saved = Файл { $name } сохранён на сервере.

folders-far-not-saved = Не удалось сохранить { $name } на сервере: { $why }. Изменения остались в копии здесь.

folders-far-renamed = { $was } переименовано в { $now }.

folders-far-connect-title = Подключиться к серверу

folders-far-connect-close = Закрыть

folders-far-connect-address = Адрес, например sftp://jo@example.com

folders-far-connect-password = Пароль, если он нужен

folders-far-connect-key = Файл ключа, если вы его выбираете

folders-far-connect-pick-key = Выбрать файл ключа

folders-far-connect-choose = Выбрать

folders-far-connect-ssh-setup = Используются ваши собственные настройки ssh: конфигурация, агент, ключи и известные серверы.

folders-far-connect-new-host = К серверу { $host } вы ещё не подключались, поэтому Trindle не может проверить, тот ли это сервер. Доверяйте ему, только если ожидали подключиться именно к нему.

folders-far-connect-host-changed = У сервера { $host } теперь другая идентификация, чем раньше. Возможно, кто-то вмешивается в соединение, поэтому оно не открыто. Если владелец сервера изменил её, сначала удалите старую запись из списка известных серверов.

folders-far-connect-needs-password = Этому серверу нужен пароль. Введите его выше и попробуйте снова.

folders-far-connect-needs-key = Этому серверу нужен ключ или пароль. Укажите файл ключа выше или добавьте его в агент ssh и попробуйте снова.

folders-far-connect-cert-new = Не удалось проверить сертификат сервера { $host }: { $why }. Если возможно, сравните его отпечаток с тем, который сообщил владелец сервера, и доверяйте, только если они совпадают.

folders-far-connect-cert-changed = Сертификат сервера { $host } не тот, которому вы доверяли раньше. Возможно, владелец его обновил, а возможно, кто-то вмешивается в соединение. Доверяйте новому, только если владелец подтвердил этот отпечаток.

folders-far-connect-connecting = Подключение

folders-far-connect-trust = Доверять и подключиться

folders-far-connect-go = Подключиться

folders-far-phones = Телефоны появляются на боковой панели сами, когда их подключают и разблокируют.
