# Gamification System

## Overview

Munux implements a comprehensive gamification engine that transforms traditional command-line learning into an engaging RPG-like experience. The system is built on four core pillars: **Experience Points (XP)**, **Achievements**, **Quests**, and **Streaks**.

## Experience Points (XP)

### XP Calculation

XP rewards are dynamically calculated based on command complexity and execution success:

```rust
Base XP = match command_type {
    Navigation     => 5 XP
    FileOperation  => 10 XP
    FileViewing    => 8 XP
    PackageManager => 50 XP
    NetworkTools   => 30 XP
    SystemAdmin    => 40 XP
    VersionControl => 25 XP
    Dangerous      => 25 XP (with warning)
}

Streak Multiplier = match streak {
    0-4   => 1.0x
    5-9   => 1.1x  (+10%)
    10-24 => 1.25x (+25%)
    25+   => 1.5x  (+50%)
}

Final XP = Base XP × Streak Multiplier
```

### Level Progression

| Level Range | Tier | XP Required | Total XP |
|-------------|------|-------------|----------|
| 1-4 | **Iniciante** | 100/level | 0-400 |
| 5-9 | **Aprendiz** | 150/level | 400-1,150 |
| 10-19 | **Terminal** | 200/level | 1,150-3,150 |
| 20-29 | **Hacker** | 250/level | 3,150-5,650 |
| 30-39 | **Cyberpunk** | 300/level | 5,650-8,650 |
| 40-49 | **Elite** | 400/level | 8,650-12,650 |
| 50+ | **Legend** | 500/level | 12,650+ |

### Leveling Benefits

Each tier unlock brings visual and functional changes:

#### Visual Evolution
- **Theme Colors**: Cyan → Matrix Green → Hacker Cyan → Cyberpunk Magenta → Elite Purple → Legend Rainbow
- **Tux Character**: 6 different ASCII art variations (Basic → Terminal → Hacker → Cyberpunk → Elite → Legend)
- **Prompt Symbol**: ➜ → ► → ▶ → ◆ → ⬢ → ⬣
- **Motivational Messages**: Context-aware level-up messages

#### Functional Unlocks
- **Level 1-4**: Basic navigation and file operations
- **Level 5**: File manipulation commands unlocked
- **Level 10**: Text editors and permissions
- **Level 20**: Network tools and Git access
- **Level 30**: Package managers and containerization
- **Level 40**: Advanced system administration

## Achievements System

### Achievement Categories

#### 1. First Steps (7 achievements)
```yaml
first_command:
  name: "First Command"
  description: "The Journey Begins"
  trigger: Execute any command
  reward: 50 XP

first_ls:
  name: "First LS"
  description: "Listing Master"
  trigger: Execute `ls` command
  reward: 20 XP

first_cd:
  name: "First CD"
  description: "Navigator"
  trigger: Execute `cd` command
  reward: 20 XP

first_file:
  name: "Creator"
  description: "Created your first file"
  trigger: Execute `touch` or `echo >`
  reward: 30 XP

first_rm:
  name: "Destroyer"
  description: "Removed your first file"
  trigger: Execute `rm` command
  reward: 25 XP

first_sudo:
  name: "With Great Power..."
  description: "Executed first sudo command"
  trigger: Execute `sudo` command
  reward: 100 XP
```

#### 2. Package Managers (5 achievements)
```yaml
first_pacman:
  name: "Arch User - BTW, I use Arch!"
  emoji: "🏔️"
  trigger: Execute `pacman` command
  reward: 50 XP

first_apt:
  name: "Debian Disciple"
  emoji: "📦"
  trigger: Execute `apt` command
  reward: 50 XP

first_git:
  name: "Version Control Initiate"
  emoji: "🌿"
  trigger: Execute `git` command
  reward: 50 XP

first_ssh:
  name: "Remote Connection Established"
  emoji: "🔐"
  trigger: Execute `ssh` command
  reward: 40 XP

first_systemctl:
  name: "System Controller"
  emoji: "⚙️"
  trigger: Execute `systemctl` command
  reward: 40 XP
```

#### 3. Milestones (4 achievements)
```yaml
commands_10:
  name: "Getting Started"
  trigger: Execute 10 total commands
  reward: 100 XP

commands_50:
  name: "Regular User"
  trigger: Execute 50 total commands
  reward: 200 XP

commands_100:
  name: "Power User"
  trigger: Execute 100 total commands
  reward: 500 XP

commands_500:
  name: "Terminal Master"
  trigger: Execute 500 total commands
  reward: 1000 XP
```

#### 4. Streak Achievements (3 achievements)
```yaml
streak_5:
  name: "On Fire!"
  emoji: "🔥"
  trigger: Achieve 5 consecutive successful commands
  reward: 50 XP

streak_10:
  name: "Unstoppable!"
  emoji: "🔥🔥"
  trigger: Achieve 10 consecutive successful commands
  reward: 150 XP

streak_25:
  name: "Legendary!"
  emoji: "🔥🔥🔥"
  trigger: Achieve 25 consecutive successful commands
  reward: 500 XP
```

#### 5. Level Achievements (5 achievements)
```yaml
level_5:
  name: "Novice Complete"
  trigger: Reach level 5
  reward: 100 XP

level_10:
  name: "Terminal User"
  trigger: Reach level 10
  reward: 200 XP

level_20:
  name: "Hacker Achieved"
  trigger: Reach level 20
  reward: 500 XP

level_30:
  name: "Cyberpunk Elite"
  trigger: Reach level 30
  reward: 1000 XP

level_50:
  name: "Legend Status"
  trigger: Reach level 50
  reward: 2000 XP
```

## Quest System

### Quest Generation

Quests are dynamically generated based on your current level:

```rust
pub enum QuestObjective {
    ExecuteCommand { command: String, count: u32 },
    CreateFile { count: u32 },
    CreateDirectory { count: u32 },
    NavigateTo { path: String },
    ReadFile { count: u32 },
    DeleteFile { count: u32 },
    ReachLevel { target_level: u32 },
    ExecuteAnyCommands { count: u32 },
}
```

### Quest Examples by Level

#### Level 1-5: Basic Operations
```yaml
- objective: "Execute your first ls command"
  type: ExecuteCommand(ls, 1)
  reward: 50 XP

- objective: "Navigate to /home directory"
  type: NavigateTo(/home)
  reward: 30 XP

- objective: "Create a file with touch"
  type: CreateFile(1)
  reward: 40 XP
```

#### Level 6-15: File Manipulation
```yaml
- objective: "Use grep to search text"
  type: ExecuteCommand(grep, 1)
  reward: 60 XP

- objective: "Create 3 directories"
  type: CreateDirectory(3)
  reward: 80 XP

- objective: "Read 5 different files"
  type: ReadFile(5)
  reward: 100 XP
```

#### Level 16-30: Advanced Operations
```yaml
- objective: "Configure Git with your name"
  type: ExecuteCommand(git config, 2)
  reward: 150 XP

- objective: "Install a package"
  type: ExecuteCommand(pacman/apt, 1)
  reward: 200 XP

- objective: "Connect via SSH"
  type: ExecuteCommand(ssh, 1)
  reward: 180 XP
```

#### Level 31+: Expert Tasks
```yaml
- objective: "Master systemctl (5 commands)"
  type: ExecuteCommand(systemctl, 5)
  reward: 300 XP

- objective: "Reach level 50"
  type: ReachLevel(50)
  reward: 2000 XP
```

### Quest Tracking

- **Active Quests**: 3 simultaneous quests
- **Progress Display**: Real-time progress bars in `quests` panel
- **Auto-completion**: Quests complete automatically when objectives are met
- **XP Rewards**: Bonus XP awarded on completion
- **Quest Refresh**: New quests generated after completion

## Streak System

### Mechanics

```rust
// Streak increases on successful command
fn record_success(&mut self) {
    self.command_streak += 1;
    self.successful_commands += 1;
    
    // Check for streak achievements
    if self.command_streak == 5 { /* unlock achievement */ }
    if self.command_streak == 10 { /* unlock achievement */ }
    if self.command_streak == 25 { /* unlock achievement */ }
}

// Streak resets on failure
fn record_failure(&mut self) {
    self.command_streak = 0;
    self.failed_commands += 1;
}
```

### Streak Bonuses

| Streak | Multiplier | Bonus |
|--------|------------|-------|
| 0-4 | 1.0x | None |
| 5-9 | 1.1x | +10% XP |
| 10-24 | 1.25x | +25% XP |
| 25+ | 1.5x | +50% XP |

### Streak Notifications

```
🔥 Streak: 5  → "You're on fire!"
🔥🔥 Streak: 10 → "Unstoppable!"
🔥🔥🔥 Streak: 25 → "LEGENDARY STREAK!"
```

## Statistics Tracking

### Metrics Collected

```rust
pub struct GameState {
    // XP and Level
    pub xp: u32,
    pub level: u32,
    
    // Command Statistics
    pub total_commands: u32,
    pub successful_commands: u32,
    pub failed_commands: u32,
    pub command_streak: u32,
    
    // Progress Tracking
    pub achievements: Vec<Achievement>,
    pub active_quests: Vec<Quest>,
    
    // Timestamps
    pub session_start: DateTime<Utc>,
}
```

### Success Rate Calculation

```rust
pub fn success_rate(&self) -> f32 {
    if self.total_commands == 0 {
        return 100.0;
    }
    (self.successful_commands as f32 / self.total_commands as f32) * 100.0
}
```

### Stats Panel

Access via `stats` command to see:
- Total commands executed
- Success rate percentage
- Current streak
- Achievements unlocked (count)
- Active quests with progress
- Level and XP progress

## Persistence (Future Feature)

### Planned Save System

```rust
// Save game state to JSON
pub fn save_state(&self) -> Result<()> {
    let save_path = dirs::data_dir()
        .unwrap()
        .join("munux")
        .join("save.json");
    
    serde_json::to_writer_pretty(File::create(save_path)?, &self)?;
    Ok(())
}

// Load game state from JSON
pub fn load_state() -> Result<GameState> {
    let save_path = dirs::data_dir()
        .unwrap()
        .join("munux")
        .join("save.json");
    
    let file = File::open(save_path)?;
    Ok(serde_json::from_reader(file)?)
}
```

## Easter Eggs

Hidden commands that unlock special achievements:

- `sl` - Steam locomotive animation
- `fortune` - Random Linux/programming quotes
- `cowsay` - Customizable ASCII cow
- `matrix` / `hack` - Matrix-style messages
- `sudo su` - Uncle Ben quote
- `hack the planet` - Hackers (1995) reference
- `konami code` - Secret bonus

## Best Practices

### Maximizing XP Gain
1. **Maintain High Streaks**: Avoid errors to keep multiplier active
2. **Complete Quests**: Bonus XP rewards
3. **Use Complex Commands**: Higher base XP (e.g., package managers)
4. **Explore New Commands**: First-time bonuses via achievements

### Efficient Leveling
1. Focus on quest objectives
2. Use package manager commands (50 XP base)
3. Maintain 10+ streak for 25% bonus
4. Complete all available quests before leveling

---

**Next:** [Package Managers Guide](package-managers.md) for distribution-specific commands.
