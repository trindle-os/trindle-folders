# machine translated (Russian), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = Folders не может прочитать содержимое этой папки.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = Можно открыть общий доступ только к самой папке, а не к ссылке на неё.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = Общий доступ можно открыть только к вашим папкам. Эта папка принадлежит другому пользователю.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = ваша домашняя папка

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = эта папка

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } и { $second }

# Name of the switch that shares the folder.
# SAFETY: needs a human review before it ships
folders-sharing-share = Открыть общий доступ к папке

# One line under the share switch when the folder is shared.
folders-sharing-shared = Доступна в домашней сети.


# Tip of the share switch.
# SAFETY: needs a human review before it ships
folders-sharing-share-tip = Открыть общий доступ в домашней сети

# Label over the box for the name other computers see.
folders-sharing-name = Имя

# Tip of the box for the name other computers see.
folders-sharing-name-tip = Имя, которое видят другие компьютеры

# Name of the switch that lets others change files, and its tip.
# SAFETY: needs a human review before it ships
folders-sharing-write = Разрешить другим менять файлы

# One line under the switch that lets others change files.
# SAFETY: needs a human review before it ships
folders-sharing-write-sub = Они могут добавлять, менять и удалять в ней файлы.

# Name of the switch that lets anyone in without a password.
# SAFETY: needs a human review before it ships
folders-sharing-guests = Пускать всех

# One line under the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-sub = Пароль не нужен.

# Tip of the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-tip = Пускать всех без пароля

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Когда доступ открыт, папка видна в разделе «Сеть» на ваших других домашних компьютерах, в том числе с Windows. В первый раз Trindle загрузит Samba, программу для общего доступа к папкам. Это займёт около минуты.
       *[no] Когда доступ открыт, папка видна в разделе «Сеть» на ваших других домашних компьютерах, в том числе с Windows.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] На других компьютерах ищите в разделе «Сеть»: «{ $name }» на «{ $host }».
       *[no] На других компьютерах ищите в разделе «Сеть»: «{ $name }».
    }

# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = Другие компьютеры пока не могут подключиться к этой папке. Откройте «Настройки», затем «Сеть» и включите «Разрешить подключение других компьютеров».

# Tip of the button that lets other computers connect.
# SAFETY: needs a human review before it ships
folders-sharing-let-connect = Разрешить подключение других компьютеров

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
# SAFETY: needs a human review before it ships
folders-sharing-visitors =
    { $n ->
        [one] Гости без пароля пока не могут её открыть, потому что { $names } закрыта для других учётных записей. Если их пропустить, они смогут дойти до этой папки, но не увидят ничего другого на пути.
        [few] Гости без пароля пока не могут её открыть, потому что { $names } закрыты для других учётных записей. Если их пропустить, они смогут дойти до этой папки, но не увидят ничего другого на пути.
        [many] Гости без пароля пока не могут её открыть, потому что { $names } закрыты для других учётных записей. Если их пропустить, они смогут дойти до этой папки, но не увидят ничего другого на пути.
       *[other] Гости без пароля пока не могут её открыть, потому что { $names } закрыты для других учётных записей. Если их пропустить, они смогут дойти до этой папки, но не увидят ничего другого на пути.
    }

# Tip of the button that lets visitors through to the folder.
# SAFETY: needs a human review before it ships
folders-sharing-let-through = Дать гостям доступ к этой папке

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
# SAFETY: needs a human review before it ships
folders-sharing-sign-in-set = Другие входят как { $user } с паролем для общего доступа.

# Tip of the button that changes the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-change-password = Изменить пароль для общего доступа

# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-set-password =
    { $n ->
        [one] Другие входят как { $user }. Задайте пароль для общего доступа не короче { $n } символа. Он отделён от пароля для входа в систему.
        [few] Другие входят как { $user }. Задайте пароль для общего доступа не короче { $n } символов. Он отделён от пароля для входа в систему.
        [many] Другие входят как { $user }. Задайте пароль для общего доступа не короче { $n } символов. Он отделён от пароля для входа в систему.
       *[other] Другие входят как { $user }. Задайте пароль для общего доступа не короче { $n } символа. Он отделён от пароля для входа в систему.
    }

# Label beside the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password = Пароль

# Tip of the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password-tip = Пароль общего доступа

# Tip of the button that keeps the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-keep-password = Сохранить пароль

# Title of the Share on the network window; { $name } is the folder's name.
# SAFETY: needs a human review before it ships
folders-sharing-card-title = Открыть доступ к «{ $name }» по сети

# Red words when the share's name is used by another share.
folders-sharing-name-taken = У вас уже есть общая папка с таким именем. Выберите другое.

# Calm words while sharing is being stopped.
folders-sharing-stopping = Общий доступ закрывается…

# Red words when the sharing password is too short; { $n } is the fewest letters.
folders-sharing-password-short =
    { $n ->
        [one] Пароль должен быть не короче { $n } символа.
        [few] Пароль должен быть не короче { $n } символов.
        [many] Пароль должен быть не короче { $n } символов.
       *[other] Пароль должен быть не короче { $n } символа.
    }

# Calm words while the sharing password is being kept.
folders-sharing-keeping-password = Пароль сохраняется. Сначала у вас запросят пароль администратора.

# Calm words while other computers are being let connect.
folders-sharing-letting-connect = Разрешается подключение других компьютеров. Сначала у вас запросят пароль администратора.

# Calm words while Samba is fetched and set up the first time.
folders-sharing-fetching = Загружается и настраивается Samba, программа для общего доступа к папкам. Сначала у вас запросят пароль администратора. Это происходит только в первый раз.

# Calm words while sharing is set up the first time and Samba is already there.
folders-sharing-setting-up = Настраивается общий доступ. Сначала у вас запросят пароль администратора. Это происходит только в первый раз.

# Calm words while an existing share is changed.
folders-sharing-changing = Общая папка изменяется…

# Calm words while a share is made.
folders-sharing-sharing-it = Открывается общий доступ к папке…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = Дайте общей папке имя.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = Имя общей папки может содержать не больше { $most } символов.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = Имя общей папки не может содержать ни один из символов % < > * ? | / \ + = ; : " и запятую.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = Имя, оканчивающееся на $, скрыло бы общую папку, поэтому его нельзя использовать.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = Имя общей папки не может начинаться с дефиса.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = Samba уже использует имя { $name }. Выберите другое.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Общая

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = Общими можно сделать только ваши собственные папки.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = Так уже называется учётная запись на этом компьютере, поэтому имя нельзя использовать для общей папки. Выберите другое.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = Настройки Samba на этом компьютере не разрешают общие папки без пароля.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = Этот компьютер не может открыть общий доступ к большему числу папок.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = Этой папки больше нет.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = Samba не назвала причину.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Сообщение Samba: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } принадлежит другому пользователю, поэтому отсюда нельзя дать гостям доступ к ней.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = Folders не удалось дать гостям доступ к { $path }: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = Trindle не удалось загрузить Samba, потому что компьютер, похоже, не в сети. Подключитесь к интернету и попробуйте снова.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = Trindle не удалось установить Samba, пока идёт другая установка. Попробуйте через несколько минут.
