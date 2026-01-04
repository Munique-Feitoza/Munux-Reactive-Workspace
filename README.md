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
+---------------------------------+------------------------------+
| FULL TERMINAL (60%)             | REACTIVE PANEL (40%)         |
|                                 |                              |
| ➜ [Beginner@munux]$             | 🐧 WELCOME TO MUNUX          |
| pacman -Syu                     | 📊 STATS & PROGRESS          |
|   ✓ System updated!             | ━━━━━━━━━━━━━━━━━━━━━━━━━    |
| 🏆 Arch User - BTW, I use Arch! | Level 5 | XP: 450/500        |
|                                 | Streak: 12 🔥                |
| yay -S firefox                  | 📋 ACTIVE QUESTS             |
|   ✓ Installing Firefox...       | ☑ First pacman (2/2)         |
| +50 XP! 🎯 Quest complete!      | ☐ Git Explorer (0/5)         |
|                                 | ☐ Network Master (1/10)      |
+---------------------------------+------------------------------+
| [Lv 5 - Terminal] XP: 450/500 ▰▰▰▰▰▱▱▱ | 🏆 12 | 🔥 Streak: 12 |
+---------------------------------+------------------------------+
```

---

<div align="center">

## 🚀 Main Features

*What makes Munux different from traditional terminals?*

</div>

### 1. 🐧 Fully Functional Terminal

**ALL Linux commands work normally!** Munux executes real commands via shell.

<table align="center">
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
<table>
<tr>
<td width="50%">

**🎯 Milestones**
- 🎯 10 Commands - "Getting Started"
- 🚀 50 Commands - "Regular User"
- 💎 100 Commands - "Power User"
- 👑 500 Commands - "Terminal Master"
</td>
</tr>

<table>
<tr>
<td width="50%">


**🔥 Streaks**
- 🔥 7-Day Streak - "Consistent"
- 🔥 30-Day Streak - "Dedicated"
- 🔥 100-Day Streak - "Unstoppable"

</td>
<td width="50%">

**🚀 Special**
- 🎯 Quest Master - Complete all quests
- 🥚 Easter Egg Hunter - Find hidden commands
- 🌈 Legend Status - Reach level 50

</td>
</tr>
</table>

#### 🎯 Dynamic Quests

Quests adapt to your level:
- 🌱 **Beginner** (Lv 1-9): "Execute your first ls", "Create a file with touch"
- 💻 **Terminal** (Lv 10-19): "Navigate to /home", "Install a package"
- 🔓 **Hacker** (Lv 20-29): "Configure Git", "Use SSH to connect"
- 🌃 **Cyberpunk** (Lv 30-39): "Compile a program", "Create a systemd service"
- 👑 **Elite** (Lv 40-49): "Write a shell script", "Optimize kernel parameters"  
**🎮 Interactive Modes:**
- 🚨 **Danger Zone** - Red alerts for risky commands
- 📈 **Stats Panel** - Success rate, streak, totals
- 🎯 **Quests Panel** - Active missions with progress
- 🥚 **Easter Eggs** - Special ASCII art surprises

</td>
</tr>
</table>

---
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
- `sudo su` - Uncle Ben
- `hack the planet` - Hackers
- `konami code` - Secret bonus

</td>
</tr>
</table>

---

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

</div>

```

### 6. 🛡️ Special Munux Commands

```
- `stats` - Shows detailed statistics and progress
- `quests` - Displays active missions with progress bars
- `achievements` - Lists all unlocked achievements
- `xp` - Shows current XP and level
- `xp <number>` - Adds XP (for testing/debugging)t blocks and Popups

---

```

<div align="center">

## 📦 Installation and Usage

*Get started in less than 5 minutes!*

</div>

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

<div align="center">

## ⌨️ Controls

</div>

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

<div align="center">

## 🔬 Technologies Used

</div>

| Crate | Version | Purpose |
|-------|---------|---------|
| **ratatui** | 0.26.3 | TUI (Terminal User Interface) framework |
| **crossterm** | 0.27.0 | Cross-platform terminal manipulation |
| **sysinfo** | 0.30.13 | System information collection (CPU, RAM, Swap) |
| **serde** | 1.0 | Serialization/deserialization (progress, state) |
| **chrono** | 0.4 | Timestamps and time management |
| **anyhow** | 1.0 | Ergonomic error handling |

---

<div align="center">

## 🛣️ Roadmap

</div>

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

Complete documentation is available in the [`docs/`](docs/) directory:

</div>


### For Users
- **[Quick Start Guide](docs/guides/quick-start.md)** - Get started in 5 minutes
- **[Installation Guide](docs/guides/installation.md)** - Detailed installation instructions
- **[Gamification System](docs/guides/gamification-system.md)** - Understanding XP, achievements, and quests
- **[Package Managers Guide](docs/guides/package-managers.md)** - Multi-distro package manager support
- **[Troubleshooting](docs/guides/troubleshooting.md)** - Common issues and solutions
- **[Testing Guide](docs/TESTING.md)** - Comprehensive testing documentation

### For Developers
- **[Architecture Overview](docs/architecture/overview.md)** - High-level design
- **[Component Breakdown](docs/architecture/)** - Detailed component documentation
- **[Core Modules API](docs/api/core-modules.md)** - Technical API documentation
- **[Build Status](docs/BUILD_STATUS.md)** - Build information and status
- **[Changelog](docs/CHANGELOG.md)** - Version history and updates

### For Contributors
- **[Code of Conduct](docs/contributing/code-of-conduct.md)** - Community standards
- **[Contributing Guide](docs/contributing/)** - How to contribute
- **[Project Stats](docs/PROJECT_STATS.md)** - Project statistics and metrics

---

<div align="center">

## ❓ FAQ

*Frequently Asked Questions*

</div>

**Q: Does Munux replace my terminal?**  
A: Yes! Munux is a fully functional terminal. All Linux commands work normally.

**Q: Which distro does it work on?**  
A: Works on **any Linux distro**. Tested on Manjaro, Ubuntu, Fedora, Debian, Arch.

**Q: How do I unlock all themes?**  
A: Level up! New themes are unlocked at levels 10, 20, 30, 40, and 50.

---

<div align="center">

## 👤 Author

*Created with passion for the Linux community*

</div>

**Munique Alves Pacheco Feitoza**

[![GitHub](https://img.shields.io/badge/GitHub-Munique--Feitoza-181717?style=for-the-badge&logo=github)](https://github.com/Munique-Feitoza)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-Munique%20Feitoza-0077B5?style=for-the-badge&logo=linkedin)](https://linkedin.com/in/munique-feitoza)

---
<div align="center">

## 🙏 Acknowledgments

</div>

- Rust Community: For the incredible ecosystem and support.
- Ratatui: For the best TUI library in existence.
- Arch/Manjaro Community: For the inspiration behind the package manager integration.
- You: For using and contributing to Munux!


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
│                                                         │
│  🐧 Munux Reactive Workspace                            │
│  Learning terminal commands, one XP at a time. 🚀       │
└─────────────────────────────────────────────────────────┘
```

---

**Copyright © 2026 Munique Alves Pacheco Feitoza**

Licensed under [GNU GPL v3.0](LICENSE) • [Report Bug](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues) • [Request Feature](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues)


</div>
