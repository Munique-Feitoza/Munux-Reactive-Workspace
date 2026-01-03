<div align="center">

# 🐧 Munux Reactive Workspace

### *Learning Terminal Commands, One XP at a Time* 🚀

[![Language](https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Framework](https://img.shields.io/badge/UI-Ratatui-blue?style=for-the-badge)](https://ratatui.rs/)
[![License](https://img.shields.io/badge/License-GPLv3-red?style=for-the-badge)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Beta%20v0.1.0-green?style=for-the-badge)](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/releases)

[![Made with Love](https://img.shields.io/badge/Made%20with-❤️%20and%20☕-ff69b4?style=for-the-badge)](https://github.com/Munique-Feitoza)
[![Linux](https://img.shields.io/badge/Platform-Linux-yellow?style=for-the-badge&logo=linux)](https://www.linux.org/)
[![Contributions Welcome](https://img.shields.io/badge/Contributions-Welcome-brightgreen?style=for-the-badge)](docs/contributing/code-of-conduct.md)

---

**Munux Reactive Workspace** is a next-generation **gamified terminal** for Linux, designed for both **education** AND **daily use**.

Unlike traditional terminals, Munux combines a **fully functional terminal** with a **complete gamification system** (XP, levels, achievements, quests) and **reactive panels** that adapt to what you're doing. Use it as your main terminal on Manjaro, Ubuntu, Fedora, or any Linux distro!

[Features](#-main-features) • [Installation](#-installation-and-usage) • [Documentation](#-documentation) • [Contributing](#-contributing) • [License](#-license)

---

### 🎥 Quick Demo

```bash
# Clone and run in 30 seconds
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo run
```

**What you'll see:**
- 🐧 Tux the penguin welcoming you
- 📊 Real-time XP tracking
- 🏆 Achievement unlocks
- 🎯 Active quest system
- 🎨 Beautiful cyberpunk themes

</div>

---

<div align="center">

## 🎮 The Concept: "Real Terminal + RPG"

*Imagine a terminal where every command earns XP, unlocks achievements, and levels you up! 🎯*

</div>

```text
+--------------------------------+------------------------------+
| FULL TERMINAL (60%)            | REACTIVE PANEL (40%)         |
|                                |                              |
| ➜ [Beginner@munux]$           | 🐧 WELCOME TO MUNUX          |
| pacman -Syu                    | 📊 STATS & PROGRESS          |
|   ✓ System updated!            | ━━━━━━━━━━━━━━━━━━━━━━━━━   |
| 🏆 Arch User - BTW, I use Arch!| Level 5 | XP: 450/500       |
|                                | Streak: 12 🔥                |
| yay -S firefox                 | 📋 ACTIVE QUESTS             |
|   ✓ Installing Firefox...     | ☑ First pacman (2/2)         |
| +50 XP! 🎯 Quest complete!     | ☐ Git Explorer (0/5)         |
|                                | ☐ Network Master (1/10)      |
+--------------------------------+------------------------------+
| [Lv 5 - Terminal] XP: 450/500 ▰▰▰▰▰▱▱▱ | 🏆 12 | 🔥 Streak: 12 |
+--------------------------------+------------------------------+
```

---

<div align="center">

## 🚀 Main Features

*What makes Munux different from traditional terminals?*

</div>

### 1. 🐧 Fully Functional Terminal

**ALL Linux commands work normally!** Munux executes real commands via shell.

<table>
<tr>
<td width="50%">

**📦 Package Management**
- `pacman`, `yay`, `paru` (Arch/Manjaro)
- `apt`, `dpkg`, `snap` (Debian/Ubuntu)
- `dnf`, `yum` (Fedora/RHEL)
- `zypper` (openSUSE)
- `flatpak` (Universal)

**🌐 Network & Remote**
- `ping`, `curl`, `wget`
- `ssh`, `scp`, `rsync`
- `netstat`, `ip`, `ifconfig`

**🗜️ Compression & Archives**
- `tar`, `zip`, `unzip`
- `gzip`, `bzip2`, `xz`
- `7z`, `rar`

</td>
<td width="50%">

**⚙️ System Administration**
- `systemctl`, `service`
- `journalctl`, `dmesg`
- `sudo`, `chmod`, `chown`
- `ps`, `top`, `htop`, `kill`

**💻 Development Tools**
- `git`, `make`, `cmake`
- `gcc`, `g++`, `clang`
- `python`, `node`, `npm`
- `cargo`, `rustc`

**📝 Text Processing**
- `sed`, `awk`, `grep`
- `cat`, `less`, `more`
- `vim`, `nano`, `emacs`

</td>
</tr>
</table>
### 2. 🎮 Complete Gamification System

<div align="center">

#### 📊 XP and Levels (6 Tiers)

| Tier | Levels | Theme | Tux Form | Symbol | Color Scheme |
|------|--------|-------|----------|--------|--------------|
| 🌱 **Beginner** | 1-9 | Cyan | Basic | ➜ | Light Blue |
| 💻 **Terminal** | 10-19 | Matrix | Terminal | ► | Matrix Green |
| 🔓 **Hacker** | 20-29 | Cyber | Hacker | ▶ | Cyan/Green |
| 🌃 **Cyberpunk** | 30-39 | Neon | Cyberpunk | ◆ | Magenta/Cyan |
| 👑 **Elite** | 40-49 | Royal | Elite | ⬢ | Purple |
| ⭐ **Legend** | 50+ | Rainbow | Legendary | ⬣ | All Colors |

</div>
- **Beginner** (Lv 1-9): Cyan theme, basic Tux, ➜ symbol
- **Terminal** (Lv 10-19): Matrix Green theme, terminal Tux, ► symbol
- **Hacker** (Lv 20-29): Hacker theme, hacker Tux, ▶ symbol
- **Cyberpunk** (Lv 30-39): Magenta/Cyan theme, cyberpunk Tux, ◆ symbol
- **Elite** (Lv 40-49): Purple theme, elite Tux, ⬢ symbol
- **Legend** (Lv 50+): Rainbow theme, legendary Tux, ⬣ symbol

#### 🏆 Achievements (25+)

<table>
<tr>
<td width="50%">

**🌟 First Steps**
- ✅ First Command - "The Journey Begins"
- ✅ First LS - "Listing Master"
- ✅ First CD - "Navigator"
- ✅ First File - "Creator"
- ✅ First RM - "Destroyer"
- ✅ First Sudo - "With Great Power..."

</td>
<td width="50%">

**📦 Package Managers**
- 🏔️ First Pacman - "Arch User - BTW!"
- 📦 First APT - "Debian Disciple"
- 🌿 First Git - "Version Control Initiate"
- 🔐 First SSH - "Remote Connection"
- ⚙️ First Systemctl - "System Controller"

</td>
</tr>
<tr>
<td width="50%">

**🎯 Milestones**
- 🎯 10 Commands - "Getting Started"
- 🚀 50 Commands - "Regular User"
- 💎 100 Commands - "Power User"
- 👑 500 Commands - "Terminal Master"
### 4. 🥚 Easter Eggs (10+)

<table>
<tr>
<td width="33%">

**🎬 Animations**
- `sl` - ASCII train
- `matrix` - Matrix rain
- `hack` - Hacker mode

</td>
<td width="33%">

**💬 Interactive**
- `cowsay` - Talking cow
- `fortune` - Linux quotes
- `whoami` - Philosophy

</td>
<td width="33%">

**🎮 Hidden**
### 3. 🎯 Smart Reactive Panels

<table>
<tr>
<td width="50%">

**📋 Context-Aware Modes:**
- 🏠 **Welcome** - Evolutionary Tux + motivational messages
- 📁 **File Tree** - Auto-displays on `ls`, `ll`, `la`
- 📄 **File Preview** - Syntax highlighting for code
- 📊 **Resource Monitor** - Real-time CPU/RAM/Swap

</td>
<td width="50%">

**🎮 Interactive Modes:**
- 🚨 **Danger Zone** - Red alerts for risky commands
- 📈 **Stats Panel** - Success rate, streak, totals
- 🎯 **Quests Panel** - Active missions with progress
- 🥚 **Easter Eggs** - Special ASCII art surprises

</td>
</tr>
</table>
- "Execute your first ls"
- "Create a file with touch"
- "Navigate to /home"
- "Install a package"
- "Configure Git"
- Progress tracked in real time!

### 3. 🎯 Smart Reactive Panels

The right panel changes automatically based on what you type:

- **Welcome**: Evolutionary Tux + motivational message
- **File Tree**: Shows files when using `ls`, `ll`, `la`
- **File Preview**: Preview with syntax highlighting
- **Resource Monitor**: CPU/RAM/Swap when using `top`, `htop`
- **Danger Zone**: Red alert on destructive commands
- **Stats**: Detailed statistics (success %, streak, total)
- **Quests**: Active missions with progress bars
- **Easter Eggs**: Special ASCII art from secret commands

### 4. 🥚 Easter Eggs (10+)

Secret commands reveal surprises:
- `sl` - Animated ASCII train
- `cowsay` - Customizable talking cow
- `fortune` - Quotes about Linux and programming
- `matrix` / `hack` - Matrix-style messages
- `sudo su` - "With great power..." (Uncle Ben)
- `hack the planet` - Hackers (1995) reference
- `konami code` - Secret bonus
- And more hidden...

### 5. 📚 Integrated Help System

```bash
help          # List of special commands
help arch     # Manjaro/Arch guide (pacman, yay, paru)
help debian   # Ubuntu/Debian guide (apt, dpkg, snap)
---

<div align="center">

## 🏗️ Architecture

*Built with modern software engineering principles*

**The Elm Architecture** (Model-View-Update) • **Zero Unsafe Code** • **Idiomatic Rust**

</div>versal Linux commands
```

### 6. 🛡️ Special Munux Commands

- `stats` - Shows statistics and progress
- `quests` - Displays active missions
- `achievements` - Lists unlocked achievements
- `xp` - Shows current XP and level
- `xp <number>` - Adds XP (for testing)

---

## 🏗️ Architecture

This project follows **The Elm Architecture** (Model-View-Update):

```
src/
├── main.rs              # Entry point and main loop
├── app.rs               # Global state (Model) + command execution
├── event.rs             # Event handler (input)
├── tui.rs               # Terminal configuration (Crossterm)
│
├── ui/                  # View Layer
│   ├── mod.rs           # Main renderer
│   ├── layout.rs        # Defines Split blocks and Popups
│   ├── terminal.rs      # Renders left panel (terminal)
│   ├── reactive.rs      # Renders right panel (reactive context)
│   ├── hud.rs           # Renders status bar and XP
│   ├── stats.rs         # Renders Stats and Quests panels
│   ├── popup.rs         # Popups and confirmations
│   └── theme.rs         # Progressive theme system (6 tiers)
│
├── core/                # Business Logic (Update)
│   ├── parser.rs        # Parses commands (11 types)
│   ├── shell.rs         # Executes system commands
│   ├── filesystem.rs    # Manages file operations
│   └── monitor.rs       # Collects system metrics (CPU/RAM)
│
└── game/                # Complete Gamification System
    ├── state.rs         # XP, levels, achievements, quests, streaks
    ├── logic.rs         # Level Up rules and XP calculation
---

<div align="center">

## 📦 Installation and Usage

*Get started in less than 5 minutes!*

</div>aster eggs and secret commands
    └── distro_guide.rs  # Command guides by distribution
```

### Data Flow

1. **Event** (`event.rs`): User presses a key
2. **Update** (`app.rs`): Application state is updated
3. **Parser** (`core/parser.rs`): Analyzes input (Navigation, PackageManager, etc.)
4. **Execute** (`app.rs`): Command executed via shell + achievements/quests checks
5. **View** (`ui/`): Ratatui renders the next frame with appropriate theme
6. **Repeat**: Event-based reactive loop

---

## 📦 Installation and Usage

### Prerequisites

```bash
# Install Rust (if you don't have it yet)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Quick Installation

```bash
# Clone the repository
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace

# Compile and run
cargo run

# OR compile in release mode (faster)
cargo build --release
./target/release/munux-reactive-workspace
```

### 🎮 Getting Started

```bash
# 1. Start Munux
cargo run

# 2. Try basic commands (earn XP!)
ls
pwd
mkdir test
cd test
touch file.txt
echo "Hello Munux" > file.txt
cat file.txt

# 3. Check your progress
stats           # Detailed statistics
quests          # Active missions
achievements    # Unlocked achievements

# 4. Use package managers (unlocks achievements!)
# Manjaro/Arch:
pacman -Syu     # Updates system
yay -S firefox  # Installs Firefox

# Ubuntu/Debian:
---

<div align="center">

## ⌨️ Controls

*Simple and intuitive keyboard shortcuts*

</div>
# 5. Explore easter eggs
sl
fortune
cowsay "Munux is awesome!"

# 6. Get help
help            # List special commands
help arch       # Guide for Manjaro/Arch
help debian     # Guide for Ubuntu/Debian
```

---

## ⌨️ Controls

| Key | Action |
|-----|--------|
| **Type normally** | Adds characters to input buffer |
| **Enter** | Executes command (earn XP on success!) |
| **Backspace** | Removes last character |
| **↑ / ↓** | Navigate command history |
| **Ctrl+C** | Exits application |
| **Ctrl+L** | Clears screen |
| **ESC** | Clears current input or cancels dangerous command |
| **Tab** | (Future) Auto-complete |

---

## 🎮 Detailed Gamification System

### 📊 Level Progression (6 Tiers)

| Level | Tier | Theme | Tux | Symbol | XP Required |
|-------|------|-------|-----|--------|-------------|
| 1-9 | **Beginner** | Cyan | Basic | ➜ | 0-900 |
| 10-19 | **Terminal** | Matrix Green | Terminal | ► | 1000-1900 |
| 20-29 | **Hacker** | Cyan/Green | Hacker | ▶ | 2000-2900 |
| 30-39 | **Cyberpunk** | Magenta/Cyan | Cyberpunk | ◆ | 3000-3900 |
| 40-49 | **Elite** | Purple | Elite | ⬢ | 4000-4900 |
| 50+ | **Legend** | Rainbow | Legend | ⬣ | 5000+ |

**Visual Evolution**: Tux and colors evolve with you! The more you use, the more cyberpunk it gets.

### 🏆 Complete Achievements

#### First Steps
- ✅ **First Command** - "The Journey Begins" (50 XP)
- ✅ **First LS** - "Listing Master" (20 XP)
- ✅ **First CD** - "Navigator" (20 XP)
- ✅ **First File** - "Creator" (30 XP)
- ✅ **First RM** - "Destroyer" (25 XP)
- ✅ **First Sudo** - "With Great Power..." (100 XP)

#### Package Managers
- 🏔️ **First Pacman** - "Arch User - BTW, I use Arch!" (50 XP)
- 📦 **First APT** - "Debian Disciple" (50 XP)
- 🌿 **First Git** - "Version Control Initiate" (50 XP)
- 🔐 **First SSH** - "Remote Connection Established" (40 XP)
- ⚙️ **First Systemctl** - "System Controller" (40 XP)

#### Command Milestones
- 🎯 **10 Commands** - "Getting Started" (100 XP)
- 🚀 **50 Commands** - "Regular User" (200 XP)
- 💎 **100 Commands** - "Power User" (500 XP)
- 👑 **500 Commands** - "Terminal Master" (1000 XP)

#### Streaks
- 🔥 **5 Streak** - "On Fire!" (50 XP)
- 🔥🔥 **10 Streak** - "Unstoppable!" (150 XP)
- 🔥🔥🔥 **25 Streak** - "Legendary!" (500 XP)

#### Levels
- ⭐ **Level 5** - "Novice Complete" (100 XP)
- ⭐⭐ **Level 10** - "Terminal User" (200 XP)
- ⭐⭐⭐ **Level 20** - "Hacker Achieved" (500 XP)
- 💫 **Level 30** - "Cyberpunk Elite" (1000 XP)
- 🌟 **Level 50** - "Legend Status" (2000 XP)

### 📋 Quest System

Quests are dynamically generated based on your level:

**Level 1-5** (Basic):
- "Execute your first ls command"
- "Navigate to /home directory"
- "Create a file with touch"
- "Read a file with cat"

**Level 6-15** (Intermediate):
- "Use grep to search text"
- "Create a directory and navigate to it"
- "Execute 5 different commands"
- "Use pipes (|) for the first time"

**Level 16-30** (Advanced):
---

<div align="center">

## 🔬 Technologies Used

*Powered by the best tools in the Rust ecosystem*

</div>emotely"
- "Execute 10 network commands"

**Level 31+** (Elite):
- "Master systemctl (5 commands)"
- "Reach level 50"
- "Unlock all achievements"

### 🔥 Streak System

- **Each correct command**: +1 streak
- **Each error**: Streak resets to 0
- **Streak 5+**: +10% XP bonus
---

<div align="center">

## 🛣️ Roadmap

*The future of Munux is bright!*

</div>
**Tip**: Keep streak high to level up faster!

---

## 🔬 Technologies Used

| Crate | Version | Purpose |
|-------|---------|---------|
| **ratatui** | 0.26.3 | TUI (Terminal User Interface) framework |
| **crossterm** | 0.27.0 | Cross-platform terminal manipulation |
| **sysinfo** | 0.30.13 | System information collection (CPU, RAM, Swap) |
| **serde** | 1.0 | Serialization/deserialization (progress, state) |
| **chrono** | 0.4 | Timestamps and time management |
| **anyhow** | 1.0 | Ergonomic error handling |

---

## 🛣️ Roadmap

### ✅ Version 0.1.0 - COMPLETE
- [x] Base architecture (The Elm Architecture)
- [x] Reactive split screen
- [x] XP and level system (6 tiers)
- [x] Command parser (11 types)
- [x] Real execution via shell
- [x] Danger mode for destructive commands
- [x] Dynamic progressive themes
- [x] Achievement system (25+)
- [x] Dynamic quest system
- [x] Easter eggs (10+)
- [x] Streak system with bonuses
---

<div align="center">

## 🤝 Contributing

*Join the community and help make Munux even better!*

[![Contributors Welcome](https://img.shields.io/badge/Contributors-Welcome-brightgreen?style=flat-square)](docs/contributing/code-of-conduct.md)

</div>ts and progress
- [x] Evolutionary Tux (6 forms)

### 🚧 Version 0.2.0 - In Planning
- [ ] Progress persistence (save/load state)
- [ ] Auto-complete (Tab completion)
- [ ] Persistent history between sessions
- [ ] Syntax highlighting in file preview
- [ ] More easter eggs and secret achievements
- [ ] Custom alias system
- [ ] Interactive tutorial mode for beginners
- [ ] Benchmark mode (test typing speed)

### 🔮 Version 0.3.0 - Future
- [ ] Customizable themes (create your own)
- [ ] Plugins and extensions
- [ ] Competitive mode (leaderboards)
- [ ] GitHub integration (commits → XP)
- [ ] Multiplayer mode (compare progress)
---

<div align="center">

## 📚 Documentation

*Comprehensive guides for everyone*

</div>
---

## 🤝 Contributing

Contributions are welcome! This is an educational and open-source project.

### How to Contribute

1. Fork the project
2. Create a branch for your feature (`git checkout -b feature/MyFeature`)
3. Commit your changes (`git commit -m 'Add MyFeature'`)
4. Push to the branch (`git push origin feature/MyFeature`)
5. Open a Pull Request

### Areas That Need Help
---

<div align="center">

## ❓ FAQ

*Frequently Asked Questions*

</div>e achievements and easter eggs
- 📦 Support for more package managers (emerge, nix, etc.)
- 🌍 Translations (Spanish, French, etc.)
- 📖 Tutorials and educational quests
- 🧪 Tests and bug fixes

---

## 📚 Documentation

Complete documentation is available in the [`docs/`](docs/) directory:

### For Users
- **[Quick Start Guide](docs/guides/quick-start.md)** - Get started in 5 minutes
- **[Installation Guide](docs/guides/installation.md)** - Detailed installation instructions
- **[Gamification System](docs/guides/gamification-system.md)** - Understanding XP, achievements, and quests
- **[Package Managers Guide](docs/guides/package-managers.md)** - Multi-distro package manager support
- **[Troubleshooting](docs/guides/troubleshooting.md)** - Common issues and solutions
- **[Testing Guide](docs/TESTING.md)** - Comprehensive testing documentation

---

<div align="center">

## 💡 Inspirations

*Standing on the shoulders of giants*

</div>API](docs/api/core-modules.md)** - Technical API documentation
- **[Build Status](docs/BUILD_STATUS.md)** - Build information and status
- **[Changelog](docs/CHANGELOG.md)** - Version history and updates

---

<div align="center">

## 📄 License

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg?style=for-the-badge)](LICENSE)

</div>Stats](docs/PROJECT_STATS.md)** - Project statistics and metrics

---

## ❓ FAQ

**Q: Does Munux replace my terminal?**
A: Yes! Munux is a fully functional terminal. All Linux commands work normally.

**Q: Which distro does it work on?**
A: Works on **any Linux distro**. Tested on Manjaro, Ubuntu, Fedora, Debian, Arch.

---

<div align="center">

## 👤 Author

*Created with passion for the Linux community*

</div>
**Q: How do I unlock all themes?**
### **Munique Alves Pacheco Feitoza**

[![GitHub](https://img.shields.io/badge/GitHub-Munique--Feitoza-181717?style=for-the-badge&logo=github)](https://github.com/Munique-Feitoza)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-Munique%20Feitoza-0077B5?style=for-the-badge&logo=linkedin)](https://linkedin.com/in/munique-feitoza)

---

## 🙏 Acknowledgments

*Thank you to the amazing open-source community!*

</div>

## 💡 Inspirations

---

<div align="center">

## ⭐ Show Your Support

If you like this project, please consider:

[![GitHub Stars](https://img.shields.io/github/stars/Munique-Feitoza/Munux-Reactive-Workspace?style=social)](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace)
[![GitHub Forks](https://img.shields.io/github/forks/Munique-Feitoza/Munux-Reactive-Workspace?style=social)](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/fork)
[![GitHub Watchers](https://img.shields.io/github/watchers/Munique-Feitoza/Munux-Reactive-Workspace?style=social)](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace)

---

### Made with ❤️ and lots of ☕ using Rust 🦀

```
┌─────────────────────────────────────────────────────────┐
│  "The best way to learn is by doing.                    │
│   The best way to do is by playing."                    │
│                                                          │
│  🐧 Munux Reactive Workspace                            │
│  Learning terminal commands, one XP at a time. 🚀       │
└─────────────────────────────────────────────────────────┘
```

---

**Copyright © 2026 Munique Alves Pacheco Feitoza**

Licensed under [GNU GPL v3.0](LICENSE) • [Report Bug](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues) • [Request Feature](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues)

</div>u can use commercially
- ⚠️ Modifications must be open-source under GPLv3
- ⚠️ Must include copyright notice

See the [LICENSE](LICENSE) file for more details.

---

## 👤 Author

**Munique Alves Pacheco Feitoza**

- GitHub: [@Munique-Feitoza](https://github.com/Munique-Feitoza)
- LinkedIn: [Munique Feitoza](https://linkedin.com/in/munique-feitoza)
- Project: [Munux-Reactive-Workspace](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace)

---

## 🙏 Acknowledgments

- Inspired by the need for better educational tools to learn Linux
- Rust community for the amazing language
- Ratatui for the powerful TUI library
- Everyone who contributes to the open-source ecosystem

---

## ⭐ Show Your Support

If you liked the project, leave a ⭐ on the repository!

---

<div align="center">

**Made with ❤️ and lots of ☕ using Rust 🦀**

*"The best way to learn is by doing. The best way to do is by playing."*

**Munux Reactive Workspace** - *Learning terminal commands, one XP at a time.* 🚀

</div>
