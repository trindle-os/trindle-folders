
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red words under the file lines when a network (NFS) server does not let this computer open an exported folder; the server, then the folder.
# SAFETY: needs a human review before it ships
folders-shares-nfs-denied = Este computador não tem permissão para abrir { $export } em { $server }.

# Red words under the file lines when a network (NFS) server has no exported folder of that name; the server, then the folder.
folders-shares-nfs-no-share = { $server } não compartilha uma pasta chamada { $export }.

# Line in Folders' settings list of kept logins: the kept login has run out and is asked for again.
folders-shares-left-none = A pergunta será feita de novo na próxima vez

# Line in Folders' settings list of kept logins: less than a day of the 90 is left.
folders-shares-left-under-day = Falta menos de um dia

# Line in Folders' settings list of kept logins: how many days of the 90 are left.
folders-shares-left-days =
    { $n ->
        [one] Falta { $n } dia
       *[other] Faltam { $n } dias
        [many] Faltam { $n } dias
    }

# Words on the sign-in card when a kept login was held back because the computer answers at an address the login was not used at before. past says whether it was used at other addresses (yes or no), before lists them, full says whether the list of remembered addresses is full (yes or no), most is how many it holds.
# SAFETY: needs a human review before it ships
folders-shares-held =
    { $past ->
        [yes] { $server } agora está em { $now }. Seu login para ele foi usado antes em outro endereço ({ $before }).
       *[no] { $server } está em { $now }. Seu login para ele nunca foi usado nesse endereço.
    }{ $full ->
        [yes] {" "}O Folders só lembra os últimos { $most } endereços.
       *[no] {""}
    } Qualquer dispositivo em uma rede pode se apresentar com qualquer nome, então o Folders não enviou seu login salvo. Se este é o seu { $server }, digite o nome de usuário e a senha para usá-lo aqui.

# Words on the sign-in card when no address could be found for the computer to show.
# SAFETY: needs a human review before it ships
folders-shares-nowhere = O Folders não conseguiu descobrir o endereço de { $server }, então nada foi enviado a ele. Nada do que você digitar aqui pode ser enviado com segurança. Abra-o pelo endereço, como smb://192.168.1.20/.

# Words on the sign-in card when the kept login was last used at this address more than the kept number of days ago.
# SAFETY: needs a human review before it ships
folders-shares-lapsed = { $server } está em { $now }. Seu login para ele foi usado ali pela última vez há mais de { $days } dias. Outro dispositivo pode ter recebido esse endereço desde então, então o Folders não enviou seu login salvo. Se este é o seu { $server }, digite o nome de usuário e a senha para usá-lo aqui.

# Words on the sign-in card when a login was used at this address before but when, or which login, was not written down.
# SAFETY: needs a human review before it ships
folders-shares-unknown = { $server } está em { $now }. Um login para ele foi usado ali antes, mas o Folders não anotou quando, nem qual login. Outro dispositivo pode ter recebido esse endereço desde então, então o Folders não enviou seu login salvo. Se este é o seu { $server }, digite o nome de usuário e a senha para usá-lo aqui.

# Words on the sign-in card when the login went to this address earlier but the address was only heard on the network this time.
# SAFETY: needs a human review before it ships
folders-shares-unshown = { $server } está em { $now }, onde seu login para ele foi usado antes. Desta vez o Folders o encontrou na rede local, onde qualquer dispositivo pode usar esse nome. Você nunca viu este endereço, então o Folders não enviou seu login. Se este é o seu { $server }, digite o nome de usuário e a senha para usá-lo aqui.

# Words on the sign-in card when it simply asks for a user name and password. heard says whether the address was only heard on the network (yes or no); at is that address.
# SAFETY: needs a human review before it ships
folders-shares-asking =
    Digite o nome de usuário e a senha de uma conta em { $server }.{ $heard ->
        [yes] {" "}Ele está em { $at }. Qualquer dispositivo em uma rede pode se apresentar com qualquer nome, então confira se este é o endereço dele antes de digitar.
       *[no] {""}
    }

# Line under the boxes of the sign-in card: where what is typed goes. heard says whether the address was only heard on the network (yes or no).
# SAFETY: needs a human review before it ships
folders-shares-where =
    O que você digita vai para { $address }{ $heard ->
        [yes] {" "}e para mais nenhum lugar. Com Lembrar ligado, o Folders guarda seu login só para este endereço.
       *[no] {"."}
    }

# Words on the sign-in card when it comes up filled in for an address only heard on the network: the computer, its address and port.
# SAFETY: needs a human review before it ships
folders-shares-again = O Folders encontrou { $server } na rede local em { $at }, porta { $port }. Qualquer dispositivo pode usar esse nome ali, então o Folders não enviou seu login. Se este for o endereço certo, pressione Abrir.

# Red words when the keyring cannot be reached; the reason follows.
folders-shares-keyring-unreachable = O Folders não conseguiu alcançar o chaveiro: { $why }.

# Words under the file lines when a share opened but its login could not be kept; the reason follows.
folders-shares-unkept = Está aberto, mas o Folders não conseguiu salvar o login. { $why }

# Red words under the file lines when a network (NFS) server does not offer version 4, the only version used; the server.
folders-shares-nfs-version = { $server } não oferece o NFS versão 4, a única versão que o Folders consegue abrir.

# Red words when a network (NFS) folder was opened but where it is could not be found.
folders-shares-nfs-opened-unknown = O compartilhamento abriu, mas o Folders não conseguiu descobrir onde ele está neste computador.

# Red words when a share was opened but where it is could not be found.
folders-shares-opened-unknown = O compartilhamento abriu, mas o Folders não conseguiu descobrir onde ele está neste computador.

# Red words when the address to remember for a login is not an address.
folders-shares-not-address = Isso não é um endereço.

# Name of the item the keyring keeps for the addresses a computer's shares were opened at; shown in the keyring's own list.
folders-shares-seen-label = Endereços usados para smb://{ $server }

# Name of the item the keyring keeps for a share's login; shown in the keyring's own list. The user, then the address of the computer or share.
# SAFETY: needs a human review before it ships
folders-shares-password-label = Senha de { $user } em { $what }

# Name of the switch on the sign-in card that keeps the login for 90 days, and the words beside it.
# SAFETY: needs a human review before it ships
folders-shares-remember = Lembrar por 90 dias

# Line under the Remember switch on the sign-in card: the 90 days start again each use.
folders-shares-remember-line = Os 90 dias recomeçam a cada vez que você o usa.

# Names a share and the computer it is on, in the title of the sign-in card and in a note.
folders-shares-share-on = { $share } em { $server }

# Title of the sign-in card: the computer, or the share on a computer.
folders-shares-card-title = Abrir { $it }

# Tip and name of the round button that closes the sign-in card.
folders-shares-close = Fechar

# Hint in the empty user name box of the sign-in card.
folders-shares-user-placeholder = Nome de usuário, como jo ou WORKGROUP\jo

# Hint in the empty password box of the sign-in card.
# SAFETY: needs a human review before it ships
folders-shares-password = Senha

# Label of the switch on the sign-in card that forgets the addresses a login was used at before.
# SAFETY: needs a human review before it ships
folders-shares-forget = Esquecer endereços anteriores

# Name a screen reader gives the switch that forgets the earlier addresses of a login.
# SAFETY: needs a human review before it ships
folders-shares-forget-spoken = Esquecer os endereços anteriores deste login

# Words beside the turning ring on the sign-in card while the share is being opened.
folders-shares-opening = Abrindo…

# Name of the main button of the sign-in card.
folders-shares-open = Abrir

# Red words under the file lines when an address does not name a computer.
folders-shares-bad-computer = Isso não é um nome de computador.

# Red words under the file lines when an address does not name a shared folder.
folders-shares-bad-path = Isso não é um caminho para uma pasta compartilhada.

# Words under the file lines while a network folder is being opened: the folder, then the computer.
folders-shares-opening-export = Abrindo { $export } em { $host }…

# Words under the file lines while a share is being opened: the share, then the computer.
folders-shares-opening-share = Abrindo { $share } em { $server }…

# Words under the file lines when another opening is tried while the sign-in card waits for an answer; the computer.
folders-shares-answer-card = Termine primeiro o login em { $it }.

# Words under the file lines when a share is open and the keyring has not yet said whether the login was kept; what the keyring said follows.
folders-shares-open-waiting = Está aberto. { $words }

# Words under the file lines when a computer shows no shared folders to the person.
folders-shares-none = { $server } não compartilha nenhuma pasta com você.

# Words under the file lines while a share is being closed (unmounted); its name.
folders-shares-closing = Fechando { $name }…

# Red words under the file lines when a share could not be closed (unmounted); its name, then the reason.
folders-shares-close-failed = O Folders não conseguiu fechar { $name }: { $why }.



# Words under the file lines while a remembered network share is being mounted again; its name.
folders-shares-reconnecting = Reconectando { $name }…

# Red words under the file lines when the list of remembered shares could not be written.
folders-shares-not-remembered = O Folders não conseguiu salvar seus compartilhamentos para a próxima vez.

# Red words when a share's user name cannot be used.
folders-shares-bad-user = Esse nome de usuário não pode ser usado em um compartilhamento.

# Red words when a share's domain cannot be used.
folders-shares-bad-domain = Esse domínio não pode ser usado em um compartilhamento.

# Red words when a share's password holds a line break.
folders-shares-bad-password = A senha de um compartilhamento não pode conter quebra de linha.

# Red words when a computer wants a user name and password; the computer, then what is being opened.
folders-shares-needs-login = { $server } precisa de um nome de usuário e uma senha para { $it }.

# Red words when the user name or password was refused; what is being opened.
folders-shares-wrong-login = O nome de usuário ou a senha de { $it } está errado.

# Red words when the account on a computer cannot be used; the computer.
folders-shares-account = Sua conta em { $server } está bloqueada ou desativada, ou a senha expirou.

# Red words when a computer has no share of that name; the computer, then the share.
folders-shares-no-share = { $server } não tem uma pasta compartilhada chamada { $share }.

# Red words when the account may not open a share; what is being opened.
folders-shares-denied = Essa conta não tem permissão para abrir { $it }.

# Red words when a computer did not answer; the computer.
folders-shares-asleep = { $server } não respondeu. Ele pode estar em repouso ou desligado. Tire-o do repouso e tente de novo.

# Red words when no computer of that name is on the network; the name.
folders-shares-no-computer = O Folders não encontrou um computador chamado { $server } nesta rede.

# Red words when a computer only speaks the old unsafe SMB1; the computer.
folders-shares-smb1 = { $server } só usa SMB1, uma forma antiga de compartilhar que não é mais segura. O Folders não a usa.

# Red words when a Debian package needed for sharing is missing; the package name.
folders-shares-missing = Isto precisa do pacote { $package } do Debian, que não está instalado.

# Red words when a share could not be opened and the system gave no reason.
folders-shares-open-failed = O Folders não conseguiu abrir o compartilhamento.

# Red words when a shared folder could not be opened and the system gave no reason.
folders-shares-open-export-failed = O Folders não conseguiu abrir a pasta compartilhada.

# Red words when a computer's shared folders could not be listed and the system gave no reason.
folders-shares-list-failed = O Folders não conseguiu listar as pastas compartilhadas dele.
