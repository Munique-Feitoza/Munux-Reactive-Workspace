# 🏗️ Visão Geral da Arquitetura

> [!NOTE]
> Este documento descreve o design de alto nível do Munux. Para detalhes de implementação, veja a referência da API em [core-modules.md](../api/core-modules.md).

## Resumo Executivo

O Munux implementa uma **arquitetura de painéis divididos reativos** baseada na **The Elm Architecture (TEA)**. Ele combina o poder bruto de um shell Unix com uma camada de interface inteligente e consciente de estado que fornece contexto em tempo real e gamificação.

---

## Arquitetura de Alto Nível

```mermaid
graph TD
    User((Usuário)):::userCls --> Input[Entrada do Terminal]:::uiCls

    subgraph UI_Layer [🎨 Camada de Interface]
        Input --> LeftPanel[Painel Terminal 60%]:::uiCls
        RightPanel[Painel Reativo 40%]:::uiCls
        HUD[Heads Up Display]:::uiCls
    end

    subgraph Core_Layer [⚙️ Núcleo da Aplicação - TEA]
        EventLoop(Event Loop / Crossterm):::coreCls --> Parser{Parser de Comandos}:::coreCls
        Parser -->|Navegação| NavState[Estado de Nav]:::coreCls
        Parser -->|Sistema| SysState[Estado do Sistema]:::coreCls
        Parser -->|Perigoso| DangerState[Estado de Perigo]:::dangerCls
        Parser -->|SSH| SshState[Estado SSH]:::sshCls

        StateMgr[Gestão de Estado TEA]:::coreCls
        NavState --> StateMgr
        SysState --> StateMgr
        DangerState --> StateMgr
        SshState --> StateMgr

        StateMgr -->|Atualizar View| RightPanel
        StateMgr -->|Atualizar Stats| HUD
    end

    subgraph System_Layer [💻 Camada de Sistema]
        Shell[Executor Shell sh/bash]:::sysCls
        FS[Sistema de Arquivos]:::sysCls
        Monitor[Monitor SysInfo]:::sysCls
        Git[Integração Git]:::sysCls

        StateMgr -->|Executar| Shell
        StateMgr -->|Ler| FS
        StateMgr -->|Poll| Monitor
        StateMgr -->|Status| Git
    end

    subgraph Remote_Layer [🌐 Camada Remota]
        SshSession[Sessão SSH ssh2]:::sshCls
        RemoteHost[(Host Remoto)]:::remoteCls
        StateMgr -->|Túnel| SshSession
        SshSession -->|TCP :22| RemoteHost
    end

    Shell --> Output[Saída do Comando]:::uiCls
    SshSession --> Output
    Output --> LeftPanel

    classDef userCls fill:#ffd166,stroke:#d4a017,stroke-width:2px,color:#1a1a1a
    classDef uiCls fill:#a0e7e5,stroke:#17a2b8,stroke-width:2px,color:#1a1a1a
    classDef coreCls fill:#b4a7f5,stroke:#6f42c1,stroke-width:2px,color:#1a1a1a
    classDef sysCls fill:#b8e994,stroke:#38a169,stroke-width:2px,color:#1a1a1a
    classDef sshCls fill:#81d4fa,stroke:#0277bd,stroke-width:2px,color:#1a1a1a
    classDef remoteCls fill:#f8bbd0,stroke:#ad1457,stroke-width:2px,color:#1a1a1a
    classDef dangerCls fill:#ff6b6b,stroke:#c0392b,stroke-width:2px,color:#fff
```

> [!TIP]
> 🟨 Usuário · 🟦 UI · 🟪 Core · 🟩 Sistema · 🟦 SSH · 🟥 Perigo — cores para leitura rápida.

---

## Diagrama de Classes do Core (UML)

```mermaid
classDiagram
    direction LR

    class App {
        +String input_buffer
        +Vec~String~ history
        +PathBuf current_dir
        +GameState game_state
        +RightPanelMode right_panel_mode
        +Option~SshSession~ ssh_session
        +I18n i18n
        +new() Result~App~
        +handle_key(KeyEvent) Result
        +execute_command() Result
        +analyze_input()
    }

    class CommandParser {
        +classify_command(str) CommandType
        +calculate_xp(CommandType) u32
        +is_dangerous(str) bool
    }

    class ShellExecutor {
        +execute(str) Result~Output~
        +execute_in_dir(str, Path) Result
    }

    class SshSession {
        +String host
        +String user
        +String remote_cwd
        -Session session
        -TcpStream _tcp
        +connect(user, host) Result~SshSession~
        +execute(cmd) Result~(String,String,i32)~
        +change_dir(path) Result
    }

    class SystemMonitor {
        +refresh()
        +cpu_usage() f32
        +memory_usage() MemoryInfo
    }

    class GameState {
        +u32 level
        +u32 xp
        +Vec~Achievement~ achievements
        +add_xp(u32) Option~LevelUpInfo~
        +check_achievements() Vec
    }

    App "1" *-- "1" GameState : possui
    App "1" *-- "0..1" SshSession : opcional
    App ..> CommandParser : usa
    App ..> ShellExecutor : usa
    App ..> SystemMonitor : usa
```

---

## Padrões de Design

### 1. The Elm Architecture (TEA)

O Munux garante uma gestão de estado previsível através de um fluxo de dados unidirecional.

```rust
// Model - Fonte única da verdade
struct App {
    state: State
}

// Update - Transformações de estado puras
fn update(msg: Msg, model: &mut App) -> Command

// View - Renderização imutável
fn view(model: &App) -> Frame
```

> [!TIP]
> **Por que TEA?** Este padrão torna a aplicação incrivelmente fácil de debugar. Como a view é uma função pura do estado, podemos reproduzir qualquer bug visual apenas conhecendo os dados do estado.

---

### 2. Strategy Pattern (Classificação de Comandos)

Categorizamos a entrada do usuário em 11 estratégias distintas para determinar a reação da UI.

| Estratégia | Gatilho | Reação da UI |
|----------|---------|-------------|
| **Navegação** | `cd`, `ls`, `pwd` | Visualização de árvore de arquivos |
| **Arquivos** | `touch`, `mkdir`, `cp` | Painel de preview |
| **Monitoramento** | `top`, `ps`, `htop` | Gráficos em tempo real (CPU/RAM) |
| **Pacotes** | `pacman`, `apt`, `dnf` | Progresso de instalação |
| **Rede** | `ping`, `curl`, `wget` | Status de rede |
| **Perigoso** | `rm -rf`, `dd`, `sudo` | 🚨 Painel vermelho de aviso |
| **Git** | comandos `git` | Status do repositório |
| **Texto** | `cat`, `grep`, `sed` | Preview de conteúdo |
| **Ajuda** | `help`, `man` | Visualizador de docs |
| **Easter Eggs** | `sl`, `cowsay`, `fortune` | Animações especiais |
| **Desconhecido** | Não reconhecido | Sistema de sugestões |

Cada estratégia determina: **recompensa de XP**, **modo do painel reativo** e **gatilhos de conquistas**.

---

### 3. Observer Pattern (Painéis Reativos)

O painel da direita "observa" o buffer de input. Conforme você digita (antes de pressionar Enter), a UI reage.

```mermaid
sequenceDiagram
    participant User as Usuário
    participant EventLoop
    participant AppState
    participant UI

    User->>EventLoop: Digita "rm -rf"
    EventLoop->>AppState: Atualiza Buffer
    AppState->>AppState: analyze_input()
    AppState-->>UI: Modo: DANGER
    UI-->>User: Renderiza Painel Vermelho 🚨

    User->>EventLoop: Aperta Enter
    EventLoop->>AppState: execute_command()
    AppState->>UI: Mostra Confirmação
```

---

## Componentes

| Componente | Arquivo | Responsabilidade |
|-----------|------|----------------|
| **Event Loop** | `event.rs` | Input de teclado (Crossterm), resize, polling 60Hz |
| **App State** | `app.rs` | Estado do jogo (XP, nível, conquistas), histórico |
| **Parser** | `core/parser.rs` | Classificação de comandos, cálculo de XP, detecção de perigo |
| **Shell Executor** | `core/shell.rs` | Executa via `sh -c`, captura stdout/stderr |
| **File System** | `core/filesystem.rs` | Listagem, preview, navegação |
| **Monitor** | `core/monitor.rs` | Métricas CPU/RAM/Swap em tempo real (SysInfo) |
| **SSH Session** | `core/ssh.rs` | Shell remoto via `ssh2` (auth por agente / chave), tracking de cwd remoto |
| **Git Integration** | `core/git.rs` | Detecção de branch/status |
| **Terminal Panel** | `ui/terminal.rs` | Painel esquerdo — saída de comandos |
| **Reactive Panel** | `ui/reactive.rs` | Painel direito — modos sensíveis ao contexto |
| **HUD** | `ui/hud.rs` | Barra inferior — XP, nível, streak, integridade |
| **Theme System** | `ui/theme.rs` | Temas progressivos (6 níveis) |
| **I18n** | `i18n.rs` | Localização em runtime (EN/PT-BR) via Fluent |

---

## Ciclo de Vida da Sessão SSH (Diagrama de Estados)

```mermaid
stateDiagram-v2
    [*] --> Ocioso
    Ocioso --> Conectando : usuário digita<br/>`ssh user@host`
    Conectando --> Autenticando : TCP :22 OK
    Conectando --> Falhou : erro TCP
    Autenticando --> Ativo : ssh-agent /<br/>pubkey OK
    Autenticando --> Falhou : erro de auth
    Ativo --> Ativo : execute(cmd)<br/>change_dir(path)
    Ativo --> Ocioso : usuário digita<br/>`exit` / `logout`
    Falhou --> Ocioso : popup exibido
    Ativo --> [*] : app encerra

    classDef ativo fill:#81d4fa,stroke:#0277bd,color:#000
    classDef ruim fill:#ff6b6b,stroke:#c0392b,color:#fff
    class Ativo ativo
    class Falhou ruim
```

### Fluxo de Comando SSH (Sequência)

```mermaid
sequenceDiagram
    autonumber
    actor Usuario as Usuário
    participant App as App (app.rs)
    participant Ssh as SshSession (core/ssh.rs)
    participant Host as Host Remoto

    Usuario->>App: ssh alice@box.dev
    App->>Ssh: SshSession::connect("alice","box.dev")
    Ssh->>Host: TCP connect :22
    Ssh->>Host: Handshake SSH
    Ssh->>Host: userauth (agent → pubkey → ~/.ssh/id_rsa)
    Host-->>Ssh: auth ok
    Ssh->>Host: exec `pwd`
    Host-->>Ssh: remote_cwd
    Ssh-->>App: Ok(SshSession { host, user, remote_cwd })
    App-->>Usuario: 🟢 popup "Conexão Estabelecida"

    loop enquanto ssh_session.is_some()
        Usuario->>App: comando remoto
        App->>Ssh: execute(cmd)
        Ssh->>Host: channel.exec(cd $cwd && $cmd)
        Host-->>Ssh: stdout / stderr / exit_code
        Ssh-->>App: saída
        App-->>Usuario: renderiza no Terminal (borda ciano)
    end

    Usuario->>App: exit
    App->>Ssh: dropa sessão
    App-->>Usuario: 🔌 desconectado
```

---

## Fluxo de Dados

```mermaid
flowchart LR
    A([Entrada do Usuário]):::in --> B[Event Loop]:::core
    B --> C{Parser}:::core
    C --> D[Update de Estado]:::core
    D --> E[Execução Shell]:::sys
    D --> S[Execução SSH]:::ssh
    E --> F[Captura Output]:::sys
    S --> F
    F --> G[Render View]:::ui
    G --> H([Exibição ao Usuário]):::out
    D -.->|Reativo| G

    classDef in fill:#ffd166,stroke:#d4a017,color:#000
    classDef out fill:#ffd166,stroke:#d4a017,color:#000
    classDef core fill:#b4a7f5,stroke:#6f42c1,color:#000
    classDef sys fill:#b8e994,stroke:#38a169,color:#000
    classDef ssh fill:#81d4fa,stroke:#0277bd,color:#000
    classDef ui fill:#a0e7e5,stroke:#17a2b8,color:#000
```

**Passo a passo:**
1. 🎹 Usuário digita comando
2. 🔄 Event loop captura entrada (Crossterm)
3. 🔍 Parser classifica o tipo do comando
4. 💾 Estado atualizado (XP, conquistas, missões)
5. 🐚 Comando executado via shell local (`sh -c`) **ou** via canal SSH remoto
6. 📋 Saída capturada (stdout/stderr)
7. 🎨 UI re-renderizada com base no novo estado
8. 🖥️ Display atualizado (60Hz refresh)

---

## Modelo de Segurança

> [!WARNING]
> **Segurança em primeiro lugar**: Munux cria uma camada de proteção, mas os comandos são executados de fato no host.

| Camada | Proteção |
|-------|-----------|
| **Detecção de Comandos Perigosos** | Pattern matching intercepta `rm`, `dd`, `chmod` antes da execução |
| **Isolamento Shell** | Comandos rodam em instâncias isoladas de `sh -c` |
| **Permissões** | Munux respeita as permissões padrão de User/Group do Linux |
| **Sem Escalada de Privilégios** | Nunca tenta ganhar root automaticamente |
| **Confirmação Explícita** | Painel vermelho + confirmação explícita para operações destrutivas |
| **SSH: só chave/agente** | Sem suporte a senha interativa — evita armazenamento em memória |

---

## Performance

| Métrica | Valor | Notas |
|--------|-------|-------|
| **Taxa de atualização** | 60 Hz | Frequência de polling do event loop |
| **Uso de memória** | ~10-20 MB | Footprint típico |
| **CPU (ocioso)** | <1% | Processamento em background mínimo |
| **Tempo de startup** | <200ms | Build release, cold start |

> [!TIP]
> Sempre use `cargo run --release` em uso real. Builds debug são 10-50x mais lentas.

---

## Melhorias Futuras

### 🌐 Rede
- [x] ✅ **Integração SSH** com view reativa (via `ssh2`, v0.1.1+)
- [ ] Auth interativa por senha (hoje só agente + pubkey)
- [ ] Monitoramento remoto via túneis SSH

### 🗂️ Persistência
- [ ] Salvar XP e conquistas em `~/.munux/state.json`
- [ ] Histórico entre sessões (SQLite)

#### Modelo de Dados Proposto (ER)

> [!NOTE]
> Ainda não implementado. Rascunho para a futura separação `~/.munux/state.json` (JSON) + `~/.munux/history.db` (SQLite).

```mermaid
erDiagram
    PROFILE ||--o{ SESSION : "tem"
    PROFILE ||--o{ UNLOCKED_ACHIEVEMENT : "ganhou"
    PROFILE ||--o{ ACTIVE_QUEST : "rastreando"
    PROFILE {
        string id PK
        string username
        int level
        int xp
        int max_streak
        datetime created_at
        datetime last_seen_at
    }
    SESSION ||--o{ COMMAND : "contém"
    SESSION {
        string id PK
        string profile_id FK
        datetime started_at
        datetime ended_at
        int commands_run
        int xp_earned
    }
    COMMAND {
        int id PK
        string session_id FK
        string raw
        string cmd_type
        int exit_code
        int xp_delta
        int duration_ms
        datetime executed_at
    }
    UNLOCKED_ACHIEVEMENT {
        string achievement_id PK
        string profile_id FK
        datetime unlocked_at
    }
    ACTIVE_QUEST {
        string quest_id PK
        string profile_id FK
        int progress
        int target
        datetime accepted_at
    }
    ACHIEVEMENT_CATALOG ||--o{ UNLOCKED_ACHIEVEMENT : "define"
    ACHIEVEMENT_CATALOG {
        string id PK
        string title
        string description
        string icon
        int xp_reward
    }
    QUEST_CATALOG ||--o{ ACTIVE_QUEST : "define"
    QUEST_CATALOG {
        string id PK
        string title
        string description
        int target
        int xp_reward
        int min_level
    }
```

**Estratégia de armazenamento:**
- 🟨 `state.json` → `PROFILE`, `UNLOCKED_ACHIEVEMENT`, `ACTIVE_QUEST` (pequeno, legível, escrita atômica).
- 🟦 `history.db` (SQLite) → `SESSION`, `COMMAND` (muitos inserts, indexado por `executed_at`).
- 🟩 Catálogos (`ACHIEVEMENT_CATALOG`, `QUEST_CATALOG`) são compilados no binário, não persistidos.

### 🔌 Sistema de Plugins
- [ ] Handlers customizados via WASM
- [ ] Conquistas definidas pelo usuário (Lua)

---

## Próximos Passos

- 📖 Leia a [Referência da API](../api/core-modules.md)
- 🔧 Confira o [Processo de Build](../../README.md#instalação)
- 🎮 Explore as [Mecânicas de Gamificação](../guides/gamification-system.md)
