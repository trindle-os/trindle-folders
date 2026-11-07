

folders-sharing-cannot-read = Esta pasta não pode ser lida.

folders-sharing-only-folder = Só uma pasta em si pode ser compartilhada, não um atalho para uma.

folders-sharing-only-yours = Só pastas que pertencem a você podem ser compartilhadas. Esta pertence a outra pessoa.

folders-sharing-your-home = sua pasta pessoal

folders-sharing-this-folder = esta pasta

folders-sharing-and = { $first } e { $second }

folders-sharing-share = Compartilhar esta pasta

folders-sharing-shared = Compartilhada na rede doméstica.

folders-sharing-not-shared = Não compartilhada.

folders-sharing-share-tip = Compartilhar esta pasta com outros computadores da rede doméstica

folders-sharing-name = Nome

folders-sharing-name-tip = O nome que os outros computadores veem

folders-sharing-write = Os outros podem alterar arquivos

folders-sharing-write-sub = Eles podem adicionar, alterar e excluir arquivos nela.

folders-sharing-guests = Deixar entrar qualquer pessoa

folders-sharing-guests-sub = Nenhuma senha é pedida.

folders-sharing-guests-tip = Deixar entrar qualquer pessoa, sem senha

folders-sharing-explain =
    { $fetch ->
        [yes] Compartilhada, ela aparece em Rede nos outros computadores da casa, inclusive nos Windows. Na primeira vez, o Trindle baixa o Samba, o programa que compartilha pastas, o que leva um minuto.
       *[no] Compartilhada, ela aparece em Rede nos outros computadores da casa, inclusive nos Windows.
    }

folders-sharing-find-it =
    { $host_known ->
        [yes] Os outros computadores a encontram em Rede, como { $name } em { $host }.
       *[no] Os outros computadores a encontram em Rede, como { $name }.
    }

folders-sharing-cannot-reach = Os outros computadores ainda não conseguem alcançá-la: Deixar outros computadores se conectarem está desligado em Configurações, Rede.

folders-sharing-let-connect = Deixar outros computadores se conectarem

folders-sharing-visitors =
    { $n ->
        [one] Visitantes sem senha ainda não conseguem abri-la: { $names } está fechada para outras contas. Deixá-los passar abre só o caminho até esta pasta; eles não veem o que mais há ali.
       *[other] Visitantes sem senha ainda não conseguem abri-la: { $names } estão fechadas para outras contas. Deixá-los passar abre só o caminho até esta pasta; eles não veem o que mais há ali.
        [many] Visitantes sem senha ainda não conseguem abri-la: { $names } estão fechadas para outras contas. Deixá-los passar abre só o caminho até esta pasta; eles não veem o que mais há ali.
    }

folders-sharing-let-through = Deixar os visitantes chegarem a esta pasta

folders-sharing-sign-in-set = Os outros entram como { $user }, com a senha definida para o compartilhamento.

folders-sharing-change-password = Mudar a senha que os outros computadores usam

folders-sharing-set-password = Os outros entram como { $user }. Defina a senha que os outros computadores usam para abrir as suas pastas compartilhadas, com pelo menos { $n } letras. Ela é guardada separada da senha do seu login.

folders-sharing-password = Senha

folders-sharing-password-tip = Senha de compartilhamento

folders-sharing-keep-password = Guardar esta senha

folders-sharing-card-title = Compartilhar { $name } na rede

folders-sharing-name-taken = Outra das suas pastas compartilhadas já tem esse nome. Escolha outro.

folders-sharing-stopping = Parando o compartilhamento…

folders-sharing-password-short = A senha precisa ter pelo menos { $n } letras.

folders-sharing-keeping-password = Guardando a senha; a senha do administrador é pedida primeiro.

folders-sharing-letting-connect = Deixando os outros computadores se conectarem; a senha do administrador é pedida primeiro.

folders-sharing-fetching = Baixando o Samba, o programa que compartilha pastas, e configurando. Isso acontece só na primeira vez, e a senha do administrador é pedida primeiro.

folders-sharing-setting-up = Configurando o compartilhamento. Isso acontece só na primeira vez, e a senha do administrador é pedida primeiro.

folders-sharing-changing = Alterando o compartilhamento…

folders-sharing-sharing-it = Compartilhando…

