# 📦 Installation Guide - Munux Reactive Workspace

## Prerequisites

### 1. Install Rust

Munux is written in Rust, so you need the Rust compiler installed.

**Linux / macOS / WSL:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, reload your shell:
```bash
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

### 2. System Dependencies (Linux)

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential
```

**Fedora/RHEL:**
```bash
sudo dnf install gcc
```

**Arch Linux/Manjaro:**
```bash
sudo pacman -S base-devel
```

**openSUSE:**
```bash
sudo zypper install -t pattern devel_basis
```

## Installation

### Method 1: From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace

# Compile the project
cargo build --release

# Run
cargo run --release
```

### Method 2: Quick Run (Debug Mode)

```bash
# Clone the repository
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace

# Run directly (compiles automatically)
cargo run
```

### Method 3: Install Binary (Future)

```bash
# Coming in v0.2.0
cargo install munux-reactive-workspace
munux
```

## Running Munux

### Debug Mode (Development)
```bash
cargo run
```

### Release Mode (Optimized)
```bash
cargo run --release
```

Or run the binary directly:
```bash
./target/release/munux-reactive-workspace
```

## Post-Installation

### Create Desktop Shortcut (Optional)

**Linux Desktop Entry:**
```bash
# Create desktop file
cat > ~/.local/share/applications/munux.desktop << 'EOF'
[Desktop Entry]
Version=1.0
Type=Application
Name=Munux Terminal
Comment=Gamified terminal for Linux
Exec=/path/to/munux-reactive-workspace/target/release/munux-reactive-workspace
Icon=utilities-terminal
Terminal=true
Categories=System;TerminalEmulator;
EOF

# Update path in Exec line to your installation path
```

### Add to PATH (Optional)

```bash
# Add to ~/.bashrc or ~/.zshrc
echo 'export PATH="$PATH:/path/to/munux-reactive-workspace/target/release"' >> ~/.bashrc
source ~/.bashrc

# Now you can run with just:
munux-reactive-workspace
```

### Create Alias (Optional)

```bash
# Add to ~/.bashrc or ~/.zshrc
echo 'alias munux="/path/to/munux-reactive-workspace/target/release/munux-reactive-workspace"' >> ~/.bashrc
source ~/.bashrc

# Now you can run with just:
munux
```

## Troubleshooting

### Error: "cargo: command not found"

Rust is not installed or not in PATH. Install following instructions above.

After installation:
```bash
source $HOME/.cargo/env
```

### Error: "linker 'cc' not found"

Build tools are not installed. Install build-essential (Ubuntu/Debian) or equivalent for your distro.

**Ubuntu/Debian:**
```bash
sudo apt install build-essential
```

**Fedora:**
```bash
sudo dnf install gcc
```

**Arch/Manjaro:**
```bash
sudo pacman -S base-devel
```

### Compilation Errors Related to Dependencies

Update dependencies:
```bash
cargo update
cargo build
```

### Error: "failed to compile munux-reactive-workspace"

Make sure you have the latest Rust version:
```bash
rustup update
```

### Terminal Display Issues

If you see ANSI codes instead of colors:

1. Check your terminal supports 256 colors:
```bash
echo $TERM
# Should show: xterm-256color or similar
```

2. If not, set it:
```bash
export TERM=xterm-256color
```

### Permission Denied

If binary won't execute:
```bash
chmod +x target/release/munux-reactive-workspace
```

## System Requirements

### Minimum
- **OS:** Any modern Linux distribution
- **RAM:** 50 MB
- **Disk:** 100 MB for source + build
- **Terminal:** Any terminal emulator with 256-color support

### Recommended
- **OS:** Ubuntu 20.04+, Fedora 35+, Arch, Manjaro, Debian 11+
- **RAM:** 100 MB
- **Disk:** 200 MB
- **Terminal:** Modern terminal (GNOME Terminal, Konsole, Alacritty, Kitty)
- **Font:** Nerd Font (for best emoji support)

## Supported Distributions

Munux has been tested on:
- ✅ Manjaro KDE
- ✅ Ubuntu 22.04
- ✅ Fedora 38
- ✅ Debian 11
- ✅ Arch Linux

Should work on any Linux distribution with Rust support.

## Updating

### Update from Git

```bash
cd Munux-Reactive-Workspace
git pull origin main
cargo build --release
```

### Update Rust (if needed)

```bash
rustup update
```

## Uninstalling

### Remove Munux

```bash
# Simply delete the directory
rm -rf Munux-Reactive-Workspace
```

### Remove Rust (if desired)

```bash
rustup self uninstall
```

## Next Steps

After installation:

1. **Read the [Quick Start Guide](quick-start.md)** - Get started in 5 minutes
2. **Check the [Testing Guide](../TESTING.md)** - Try all features
3. **Explore [Gamification System](gamification-system.md)** - Understand XP and achievements
4. **Read [Package Managers Guide](package-managers.md)** - Multi-distro support

---

**Installation Support:** If you encounter issues, check [Troubleshooting](troubleshooting.md) or open an issue on [GitHub](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues).
