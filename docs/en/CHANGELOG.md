# 📋 Changelog

All notable changes to Munux Reactive Workspace will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

![Version](https://img.shields.io/badge/Latest-v0.3.0-blue) ![Status](https://img.shields.io/badge/Status-Beta-yellow)

---

## [Unreleased]

### Planned Features
- [ ] Customizable themes (build your own)
- [ ] Plugin system for custom commands
- [ ] Competitive mode / leaderboards
- [ ] Multiplayer challenges (compare progress)
- [ ] Cloud sync for progression

---

## [0.3.0] - 2026-06-22

### ✨ Added
- **⌨️ Interactive file navigation** — with the file tree visible and the input empty, arrow keys select an entry and Enter opens it (file → preview, directory → cd), with a highlighted selection and an `↑↓ navigate • Enter open` hint.
- **📊 Top-5 process panel** ([src/ui/reactive.rs](../../src/ui/reactive.rs)) — the resource monitor now lists the 5 processes by CPU usage (pid, name, cpu%, memory).
- **🏅 Rank progression** — the stats panel shows the next rank and the level that unlocks it; `game::tier::Tier` is the single source for rank, theme, prompt symbol and border.
- **🛟 RAII terminal guard** ([src/tui.rs](../../src/tui.rs)) — `TerminalGuard` owns the terminal and restores it on every exit path (normal return, `?`, panic).

### 🔒 Security
- **Safe-mode bypass closed** ([src/core/parser.rs](../../src/core/parser.rs)) — safe mode now validates **every** command segment (`;`, `&&`, `||`, `|`) and blocks command substitution (`$(...)`/backticks). Previously only the first token was checked, so `echo ok; rm -rf /tmp/x` slipped through.
- **SSH shell-quoting** ([src/core/ssh.rs](../../src/core/ssh.rs)) — `remote_cwd` (from the server) and remote paths are POSIX-quoted, preventing shell injection.
- **Danger-zone confirmation** — destructive commands require an explicit `yes`/`sim` instead of running on Enter.
- **`xp` cheat gated** — the XP cheat compiles only in debug builds.

### ♻️ Changed / Refactored
- **Single sources of truth**: `core/commands` (command catalog → classification, autocomplete, coloring), `core/filetype` (extension → language/icon/color) and `game/tier` (rank tiers). One persistent `SystemMonitor`. New helpers `git()`, `panel_block`, `try_t`, `parse_cd_arg`, `git_segment_spans`, `local_prompt_prefix`.
- **Full internationalization** — every user-visible string moved to `locales/{pt-BR,en-US}` (~70 new keys); a test verifies all keys resolve in both locales.
- `command_to_panel_mode` moved out of `core` (removes the core→app dependency); `execute_command` slimmed (special-command and SSH-session handlers extracted).
- **Removed all dead code** — zero `#[allow(dead_code)]` remaining.

### 🐛 Fixed
- `.ts` files were highlighted as JavaScript (the TypeScript branch was unreachable).
- Division by zero in memory %, gauge panics (now clamped to 100), `benchmark parar` not cancelling, wrong distro-guide titles, and "did you mean?" suggestions searching the wrong directory after `cd`.

### 🧪 Tests
- 42 tests total — `GameState`, `Quest::update_progress`, `AchievementChecker`, i18n key resolution, and a safe-mode security regression.

---

## [0.2.0] - 2026-05-12

### ✨ Added
- **💾 Progress persistence** ([src/core/persistence.rs](../../src/core/persistence.rs)) — XP, level, achievements, streak, command history and aliases are saved to `$XDG_DATA_HOME/munux/state.json` after every command and on exit. Atomic writes (temp file + rename) and a `.bak` fallback when a save is corrupted. Daily streak updates on the next session via `GameState::update_daily_streak`.
- **⌨️ Tab auto-complete** ([src/core/completion.rs](../../src/core/completion.rs)) — completes the first word from built-in commands + `$PATH` executables, and arguments from filesystem paths in the current directory. Extends to the longest common prefix and lists candidates when ambiguous.
- **🎨 Syntax highlighting in file preview** ([src/ui/highlight.rs](../../src/ui/highlight.rs)) — lightweight per-line highlighter (no extra dependencies) for Rust, Python, JavaScript/TypeScript, Bash, JSON and TOML.
- **🔗 Custom alias system** — `alias name='cmd'`, `alias` (list), `unalias name`. Aliases are expanded at execution time (recursion-limited to avoid cycles) and persisted with the rest of the progress.
- **🎓 Interactive tutorial mode** ([src/game/tutorial.rs](../../src/game/tutorial.rs)) — `tutorial` starts a 5-step guided walkthrough for beginners (`help` → `pwd` → `ls` → `stats` → `cat <file>`) with a +100 XP completion bonus. `tutorial sair` exits.
- **⏱️ Benchmark mode** ([src/game/benchmark.rs](../../src/game/benchmark.rs)) — `benchmark` runs a typing speed test (WPM + accuracy) and rewards XP proportional to performance.
- **🥚 More easter eggs & secret achievements** — new eggs (xkcd sandwich, `42`, `xyzzy`, "the cake is a lie", Vim escape reflex, Star Wars) plus matching secret achievements and the meta-achievement **Easter Egg Hunter** (find 5+ eggs). Easter egg unlocks now show an achievement popup.
- **🌐 SSH Session Support** ([src/core/ssh.rs](../../src/core/ssh.rs)) — persistent remote shell via the `ssh2` crate.
  - Auth chain: `ssh-agent` → `userauth_agent` → `~/.ssh/id_rsa` (no password prompt yet).
  - Remote `cwd` tracking with dedicated `change_dir()` resolver.
  - Cyan-bordered terminal panel + remote prompt `user@host cwd$` when a session is active.
  - Auto-injected `--color=always` for `ls`/`grep` to preserve ANSI colors via `ansi-to-tui`.
  - `exit`/`logout` drops the session and returns to the local shell.
- **🌍 Internationalization** — full i18n via Project Fluent (EN / PT-BR), auto-detected from the system locale.
- **📚 Docs refresh** — architecture and API docs include colored UML diagrams (class, state, sequence, flow) in both EN and PT-BR.

### ♻️ Changed / Refactored
- Easter egg recognition consolidated into a single `EasterEggs::classify` → `Egg` enum, the single source of truth for both the rendered art and the awarded achievement.
- `App::award_achievement` extracted to remove duplication between achievement-unlock sites.
- `FileEntry::get_icon` rewritten table-driven; progress saving centralized to a single point (after each command + on exit).
- Cleanup pass: resolved all `clippy` warnings (collapsed nested conditionals, `parts.first()`, `rsplit().next()` instead of `last()` on a double-ended iterator, `Language::to_langid` by value).

### 🐛 Fixed
- Locale key typo `achievement_easter_egg_nuke-desc` (was using `_` instead of `-`), which left the achievement description missing.

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
