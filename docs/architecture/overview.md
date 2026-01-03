# Architecture Overview

## Executive Summary

Munux implements a **reactive split-panel architecture** based on **The Elm Architecture (TEA)**, providing a robust Model-View-Update pattern that ensures predictable state management and smooth user interactions. The system combines traditional Unix shell execution with an intelligent gamification layer.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     USER INTERFACE LAYER                     │
│  ┌──────────────────────┐      ┌──────────────────────────┐ │
│  │  Terminal Panel      │      │  Reactive Panel          │ │
│  │  (60% - Left)        │      │  (40% - Right)           │ │
│  │                      │      │                          │ │
│  │  • Command Input     │      │  • File Tree             │ │
│  │  • Output Display    │      │  • File Preview          │ │
│  │  • History           │      │  • Resource Monitor      │ │
│  │  • Syntax Highlight  │      │  • Danger Zone           │ │
│  │                      │      │  • Stats/Quests          │ │
│  └──────────────────────┘      └──────────────────────────┘ │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              HUD (Status Bar)                        │   │
│  │  Level | XP Bar | Achievements | Streak | System    │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    APPLICATION CORE LAYER                    │
│                                                              │
│  ┌────────────────┐  ┌────────────────┐  ┌───────────────┐ │
│  │  Event Loop    │  │  Command       │  │  Game State   │ │
│  │  (Crossterm)   │→ │  Parser        │→ │  Manager      │ │
│  └────────────────┘  └────────────────┘  └───────────────┘ │
│                              │                              │
│                              ▼                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              State Management (TEA Pattern)            │ │
│  │                                                         │ │
│  │  Model  ────→  Update  ────→  View  ────→  Render     │ │
│  │    ↑                                           │       │ │
│  │    └───────────────────────────────────────────┘       │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      SYSTEM LAYER                            │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ Shell        │  │ File System  │  │ System Monitor   │  │
│  │ Executor     │  │ Manager      │  │ (CPU/RAM/Proc)   │  │
│  │ (sh/bash)    │  │              │  │                  │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Design Patterns

### 1. The Elm Architecture (TEA)

Munux implements TEA for predictable state management:

```rust
// Model - Single source of truth
pub struct App {
    input_buffer: String,
    game_state: GameState,
    right_panel_mode: RightPanelMode,
    // ...
}

// Update - State transformations
impl App {
    pub fn execute_command(&mut self) -> Result<()> {
        // Pure state transformation
    }
}

// View - Pure rendering
pub fn render(frame: &mut Frame, app: &App) {
    // Immutable rendering from state
}
```

**Benefits:**
- Predictable state updates
- Easy to test and debug
- Clear separation of concerns
- Unidirectional data flow

### 2. Strategy Pattern - Command Classification

Commands are classified using a strategy pattern for extensibility:

```rust
pub enum CommandType {
    Navigation,
    FileOperation,
    PackageManager,
    Dangerous,
    // ... 11 total types
}

impl CommandParser {
    pub fn classify_command(input: &str) -> CommandType {
        // Strategy-based classification
    }
}
```

### 3. Observer Pattern - Reactive Panels

The right panel observes input buffer changes and reacts accordingly:

```rust
fn analyze_input(&mut self) {
    let cmd_type = CommandParser::classify_command(&self.input_buffer);
    
    self.right_panel_mode = match cmd_type {
        CommandType::Dangerous => RightPanelMode::DangerZone { /* ... */ },
        CommandType::SystemMonitoring => RightPanelMode::ResourceMonitor { /* ... */ },
        // ... reactive transformations
    };
}
```

### 4. State Pattern - Panel Modes

Different panel modes encapsulate different behaviors:

```rust
pub enum RightPanelMode {
    Welcome,
    FileTree { path: PathBuf },
    FilePreview { path: PathBuf, content: String, language: String },
    ResourceMonitor { cpu_usage: f32, memory_used: u64, /* ... */ },
    DangerZone { warning: String, command: String },
    Stats,
    Quests,
    Help { content: String, title: String },
    EasterEgg { content: String },
}
```

## Component Interaction

### Typical Command Execution Flow

```
1. User Types Command
         │
         ▼
2. Event Handler (main.rs)
         │
         ▼
3. Input Buffer Updated (app.rs)
         │
         ├─→ 4a. analyze_input() → Update Panel Mode
         │
         └─→ 4b. On Enter: execute_command()
                   │
                   ├─→ Check Easter Eggs
                   │
                   ├─→ Execute Shell Command
                   │
                   ├─→ Check Achievements
                   │
                   ├─→ Update Quests
                   │
                   ├─→ Calculate XP Reward
                   │
                   └─→ Update Streak
         │
         ▼
5. Render Updated State (ui/mod.rs)
         │
         ├─→ render_terminal_panel()
         │
         ├─→ render_reactive_panel()
         │
         └─→ render_hud()
```

## Performance Considerations

### 1. Lazy Rendering
Only components that changed are re-rendered using Ratatui's differential rendering.

### 2. Command Parsing Optimization
```rust
// Fast path for common commands
match first_word {
    "ls" | "cd" | "pwd" => CommandType::Navigation,
    // Compiled to jump table by LLVM
}
```

### 3. Resource Monitoring Throttling
System metrics are only updated when `ResourceMonitor` panel is active:

```rust
if matches!(app.right_panel_mode, RightPanelMode::ResourceMonitor { .. }) {
    update_system_monitor(app);
}
```

### 4. Zero-Copy String Operations
Extensive use of string slices (`&str`) instead of owned strings where possible.

## Security Model

### Command Validation Layer
```
User Input
    │
    ▼
Dangerous Command Detection
    │
    ├─→ Safe → Execute
    │
    └─→ Dangerous → Show Warning
              │
              ├─→ ESC → Cancel
              │
              └─→ Enter → Execute with Confirmation
```

### Protection Mechanisms

1. **Dangerous Command Detection**
   - Pattern matching for destructive operations
   - Real-time visual warnings
   - Explicit user confirmation required

2. **Shell Injection Prevention**
   - Commands executed via controlled shell interface
   - No direct string interpolation
   - User input sanitization

3. **Filesystem Boundaries**
   - Respects user permissions
   - No privilege escalation (unless explicit `sudo`)

## Scalability

### Modular Architecture
Each component is loosely coupled and can be extended independently:

```
src/
├── core/          # Business logic (stateless)
├── ui/            # Presentation layer (pure functions)
├── game/          # Gamification engine (isolated)
└── app.rs         # State orchestration
```

### Plugin-Ready Design
The architecture supports future plugin systems:
- Custom command handlers
- Theme extensions
- Achievement modules
- Quest generators

## Error Handling Strategy

```rust
// Layered error handling
pub type Result<T> = anyhow::Result<T>;

// User-facing errors are caught and displayed gracefully
match self.execute_command() {
    Ok(_) => self.record_success(),
    Err(e) => {
        self.last_output = format!("✗ Error: {}", e);
        self.record_failure();
    }
}
```

## Testing Strategy

1. **Unit Tests** - Core business logic
2. **Integration Tests** - Component interactions
3. **Manual Testing** - UI/UX validation
4. **Performance Benchmarks** - Rendering performance

## Future Architecture Improvements

- [ ] Plugin system with dynamic loading
- [ ] Network-based multiplayer support
- [ ] Cloud save/sync for game state
- [ ] WebAssembly compilation for browser deployment
- [ ] AI-powered command suggestions

---

**Next:** [Component Breakdown](components.md) for detailed component documentation.
