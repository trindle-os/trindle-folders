

folders-shares-nfs-denied = { $server } não deixa este computador abrir { $export }.

folders-shares-nfs-no-share = { $server } não compartilha nada chamado { $export }.

folders-shares-left-none = Pergunta de novo na próxima abertura

folders-shares-left-under-day = Falta menos de um dia

folders-shares-left-days =
    { $n ->
        [one] Falta { $n } dia
       *[other] Faltam { $n } dias
        [many] Faltam { $n } dias
    }

folders-shares-held =
    { $past ->
        [yes] { $server } agora responde em { $now }, um endereço diferente daquele em que o seu login para ele foi usado antes ({ $before }).
       *[no] { $server } responde em { $now }, um endereço em que o seu login para ele nunca foi usado.
    }{ $full ->
        [yes] {" "}Só os últimos { $most } endereços em que foi usado são lembrados.
       *[no] {""}
    } Qualquer dispositivo em uma rede pode responder a um nome, então o login guardado para ele não foi enviado. Se este é o seu { $server }, digite o nome de usuário e a senha para usá-lo aqui.

folders-shares-nowhere = O endereço em que { $server } responde não pôde ser encontrado antes, então nada foi enviado a ele e o que for digitado aqui não tem para onde ir com segurança. Abra-o pelo endereço, como smb://192.168.1.20/.

folders-shares-lapsed = { $server } responde em { $now }, onde o seu login para ele foi usado pela última vez há mais de { $days } dias. Outro dispositivo pode ter recebido esse endereço desde então, então o login guardado para ele não foi enviado. Se este é o seu { $server }, digite o nome de usuário e a senha para usá-lo aqui.

folders-shares-unknown = { $server } responde em { $now }, onde um login para ele foi usado antes, mas não foi anotado quando, nem qual login. Outro dispositivo pode ter recebido esse endereço desde então, então o login guardado para ele não foi enviado. Se este é o seu { $server }, digite o nome de usuário e a senha para usá-lo aqui.

folders-shares-unshown = { $server } responde em { $now }, o endereço para onde o seu login para ele foi enviado antes. Desta vez esse endereço só foi ouvido na rede, onde qualquer dispositivo pode responder a um nome, e ele não foi mostrado a você aqui antes, então o login não foi enviado. Se este é o seu { $server }, digite o nome de usuário e a senha para usá-lo aqui.

folders-shares-asking =
    { $server } pede um nome de usuário e uma senha. São os de uma conta nesse computador.{ $heard ->
        [yes] {" "}Ele responde em { $at }. Qualquer dispositivo em uma rede pode responder a um nome, então confira se esse é o endereço dele antes de digitar.
       *[no] {""}
    }

folders-shares-where =
    O que você digita vai para { $address }{ $heard ->
        [yes] {" "}e para mais nenhum lugar. Com Lembrar ligado, o seu login é guardado só para esse endereço.
       *[no] {"."}
    }

folders-shares-again = { $server } responde em { $at }, porta { $port }. Esse endereço só foi ouvido na rede, onde qualquer dispositivo pode responder a um nome, então o seu login para ele não foi enviado. Se esse é o endereço dele, pressione Abrir.

folders-shares-keyring-unreachable = Não foi possível alcançar o chaveiro: { $why }.

folders-shares-unkept = Está aberto, mas o login não foi guardado. { $why }

folders-shares-nfs-version = { $server } não oferece o NFS versão 4, o único tipo que o Trindle abre.

folders-shares-nfs-opened-unknown = Abriu, mas não se sabe onde.

folders-shares-opened-unknown = O compartilhamento abriu, mas não se sabe onde.

folders-shares-not-address = Isso não é um endereço.

folders-shares-seen-label = Endereços em que smb://{ $server } foi aberto

folders-shares-password-label = Senha de { $user } em { $what }

folders-shares-remember = Lembrar por 90 dias

folders-shares-remember-line = Recomeça a cada vez que você o usa.

folders-shares-share-on = { $share } em { $server }

folders-shares-card-title = Abrir { $it }

folders-shares-close = Fechar

folders-shares-user-placeholder = Nome de usuário, como jo, ou WORKGROUP\jo

folders-shares-password = Senha

folders-shares-forget = Esquecer os endereços anteriores

folders-shares-forget-spoken = Esquecer os endereços em que este login foi usado antes

folders-shares-opening = Abrindo

folders-shares-open = Abrir

folders-shares-bad-computer = Esse não é o nome de um computador.

folders-shares-bad-path = Esse não é o caminho de uma pasta compartilhada.

folders-shares-opening-export = Abrindo { $export } em { $host }…

folders-shares-opening-share = Abrindo { $share } em { $server }…

folders-shares-answer-card = Responda primeiro ao cartão de { $it }.

folders-shares-open-waiting = Está aberto. { $words }

folders-shares-none = { $server } não compartilha nenhuma pasta com você.

folders-shares-closing = Fechando { $name }…

folders-shares-close-failed = Não foi possível fechar { $name }: { $why }.

folders-shares-reconnecting = Reconectando { $name }…

folders-shares-not-remembered = Não foi possível guardar os compartilhamentos para a próxima vez.

