

folders-far-failed-why = { $name } ({ $why })

folders-far-no-server = nenhum servidor chamado { $host } foi encontrado

folders-far-not-all-arrived = nem tudo chegou, então continua onde estava

folders-far-not-followed = é um atalho, que não é seguido, então não foi enviado

folders-far-not-sent = não é um arquivo nem uma pasta, então não foi enviado

folders-far-not-told = o servidor não diz o que é, então foi deixado

folders-far-ftp-cannot-refuse = Este servidor pode dar o mesmo nome a dois arquivos.

folders-far-cannot-do = Isso não pode ser feito com os arquivos de um servidor

folders-far-not-looked-at = não foi possível examiná-lo

folders-far-not-opened = Não foi possível abri-lo.

folders-far-inside-itself = { $name } (uma pasta não pode ficar dentro de si mesma)

folders-far-taken-by-folder = { $name } (já existe uma pasta com esse nome)

folders-far-taken-by-file = { $name } (já existe um arquivo com esse nome)

folders-far-address-unknown = O endereço do servidor não é conhecido.

folders-far-copy-wrong-length = a cópia enviada não tinha o tamanho que devia ter

folders-far-only-server-file = Só um arquivo em um servidor tem esse nome.

folders-far-aside-both = o antigo está lá inteiro como { $old }, e o novo como { $new }: { $why }

folders-far-aside-kept = não foi possível remover o antigo, e ele está lá inteiro como { $old }

folders-far-aside-kept-why = não foi possível remover o antigo, e ele está lá inteiro como { $old }: { $why }

folders-far-not-empty = { $name } não está vazio.

folders-far-unsafe-name = Ignorado: o servidor enviou um nome inseguro.

folders-far-unsafe-names =
    { $n ->
        [one] Ignorado: o servidor enviou um nome inseguro: { $names }.
       *[other] { $n } ignorados: o servidor enviou nomes inseguros: { $names }.
        [many] { $n } ignorados: o servidor enviou nomes inseguros: { $names }.
    }

folders-far-fetching = Buscando { $name }…

folders-far-cannot-fetch = Não é possível buscar { $name }: { $why }.

folders-far-opened-copy = Aberta uma cópia de { $name }. Ao salvá-la, as alterações voltam para o servidor.

folders-far-saved = { $name } salvo no servidor.

folders-far-not-saved = Não foi possível salvar { $name } no servidor: { $why }. A cópia daqui guarda as alterações.

folders-far-renamed = { $was } renomeado para { $now }.

folders-far-connect-title = Conectar a um servidor

folders-far-connect-close = Fechar

folders-far-connect-address = Endereço, como sftp://jo@example.com

folders-far-connect-password = Senha, se precisar de uma

folders-far-connect-key = Arquivo de chave, se você escolher um

folders-far-connect-pick-key = Escolher o arquivo de chave

folders-far-connect-choose = Escolher

folders-far-connect-ssh-setup = Sua própria configuração do ssh é usada: o arquivo de configuração, o agente, as chaves e os servidores conhecidos.

folders-far-connect-new-host = { $host } nunca foi visitado antes, então o Trindle não pode saber se é o certo. Confie nele se você esperava chegar a ele.

folders-far-connect-host-changed = { $host } não tem mais a identidade que tinha antes. Isso pode significar que alguém está no caminho, então não foi aberto. Se o dono dele a mudou, remova o antigo dos seus servidores conhecidos primeiro.

folders-far-connect-needs-password = Este servidor quer uma senha. Digite-a acima e tente de novo.

folders-far-connect-needs-key = Este servidor quer uma chave ou uma senha. Informe o arquivo da sua chave acima, ou adicione-a ao seu agente ssh, e tente de novo.

folders-far-connect-cert-new = Não foi possível verificar o certificado de { $host }: { $why }. Se puder, compare a impressão digital dele com a que o dono do servidor informa, e confie nele só se forem iguais.

folders-far-connect-cert-changed = O certificado de { $host } não é o mesmo em que você confiou antes. O dono pode tê-lo renovado, ou alguém pode estar no caminho. Confie no novo só se o dono confirmar esta impressão digital.

folders-far-connect-connecting = Conectando

folders-far-connect-trust = Confiar e conectar

folders-far-connect-go = Conectar

folders-far-phones = Os telefones aparecem sozinhos na barra lateral quando são ligados e desbloqueados.

