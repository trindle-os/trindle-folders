# Words Folders carries that Trindle also uses elsewhere: a refused password, the keyring, a battery charge, the More button and the password window.

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-cancelled = você não informou a senha de administrador
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-wrong = a senha de administrador não foi aceita
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-not-allowed = sua conta não tem permissão para fazer isso, nem com a senha de administrador
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unasked = não havia como pedir a senha de administrador a você
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
# SAFETY: needs a human review before it ships
common-refused-why-unclear = a senha de administrador não foi aceita, ou sua conta não tem permissão para fazer isso
# Sentence of its own saying a step was refused: the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-sentence-cancelled = Você não informou a senha de administrador
# Sentence of its own saying a step was refused: the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-sentence-wrong = A senha de administrador não foi aceita
# Sentence of its own saying a step was refused: this login may never do it.
# SAFETY: needs a human review before it ships
common-refused-sentence-not-allowed = Sua conta não tem permissão para fazer isso, nem com a senha de administrador
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
# SAFETY: needs a human review before it ships
common-refused-sentence-unasked = Não havia como pedir a senha de administrador a você
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
# SAFETY: needs a human review before it ships
common-refused-sentence-unclear = A senha de administrador não foi aceita, ou sua conta não tem permissão para fazer isso
# Red words when a step was refused because the password box was closed.
# SAFETY: needs a human review before it ships
common-refused-not-done-cancelled = Isso não foi feito porque você não informou a senha de administrador.
# Red words when a step was refused because the password was not accepted.
# SAFETY: needs a human review before it ships
common-refused-not-done-wrong = Isso não foi feito porque a senha de administrador não foi aceita.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-not-allowed = Isso não foi feito. Sua conta não tem permissão para fazer isso, nem com a senha de administrador. Saia da sessão, entre de novo e tente outra vez.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unasked = Isso não foi feito. Não havia como pedir a senha de administrador a você. Saia da sessão, entre de novo e tente outra vez.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
# SAFETY: needs a human review before it ships
common-refused-not-done-unclear = Isso não foi feito. A senha de administrador não foi aceita, ou sua conta não tem permissão para fazer isso. Se não pediram uma senha a você, saia da sessão, entre de novo e tente outra vez.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = Isso não funcionou, e nenhum motivo foi informado.
# Red words when the keyring will not open without the person's password.
# SAFETY: needs a human review before it ships
common-keyring-locked = O chaveiro está bloqueado. Desbloqueie-o, ou saia da sessão e entre de novo, e tente outra vez.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = Não há chaveiro neste computador para salvá-lo.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = O Folders não conseguiu alcançar o chaveiro a tempo.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = O Folders desistiu de esperar pelo chaveiro.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = O Folders não conseguiu acesso ao chaveiro ({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] O Folders ainda está esperando o chaveiro, então o(a) { $what } ainda não foi removido(a).
       *[other] O Folders ainda está esperando o chaveiro, então o(a) { $what } ainda não foi salvo(a).
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] Após uma demora, o(a) { $what } foi removido(a) do chaveiro.
       *[other] Após uma demora, o(a) { $what } foi salvo(a) no chaveiro.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] O Folders desistiu de esperar pelo chaveiro, então o(a) { $what } não foi removido(a).
       *[other] O Folders desistiu de esperar pelo chaveiro, então o(a) { $what } não foi salvo(a).
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] O Folders não conseguiu remover o(a) { $what } do chaveiro. { $why }
       *[other] O Folders não conseguiu salvar o(a) { $what } no chaveiro. { $why }
    }
# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = Mais
common-more-spoken = Abrir a Ajuda sobre isto
# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
folders-volumes-no-password = Você não informou uma senha, então nada foi alterado.
# Red words for a refusal. { $reason } says why, without a full stop.
folders-volumes-nothing-changed = { $reason }, então nada foi alterado.
# Red words when a drive is in use and cannot be unmounted.
folders-volumes-busy = Algo ainda a está usando. Feche as janelas ou programas que mostram os arquivos dela e tente de novo.
# Red words when there is not enough room for what is on a part at the size asked for.
folders-volumes-no-room = Não há espaço suficiente para o que há nela nesse tamanho.
# SAFETY: needs a human review before it ships
# Heading over the password box, and the tip shown in the box while it is empty.
folders-prompt-password = Senha
# SAFETY: needs a human review before it ships
# Main button at the bottom of the password window that checks the password.
folders-prompt-unlock = Desbloquear
# SAFETY: needs a human review before it ships
# Button at the bottom of the password window that closes it without unlocking.
folders-prompt-cancel = Cancelar
# SAFETY: needs a human review before it ships
# Shown under the password box when Unlock is pressed with the box empty.
folders-prompt-type-first = Digite a senha primeiro.
# Shown under the password box while the password is being checked.
folders-prompt-checking = Verificando…
# A reason a wallpaper was left out: the picture is larger than 4K.
folders-wallpaper-larger-4k = maior que 4K
# A reason a wallpaper was left out: the film is bigger than the screen.
folders-wallpaper-larger-screen = maior que a sua tela
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
folders-wallpaper-over-20-or-200 = acima de 20 MB (200 MB se animado)
# A reason a wallpaper was left out: its file is over 20 MB.
folders-wallpaper-over-20 = acima de 20 MB
# A reason a wallpaper was left out: its file is over 200 MB.
folders-wallpaper-over-200 = acima de 200 MB
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
folders-wallpaper-reasons-or = { $first } ou { $second }

# Row in the sidebar for a drive that is locked; the drive's name, so a narrow row still says it is locked.
folders-volumes-locked-row = Bloqueada: { $name }

# Name given to a drive that has none.
folders-volumes-drive = Unidade

# Red words under the password box when the passphrase did not open an encrypted drive.
folders-volumes-wrong-passphrase = Essa frase secreta não desbloqueou a unidade.

# Said when a drive that is open already is opened again.
folders-volumes-already-open = Ela já está aberta.

# Said when a locked drive is opened without a passphrase.
folders-volumes-needs-passphrase = Você precisa de uma frase secreta para abri-la.

# Line in the passphrase window saying which drive it is for; the drive's name.
folders-volumes-passphrase-for = Frase secreta de { $name }

# Title of the passphrase window for a locked drive.
folders-volumes-unlock-title = Desbloquear unidade

# Said when the passphrase window could not be opened.
folders-volumes-prompt-failed = O Folders não conseguiu abrir a janela da frase secreta.
