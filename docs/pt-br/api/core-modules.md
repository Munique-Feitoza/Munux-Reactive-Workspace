# 🔬 Referência da API - Módulos Core

Documentação completa da API para os componentes principais do Munux Reactive Workspace.

![Rust](https://img.shields.io/badge/Linguagem-Rust-orange) ![API](https://img.shields.io/badge/API-Estável-green) ![Docs](https://img.shields.io/badge/Cobertura-100%25-brightgreen)

> [!NOTE]
> Este documento descreve a API interna para a versão v0.1.0. Para padrões de arquitetura, veja a [Visão Geral da Arquitetura](../architecture/overview.md).

---

## Organização de Módulos

```mermaid
graph TD
    A[main.rs]:::entry --> B[app.rs]:::app
    A --> C[event.rs]:::app
    A --> D[tui.rs]:::app
    A --> I[i18n.rs]:::app

    B --> E[core/*]:::core
    B --> F[game/*]:::game
    B --> G[ui/*]:::ui

    E --> E1[parser.rs]:::core
    E --> E2[shell.rs]:::core
    E --> E3[filesystem.rs]:::core
    E --> E4[monitor.rs]:::core
    E --> E5[git.rs]:::core
    E --> E6[ssh.rs]:::ssh

    F --> F1[state.rs]:::game
    F --> F2[logic.rs]:::game
    F --> F3[achievements.rs]:::game
    F --> F4[quests.rs]:::game

    G --> G1[terminal.rs]:::ui
    G --> G2[reactive.rs]:::ui
    G --> G3[theme.rs]:::ui
    G --> G4[hud.rs]:::ui
    G --> G5[stats.rs]:::ui
    G --> G6[popup.rs]:::ui
    G --> G7[layout.rs]:::ui

    classDef entry fill:#ffd166,stroke:#d4a017,color:#000
    classDef app fill:#b4a7f5,stroke:#6f42c1,color:#000
    classDef core fill:#b8e994,stroke:#38a169,color:#000
    classDef game fill:#f9a8d4,stroke:#be185d,color:#000
    classDef ui fill:#a0e7e5,stroke:#17a2b8,color:#000
    classDef ssh fill:#81d4fa,stroke:#0277bd,color:#000
```

### Diagrama de Classes Cross-Módulo (UML)

```mermaid
classDiagram
    direction TB

    class App {
        +String input_buffer
        +PathBuf current_dir
        +GameState game_state
        +RightPanelMode right_panel_mode
        +Option~SshSession~ ssh_session
        +I18n i18n
        +execute_command() Result
        +handle_key(KeyEvent) Result
        +analyze_input()
    }

    class CommandParser {
        <<service>>
        +classify_command(str)$ CommandType
        +calculate_xp(CommandType)$ u32
    }

    class ShellExecutor {
        <<service>>
        +execute(str)$ Result~Output~
    }

    class SshSession {
        +String host
        +String user
        +String remote_cwd
        +connect(user, host)$ Result~SshSession~
        +execute(cmd) Result
        +change_dir(path) Result
    }

    class SystemMonitor {
        +refresh()
        +cpu_usage() f32
        +memory_usage() MemoryInfo
    }

    class GameState
    class Theme
    class RightPanelMode {
        <<enum>>
        Welcome
        CommandOutput
        FileTree
        DangerZone
        Stats
        SshActive
    }

    App *-- GameState
    App *-- "0..1" SshSession
    App --> RightPanelMode
    App ..> CommandParser
    App ..> ShellExecutor
    App ..> SystemMonitor
    GameState ..> Theme
```

---

## Ponto de Entrada

### `main.rs`

Ponto de entrada da aplicação e loop de eventos.

**Responsabilidades:**

- Inicialização e restauração do terminal
- Gerenciamento do loop de eventos
- Renderização de frames (60 FPS)

---

## Estado da Aplicação

### `app.rs` - Struct `App`

Estado central da aplicação seguindo a **The Elm Architecture (TEA)**.

#### Métodos Principais

| Método | Assinatura | Propósito |
|:-------|:----------|:--------|
| `new()` | `fn new() -> Self` | Inicializa com valores padrão |
| `handle_key()` | `fn handle_key(&mut self, key: KeyEvent) -> Result<()>` | Processa entrada do teclado |
| `execute_command()` | `fn execute_command(&mut self, cmd: &str) -> Result<()>` | Executa comando shell |

---

## Módulos Core (`src/core/`)

### `parser.rs` - Classificação de Comandos

Analisa comandos e os categoriza usando padrões regex.

```rust
pub enum CommandType {
    Navigation,      // cd, ls, pwd
    FileOps,        // touch, mkdir, cp, mv, rm
    TextProcessing, // cat, grep, sed, awk
    System,         // ps, top, htop, kill
    PackageManager, // pacman, apt, dnf, yay
    Network,        // ping, curl, wget, ssh
    Git,           // comandos git
    Dangerous,     // rm -rf, dd, chmod 000
    Help,          // help, man
    Unknown,       // Não reconhecido
}
```

#### API do Parser

- `classify(cmd: &str) -> CommandType`: Classifica uma string de comando.
- `calculate_xp(cmd_type: &CommandType) -> u32`: Calcula a recompensa de XP.
- `is_dangerous(cmd: &str) -> bool`: Verifica se o comando é perigoso.

---

### `shell.rs` - Execução de Comandos

Executa comandos via shell do sistema e captura a saída.

> [!WARNING]
> Todos os comandos rodam via `sh -c`. Garanta o escape correto de strings na entrada do usuário!

---

### `filesystem.rs` - Operações de Arquivo

Navegação e leitura segura do sistema de arquivos.

**Garantias de Segurança:**

- ✅ Valida caminhos antes do acesso
- ✅ Respeita permissões do Linux
- ✅ Limita o tamanho de leitura (previne OOM em arquivos gigantes)
- ✅ Sem código `unsafe` (segurança Rust total)

---

### `ssh.rs` - Sessão de Shell Remoto

Encapsula a crate [`ssh2`](https://crates.io/crates/ssh2) para fornecer uma sessão remota persistente com tracking de cwd.

```rust
pub struct SshSession {
    session: ssh2::Session,
    _tcp: TcpStream,
    pub host: String,
    pub user: String,
    pub remote_cwd: String,
}

impl SshSession {
    /// Abre conexão TCP :22 + handshake + autenticação.
    /// Ordem: ssh-agent → userauth_agent → ~/.ssh/id_rsa.
    pub fn connect(user: &str, host: &str) -> Result<Self>;

    /// Executa um comando no cwd remoto atual.
    /// Retorna (stdout, stderr, exit_code).
    pub fn execute(&mut self, command: &str) -> Result<(String, String, i32)>;

    /// Atualiza `remote_cwd` resolvendo `cd $path && pwd` no servidor.
    pub fn change_dir(&mut self, path: &str) -> Result<()>;
}
```

**Cadeia de autenticação (UML colorido):**

```mermaid
flowchart LR
    A[connect]:::ok --> B{ssh-agent<br/>tem chaves?}:::q
    B -- sim --> OK([✅ Autenticado]):::ok
    B -- não --> C{userauth_agent<br/>fallback?}:::q
    C -- sim --> OK
    C -- não --> D{~/.ssh/id_rsa<br/>existe?}:::q
    D -- sim --> OK
    D -- não --> FAIL([❌ bail!<br/>configure chaves]):::bad

    classDef ok fill:#b8e994,stroke:#38a169,color:#000
    classDef q  fill:#ffd166,stroke:#d4a017,color:#000
    classDef bad fill:#ff6b6b,stroke:#c0392b,color:#fff
```

> [!WARNING]
> Autenticação por senha **não** é suportada ainda — é intencional para servidores estilo RunCloud. Use `ssh-agent` ou um `id_rsa` sem passphrase. Prompt interativo de senha requer um modal TUI, que está no roadmap.

**Comportamento dentro de `App::execute_command()`** (ver [app.rs:231-328](../../../src/app.rs#L231-L328)):

- Quando `ssh_session.is_some()`, o parsing local é ignorado.
- `cd <path>` → `SshSession::change_dir`
- `exit`/`logout` → encerra a sessão e volta ao shell local
- Qualquer outro comando → `SshSession::execute`, saída renderizada com borda ciano e prompt remoto (`user@host cwd$ …`)
- `ls`/`grep` recebem `--color=always` automaticamente para ANSI via `ansi-to-tui`

---

## Módulos de Jogo (`src/game/`)

### `state.rs` - Estado do Jogo

Rastreamento de progressão persistente.

- **Level**: Nível atual do jogador.
- **XP**: Pontos de experiência acumulados.
- **Streak**: Sequência de comandos bem-sucedidos.
- **Achievements/Quests**: Listas de conquistas e missões ativas.

---

## Módulos de Interface (`src/ui/`)

### `theme.rs` - Sistema de Temas

Temas progressivos baseados no nível do jogador.

- 🌱 **Iniciante** (Ciano)
- 💻 **Terminal** (Matrix Green)
- 🔓 **Hacker** (Cyber Pulse)
- 🌃 **Cyberpunk** (Night City)
- 👑 **Elite** (Roxo)
- ⭐ **Legend** (RGB/Rainbow)

---

## Próximos Passos

- 🏗️ [Visão Geral da Arquitetura](../architecture/overview.md)
- 🧪 [Guia de Testes](../TESTING.md)
- 🤝 [Contribuição](../contributing/code-of-conduct.md)
