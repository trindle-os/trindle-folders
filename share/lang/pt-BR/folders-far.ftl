
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Line in a list of things that could not be done on a server: the name, then in brackets why not.
folders-far-failed-why = { $name } ({ $why })

# Error when the name of a server cannot be found on the network; the name of the server.
folders-far-no-server = não existe nenhum servidor chamado { $host }

# Reason a file sent to a server was left where it was: its copy was shorter than the first.
folders-far-not-all-arrived = nem tudo chegou, então o original não foi movido

# Reason a link was not sent to a server.
folders-far-not-followed = é um atalho, então o Folders não o enviou

# Reason a pipe, device or socket was not sent to a server.
folders-far-not-sent = não é um arquivo nem uma pasta, então o Folders não o enviou

# Reason a thing the server names and says nothing more about was left.
folders-far-not-told = o Folders não conseguiu saber que tipo de item é, então o deixou lá

# Note on the Connect to a server card for a kind of server that cannot refuse a name that is already taken.
folders-far-ftp-cannot-refuse = Neste tipo de servidor, um arquivo pode substituir outro com o mesmo nome.

# Failure when something other than copying, moving or deleting is asked of a server's files.
folders-far-cannot-do = Você não pode fazer isso com arquivos em um servidor

# Reason a file in a folder could not be looked at, in the list of what was left.
folders-far-not-looked-at = o Folders não conseguiu ler

# Failure when a file or folder to be copied could not be opened.
folders-far-not-opened = O Folders não conseguiu abri-lo.

# Reason after the name of a folder that is copied or moved into itself.
folders-far-inside-itself = { $name } (uma pasta não pode ficar dentro de si mesma)

# Reason after a name that is to replace a file but a folder of that name is there.
folders-far-taken-by-folder = { $name } (já existe uma pasta com esse nome)

# Reason after a name that is to replace a folder but a file of that name is there.
folders-far-taken-by-file = { $name } (já existe um arquivo com esse nome)

# Failure when the address of a place on a server is not known.
folders-far-address-unknown = O Folders não conhece o endereço do servidor.

# Failure when a file saved back to a server arrived in a different length.
folders-far-copy-wrong-length = a cópia que foi enviada tinha o tamanho errado

# Failure when a rename is asked of something that is not a file on a server.
folders-far-only-server-file = Isso só funciona com arquivos em um servidor.

# Failure when a replacement could not be put back: where the old and the new one lie, and why.
folders-far-aside-both = O Folders não conseguiu colocar o arquivo novo no lugar, então o antigo continua lá, agora chamado { $old }, e o novo se chama { $new }: { $why }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies.
folders-far-aside-kept = O Folders não conseguiu remover o arquivo antigo. Ele continua lá, agora chamado { $old }

# Failure when the old file could not be removed after its replacement arrived; where the old one lies, and why.
folders-far-aside-kept-why = O Folders não conseguiu remover o arquivo antigo, então ele continua lá, agora chamado { $old }: { $why }

# Failure when a folder on a server is deleted but is not empty; the folder's name.
folders-far-not-empty = { $name } não está vazio.

# Red note when a name sent by a server is unsafe to use and the file is skipped.
folders-far-unsafe-name = O Folders ignorou um arquivo porque o nome dele não é seguro para usar.

# Red note when names sent by a server in a folder listing are unsafe; how many, and the names.
folders-far-unsafe-names =
    { $n ->
        [one] O Folders ignorou um item porque o nome dele não é seguro para usar: { $names }.
       *[other] O Folders ignorou { $n } itens porque os nomes deles não são seguros para usar: { $names }.
    }

# Note while a file from a server is being fetched to be opened; the file's name.
folders-far-fetching = Baixando { $name }…

# Failure when a file on a server changed while it was being fetched; the file's name and why.
folders-far-cannot-fetch = O Folders não conseguiu baixar { $name }: { $why }.

# Note when a copy of a server's file has been opened here; the file's name.
folders-far-opened-copy = Aberta uma cópia de { $name }. Ao salvá-la, suas alterações voltam para o servidor.

# Note when a changed copy has been sent back to the server; the file's name.
folders-far-saved = { $name } salvo no servidor.

# Red note when a changed copy could not be sent back to the server; the file's name and why.
folders-far-not-saved = O Folders não conseguiu salvar { $name } no servidor: { $why }. Suas alterações continuam na cópia deste computador.

# Note when a thing on a server was renamed; its old and new names.
folders-far-renamed = { $was } renomeado para { $now }.

# Heading of the Connect to a server card.
folders-far-connect-title = Conectar a um servidor

# Tip and name of the round button that closes the Connect to a server card.
folders-far-connect-close = Fechar

# Placeholder of the box on the Connect to a server card where the server's address is typed.
folders-far-connect-address = Endereço, como sftp://jo@example.com

# Placeholder of the box on the Connect to a server card where a password is typed, for servers that take one.
# SAFETY: needs a human review before it ships
folders-far-connect-password = Senha, se necessária

# Placeholder of the box on the Connect to a server card where the key file is named.
folders-far-connect-key = Arquivo de chave, se você usar um

# Tip of the button beside the key file box that opens the file chooser, and title of that chooser.
folders-far-connect-pick-key = Escolher o arquivo de chave


# Note on the Connect to a server card saying where the connection settings come from.
folders-far-connect-ssh-setup = O Folders usa suas configurações do ssh, o agente ssh, as chaves e a lista de servidores conhecidos.

# Note on the Connect to a server card for a server not visited before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-new-host = Você nunca se conectou a { $host } antes, então o Folders não consegue verificar se é o servidor certo. Só confie nele se você esperava se conectar a ele.

# Red note on the Connect to a server card when a known server has a different identity; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-host-changed = { $host } tem uma identidade diferente da última vez. Alguém pode estar tentando interceptar sua conexão, por isso o Folders não se conectou. Se o dono a mudou, remova a entrada antiga dos seus servidores conhecidos primeiro.

# Note on the Connect to a server card when a server of the kind that takes a password asks for one.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-password = Este servidor precisa de uma senha. Digite-a acima e tente de novo.

# Note on the Connect to a server card when a server asks for a key or password.
# SAFETY: needs a human review before it ships
folders-far-connect-needs-key = Este servidor precisa de uma chave ou de uma senha. Escolha o arquivo da sua chave acima, ou adicione-a ao seu agente ssh, e tente de novo.

# Note on the Connect to a server card when a server's certificate cannot be checked; the server's name and why.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-new = O Folders não conseguiu verificar o certificado de { $host }: { $why }. Se puder, compare a impressão digital dele com a que o dono informa. Só confie nele se forem iguais.

# Note on the Connect to a server card when a server's certificate is not the one trusted before; the server's name.
# SAFETY: needs a human review before it ships
folders-far-connect-cert-changed = O certificado de { $host } mudou desde que você confiou nele. O dono pode tê-lo renovado, ou alguém pode estar tentando interceptar sua conexão. Só confie nele se o dono confirmar esta impressão digital.

# Words beside the turning ring on the Connect to a server card while it connects.
folders-far-connect-connecting = Conectando

# Name and tip of the main button of the Connect to a server card when the server's identity must be trusted first.
# SAFETY: needs a human review before it ships
folders-far-connect-trust = Confiar e conectar

# Name and tip of the main button of the Connect to a server card.
folders-far-connect-go = Conectar

# Red note on the Connect to a server card when a phone's address is typed.
folders-far-phones = Os telefones aparecem na barra lateral quando você os conecta e desbloqueia.

