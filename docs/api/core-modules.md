# Core Modules API Reference

## Overview

This document provides technical API documentation for Munux's core modules. These modules handle command parsing, shell execution, filesystem operations, and system monitoring.

## Command Parser Module

**Location**: `src/core/parser.rs`

### CommandType Enum

Classifies commands into 11 distinct categories:

```rust
pub enum CommandType {
    Navigation,        // cd, ls, pwd
    FileOperation,     // mkdir, touch, cp, mv, rmdir
    FileViewing,       // cat, less, more, head, tail
    SystemMonitoring,  // top, htop, ps, free
    Search,            // grep, find, locate
    Dangerous,         // rm, sudo, dd, fdisk, chmod
    VersionControl,    // git
    PackageManager,    // pacman, apt, dnf, zypper
    NetworkTools,      // ping, curl, wget, ssh
    Compression,       // tar, zip, gzip
    TextProcessing,    // sed, awk, cut, sort
    SystemAdmin,       // systemctl, journalctl
    MunuxSpecial,      // stats, quests, achievements
    EasterEgg,         // sl, fortune, cowsay
    Unknown,           // Unrecognized commands
}
```

### CommandParser

#### Methods

##### `classify_command`

Analyzes input and returns command type.

```rust
pub fn classify_command(input: &str) -> CommandType
```

**Parameters:**
- `input`: &str - The command string to analyze

**Returns:**
- `CommandType` - The classified command type

**Example:**
```rust
let cmd_type = CommandParser::classify_command("pacman -S firefox");
assert_eq!(cmd_type, CommandType::PackageManager);
```

##### `command_to_panel_mode`

Converts command input to appropriate panel mode.

```rust
pub fn command_to_panel_mode(input: &str, current_dir: &PathBuf) -> RightPanelMode
```

**Parameters:**
- `input`: &str - The command string
- `current_dir`: &PathBuf - Current working directory

**Returns:**
- `RightPanelMode` - The panel mode to display

**Example:**
```rust
let mode = CommandParser::command_to_panel_mode("cat file.txt", &current_dir);
// Returns FilePreview mode
```

##### `detect_language`

Detects programming language from filename extension.

```rust
pub fn detect_language(filename: &str) -> String
```

**Supported Languages:**
- Rust (.rs)
- Python (.py)
- JavaScript/TypeScript (.js, .ts)
- Bash (.sh)
- TOML (.toml)
- JSON (.json)
- Markdown (.md)

##### `requires_sudo`

Checks if command requires elevated privileges.

```rust
pub fn requires_sudo(input: &str) -> bool
```

**Returns:** `true` if command typically requires sudo

---

## Shell Executor Module

**Location**: `src/core/shell.rs`

### ShellExecutor

Handles safe command execution via system shell.

#### Methods

##### `execute`

Executes a shell command and returns output.

```rust
pub fn execute(command: &str) -> Result<String>
```

**Parameters:**
- `command`: &str - Command to execute

**Returns:**
- `Ok(String)` - Command output on success
- `Err(anyhow::Error)` - Error if execution fails

**Example:**
```rust
match ShellExecutor::execute("ls -la") {
    Ok(output) => println!("{}", output),
    Err(e) => eprintln!("Error: {}", e),
}
```

**Security:**
- Commands are executed via `sh -c` on Unix systems
- No shell injection vulnerabilities (commands run in isolated shell)
- User permissions respected

---

## Filesystem Manager Module

**Location**: `src/core/filesystem.rs`

### FileSystemManager

Manages file operations and directory navigation.

#### Methods

##### `read_file_preview`

Reads file content with size limit for preview.

```rust
pub fn read_file_preview(path: &Path) -> Result<String>
```

**Parameters:**
- `path`: &Path - Path to file

**Returns:**
- File content (max 10KB for performance)

##### `list_directory`

Lists directory contents with metadata.

```rust
pub fn list_directory(path: &Path) -> Result<Vec<DirEntry>>
```

##### `change_directory`

Changes current working directory.

```rust
pub fn change_directory(app: &mut App, path: &str) -> Result<()>
```

**Special Paths:**
- `~` - Home directory
- `-` - Previous directory
- `..` - Parent directory
- `/` - Root directory

---

## System Monitor Module

**Location**: `src/core/monitor.rs`

### SystemMonitor

Collects real-time system metrics.

#### SystemSummary Struct

```rust
pub struct SystemSummary {
    pub cpu_usage: f32,         // CPU percentage (0.0-100.0)
    pub memory_used: u64,       // Used memory in bytes
    pub memory_total: u64,      // Total memory in bytes
    pub swap_used: u64,         // Used swap in bytes
    pub swap_total: u64,        // Total swap in bytes
    pub process_count: usize,   // Number of running processes
}
```

#### Methods

##### `new`

Creates new system monitor instance.

```rust
pub fn new() -> Self
```

##### `get_system_summary`

Retrieves current system metrics.

```rust
pub fn get_system_summary(&mut self) -> SystemSummary
```

**Performance:**
- Updates throttled to prevent excessive CPU usage
- Cached for 1 second between updates

**Example:**
```rust
let mut monitor = SystemMonitor::new();
let summary = monitor.get_system_summary();
println!("CPU: {:.1}%", summary.cpu_usage);
println!("RAM: {} / {} MB", 
    summary.memory_used / 1024 / 1024,
    summary.memory_total / 1024 / 1024
);
```

---

## Application State Module

**Location**: `src/app.rs`

### App Struct

Main application state container.

```rust
pub struct App {
    pub input_buffer: String,
    pub command_history: Vec<String>,
    pub history_index: Option<usize>,
    pub right_panel_mode: RightPanelMode,
    pub game_state: GameState,
    pub current_dir: PathBuf,
    pub last_output: String,
    pub danger_mode_active: bool,
}
```

#### Key Methods

##### `execute_command`

Processes and executes user commands.

```rust
pub fn execute_command(&mut self) -> Result<()>
```

**Flow:**
1. Check for easter eggs
2. Process Munux special commands
3. Execute internal commands (cd)
4. Execute shell commands
5. Check achievements
6. Update quests
7. Calculate XP rewards
8. Update streak

##### `analyze_input`

Analyzes input and updates panel mode reactively.

```rust
fn analyze_input(&mut self)
```

**Triggers on:**
- Every keystroke in input buffer
- Determines appropriate panel mode
- Updates danger mode flag

##### `add_char` / `delete_char`

Input buffer manipulation.

```rust
pub fn add_char(&mut self, c: char)
pub fn delete_char(&mut self)
```

##### `history_previous` / `history_next`

Navigate command history.

```rust
pub fn history_previous(&mut self)
pub fn history_next(&mut self)
```

---

## Error Handling

All modules use `anyhow::Result<T>` for error handling:

```rust
use anyhow::{Result, Context};

pub fn risky_operation() -> Result<String> {
    let data = read_file(path)
        .context("Failed to read configuration file")?;
    Ok(data)
}
```

**Best Practices:**
- Use `.context()` to add error context
- Return `Result` for fallible operations
- Display user-friendly errors in UI

---

## Performance Considerations

### Command Parsing

- Uses match expressions (compiled to jump tables)
- O(1) complexity for most commands
- String slicing (`&str`) instead of allocation

### File Operations

- Preview size limited to 10KB
- Directory listings cached when possible
- Async I/O for large operations (future)

### System Monitoring

- Updates throttled to 1 second intervals
- Only active when ResourceMonitor panel displayed
- Minimal CPU overhead (<0.1%)

---

## Thread Safety

Current implementation is single-threaded. All state mutations happen on main thread through The Elm Architecture pattern.

**Future Considerations:**
- Async shell execution
- Background file operations
- Network operations threading

---

## Testing

### Unit Tests Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_package_manager() {
        assert_eq!(
            CommandParser::classify_command("pacman -S vim"),
            CommandType::PackageManager
        );
    }

    #[test]
    fn test_detect_rust_file() {
        assert_eq!(
            CommandParser::detect_language("main.rs"),
            "rust"
        );
    }
}
```

### Integration Tests

```rust
#[test]
fn test_command_execution_flow() {
    let mut app = App::new();
    app.input_buffer = "ls".to_string();
    app.execute_command().unwrap();
    assert!(app.game_state.total_commands > 0);
}
```

---

**Next:** [UI Components API](ui-components.md) for interface documentation.
