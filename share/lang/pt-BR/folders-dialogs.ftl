
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Tip and spoken name of the round button at the top right of a small window that shuts it.
folders-dialogs-close = Fechar

# Name of the person's Home folder as a place to send things to.
folders-dialogs-home = Início

# Heading of the Open with window; the name of the file being opened.
folders-dialogs-open-with-title = Abrir { $name } com

# Small line beside the app that opens the file now, in the Open with list.
folders-dialogs-opens-now = Aplicativo atual

# Note in the Open with window when no app says it opens this kind of file.
folders-dialogs-no-apps = Nenhum aplicativo neste computador está configurado para abrir este tipo de arquivo.

# Name of the switch in the Open with window that makes the chosen app the usual one for this kind of file.
folders-dialogs-always-use = Sempre usar este aplicativo

# Line under the Always use it switch; the kind of file, such as text/plain.
folders-dialogs-always-sub = Use este aplicativo para todo arquivo { $kind }.

# Tip and spoken name of the Always use it switch.
folders-dialogs-always-tip = Definir como aplicativo padrão

# Tip of the round button in the Open with window that shows only the apps that suit the file.
folders-dialogs-show-suggested = Mostrar só aplicativos sugeridos

# Tip of the round button in the Open with window that shows every app.
folders-dialogs-show-every = Mostrar todos os aplicativos

# Main button of the Open with window that opens the file with the chosen app.
folders-dialogs-open = Abrir

# Red words when Open is pressed in the Open with window and no app is chosen.
folders-dialogs-choose-app = Escolha um aplicativo primeiro.

# Red words when the chosen app could not be made the usual app; the app's name.
folders-dialogs-usual-failed = O Folders não conseguiu tornar { $name } o aplicativo padrão.

# Red words when the chosen app could not be started; the app's name.
folders-dialogs-start-failed = O Folders não conseguiu iniciar { $name }.

# Heading of the Properties window.
folders-dialogs-properties = Propriedades

# Spoken name of the choice of pages at the top of the Properties window.
folders-dialogs-properties-pages = Páginas de Propriedades

# Name of the first page of Properties, with the name, kind, size and place.
folders-dialogs-page-general = Geral

# Name of the page of Properties with a song's, a photo's or a camera's notes.
folders-dialogs-page-details = Detalhes

# Name of the page of Properties for sharing a folder on the home network.
folders-dialogs-page-sharing = Compartilhamento



# Label in Properties of the name of the file or folder.
folders-dialogs-row-name = Nome

# Label in Properties of the folder the item is in.
folders-dialogs-row-where = Onde

# Label in Properties of the kind of file.
folders-dialogs-row-kind = Tipo

# Label in Properties of where a link points.
folders-dialogs-row-link-to = Atalho para

# Label in Properties of when the item was last changed.
folders-dialogs-row-changed = Alterado

# Label in Properties of who owns the item.
folders-dialogs-row-owner = Dono

# Label in Properties of who may open and change the item.
folders-dialogs-row-rights = Permissões

# Label in Properties of the size of the item.
folders-dialogs-row-size = Tamanho

# Kind shown in Properties when the kind of file is not known.
folders-dialogs-kind-unknown = Desconhecido

# Kind shown in Properties for a file: the kind in words, then its technical name in brackets.
folders-dialogs-kind-with-name = { $words } ({ $kind })

# Rights shown in Properties: the letters such as rw-r--r--, then what the person can do; when the person can open and change it.
folders-dialogs-rights-open-change = { $letters }, você pode abrir e alterar

# Rights shown in Properties: the letters such as r--r--r--, then what the person can do; when the person can open it but not change it.
folders-dialogs-rights-open-only = { $letters }, você pode abrir, mas não alterar

# Rights shown in Properties: the letters, then what the person can do; when the person can change it but not open it.
folders-dialogs-rights-change-only = { $letters }, você pode alterar, mas não abrir

# Rights shown in Properties: the letters such as rwx------, then what the person can do; when the person cannot open it.
folders-dialogs-rights-none = { $letters }, você não pode abrir

# Size shown in Properties: the size so far or in all, then for a folder or several items how many are inside, then whether some could not be read, then whether it is still adding up. The variables counted, unread and adding are yes or no; items is the finished count such as 3 items.
folders-dialogs-size =
    { $size }{ $counted ->
        [yes] , { $items }
       *[other]
    }{ $unread ->
        [yes] , alguns não puderam ser lidos
       *[other]
    }{ $adding ->
        [yes] , ainda somando
       *[other]
    }

# Size shown in Properties before the first count comes in.
folders-dialogs-size-adding = Calculando o tamanho

# Label in Properties of the row about checksums.
folders-dialogs-checksums = Somas de verificação

# Tip of the round button in Properties that works out a file's checksums.
folders-dialogs-sums-work-out = Calcular somas de verificação

# Words beside the ring while a file's checksums are being worked out.
folders-dialogs-sums-working = Calculando as somas de verificação

# Red words in Properties when a file's checksums could not be worked out.
folders-dialogs-sums-failed = O Folders não conseguiu ler o arquivo.

# Placeholder of the box in Properties where a checksum is pasted to check the file against.
folders-dialogs-sums-paste = Cole uma soma de verificação para comparar

# Words beside the checksum box when the pasted checksum matches; the name of the checksum such as SHA-256.
folders-dialogs-sums-matches = { $kind } confere

# Red words beside the checksum box when the pasted checksum matches none.
folders-dialogs-sums-no-match = Não confere

# Label in Properties of the row with the box to check a checksum.
folders-dialogs-sums-check = Conferir


# Heading of the Send to window: the name of the one item being sent, or how many items.
folders-dialogs-send-title =
    { $n ->
        [one] Enviar { $name } para
       *[other] Enviar { $n } itens para
        [many] Enviar { $n } itens para
    }

# Note in the Send to window saying what sending does.
folders-dialogs-send-note = Isso cria uma cópia lá. Os originais ficam onde estão.

# Name of the place in the Send to list that is the folder open in the other pane; the folder's name.
folders-dialogs-send-other = O outro painel, { $name }

# Heading of the window that chooses the columns of the List view.
folders-dialogs-columns-title = Colunas da visualização em lista

# Note in the Columns window.
folders-dialogs-columns-note = O nome sempre aparece. As colunas que não cabem ficam ocultas até o painel ficar mais largo.

# Heading over the columns that suit every file.
folders-dialogs-columns-files = Arquivos

# Heading over the columns that suit music, photos and PDFs.
folders-dialogs-columns-media = Músicas, fotos, PDFs

# Tip and spoken name of a column's switch in the Columns window; the column's name.
folders-dialogs-columns-show = Mostrar { $name }

# Red words when the choice of columns could not be saved.
folders-dialogs-columns-failed = O Folders não conseguiu salvar as colunas.

# Heading of the Folder colour window: the name of the one folder, or how many folders.
folders-dialogs-colour-title =
    { $n ->
        [one] Cor de { $name }
       *[other] Cor de { $n } pastas
        [many] Cor de { $n } pastas
    }

# Tip of the round button, and the note beside it, in the Folder colour window that takes the colour away.
folders-dialogs-colour-none = Sem cor

# Red words when the folder colour could not be saved.
folders-dialogs-colour-failed = O Folders não conseguiu salvar a cor da pasta.


folders-dialogs-no-match = Nenhum aplicativo corresponde a essas palavras.


folders-dialogs-find-app = Procurar um aplicativo


folders-dialogs-find-place = Procurar um lugar


folders-dialogs-no-match-place = Nenhum lugar corresponde a essas palavras.
