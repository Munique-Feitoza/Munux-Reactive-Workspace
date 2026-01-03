# 🐧 Munux Reactive Workspace

![Language](https://img.shields.io/badge/Language-Rust-orange)
![Framework](https://img.shields.io/badge/UI-Ratatui-blue)
![License](https://img.shields.io/badge/License-GPLv3-red)
![Status](https://img.shields.io/badge/Status-Beta-green)

**Munux Reactive Workspace** é um terminal gamificado de próxima geração para Linux, projetado para educação E uso diário.

Diferente dos terminais tradicionais, o Munux combina um **terminal totalmente funcional** com um sistema de **gamificação completo** (XP, níveis, conquistas, missões) e **painéis reativos** que se adaptam ao que você está fazendo. Use como seu terminal principal no Manjaro, Ubuntu, Fedora ou qualquer distro Linux!

---

## 🎮 O Conceito: "Terminal Real + RPG"

```text
+--------------------------------+------------------------------+
| TERMINAL COMPLETO (60%)        | PAINEL REATIVO (40%)         |
|                                |                              |
| ➜ [Iniciante@munux]$          | 🐧 BEM-VINDO AO MUNUX        |
| pacman -Syu                    | 📊 STATS & PROGRESSO         |
|   ✓ Sistema atualizado!        | ━━━━━━━━━━━━━━━━━━━━━━━━━   |
| 🏆 Arch User - BTW, I use Arch!| Level 5 | XP: 450/500       |
|                                | Streak: 12 🔥                |
| yay -S firefox                 | 📋 QUESTS ATIVAS             |
|   ✓ Instalando Firefox...     | ☑ Primeiro pacman (2/2)      |
| +50 XP! 🎯 Quest completa!     | ☐ Explorador Git (0/5)       |
|                                | ☐ Mestre de Rede (1/10)      |
+--------------------------------+------------------------------+
| [Nv 5 - Terminal] XP: 450/500 ▰▰▰▰▰▱▱▱ | 🏆 12 | 🔥 Streak: 12 |
+--------------------------------+------------------------------+
```

---

## 🚀 Recursos Principais

### 1. 🐧 Terminal Totalmente Funcional

**TODOS os comandos Linux funcionam normalmente!** Munux executa comandos reais via shell:

- ✅ **Gerenciadores de Pacotes**: `pacman`, `yay`, `paru` (Arch/Manjaro), `apt`, `dpkg`, `snap` (Debian/Ubuntu), `dnf`, `yum` (Fedora), `zypper` (openSUSE)
- ✅ **Ferramentas de Rede**: `ping`, `curl`, `wget`, `ssh`, `scp`, `rsync`, `netstat`, `ip`
- ✅ **Compressão**: `tar`, `zip`, `unzip`, `gzip`, `7z`, `rar`
- ✅ **Administração**: `systemctl`, `journalctl`, `dmesg`, `sudo`, `chmod`, `chown`
- ✅ **Desenvolvimento**: `git`, `make`, `gcc`, `python`, `node`, `npm`, `cargo`
- ✅ **Texto**: `sed`, `awk`, `grep`, `find`, `cat`, `vim`, `nano`
- ✅ **Navegação**: `cd`, `ls`, `pwd`, `mkdir`, `rm`, `cp`, `mv`

### 2. 🎮 Sistema de Gamificação Completo

#### XP e Níveis (6 Tiers)
- **Iniciante** (Nv 1-9): Tema Cyan, Tux básico, símbolo ➜
- **Terminal** (Nv 10-19): Tema Matrix Verde, Tux terminal, símbolo ►  
- **Hacker** (Nv 20-29): Tema Hacker, Tux hacker, símbolo ▶
- **Cyberpunk** (Nv 30-39): Tema Magenta/Cyan, Tux cyberpunk, símbolo ◆
- **Elite** (Nv 40-49): Tema Roxo, Tux elite, símbolo ⬢
- **Legend** (Nv 50+): Tema Rainbow, Tux legend, símbolo ⬣

#### 🏆 Conquistas (25+)
- **Primeiros Passos**: first_command, first_ls, first_cd, first_file
- **Gerenciadores de Pacotes**: 
  - `first_pacman` - "Arch User - BTW, I use Arch!" 🏔️
  - `first_apt` - "Debian Disciple" 📦
  - `first_git` - "Version Control Initiate" 🌿
  - `first_ssh` - "Remote Connection Established" 🔐
  - `first_systemctl` - "System Controller" ⚙️
- **Marcos**: 10, 50, 100, 500 comandos executados
- **Streaks**: 5, 10, 25 comandos consecutivos corretos
- **Níveis**: Level 5, 10, 20, 30, 50

#### 📋 Sistema de Missões
Quests geradas dinamicamente por nível:
- "Execute seu primeiro ls"
- "Crie um arquivo com touch"
- "Navegue para /home"
- "Instale um pacote"
- "Configure o Git"
- Progresso rastreado em tempo real!

### 3. 🎯 Painéis Reativos Inteligentes

O painel direito muda automaticamente baseado no que você digita:

- **Welcome**: Tux evolutivo + mensagem motivacional
- **File Tree**: Mostra arquivos ao usar `ls`, `ll`, `la`
- **File Preview**: Preview com syntax highlighting
- **Resource Monitor**: CPU/RAM/Swap ao usar `top`, `htop`
- **Danger Zone**: Alerta vermelho em comandos destrutivos
- **Stats**: Estatísticas detalhadas (sucesso %, streak, total)
- **Quests**: Missões ativas com barra de progresso
- **Easter Eggs**: ASCII art especial de comandos secretos

### 4. 🥚 Easter Eggs (10+)

Comandos secretos revelam surpresas:
- `sl` - Trem ASCII animado
- `cowsay` - Vaca falante customizável
- `fortune` - Frases sobre Linux e programação
- `matrix` / `hack` - Mensagens estilo Matrix
- `sudo su` - "With great power..." (Tio Ben)
- `hack the planet` - Referência Hackers (1995)
- `konami code` - Bônus secreto
- E mais escondidos...

### 5. 📚 Sistema de Ajuda Integrado

```bash
help          # Lista de comandos especiais
help arch     # Guia Manjaro/Arch (pacman, yay, paru)
help debian   # Guia Ubuntu/Debian (apt, dpkg, snap)
help fedora   # Guia Fedora/RHEL (dnf, rpm)
help opensuse # Guia openSUSE (zypper)
help linux    # Comandos universais Linux
```

### 6. 🛡️ Comandos Especiais Munux

- `stats` - Mostra estatísticas e progresso
- `quests` - Exibe missões ativas
- `achievements` - Lista conquistas desbloqueadas
- `xp` - Mostra XP e nível atual
- `xp <número>` - Adiciona XP (para testes)

---

## 🏗️ Arquitetura

Este projeto segue **The Elm Architecture** (Model-View-Update):

```
src/
├── main.rs              # Entry point e loop principal
├── app.rs               # Estado global (Model) + execução de comandos
├── event.rs             # Gerenciador de eventos (input)
├── tui.rs               # Configuração do terminal (Crossterm)
│
├── ui/                  # Camada de Visualização (View)
│   ├── mod.rs           # Renderizador principal
│   ├── layout.rs        # Define os blocos Split e Popup
│   ├── terminal.rs      # Renderiza o painel esquerdo (terminal)
│   ├── reactive.rs      # Renderiza o painel direito (contexto reativo)
│   ├── hud.rs           # Renderiza a barra de status e XP
│   ├── stats.rs         # Renderiza painéis de Stats e Quests
│   ├── popup.rs         # Popups e confirmações
│   └── theme.rs         # Sistema de temas progressivos (6 tiers)
│
├── core/                # Lógica de Negócio (Update)
│   ├── parser.rs        # Analisa comandos (11 tipos)
│   ├── shell.rs         # Executa comandos do sistema
│   ├── filesystem.rs    # Gerencia operações de arquivos
│   └── monitor.rs       # Coleta métricas do sistema (CPU/RAM)
│
└── game/                # Sistema de Gamificação Completo
    ├── state.rs         # XP, níveis, conquistas, quests, streaks
    ├── logic.rs         # Regras de Level Up e cálculo de XP
    ├── achievements.rs  # Sistema de conquistas (25+)
    ├── quests.rs        # Sistema de missões dinâmicas
    ├── easter_eggs.rs   # Easter eggs e comandos secretos
    └── distro_guide.rs  # Guias de comandos por distribuição
```

### Fluxo de Dados

1. **Event** (`event.rs`): Usuário pressiona uma tecla
2. **Update** (`app.rs`): O estado da aplicação é atualizado
3. **Parser** (`core/parser.rs`): Analisa o input (Navigation, PackageManager, etc.)
4. **Execute** (`app.rs`): Comando executado via shell + checks de achievements/quests
5. **View** (`ui/`): Ratatui renderiza o próximo frame com tema apropriado
6. **Repeat**: Loop reativo baseado em eventos

---

## 📦 Instalação e Uso

### Pré-requisitos

```bash
# Instale Rust (se ainda não tiver)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Instalação Rápida

```bash
# Clone o repositório
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace

# Compile e execute
cargo run

# OU compile em modo release (mais rápido)
cargo build --release
./target/release/munux-reactive-workspace
```

### 🎮 Primeiros Passos

```bash
# 1. Inicie o Munux
cargo run

# 2. Experimente comandos básicos (ganhe XP!)
ls
pwd
mkdir test
cd test
touch file.txt
echo "Hello Munux" > file.txt
cat file.txt

# 3. Veja seu progresso
stats           # Estatísticas detalhadas
quests          # Missões ativas
achievements    # Conquistas desbloqueadas

# 4. Use gerenciadores de pacotes (desbloqueia conquistas!)
# Manjaro/Arch:
pacman -Syu     # Atualiza sistema
yay -S firefox  # Instala Firefox

# Ubuntu/Debian:
sudo apt update
sudo apt install firefox

# 5. Explore easter eggs
sl
fortune
cowsay "Munux is awesome!"

# 6. Peça ajuda
help            # Lista comandos especiais
help arch       # Guia para Manjaro/Arch
help debian     # Guia para Ubuntu/Debian
```

---

## ⌨️ Controles

| Tecla | Ação |
|-------|------|
| **Digitar normalmente** | Adiciona caracteres ao buffer de input |
| **Enter** | Executa o comando (ganha XP se sucesso!) |
| **Backspace** | Remove o último caractere |
| **↑ / ↓** | Navega pelo histórico de comandos |
| **Ctrl+C** | Sai da aplicação |
| **Ctrl+L** | Limpa a tela |
| **ESC** | Limpa o input atual ou cancela comando perigoso |
| **Tab** | (Futuro) Auto-completar |

---

## 🎮 Sistema de Gamificação Detalhado

### 📊 Progressão de Níveis (6 Tiers)

| Nível | Tier | Tema | Tux | Símbolo | XP Necessário |
|-------|------|------|-----|---------|---------------|
| 1-9 | **Iniciante** | Cyan | Básico | ➜ | 0-900 |
| 10-19 | **Terminal** | Matrix Green | Terminal | ► | 1000-1900 |
| 20-29 | **Hacker** | Cyan/Green | Hacker | ▶ | 2000-2900 |
| 30-39 | **Cyberpunk** | Magenta/Cyan | Cyberpunk | ◆ | 3000-3900 |
| 40-49 | **Elite** | Purple | Elite | ⬢ | 4000-4900 |
| 50+ | **Legend** | Rainbow | Legend | ⬣ | 5000+ |

**Evolução Visual**: Tux e cores evoluem com você! Quanto mais você usa, mais cyberpunk fica.

### 🏆 Conquistas Completas

#### Primeiros Passos
- ✅ **First Command** - "The Journey Begins" (50 XP)
- ✅ **First LS** - "Listing Master" (20 XP)
- ✅ **First CD** - "Navigator" (20 XP)
- ✅ **First File** - "Creator" (30 XP)
- ✅ **First RM** - "Destroyer" (25 XP)
- ✅ **First Sudo** - "With Great Power..." (100 XP)

#### Gerenciadores de Pacotes
- 🏔️ **First Pacman** - "Arch User - BTW, I use Arch!" (50 XP)
- 📦 **First APT** - "Debian Disciple" (50 XP)
- 🌿 **First Git** - "Version Control Initiate" (50 XP)
- 🔐 **First SSH** - "Remote Connection Established" (40 XP)
- ⚙️ **First Systemctl** - "System Controller" (40 XP)

#### Marcos de Comandos
- 🎯 **10 Commands** - "Getting Started" (100 XP)
- 🚀 **50 Commands** - "Regular User" (200 XP)
- 💎 **100 Commands** - "Power User" (500 XP)
- 👑 **500 Commands** - "Terminal Master" (1000 XP)

#### Streaks
- 🔥 **5 Streak** - "On Fire!" (50 XP)
- 🔥🔥 **10 Streak** - "Unstoppable!" (150 XP)
- 🔥🔥🔥 **25 Streak** - "Legendary!" (500 XP)

#### Níveis
- ⭐ **Level 5** - "Novice Complete" (100 XP)
- ⭐⭐ **Level 10** - "Terminal User" (200 XP)
- ⭐⭐⭐ **Level 20** - "Hacker Achieved" (500 XP)
- 💫 **Level 30** - "Cyberpunk Elite" (1000 XP)
- 🌟 **Level 50** - "Legend Status" (2000 XP)

### 📋 Sistema de Quests

Quests são geradas dinamicamente baseadas no seu nível:

**Nível 1-5** (Básico):
- "Execute seu primeiro comando ls"
- "Navegue para o diretório /home"
- "Crie um arquivo com touch"
- "Leia um arquivo com cat"

**Nível 6-15** (Intermediário):
- "Use grep para buscar texto"
- "Crie um diretório e navegue até ele"
- "Execute 5 comandos diferentes"
- "Use pipes (|) pela primeira vez"

**Nível 16-30** (Avançado):
- "Configure o Git com seu nome"
- "Instale um pacote com pacman/apt"
- "Use SSH para conectar remotamente"
- "Execute 10 comandos de rede"

**Nível 31+** (Elite):
- "Domine systemctl (5 comandos)"
- "Atinja level 50"
- "Desbloqueie todas as conquistas"

### 🔥 Sistema de Streak

- **Cada comando correto**: +1 streak
- **Cada erro**: Streak reseta para 0
- **Streak 5+**: Bônus de +10% XP
- **Streak 10+**: Bônus de +25% XP
- **Streak 25+**: Bônus de +50% XP

**Dica**: Mantenha streak alto para subir de nível mais rápido!


---

## 🔬 Tecnologias Utilizadas

| Crate | Versão | Propósito |
|-------|--------|----------|
| **ratatui** | 0.26.3 | Framework de TUI (Terminal User Interface) |
| **crossterm** | 0.27.0 | Manipulação de terminal cross-platform |
| **sysinfo** | 0.30.13 | Coleta de informações do sistema (CPU, RAM, Swap) |
| **serde** | 1.0 | Serialização/deserialização (progresso, state) |
| **chrono** | 0.4 | Timestamps e gerenciamento de tempo |
| **anyhow** | 1.0 | Tratamento de erros ergonômico |

---

## 🛣️ Roadmap

### ✅ Versão 0.1.0 - COMPLETO
- [x] Arquitetura base (The Elm Architecture)
- [x] Split Screen reativo
- [x] Sistema de XP e níveis (6 tiers)
- [x] Parser de comandos (11 tipos)
- [x] Execução real via shell
- [x] Modo de perigo para comandos destrutivos
- [x] Temas progressivos dinâmicos
- [x] Sistema de conquistas (25+)
- [x] Sistema de quests dinâmicas
- [x] Easter eggs (10+)
- [x] Sistema de streak com bônus
- [x] Suporte a gerenciadores de pacotes
- [x] Guias de distribuições integrados
- [x] Stats e progresso detalhado
- [x] Tux evolutivo (6 formas)

### 🚧 Versão 0.2.0 - Em Planejamento
- [ ] Persistência de progresso (salvar/carregar state)
- [ ] Auto-completar (Tab completion)
- [ ] Histórico persistente entre sessões
- [ ] Syntax highlighting no preview de arquivos
- [ ] Mais easter eggs e conquistas secretas
- [ ] Sistema de aliases personalizados
- [ ] Modo tutorial interativo para iniciantes
- [ ] Benchmark mode (testar velocidade de digitação)

### 🔮 Versão 0.3.0 - Futuro
- [ ] Temas customizáveis (criar seus próprios)
- [ ] Plugins e extensões
- [ ] Modo competitivo (leaderboards)
- [ ] Integração com GitHub (commits → XP)
- [ ] Modo multiplayer (comparar progresso)
- [ ] Suporte a scripts custom
- [ ] AI Assistant integration

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Este é um projeto educacional e open-source.

### Como contribuir

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/MinhaFeature`)
3. Commit suas mudanças (`git commit -m 'Adiciona MinhaFeature'`)
4. Push para a branch (`git push origin feature/MinhaFeature`)
5. Abra um Pull Request

### Áreas que precisam de ajuda

- 🎨 Novos temas e skins para Tux
- 🏆 Mais conquistas e easter eggs
- 📦 Suporte a mais gerenciadores de pacotes (emerge, nix, etc.)
- 🌍 Traduções (inglês, espanhol, etc.)
- 📖 Tutoriais e quests educacionais
- 🧪 Testes e correção de bugs

---

## 📚 Documentation

Complete documentation is available in the [`docs/`](docs/) directory:

- **[Quick Start Guide](docs/guides/quick-start.md)** - Get started in 5 minutes
- **[Architecture Overview](docs/architecture/overview.md)** - System design and patterns
- **[Gamification System](docs/guides/gamification-system.md)** - XP, achievements, quests
- **[Testing Guide](docs/TESTING.md)** - Comprehensive testing documentation
- **[Changelog](docs/CHANGELOG.md)** - Version history and updates

For developers:
- **[Component API](docs/api/core-modules.md)** - Technical API reference
- **[Contributing Guide](docs/contributing/development-setup.md)** - How to contribute
- **[Code of Conduct](docs/contributing/code-of-conduct.md)** - Community guidelines

---

## ❓ FAQ

**P: Munux substitui meu terminal?**
R: Sim! Munux é um terminal totalmente funcional. Todos os comandos Linux funcionam normalmente.

**P: Funciona em qual distro?**
R: Funciona em **qualquer distro Linux**. Testado em Manjaro, Ubuntu, Fedora, Debian, Arch.

**P: Meu progresso é salvo?**
R: Atualmente não (v0.1.0), mas persistência de dados está planejada para v0.2.0.

**P: Como desbloquear todos os temas?**
R: Use o comando `xp 5000` para adicionar XP e subir de nível rapidamente (modo teste).

**P: Quais pacotes posso instalar?**
R: **TODOS**! Munux executa comandos reais. `pacman -S firefox` instala Firefox de verdade.

**P: Por que BTW, I use Arch?**
R: É um meme da comunidade Arch Linux. Ao usar `pacman` pela primeira vez, você desbloqueia a conquista! 🏔️

---

## 💡 Inspirações

Este projeto foi inspirado por:
- **Vim Adventures**: Gamificação de aprendizado
- **HackerRank**: Sistema de XP e rankings
- **Terminals tradicionais**: Fish, Zsh, Oh-My-Zsh
- **The Hackers Movie (1995)**: Estética cyberpunk
- **Matrix**: Temas visuais e easter eggs
- **Comunidade Linux**: Filosofia de educação e open-source

---

## 📄 Licença

Este projeto está licenciado sob a **GNU General Public License v3.0** (GPLv3).

Isso significa:
- ✅ Você pode usar, modificar e distribuir livremente
- ✅ Você pode usar comercialmente
- ⚠️ Modificações devem ser open-source sob GPLv3
- ⚠️ Deve incluir o aviso de copyright

Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

---

## 👤 Autora

**Munique Feitoza**

- GitHub: [@Munique-Feitoza](https://github.com/Munique-Feitoza)
- Projeto: [Munux-Reactive-Workspace](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace)

---

## ⭐ Agradecimentos

Se você gostou do projeto, deixe uma ⭐ no repositório!

---

<div align="center">

**Feito com ❤️ e muito ☕ usando Rust 🦀**

*"The best way to learn is by doing. The best way to do is by playing."*

</div>

**Munique Alves Pacheco Feitoza**

- GitHub: [@Munique-Feitoza](https://github.com/Munique-Feitoza)
- LinkedIn: [Munique Feitoza](https://linkedin.com/in/munique-feitoza)

---

## 🙏 Agradecimentos

- Inspirado pela necessidade de ferramentas educacionais melhores para aprender Linux
- Comunidade Rust pela linguagem incrível
- Ratatui pela biblioteca de TUI poderosa
- Todos que contribuem para o ecossistema open-source

---

**Munux Reactive Workspace** - *Aprendendo comandos de terminal, um XP por vez.* 🚀