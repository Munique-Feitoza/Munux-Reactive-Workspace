# 🏗️ Architecture Overview

> [!NOTE]
> This document describes the high-level design of Munux v0.1.0. For implementation details, see the API reference in [core-modules.md](../api/core-modules.md).

## Executive Summary

Munux implements a **reactive split-panel architecture** based on **The Elm Architecture (TEA)**. It combines the raw power of a Unix shell with an intelligent, state-aware UI layer that provides real-time context and gamification.

---

## High-Level Architecture

```mermaid
graph TD
    User((User)):::userCls --> Input[Terminal Input]:::uiCls

    subgraph UI_Layer [🎨 User Interface Layer]
        Input --> LeftPanel[Terminal Panel 60%]:::uiCls
        RightPanel[Reactive Panel 40%]:::uiCls
        HUD[Heads Up Display]:::uiCls
    end

    subgraph Core_Layer [⚙️ Application Core - TEA]
        EventLoop(Event Loop / Crossterm):::coreCls --> Parser{Command Parser}:::coreCls
        Parser -->|Navigation| NavState[Nav State]:::coreCls
        Parser -->|System| SysState[System State]:::coreCls
        Parser -->|Dangerous| DangerState[Danger State]:::dangerCls
        Parser -->|SSH| SshState[SSH State]:::sshCls

        StateMgr[State Management TEA]:::coreCls
        NavState --> StateMgr
        SysState --> StateMgr
        DangerState --> StateMgr
        SshState --> StateMgr

        StateMgr -->|Update View| RightPanel
        StateMgr -->|Update Stats| HUD
    end

    subgraph System_Layer [💻 System Layer]
        Shell[Shell Executor sh/bash]:::sysCls
        FS[File System]:::sysCls
        Monitor[SysInfo Monitor]:::sysCls
        Git[Git Integration]:::sysCls

        StateMgr -->|Execute| Shell
        StateMgr -->|Read| FS
        StateMgr -->|Poll| Monitor
        StateMgr -->|Status| Git
    end

    subgraph Remote_Layer [🌐 Remote Layer]
        SshSession[SSH Session ssh2]:::sshCls
        RemoteHost[(Remote Host)]:::remoteCls
        StateMgr -->|Tunnel| SshSession
        SshSession -->|TCP :22| RemoteHost
    end

    Shell --> Output[Command Output]:::uiCls
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
> 🟨 User · 🟦 UI · 🟪 Core · 🟩 System · 🟦 SSH · 🟥 Danger — color-coded for fast scanning.

---

## Core Class Diagram (UML)

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

    App "1" *-- "1" GameState : owns
    App "1" *-- "0..1" SshSession : optional
    App ..> CommandParser : uses
    App ..> ShellExecutor : uses
    App ..> SystemMonitor : uses
```

---

## Design Patterns

### 1. The Elm Architecture (TEA)

Munux ensures predictable state management through a unidirectional data flow.

```rust
// Model - Single source of truth
struct App { 
    state: State 
}

// Update - Pure state transformations
fn update(msg: Msg, model: &mut App) -> Command

// View - Immutable rendering
fn view(model: &App) -> Frame
```

> [!TIP]
> **Why TEA?** This pattern makes the application incredibly easy to debug. Since the view is a pure function of the state, we can reproduce any visual bug just by knowing the state data.

---

### 2. Strategy Pattern (Command Classification)

We categorize user input into 11 distinct strategies to determine UI reaction.

| Strategy | Trigger | UI Reaction |
|----------|---------|-------------|
| **Navigation** | `cd`, `ls`, `pwd` | File Tree visualization |
| **File Operations** | `touch`, `mkdir`, `cp` | File preview panel |
| **Monitoring** | `top`, `ps`, `htop` | Real-time graphs (CPU/RAM) |
| **Package Mgmt** | `pacman`, `apt`, `dnf` | Installation progress |
| **Network** | `ping`, `curl`, `wget` | Network status |
| **Dangerous** | `rm -rf`, `dd`, `sudo` | 🚨 Red Warning Panel |
| **Git** | `git` commands | Repository status |
| **Text Processing** | `cat`, `grep`, `sed` | Content preview |
| **Help** | `help`, `man` | Documentation viewer |
| **Easter Eggs** | `sl`, `cowsay`, `fortune` | Special animations |
| **Unknown** | Unrecognized | Suggestion system |

Each strategy determines: **XP reward**, **reactive panel mode**, and **achievement triggers**.

---

### 3. Observer Pattern (Reactive Panels)

The Right Panel "observes" the input buffer. As you type (before pressing Enter), the UI reacts.

```mermaid
sequenceDiagram
    participant User
    participant EventLoop
    participant AppState
    participant UI

    User->>EventLoop: Types "rm -rf"
    EventLoop->>AppState: Update Input Buffer
    AppState->>AppState: analyze_input()
    AppState-->>UI: Set Mode: DANGER
    UI-->>User: Render Red Panel 🚨
    
    User->>EventLoop: Press Enter
    EventLoop->>AppState: execute_command()
    AppState->>UI: Show Confirmation
```

---

## Component Breakdown

| Component | File | Responsibility |
|-----------|------|----------------|
| **Event Loop** | `event.rs` | Keyboard input (Crossterm), resize events, 60Hz polling |
| **App State** | `app.rs` | Game state (XP, level, achievements), command history |
| **Parser** | `core/parser.rs` | Command classification (regex), XP calculation, danger detection |
| **Shell Executor** | `core/shell.rs` | Executes via `sh -c`, captures stdout/stderr |
| **File System** | `core/filesystem.rs` | Directory listing, file preview, navigation |
| **Monitor** | `core/monitor.rs` | Real-time CPU/RAM/Swap metrics (SysInfo) |
| **Terminal Panel** | `ui/terminal.rs` | Left panel - command output display |
| **Reactive Panel** | `ui/reactive.rs` | Right panel - context-aware modes (9 types) |
| **HUD** | `ui/hud.rs` | Bottom bar - XP, level, streak, integrity |
| **Theme System** | `ui/theme.rs` | Progressive themes (6 tiers: Beginner→Legend) |
| **Stats Panel** | `ui/stats.rs` | Detailed statistics and quest tracking |
| **Popup System** | `ui/popup.rs` | Achievement notifications, warnings |
| **SSH Session** | `core/ssh.rs` | Remote shell via `ssh2` (agent / pubkey auth), remote cwd tracking |
| **Git Integration** | `core/git.rs` | Repository branch/status detection |
| **I18n** | `i18n.rs` | Runtime localization (EN / PT-BR) via Fluent |

---

## SSH Session Lifecycle (State Diagram)

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Connecting : user types<br/>`ssh user@host`
    Connecting --> Authenticating : TCP :22 OK
    Connecting --> Failed : TCP error
    Authenticating --> Active : ssh-agent /<br/>pubkey OK
    Authenticating --> Failed : auth error
    Active --> Active : execute(cmd)<br/>change_dir(path)
    Active --> Idle : user types<br/>`exit` / `logout`
    Failed --> Idle : popup shown
    Active --> [*] : App quits

    classDef active fill:#81d4fa,stroke:#0277bd,color:#000
    classDef bad fill:#ff6b6b,stroke:#c0392b,color:#fff
    class Active active
    class Failed bad
```

### SSH Command Flow (Sequence)

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant App as App (app.rs)
    participant Ssh as SshSession (core/ssh.rs)
    participant Host as Remote Host

    User->>App: ssh alice@box.dev
    App->>Ssh: SshSession::connect("alice","box.dev")
    Ssh->>Host: TCP connect :22
    Ssh->>Host: SSH handshake
    Ssh->>Host: userauth (agent → pubkey → ~/.ssh/id_rsa)
    Host-->>Ssh: auth ok
    Ssh->>Host: exec `pwd`
    Host-->>Ssh: remote_cwd
    Ssh-->>App: Ok(SshSession { host, user, remote_cwd })
    App-->>User: 🟢 popup "Connection Established"

    loop while ssh_session.is_some()
        User->>App: remote command
        App->>Ssh: execute(cmd)
        Ssh->>Host: channel.exec(cd $cwd && $cmd)
        Host-->>Ssh: stdout / stderr / exit_code
        Ssh-->>App: output
        App-->>User: render in Terminal Panel (cyan border)
    end

    User->>App: exit
    App->>Ssh: drop session
    App-->>User: 🔌 disconnected
```

---

## Data Flow

```mermaid
flowchart LR
    A([User Input]):::in --> B[Event Loop]:::core
    B --> C{Parser}:::core
    C --> D[State Update]:::core
    D --> E[Shell Execution]:::sys
    D --> S[SSH Execute]:::ssh
    E --> F[Capture Output]:::sys
    S --> F
    F --> G[View Render]:::ui
    G --> H([Display to User]):::out
    D -.->|Reactive| G

    classDef in fill:#ffd166,stroke:#d4a017,color:#000
    classDef out fill:#ffd166,stroke:#d4a017,color:#000
    classDef core fill:#b4a7f5,stroke:#6f42c1,color:#000
    classDef sys fill:#b8e994,stroke:#38a169,color:#000
    classDef ssh fill:#81d4fa,stroke:#0277bd,color:#000
    classDef ui fill:#a0e7e5,stroke:#17a2b8,color:#000
```

**Step-by-step:**
1. 🎹 User types command
2. 🔄 Event loop captures input (Crossterm)
3. 🔍 Parser classifies command type
4. 💾 State updated (XP, achievements, quests)
5. 🐚 Command executed via shell (`sh -c`)
6. 📋 Output captured (stdout/stderr)
7. 🎨 UI re-rendered based on new state
8. 🖥️ Display updates (60Hz refresh)

---

## Security Model

> [!WARNING]
> **Safety First**: Munux creates a safety layer, but actual commands are executed on the host system.

| Layer | Protection |
|-------|-----------|
| **Dangerous Command Detection** | Pattern matching intercepts `rm`, `dd`, `chmod` before execution |
| **Shell Isolation** | Commands run in isolated `sh -c` instances |
| **Permissions** | Munux respects standard Linux User/Group permissions |
| **No Privilege Escalation** | Never attempts to gain root access automatically |
| **Confirmation Dialogs** | Red warning panel + explicit confirmation for destructive ops |
| **State Isolation** | XP/achievements stored in memory only (no persistence) |

### File System Access
✅ Respects standard Linux permissions  
✅ Uses safe Rust APIs (no unsafe blocks)  
✅ Clean exit on `Ctrl+C` with terminal restoration

---

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| **Refresh Rate** | 60 Hz | Event loop polling frequency |
| **Memory Usage** | ~10-20 MB | Typical runtime footprint |
| **CPU (Idle)** | <1% | Minimal background processing |
| **CPU (Active)** | Spikes during execution | Shell commands inherit system load |
| **Startup Time** | <200ms | Release build cold start |

> [!TIP]
> Always use `cargo run --release` for production usage. Debug builds are 10-50x slower due to runtime checks.

---

## Future Enhancements

### 🗂️ Persistence Layer
- [ ] Save XP and achievements to `~/.munux/state.json`
- [ ] Command history across sessions (SQLite)
- [ ] Cloud sync for multi-device progression

#### Proposed Data Model (ER)

> [!NOTE]
> Not implemented yet. Sketch for the upcoming `~/.munux/state.json` (JSON) + `~/.munux/history.db` (SQLite) split.

```mermaid
erDiagram
    PROFILE ||--o{ SESSION : "has"
    PROFILE ||--o{ UNLOCKED_ACHIEVEMENT : "earned"
    PROFILE ||--o{ ACTIVE_QUEST : "tracking"
    PROFILE {
        string id PK
        string username
        int level
        int xp
        int max_streak
        datetime created_at
        datetime last_seen_at
    }
    SESSION ||--o{ COMMAND : "contains"
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
    ACHIEVEMENT_CATALOG ||--o{ UNLOCKED_ACHIEVEMENT : "defines"
    ACHIEVEMENT_CATALOG {
        string id PK
        string title
        string description
        string icon
        int xp_reward
    }
    QUEST_CATALOG ||--o{ ACTIVE_QUEST : "defines"
    QUEST_CATALOG {
        string id PK
        string title
        string description
        int target
        int xp_reward
        int min_level
    }
```

**Storage strategy:**
- 🟨 `state.json` → `PROFILE`, `UNLOCKED_ACHIEVEMENT`, `ACTIVE_QUEST` (small, human-readable, edited atomically).
- 🟦 `history.db` (SQLite) → `SESSION`, `COMMAND` (append-heavy, indexed by `executed_at`).
- 🟩 Catalogs (`ACHIEVEMENT_CATALOG`, `QUEST_CATALOG`) are compiled-in at build time, not persisted.

### 🌐 Network Features
- [x] ✅ **SSH integration** with reactive terminal view (via `ssh2`, v0.1.1+)
- [ ] Interactive password auth (currently agent + pubkey only)
- [ ] Remote system monitoring over SSH tunnels
- [ ] Distributed quest completion (team challenges)

### 🔌 Plugin System
- [ ] Custom command handlers via WASM
- [ ] User-defined achievements (Lua scripting)
- [ ] Theme marketplace

### 🤖 AI Assistance
- [ ] Command suggestions based on context (LLM integration)
- [ ] Error explanation and auto-fixes
- [ ] Natural language to shell translation

---

## Next Steps

- 📖 Read [Component API Reference](../api/core-modules.md)
- 🔧 Check [Build Process](../../README.md#installation)
- 🎮 Explore [Gamification Mechanics](../guides/gamification-system.md)
