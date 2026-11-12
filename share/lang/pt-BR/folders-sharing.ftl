
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red words when a folder cannot be shared because it cannot be read.
folders-sharing-cannot-read = O Folders não consegue ler o que há nesta pasta.

# Red words when something that is not a folder, such as a link, is chosen to share.
folders-sharing-only-folder = Você só pode compartilhar uma pasta em si, não um atalho para uma.

# Red words when a folder that belongs to someone else is chosen to share.
folders-sharing-only-yours = Você só pode compartilhar pastas que pertencem a você. Esta pertence a outra pessoa.

# A folder in the list of folders closed to visitors: the home folder.
folders-sharing-your-home = sua pasta pessoal

# A folder in the list of folders closed to visitors: the folder being shared.
folders-sharing-this-folder = esta pasta

# Joins two folders in a list of folders; { $first } is what is listed so far and { $second } the next one.
folders-sharing-and = { $first } e { $second }

# Name of the switch that shares the folder.
# SAFETY: needs a human review before it ships
folders-sharing-share = Compartilhar esta pasta

# One line under the share switch when the folder is shared.
# SAFETY: needs a human review before it ships
folders-sharing-shared = Compartilhada na rede doméstica.


# Tip of the share switch.
# SAFETY: needs a human review before it ships
folders-sharing-share-tip = Compartilhar na rede doméstica

# Label over the box for the name other computers see.
folders-sharing-name = Nome

# Tip of the box for the name other computers see.
folders-sharing-name-tip = O nome que os outros computadores veem

# Name of the switch that lets others change files, and its tip.
# SAFETY: needs a human review before it ships
folders-sharing-write = Deixar os outros alterarem arquivos

# One line under the switch that lets others change files.
# SAFETY: needs a human review before it ships
folders-sharing-write-sub = Eles podem adicionar, alterar e excluir arquivos nela.

# Name of the switch that lets anyone in without a password.
# SAFETY: needs a human review before it ships
folders-sharing-guests = Deixar qualquer pessoa entrar

# One line under the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-sub = Sem necessidade de senha.

# Tip of the let in anyone switch.
# SAFETY: needs a human review before it ships
folders-sharing-guests-tip = Deixar qualquer pessoa entrar sem senha

# Note under the sharing switches while the folder is not shared; { $fetch } is yes when Samba has to be fetched first, otherwise no.
folders-sharing-explain =
    { $fetch ->
        [yes] Depois de compartilhada, ela aparece em Rede nos seus outros computadores da casa, inclusive nos Windows. Na primeira vez, o Trindle baixa o Samba, o programa que compartilha pastas. Isso leva um minuto.
       *[no] Depois de compartilhada, ela aparece em Rede nos seus outros computadores da casa, inclusive nos Windows.
    }

# Note saying how other computers find the share; { $host_known } is yes when this computer's name is known, { $name } is the share's name and { $host } this computer's name.
folders-sharing-find-it =
    { $host_known ->
        [yes] Nos outros computadores, procure em Rede por { $name } em { $host }.
       *[no] Nos outros computadores, procure em Rede por { $name }.
    }

# Note when other computers are not allowed to connect yet.
folders-sharing-cannot-reach = Os outros computadores ainda não conseguem alcançar esta pasta. Em Configurações, abra Rede e ative "Deixar outros computadores se conectarem".

# Tip of the button that lets other computers connect.
# SAFETY: needs a human review before it ships
folders-sharing-let-connect = Deixar outros computadores se conectarem

# Note when folders are closed to visitors; { $names } lists them and { $n } is how many.
# SAFETY: needs a human review before it ships
folders-sharing-visitors =
    { $n ->
        [one] Convidados sem senha ainda não conseguem abri-la, porque { $names } está fechada para outras contas. Se você os deixar passar, eles chegam a esta pasta, mas não veem mais nada pelo caminho.
       *[other] Convidados sem senha ainda não conseguem abri-la, porque { $names } estão fechadas para outras contas. Se você os deixar passar, eles chegam a esta pasta, mas não veem mais nada pelo caminho.
    }

# Tip of the button that lets visitors through to the folder.
# SAFETY: needs a human review before it ships
folders-sharing-let-through = Deixar convidados chegarem a esta pasta

# Note saying others sign in with the password already set for sharing; { $user } is the person's user name.
# SAFETY: needs a human review before it ships
folders-sharing-sign-in-set = Os outros entram como { $user } com a senha de compartilhamento.

# Tip of the button that changes the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-change-password = Mudar a senha de compartilhamento

# Note over the sharing password box; { $user } is the person's user name and { $n } the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-set-password = Os outros entram como { $user }. Defina uma senha de compartilhamento com pelo menos { $n } caracteres. Ela é separada da senha do seu login.

# Label beside the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password = Senha

# Tip of the sharing password box.
# SAFETY: needs a human review before it ships
folders-sharing-password-tip = Senha de compartilhamento

# Tip of the button that keeps the sharing password.
# SAFETY: needs a human review before it ships
folders-sharing-keep-password = Salvar senha

# Title of the Share on the network window; { $name } is the folder's name.
# SAFETY: needs a human review before it ships
folders-sharing-card-title = Compartilhar { $name } na rede

# Red words when the share's name is used by another share.
folders-sharing-name-taken = Você já tem uma pasta compartilhada com esse nome. Escolha outro.

# Calm words while sharing is being stopped.
folders-sharing-stopping = Parando o compartilhamento…

# Red words when the sharing password is too short; { $n } is the fewest letters.
# SAFETY: needs a human review before it ships
folders-sharing-password-short = A senha precisa ter pelo menos { $n } caracteres.

# Calm words while the sharing password is being kept.
# SAFETY: needs a human review before it ships
folders-sharing-keeping-password = Salvando a senha. A senha do administrador será pedida primeiro.

# Calm words while other computers are being let connect.
# SAFETY: needs a human review before it ships
folders-sharing-letting-connect = Deixando os outros computadores se conectarem. A senha do administrador será pedida primeiro.

# Calm words while Samba is fetched and set up the first time.
# SAFETY: needs a human review before it ships
folders-sharing-fetching = Baixando e configurando o Samba, o programa que compartilha pastas. A senha do administrador será pedida primeiro. Isso só acontece na primeira vez.

# Calm words while sharing is set up the first time and Samba is already there.
# SAFETY: needs a human review before it ships
folders-sharing-setting-up = Configurando o compartilhamento. A senha do administrador será pedida primeiro. Isso só acontece na primeira vez.

# Calm words while an existing share is changed.
folders-sharing-changing = Alterando o compartilhamento…

# Calm words while a share is made.
folders-sharing-sharing-it = Compartilhando a pasta…

# Red words under the name box when a share has no name.
folders-sharing-name-empty = Dê um nome à pasta compartilhada.

# Red words under the name box when a share's name is too long; the most letters allowed.
folders-sharing-name-long = O nome de uma pasta compartilhada não pode ter mais de { $most } caracteres.

# Red words when a share's name holds a letter that is not allowed.
folders-sharing-name-bad-letters = O nome de uma pasta compartilhada não pode conter nenhum destes: % < > * ? | / \ + = ; : " ou vírgula.

# Red words when a share's name ends in a dollar sign, which would hide it.
folders-sharing-name-dollar = Um nome terminado em $ esconderia o compartilhamento, então você não pode usá-lo.

# Red words when a share's name starts with a dash.
folders-sharing-name-dash = O nome de uma pasta compartilhada não pode começar com hífen.

# Red words when a share's name is one Samba keeps for itself; the name.
folders-sharing-name-reserved = O Samba já usa o nome { $name }. Escolha outro.

# Name offered for a share when the folder's own name cannot be used.
folders-sharing-default-name = Compartilhada

# Red words when Samba says only the person's own folders can be shared.
folders-sharing-only-yours-short = Você só pode compartilhar pastas que pertencem a você.

# Red words when a share's name is someone's login on this computer.
folders-sharing-name-is-login = Esse nome já é o nome de uma conta neste computador, então você não pode usá-lo em um compartilhamento. Escolha outro.

# Red words when this computer's Samba settings do not allow shares without a password.
folders-sharing-no-guests-allowed = As configurações do Samba deste computador não permitem compartilhamentos sem senha.

# Red words when the most folders Samba allows are already shared.
folders-sharing-too-many = Este computador não pode compartilhar mais pastas.

# Red words when the folder to share has gone.
folders-sharing-folder-gone = Essa pasta não está mais lá.

# Red words when Samba failed without saying why.
folders-sharing-samba-silent = O Samba não informou o motivo.

# Red words repeating what Samba said when it failed; Samba's last line.
folders-sharing-samba-said = Mensagem do Samba: { $said }

# Red words when a folder that must be opened to guests belongs to someone else; its path.
folders-sharing-guests-not-yours = { $path } pertence a outra pessoa, então daqui não dá para liberar o acesso de convidados a ela.

# Red words when a folder's permissions could not be changed to let guests in; its path, then why.
folders-sharing-guests-not-changed = O Folders não conseguiu liberar o acesso de convidados a { $path }: { $why }

# Red words when setting up sharing cannot fetch Samba because the computer is offline.
folders-sharing-setup-offline = O Trindle não conseguiu baixar o Samba porque este computador parece estar sem internet. Conecte-se à internet e tente de novo.

# Red words when setting up sharing cannot go on because another install is running.
folders-sharing-setup-busy = O Trindle não conseguiu instalar o Samba enquanto outra instalação está em andamento. Tente de novo em alguns minutos.
