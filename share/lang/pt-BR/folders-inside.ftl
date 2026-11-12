
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = Selecione algo no arquivo compactado primeiro.

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = Você só pode extrair itens de um arquivo compactado para uma pasta deste computador.

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = O Folders não conseguiu ler { $name }: { $why }.

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
        [one] O Folders não conseguiu extrair 1 item, porque o nome dele aponta para fora da pasta ou há algo no caminho.
       *[other] O Folders não conseguiu extrair { $n } itens, porque os nomes deles apontam para fora da pasta ou há algo no caminho.
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = O Folders não conseguiu extraí-lo do arquivo compactado: { $why }.

