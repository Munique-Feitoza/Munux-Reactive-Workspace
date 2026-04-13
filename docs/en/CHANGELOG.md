# 📋 Changelog

All notable changes to Munux Reactive Workspace will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

![Version](https://img.shields.io/badge/Latest-v0.1.0-blue) ![Status](https://img.shields.io/badge/Status-Beta-yellow)

---

## [Unreleased]

### ✨ Added
- **🌐 SSH Session Support** ([src/core/ssh.rs](../../src/core/ssh.rs)) — persistent remote shell via the `ssh2` crate.
  - Auth chain: `ssh-agent` → `userauth_agent` → `~/.ssh/id_rsa` (no password prompt yet).
  - Remote `cwd` tracking with dedicated `change_dir()` resolver.
  - Cyan-bordered terminal panel + remote prompt `user@host cwd$` when a session is active.
  - Auto-injected `--color=always` for `ls`/`grep` to preserve ANSI colors via `ansi-to-tui`.
  - `exit`/`logout` drops the session and returns to the local shell.
- **📚 Docs refresh** — architecture and API docs now include **colored UML diagrams** (class, state, sequence, flow) in both EN and PT-BR.

### Planned Features
- [ ] Persistent state (save XP/achievements to disk)
- [ ] Command history persistence across sessions
- [ ] Multiplayer challenges (compete with friends)
- [ ] Plugin system for custom commands
- [ ] Cloud sync for progression
- [ ] Mobile companion app
- [ ] AI-powered command suggestions

---

## [0.1.0] - 2026-01-03 (BETA)

> [!IMPORTANT]
> **Initial Beta Release** - First public version of Munux Reactive Workspace!

### ✨ Added

#### Core Features
- 🐧 **Fully functional terminal** with shell execution via `sh -c`
- 🎮 **Complete gamification system** with XP, levels, achievements, quests
- 📊 **Reactive split-panel UI** (60/40 layout) that adapts to user input
- 🎨 **6 progressive themes** unlocked by leveling up (Beginner → Legend)
- 🔥 **Streak system** with XP multipliers for consistent correct usage

#### Gamification
- **6 Tier Levels**: Beginner (1-9), Terminal (10-19), Hacker (20-29), Cyberpunk (30-39), Elite (40-49), Legend (50+)
- **25+ Achievements** across categories: First Steps, Package Managers, Milestones, Streaks
- **Dynamic Quest System** that generates level-appropriate missions
- **XP Formula**: Base XP × Streak Multiplier (up to 2.0x at 25+ streak)
- **Evolution System**: Tux penguin evolves visual form with each tier

#### Command Support
- ✅ **60+ commands** across 11 categories
- ✅ **Multi-distro package managers**: pacman, yay, paru (Arch), apt, dpkg, snap (Debian), dnf, yum (Fedora), zypper (openSUSE), flatpak (Universal)
- ✅ **File operations**: ls, cd, pwd, mkdir, touch, cp, mv, rm, cat, grep, find
- ✅ **System monitoring**: top, htop, ps, kill, systemctl, journalctl
- ✅ **Network tools**: ping, curl, wget, ssh, scp, netstat
- ✅ **Git integration**: Full git command support
- ✅ **Text processing**: sed, awk, grep, cat, less, more

#### UI Features
- **9 Reactive Panel Modes**: Welcome, FileTree, FilePreview, ResourceMonitor, DangerZone, Stats, Quests, Help, EasterEgg
- **Real-time System Monitoring**: CPU, RAM, Swap usage graphs
- **Syntax Highlighting**: Code preview with language detection
- **Danger Zone Detection**: Red warning panel for destructive commands (`rm -rf`, `dd`, etc.)
- **HUD (Heads-Up Display)**: Bottom bar showing Level, XP, Achievements, Streak, Integrity

#### Easter Eggs
- 🚂 `sl` - ASCII train animation
- 🐄 `cowsay` - Talking cow with custom messages
- 🔮 `fortune` - Linux philosophy quotes
- 🌧️ `matrix` - Matrix-style rain effect
- 💻 `hack` - Hacker mode messages
- 🦸 `sudo su` - Uncle Ben quote
- 🌍 `hack the planet` - Hackers (1995) reference
- 🎮 `konami code` - Secret bonus
- 👤 `whoami` - Philosophy mode
- 🎲 And more hidden surprises!

#### Documentation
- 📚 **Complete English documentation** (~4,700+ lines)
- 📖 Guides: Quick Start, Installation, Gamification, Package Managers, Fonts, Troubleshooting
- 🏗️ Technical docs: Architecture Overview, API Reference, Testing Guide
- 🤝 Contributing guidelines with Code of Conduct

#### Developer Experience
- 🦀 Written in **Rust Edition 2021** (~3,500+ lines)
- 🏛️ **The Elm Architecture (TEA)** for predictable state management
- ✅ **Zero unsafe code** - 100% safe Rust
- 🧪 **~108 unit tests** with 85% code coverage
- 📊 Uses **Ratatui 0.26.3** for TUI, **Crossterm 0.27** for terminal handling

### 🔧 Technical Details

#### Dependencies
- **ratatui** 0.26.3 - Terminal UI framework
- **crossterm** 0.27.0 - Cross-platform terminal manipulation
- **sysinfo** 0.30.13 - System information gathering
- **serde** 1.0 + **serde_json** 1.0 - Serialization (future persistence)
- **chrono** 0.4 - Date and time handling
- **anyhow** 1.0 - Ergonomic error handling

#### Performance
- 🚀 **Startup time**: <200ms (release build)
- 💾 **Memory usage**: ~10-20 MB at runtime
- 🖥️ **CPU usage**: <1% when idle
- 🔄 **Refresh rate**: 60 Hz event loop

#### Supported Platforms
- ✅ **Arch Linux** / Manjaro
- ✅ **Ubuntu** / Debian / Linux Mint
- ✅ **Fedora** / RHEL / CentOS
- ✅ **openSUSE** Leap / Tumbleweed
- ✅ Any Linux distribution with Rust 1.70+

### 🎨 Visual Features

#### Themes
1. 🌱 **Cyan Dreams** (Beginner) - Light blue, welcoming
2. 💻 **Matrix Vision** (Terminal) - Green, classic hacker
3. 🔓 **Cyber Pulse** (Hacker) - Cyan/Magenta, futuristic
4. 🌃 **Night City** (Cyberpunk) - Magenta/Yellow, CP2077-inspired
5. 👑 **Royal Court** (Elite) - Purple/Gold, elegant
6. ⭐ **Legend Mode** (Legend) - Rainbow, dynamic RGB

#### Icons & Symbols
- Nerd Font integration for rich icons
- Unicode emoji support (🐧 🏆 🔥 📊 📁)
- Custom ASCII art for Tux evolution
- Tier-specific prompt symbols (➜ ► ▶ ◆ ⬢ ⬣)

### 📦 Installation Methods

```bash
# Method 1: From source (recommended)
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo build --release

# Method 2: Quick run
cargo run --release

# Method 3: Helper scripts
./setup.sh
./run.sh
```

### 🐛 Known Issues

> [!WARNING]
> **Beta Release Limitations:**

1. **No persistence**: XP and achievements reset on exit (planned for v0.2.0)
2. **No command history persistence**: History lost between sessions
3. **Limited error messages**: Some errors may not have helpful descriptions
4. **No auto-update**: Manual git pull required for updates
5. **Nerd Fonts required**: Icons display as boxes without Nerd Font

### 🔒 Security

- ✅ All commands executed in isolated `sh -c` instances
- ✅ Dangerous command detection with confirmation dialogs
- ✅ Respects standard Linux user/group permissions
- ✅ No privilege escalation attempts
- ✅ Clean terminal restoration on exit

### 📝 Notes

- First public beta release
- Extensive testing on Arch, Ubuntu, Fedora
- Community feedback welcome via GitHub Issues
- Documentation written in professional English (Big Tech style)
- Follows Rust best practices and idioms

---

## [0.0.1] - 2025-12-15 (Alpha - Internal)

### Added
- Initial proof of concept
- Basic terminal emulation
- Simple XP system
- Prototype UI with Ratatui

### Changed
- Migrated from Python to Rust for performance
- Redesigned UI to split-panel architecture

### Removed
- Python prototype code

---

## Release Versioning

Munux follows [Semantic Versioning](https://semver.org/):

```
MAJOR.MINOR.PATCH

MAJOR: Breaking changes (API incompatibilities)
MINOR: New features (backwards compatible)
PATCH: Bug fixes (backwards compatible)
```

**Pre-release tags:**
- `alpha` - Internal testing
- `beta` - Public testing (current)
- `rc` - Release candidate
- (none) - Stable release

---

## Roadmap

### v0.2.0 (Q1 2026)
- [ ] Persistent state (JSON storage)
- [ ] Command history saved to disk
- [ ] Custom theme editor
- [ ] Improved error messages
- [ ] Auto-update mechanism

### v0.3.0 (Q2 2026)
- [ ] Plugin system (WASM-based)
- [ ] Multiplayer mode (compete on challenges)
- [ ] Cloud sync (optional)
- [ ] Mobile companion app

### v1.0.0 (Q3 2026)
- [ ] Production-ready stable release
- [ ] Complete documentation
- [ ] Package manager releases (AUR, PPA, etc.)
- [ ] Performance optimizations
- [ ] Accessibility features

---

## Contributing

See [CONTRIBUTING.md](contributing/code-of-conduct.md) for guidelines on:
- Reporting bugs
- Suggesting features
- Submitting pull requests
- Code style guidelines

---

## Links

- 🌐 **Repository**: [github.com/Munique-Feitoza/Munux-Reactive-Workspace](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace)
- 📖 **Documentation**: [docs/README.md](README.md)
- 🐛 **Issues**: [GitHub Issues](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/discussions)

---

[Unreleased]: https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/releases/tag/v0.1.0
[0.0.1]: https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/releases/tag/v0.0.1
