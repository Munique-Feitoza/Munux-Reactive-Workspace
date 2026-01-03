# Quick Start Guide

Get up and running with Munux in under 5 minutes.

## Prerequisites

- **Operating System**: Linux (any distribution)
- **Rust**: 1.70 or higher
- **Terminal**: Any modern terminal emulator

## Installation

### Option 1: From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace

# Build and run
cargo run --release
```

### Option 2: Install Binary

```bash
# Build optimized binary
cargo build --release

# Copy to PATH (optional)
sudo cp target/release/munux-reactive-workspace /usr/local/bin/munux

# Run from anywhere
munux
```

## First Launch

When you start Munux for the first time, you'll see:

```
┌─────────────────────────┬──────────────────────────┐
│ Terminal Panel          │ Welcome Screen           │
│                         │                          │
│ ➜ [Aprendiz@munux]$    │      🐧 TUX              │
│                         │   [HACKER MODE]          │
│                         │                          │
│                         │ Level 1 - Iniciante      │
│                         │ XP: 0/100                │
│                         │                          │
│                         │ Commands to start:       │
│                         │  ls    - list files      │
│                         │  cd    - change dir      │
│                         │  pwd   - show location   │
│                         │  mkdir - create folder   │
│                         │  touch - create file     │
└─────────────────────────┴──────────────────────────┘
```

## Basic Commands

### 1. Navigation
```bash
ls          # List files in current directory
cd <dir>    # Change to directory
pwd         # Show current location
```

### 2. File Operations
```bash
touch file.txt    # Create a file
mkdir folder      # Create a folder
cat file.txt      # Show file contents
rm file.txt       # Remove file (CAREFUL!)
```

### 3. Special Munux Commands
```bash
stats         # Show your progress and statistics
quests        # View active quests/missions
achievements  # List unlocked achievements
xp            # Check your XP and level
help          # Show help system
```

### 4. Distribution-Specific Help
```bash
help arch     # Manjaro/Arch Linux commands (pacman, yay)
help debian   # Ubuntu/Debian commands (apt, dpkg)
help fedora   # Fedora/RHEL commands (dnf, rpm)
help opensuse # openSUSE commands (zypper)
help linux    # Universal Linux commands
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Enter` | Execute command |
| `Backspace` | Delete character |
| `↑` / `↓` | Navigate command history |
| `Ctrl+C` | Exit Munux |
| `Ctrl+L` | Clear screen |
| `ESC` | Cancel input / Exit special panels |

## Understanding the Interface

### Terminal Panel (Left - 60%)
- Your command input and output
- Command history
- Syntax highlighting for valid commands
- Real-time command execution

### Reactive Panel (Right - 40%)
The right panel changes based on what you type:

- **Default**: Welcome screen with Tux
- **`ls` / `ll` / `la`**: File tree view
- **`cat <file>`**: File preview
- **`top` / `htop`**: Resource monitor (CPU/RAM)
- **`rm -rf`**: Danger zone warning
- **`stats`**: Statistics panel
- **`quests`**: Active quests panel
- **`help`**: Help documentation

### HUD (Bottom)
Shows your current level, XP progress, achievements count, and streak.

## Gamification Basics

### XP and Levels
- Execute successful commands to earn XP
- Level up to unlock new features
- 6 progression tiers: Iniciante → Terminal → Hacker → Cyberpunk → Elite → Legend

### Achievements
- Unlock 25+ achievements
- Special badges for package manager usage
- Milestone achievements (10, 50, 100, 500 commands)

### Quests
- Dynamic missions based on your level
- Track progress in real-time
- Complete quests for bonus XP

### Streak System
- Consecutive successful commands build your streak
- Streak bonuses:
  - 5+ streak: +10% XP
  - 10+ streak: +25% XP
  - 25+ streak: +50% XP

## Example Session

```bash
# Start Munux
munux

# Try basic navigation
ls
cd Documents
pwd

# Create some files
mkdir test
cd test
touch hello.txt
echo "Hello Munux!" > hello.txt
cat hello.txt

# Check your progress
stats

# View your quests
quests

# Get help for your distro
help arch    # If on Manjaro/Arch

# Try an easter egg
fortune
sl
cowsay "I love Linux!"
```

## Tips for Beginners

1. **Watch the Right Panel**: It provides context for your commands
2. **Maintain Your Streak**: Consecutive correct commands = bonus XP
3. **Complete Quests**: They guide your learning progression
4. **Explore Easter Eggs**: Try unusual commands like `sl`, `fortune`, `cowsay`
5. **Use Help**: The `help` command provides detailed guides
6. **Be Careful with `rm`**: The danger zone will warn you about destructive commands

## Progression Milestones

| Level | Tier | Unlocks |
|-------|------|---------|
| 1-4 | Iniciante | Basic navigation |
| 5-9 | Aprendiz | File manipulation |
| 10-19 | Terminal | Text editors, permissions |
| 20-29 | Hacker | Network tools, Git |
| 30-39 | Cyberpunk | Package managers, Docker |
| 40+ | Elite | Advanced system administration |

## Getting Help

- **In-app help**: Type `help` for interactive guides
- **GitHub Issues**: [Report bugs or request features](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues)
- **Documentation**: Check [docs/](../README.md) for detailed guides

## Next Steps

1. ✅ Complete your first quest
2. ✅ Unlock your first achievement
3. ✅ Reach Level 5
4. ✅ Try package managers specific to your distro
5. ✅ Discover all easter eggs
6. ✅ Build a 25+ command streak

---

**Happy hacking! 🚀**

For advanced features, see:
- [Gamification System](gamification-system.md)
- [Package Managers Guide](package-managers.md)
- [Troubleshooting](troubleshooting.md)
