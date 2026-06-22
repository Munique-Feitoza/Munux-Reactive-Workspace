# Munux TUI - Português (Brasil)
# Localização Fluent

# UI Labels
ui-welcome-title = Bem-vindo ao Munux TUI
ui-stats-title = Estatísticas
ui-quests-title = Missões Ativas
ui-danger-zone-title = Zona de Perigo
ui-help-title = Ajuda
ui-terminal-prompt = digite um comando...
ui-total-commands = Comandos Totais
ui-successful-commands = Bem-sucedidos
ui-failed-commands = Falhas
ui-success-rate = Taxa de Acerto
ui-current-streak = Streak Atual
ui-integrity = Integridade
ui-achievements = Conquistas
ui-last-unlocked = Últimas desbloqueadas
ui-active-quests = Missões Ativas
ui-level = Nível
ui-rank = Patente
ui-no-achievements = Nenhuma conquista ainda. Execute comandos para desbloquear!
ui-all-quests-done = Todas as missões concluídas!
ui-new-quests-level = Novas missões serão desbloqueadas ao subir de nível.
ui-navigation = Navegação
ui-err-read-dir = Erro ao ler diretório
ui-empty-file = Arquivo vazio
ui-err-read-file = Erro ao ler arquivo (muito grande ou sem permissão)
ui-processes = Processos
ui-memory = Memória
ui-examples = Exemplos
ui-tip = Dica
ui-warning = AVISO
ui-terminal-active = Terminal Ativo
ui-stats = Estatísticas
ui-streak = Streak Atual
ui-streak-commands = comandos
ui-recent-achievements = Conquistas Recentes
ui-achievement-unlocked = CONQUISTA DESBLOQUEADA!
ui-achievement-reward = Recompensa: { $xp } XP
ui-terminal-title = Terminal Educacional Reativo
ui-reactive-desc = O painel da direita muda conforme
ui-reactive-desc-2 = você digita comandos!
ui-start-commands = Comandos para começar

ui-esc-to-back = ESC para voltar
ui-scroll-hint = Use as Setas para Scroll
ui-preview-title = Preview: { $filename }
ui-resource-title = Monitor de Recursos
ui-cpu-usage = Uso de CPU
ui-level-up-title = Novo Nível!

# Game States
game-rank-novice = Novato
game-rank-apprentice = Aprendiz
game-rank-hacker = Hacker
game-rank-elite = Elite
game-rank-legend = Lenda

# Quests
quest-explorer-title = Explorador
quest-explorer-desc = Liste os arquivos do diretório atual
quest-location-title = Localização
quest-location-desc = Veja em qual diretório você está
quest-architect-title = Arquiteto
quest-architect-desc = Crie uma pasta chamada 'munux'
quest-reader-title = Leitor
quest-reader-desc = Leia o arquivo README
quest-cleaner-title = Faxineiro
quest-cleaner-desc = Remova um arquivo temporário
quest-focus-title = Foco
quest-focus-desc = Execute 10 comandos

# Quest Progress
quest-progress-run = Execute '{ $command }' { $current }/{ $count } vezes
quest-progress-created = ✓ { $item } '{ $name }' criado(a)
quest-progress-create = Crie { $item } chamado(a) '{ $name }'
quest-progress-navigated = ✓ Navegou para { $path }
quest-progress-navigate = Vá para o diretório { $path }
quest-progress-read = ✓ Conteúdo de '{ $name }' lido
quest-progress-read-action = Leia o arquivo '{ $name }' usando cat
quest-progress-deleted = ✓ Arquivo '{ $name }' removido
quest-progress-delete-action = Remova o arquivo '{ $name }'
quest-progress-reach-level = Alcance o nível { $level }
quest-progress-any-command = Execute qualquer comando: { $current }/{ $count }
quest-progress-grep = Use grep { $current }/{ $count } vezes
quest-progress-git-done = ✓ Git utilizado
quest-progress-git-todo = Use qualquer comando git
quest-progress-ssh-done = ✓ SSH utilizado
quest-progress-ssh-todo = Use o comando ssh
quest-progress-pkg-done = ✓ Gerenciador de pacotes utilizado
quest-progress-pkg-todo = Use o gerenciador de pacotes
quest-progress-pipe-done = ✓ Pipe (|) utilizado
quest-progress-pipe-todo = Use um pipe (|) para ligar comandos
quest-progress-symlink-done = ✓ Link simbólico criado
quest-progress-symlink-todo = Crie um link simbólico com ln -s
quest-progress-editor-done = ✓ { $editor } utilizado
quest-progress-editor-todo = Use o editor { $editor }
quest-progress-systemctl-done = ✓ systemctl utilizado
quest-progress-systemctl-todo = Use o comando systemctl
quest-progress-script-done = ✓ Script executado
quest-progress-script-todo = Escreva e execute um script .sh

sys-destructive-detected = Comando potencialmente destrutivo detectado!
sys-leveled-up = PARABÉNS! Você alcançou o nível { $level }!
sys-integrity-warning = INTEGRIDADE CRITICAMENTE BAIXA!
sys-danger-detected = COMANDO DESTRUTIVO DETECTADO
sys-consequences = CONSEQUÊNCIAS POSSÍVEIS
sys-access-denied-title = ⚠️ Acesso Negado
sys-access-denied-body = Seu nível atual não permite este comando. Suba de nível para desbloquear o sistema!
sys-access-denied = ✗ Acesso Negado: Modo de Segurança Ativo! Este comando é restrito em níveis baixos.
sys-level-up-msg = Parabéns! Você alcançou o nível { $level }!
sys-welcome-body = Um terminal reativo e gamificado para aprender comandos Linux.

game-msg-level-1 = Bem-vindo ao terminal!
game-msg-level-2 = Você está aprendendo!
game-msg-level-5 = Modo de segurança DESATIVADO!
game-msg-level-6 = Comandos perigosos liberados.
game-msg-level-10 = Você é um hacker agora!
game-msg-level-11 = O sistema é seu playground.
game-msg-level-20 = Entrando no Cyberpunk...
game-msg-level-21 = Você domina o terminal.
game-msg-level-30 = Modo Elite ATIVADO!
game-msg-level-31 = Poucos chegam aqui...
game-msg-level-50 = VOCÊ É UMA LENDA!
game-msg-level-default = All your base are belong to us.

game-art-terminal-tag = [MODO TERMINAL]
game-art-hacker-tag = [MODO HACKER]
game-art-cyberpunk-tag = [CYBERPUNK]
game-art-elite-tag = [HACKER ELITE]
game-art-legend-tag = [LENDA]

# Achievements
achievement-first_command-name = Primeiro Contato
achievement-first_command-desc = Execute seu primeiro comando
achievement-first_ls-name = Olho do Tigre
achievement-first_ls-desc = Use 'ls' pela primeira vez
achievement-first_cd-name = Viajante
achievement-first_cd-desc = Navegue entre diretórios
achievement-first_file-name = Criador
achievement-first_file-desc = Crie um arquivo com 'touch'
achievement-first_dir-name = Construtor
achievement-first_dir-desc = Crie um diretório com 'mkdir'
achievement-first_cat-name = Leitor
achievement-first_cat-desc = Veja o conteúdo de um arquivo com 'cat'
achievement-first_rm-name = Apagador
achievement-first_rm-desc = Delete um arquivo com 'rm'
achievement-first_sudo-name = Acesso Root
achievement-first_sudo-desc = Use 'sudo' pela primeira vez
achievement-first_pacman-name = Empacotador
achievement-first_pacman-desc = Use o gerenciador de pacotes
achievement-first_git-name = Controlador de Versão
achievement-first_git-desc = Use qualquer comando 'git'
achievement-first_systemctl-name = System Admin
achievement-first_systemctl-desc = Gerencie serviços com 'systemctl'
achievement-first_ssh-name = Acesso Remoto
achievement-first_ssh-desc = Use 'ssh' para se conectar
achievement-commands_10-name = Dedicado
achievement-commands_10-desc = Execute 10 comandos
achievement-commands_50-name = Profissional
achievement-commands_50-desc = Execute 50 comandos
achievement-commands_100-name = Mestre
achievement-commands_100-desc = Execute 100 comandos
achievement-commands_500-name = Lenda
achievement-commands_500-desc = Execute 500 comandos
achievement-pipe_master-name = Mestre dos Pipes
achievement-pipe_master-desc = Conecte comandos com '|'
achievement-streak_5-name = Pegando Fogo
achievement-streak_5-desc = 5 comandos sem erros
achievement-streak_10-name = Intocável
achievement-streak_10-desc = 10 comandos sem erros
achievement-streak_25-name = God Mode
achievement-streak_25-desc = 25 comandos sem erros
achievement-level_5-name = Aprendiz
achievement-level_5-desc = Alcance o nível 5
achievement-level_10-name = Hacker
achievement-level_10-desc = Alcance o nível 10
achievement-level_20-name = Hacker Pro
achievement-level_20-desc = Alcance o nível 20
achievement-level_30-name = Elite Cibernética
achievement-level_30-desc = Alcance o nível 30
achievement-level_50-name = Transcendente
achievement-level_50-desc = Alcance o nível 50
achievement-easter_egg_nuke-name = System Breaker
achievement-easter_egg_nuke-desc = Tentou apagar tudo...
achievement-easter_egg_train-name = Choocoo!
achievement-easter_egg_train-desc = Viu a locomotiva a vapor
achievement-easter_egg_cow-name = Vaca Tagarela
achievement-easter_egg_cow-desc = Fez a vaca falar com 'cowsay'
achievement-easter_egg_matrix-name = O Escolhido
achievement-easter_egg_matrix-desc = Seguiu o coelho branco com 'matrix'
achievement-easter_egg_konami-name = Old School Gamer
achievement-easter_egg_konami-desc = Inseriu o Konami Code
achievement-easter_egg_sandwich-name = Faça Você Mesmo
achievement-easter_egg_sandwich-desc = "sudo make me a sandwich" (xkcd #149)
achievement-easter_egg_42-name = Dom Sabe Tudo
achievement-easter_egg_42-desc = Descobriu a resposta para tudo
achievement-easter_egg_xyzzy-name = Aventureiro
achievement-easter_egg_xyzzy-desc = Disse a palavra mágica 'xyzzy'
achievement-easter_egg_cake-name = O Bolo é uma Mentira
achievement-easter_egg_cake-desc = Encontrou a referência de Portal
achievement-easter_egg_vim-name = :wq
achievement-easter_egg_vim-desc = Reflexo de quem já se perdeu no Vim
achievement-easter_egg_starwars-name = Que a Força Esteja com Você
achievement-easter_egg_starwars-desc = Invocou Star Wars no terminal
achievement-easter_egg_hunter-name = Caçador de Easter Eggs
achievement-easter_egg_hunter-desc = Encontrou 5 easter eggs secretos
sys-xp-gain = ✓ +{ $amount } XP | { $current }/{ $total } para o nível { $next }
sys-xp-usage = ✗ Uso: xp <quantidade>
ui-err-is-dir-hint = 💡 Use 'ls { $name }' para listar o conteúdo.

# Ajuda de Comandos
help-ls-desc = Lista arquivos e diretórios no diretório atual.
help-ls-hint = 💡 'ls' vem de 'list'. Use 'ls -R' para listar recursivamente subpastas!
help-cd-desc = Navega entre diretórios (Change Directory).
help-cd-hint = 💡 Use a tecla TAB para autocompletar nomes de pastas!
help-grep-desc = Busca textos dentro de arquivos ou outputs.
help-grep-hint = 💡 Global Regular Expression Print. Ferramenta poderosa para filtrar logs!
help-cat-desc = Mostra o conteúdo de um arquivo na tela.
help-cat-hint = 💡 Cuidado com arquivos binários! Use 'less' para arquivos grandes.
help-sudo-desc = Executa comandos com privilégios de superusuário (Root).
help-sudo-hint = ⚠️ Com grandes poderes vêm grandes responsabilidades. Use com cuidado!

    Use as teclas normais para digitar.
    Pressione Enter para executar.
    Pressione Ctrl+C para sair.

# Dicas de Comandos (Tela Inicial)
hint-ls = Listar conteúdo do diretório
hint-pwd = Mostrar diretório atual
hint-mkdir = Criar novo diretório
hint-cat = Mostrar conteúdo de arquivo
hint-rm = Remover arquivo
hint-cp = Copiar arquivo
hint-mv = Mover ou renomear arquivo
hint-ssh = Acesso remoto
hint-grep = Buscar texto
hint-systemctl = Gerenciar serviços

# Zona de Perigo (DangerZone) e confirmação
ui-attention-max = ATENÇÃO MÁXIMA
ui-command-detected = Comando detectado
ui-risk = Risco
ui-data-loss = Perda irreversível de dados
ui-unstable-system = Sistema pode ficar instável
ui-irreversible-damage = Dano impossível de desfazer
ui-available-actions = Ações disponíveis
ui-cancel-rec = Cancelar (recomendado)
ui-execute-anyway = Executar mesmo assim
ui-backup-tip = Faça backup antes de executar comandos destrutivos.
sys-danger-confirm = ⚠️ Comando perigoso. Digite 'sim' e Enter para confirmar, ou ESC para cancelar.
sys-danger-cancelled = ✓ Comando perigoso cancelado.

# Progressão de patente
ui-next-rank = 🎯 Próxima patente: { $rank } (nível { $level })
ui-max-rank = 🏆 Patente máxima alcançada!

# Alias
sys-alias-none = Nenhum alias definido. Use: alias nome='comando'
sys-alias-list-title = 📎 Aliases definidos:
sys-alias-removed = ✓ Alias '{ $name }' removido.
sys-alias-missing = ✗ Alias '{ $name }' não existe.
sys-alias-usage = Uso: alias nome='comando'
sys-alias-no-spaces = ✗ O nome do alias não pode conter espaços.
sys-alias-created = ✓ Alias criado: { $name } = { $value }

# Tutorial
sys-tutorial-ended = 🎓 Tutorial encerrado. Volte quando quiser: 'tutorial'.
sys-tutorial-none = Nenhum tutorial em andamento.
sys-tutorial-started = 🎓 Tutorial iniciado! Siga as instruções no quadro.
sys-tutorial-mode-title = 🎓 Modo Tutorial
sys-tutorial-step-done-title = ✅ Passo concluído!
sys-tutorial-complete-title = 🎉 Tutorial concluído!
sys-tutorial-complete-body = Parabéns! Você dominou o básico do Munux.{ "" }
    { "" }
    +{ $xp } XP de bônus!{ "" }
    { "" }
    Agora explore à vontade — use 'help' sempre que precisar.

# Benchmark
sys-bench-none = Nenhum benchmark em andamento.
sys-bench-cancelled = ⏱️ Benchmark cancelado.
sys-bench-result-title = ⏱️ Resultado do Benchmark
sys-bench-popup-title = ⏱️ Benchmark de Digitação
sys-bench-result = ⏱️ { $seconds }s  •  { $wpm } WPM  •  { $accuracy }% de precisão  •  +{ $xp } XP
sys-bench-start = ⏱️ BENCHMARK DE DIGITAÇÃO{ "" }
    { "" }
    Digite a frase abaixo e pressione Enter:{ "" }
    { "" }
      { $phrase }{ "" }
    { "" }
    ('benchmark sair' cancela)
sys-bench-popup-body = Digite exatamente esta frase e pressione Enter:{ "" }
    { "" }
    { $phrase }{ "" }
    { "" }
    O cronômetro já começou! ('benchmark sair' para cancelar)

# SSH
sys-error = ✗ Erro: { $msg }
sys-ssh-disconnected = 🔌 Desconectado do servidor remoto.
sys-ssh-cd-ok = ✓ Diretório remoto alterado para: { $dir }
sys-ssh-exec-error = ✗ Erro de execução remota: { $msg }
sys-ssh-connecting = 🔄 Conectando a { $user }@{ $host }...
sys-ssh-connected = ✓ Conectado a { $host } em { $dir }
sys-ssh-conn-title = Conexão Estabelecida
sys-ssh-conn-body = Conectado com sucesso a { $user }@{ $host }{ "" }
    { "" }
    Diretório: { $dir }
sys-ssh-fail = ✗ Falha na conexão: { $msg }
sys-ssh-fail-title = Erro de Conexão
sys-ssh-fail-body = Não foi possível conectar a { $target }:{ "" }
    { $msg }

# Comandos especiais / help
sys-showing-stats = ✓ Mostrando estatísticas
sys-showing-quests = ✓ Mostrando missões ativas
sys-tip-title = 💡 Dica do Dia
sys-tip-body = Use o comando 'help' para listar todos os comandos disponíveis.{ "" }
    { "" }
    Experimente 'stats' para ver seu progresso!
sys-tip-showing = Mostrando dica...
sys-help-cmd = 📚 Ajuda do comando: { $topic }
sys-help-showing-title = 📚 Mostrando: { $title } (Pressione ESC para voltar)
sys-help-showing = 📚 Mostrando ajuda (Pressione ESC para voltar)
help-system-title = Sistema de Ajuda Munux
help-system-body =
    📚 MUNUX HELP SYSTEM

    Use: help <distro>

    Distribuições suportadas:
      help arch     - Manjaro, Arch Linux (pacman, yay, paru)
      help debian   - Ubuntu, Debian, Mint (apt, dpkg, snap)
      help fedora   - Fedora, RHEL, CentOS (dnf, rpm)
      help opensuse - openSUSE (zypper)
      help linux    - Comandos universais Linux

    Comandos especiais Munux:
      stats         → Estatísticas e progresso
      quests        → Missões ativas
      achievements  → Conquistas desbloqueadas
      tutorial      → Tutorial interativo para iniciantes
      benchmark     → Teste de velocidade de digitação
      alias n='cmd' → Cria um atalho de comando (unalias n remove)

    Pressione ESC para voltar ao modo normal.

# Execução de comandos (shell, cd, ls)
sys-cd-ok = ✓ Diretório alterado para: { $dir }
sys-cd-notfound = Diretório não encontrado: { $path }
sys-ls-listed = 📂 Arquivos listados no painel direito →
sys-cmd-ok = ✓ Comando executado com sucesso
sys-cmd-error = ✗ Erro na execução do comando
sys-cmd-exec-error = ✗ Erro ao executar comando: { $msg }
sys-quest-complete = { "" }
    📋 MISSÃO COMPLETA!{ "" }
    { $title }{ "" }
    +{ $xp } XP

# Level up / conquistas
sys-levelup-title = 🎉 LEVEL UP!
sys-levelup-body = Nível { $old } → { $new }{ "" }
    { "" }
    { $rank }{ "" }
    { "" }
    { $msg }
sys-achievement-title = 🏆 Conquista Desbloqueada!
sys-achievement-announce = 🏆 CONQUISTA DESBLOQUEADA!{ "" }
    { "" }
    { $name }{ "" }
    { $desc }{ "" }
    { "" }
    +{ $xp } XP

# Dicas educativas (texto; os matchers de saída do shell ficam no código)
hint-err-rm-isdir = { "" }
    { "" }
    💡 DICA: 'rm' remove ARQUIVOS.{ "" }
       Para remover diretórios use:{ "" }
       - 'rmdir nome'     (diretório vazio){ "" }
       - 'rm -r nome'     (diretório com conteúdo){ "" }
       - 'rm -rf nome'    (força remoção - CUIDADO!)
hint-err-rmdir-notempty = { "" }
    { "" }
    💡 DICA: 'rmdir' só remove diretórios VAZIOS.{ "" }
       Para remover com conteúdo use: 'rm -r nome'
hint-err-cat-isdir = { "" }
    { "" }
    💡 DICA: 'cat' mostra conteúdo de ARQUIVOS.{ "" }
       Para listar diretórios use: 'ls nome'
hint-err-cd-notdir = { "" }
    { "" }
    💡 DICA: 'cd' navega para DIRETÓRIOS.{ "" }
       Para abrir arquivos use: 'cat nome' ou 'nano nome'
hint-err-mkdir-dots = { "" }
    { "" }
    💡 DICA: 'mkdir' cria DIRETÓRIOS (pastas).{ "" }
       Para criar arquivos use:{ "" }
       - 'touch arquivo.txt'          (arquivo vazio){ "" }
       - 'echo "texto" > arquivo.txt'  (arquivo com conteúdo)
hint-err-permission = { "" }
    { "" }
    💡 DICA: Você não tem permissão.{ "" }
       Tente com 'sudo' antes do comando (cuidado!)
hint-err-notfound = { "" }
    { "" }
    💡 DICA: Comando não existe ou não está instalado.{ "" }
       - Verifique se digitou corretamente{ "" }
       - Use 'which comando' para verificar se existe

# Avisos da zona de perigo (texto mostrado no painel DangerZone)
danger-rm-root = REMOÇÃO RECURSIVA EM DIRETÓRIO RAIZ!
danger-rm-rf = Remoção recursiva e forçada de arquivos
danger-rm = Remoção de arquivo(s) - operação irreversível
danger-sudo = Execução com privilégios de superusuário
danger-dd = Cópia de baixo nível - pode sobrescrever dados
danger-fs = Modificação de partições/sistema de arquivos
danger-perm = Modificação de permissões/propriedade de arquivos
danger-power = Desligamento/reinicialização do sistema
danger-generic = Comando potencialmente destrutivo detectado

# Diversos
ui-top-processes = Top processos (CPU)
ui-browse-hint = ↑↓ navegar • Enter abrir
ui-back-to-normal = Voltando ao modo normal
sys-file-not-found = ❌ Arquivo '{ $name }' não encontrado
sys-files-found = 💡 Arquivos encontrados:
