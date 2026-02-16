# 📥 Installation Guide

Complete installation instructions for Munux Reactive Workspace across all major Linux distributions.

![Platform](https://img.shields.io/badge/Platform-Linux-yellow) ![Rust](https://img.shields.io/badge/Rust-1.70+-orange) ![License](https://img.shields.io/badge/License-GPLv3-blue)

---

## Prerequisites

### Required

| Component | Version | Purpose |
|:----------|:--------|:--------|
| **Rust** | 1.70+ | Compilation toolchain |
| **Cargo** | Latest | Package manager |
| **Git** | 2.0+ | Source code download |

### Recommended

| Component | Purpose |
|:----------|:--------|
| **Nerd Font** | Icon display (JetBrains Mono, Fira Code) |
| **256-color terminal** | Full theme support |
| **Unicode support** | Emoji and special characters |

> [!TIP]
> Don't have Rust? Install it in 30 seconds: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

---

## Installation Methods

### Method 1: From Source (Recommended)

> [!IMPORTANT]
> This is the **recommended method** for getting the latest features and updates.

```bash
# 1. Clone the repository
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git

# 2. Navigate to directory
cd Munux-Reactive-Workspace

# 3. Build in release mode (IMPORTANT for performance)
cargo build --release

# 4. Run the application
./target/release/munux-reactive-workspace
```

**Build time:** ~2-5 minutes (first build only)  
**Binary size:** ~8-12 MB  
**Memory usage:** ~10-20 MB at runtime

---

### Method 2: Quick Run with Cargo

```bash
# Clone and run in one step
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo run --release
```

> [!WARNING]
> Never use `cargo run` without `--release` for actual usage. Debug builds are 10-50x slower!

---

### Method 3: Using Helper Scripts

```bash
# Automated setup (installs dependencies if needed)
chmod +x setup.sh
./setup.sh

# Quick launch
chmod +x run.sh
./run.sh
```

**What `setup.sh` does:**
- ✅ Checks for Rust installation
- ✅ Installs build dependencies
- ✅ Builds in release mode
- ✅ Verifies successful compilation

---

## Distribution-Specific Setup

### Arch Linux / Manjaro

```bash
# Install build dependencies
sudo pacman -S base-devel git rust

# Install recommended font
yay -S ttf-jetbrains-mono-nerd

# Clone and build
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo build --release
```

---

### Ubuntu / Debian

```bash
# Install build dependencies
sudo apt update
sudo apt install -y build-essential git curl pkg-config libssl-dev

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install recommended font
mkdir -p ~/.fonts
cd ~/.fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip
unzip JetBrainsMono.zip
fc-cache -fv
cd -

# Clone and build
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo build --release
```

---

### Fedora / RHEL

```bash
# Install build dependencies
sudo dnf groupinstall "Development Tools"
sudo dnf install git rust cargo openssl-devel

# Install recommended font
sudo dnf install -y jetbrains-mono-fonts-all

# Clone and build
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo build --release
```

---

### openSUSE

```bash
# Install build dependencies
sudo zypper install -t pattern devel_basis
sudo zypper install git rust cargo

# Clone and build
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
cargo build --release
```

---

## Post-Installation

### 1. Add to PATH (Optional)

To run Munux from anywhere:

```bash
# Copy binary to /usr/local/bin
sudo cp target/release/munux-reactive-workspace /usr/local/bin/munux

# Now you can run it anywhere
munux
```

---

### 2. Create Desktop Entry (Optional)

For GUI launchers:

```bash
# Create desktop entry
cat > ~/.local/share/applications/munux.desktop << EOF
[Desktop Entry]
Name=Munux Reactive Workspace
Comment=Gamified Learning Terminal
Exec=/usr/local/bin/munux
Icon=utilities-terminal
Terminal=true
Type=Application
Categories=System;TerminalEmulator;
EOF
```

---

### 3. Configure Your Terminal

For best experience:

1. **Set Nerd Font:**
   - Open your terminal settings (Konsole, GNOME Terminal, etc.)
   - Font: **JetBrains Mono Nerd Font**
   - Size: **11** or **12**

2. **Enable 256 colors:**

```bash
# Add to ~/.bashrc or ~/.zshrc
export TERM=xterm-256color
```

3. **Test Unicode support:**

```bash
echo "🐧 🏆 🔥 ➜ ▶ ◆ ⬢ ⬣"
```

If you see colorful icons, you're ready!

---

## Verification

Test your installation:

```bash
# Check version (when implemented)
munux --version

# Run in test mode
cargo run --release

# Check binary size
ls -lh target/release/munux-reactive-workspace
```

**Expected output:**
```
-rwxr-xr-x 1 user user 8.5M Jan  3 12:00 munux-reactive-workspace
```

---

## Update Instructions

```bash
# Navigate to repository
cd Munux-Reactive-Workspace

# Pull latest changes
git pull origin main

# Rebuild
cargo build --release

# If binary is in PATH, update it
sudo cp target/release/munux-reactive-workspace /usr/local/bin/munux
```

> [!NOTE]
> Future versions may include auto-update functionality.

---

## Uninstall

```bash
# Remove binary from PATH
sudo rm /usr/local/bin/munux

# Remove desktop entry
rm ~/.local/share/applications/munux.desktop

# Remove source code
rm -rf ~/Munux-Reactive-Workspace
```

---

## Troubleshooting Installation

### Issue: "linker 'cc' not found"

**Solution:** Install C compiler.

```bash
# Ubuntu/Debian
sudo apt install build-essential

# Arch/Manjaro
sudo pacman -S base-devel

# Fedora
sudo dnf groupinstall "Development Tools"
```

---

### Issue: "failed to run custom build command"

**Solution:** Install OpenSSL development libraries.

```bash
# Ubuntu/Debian
sudo apt install libssl-dev pkg-config

# Arch/Manjaro
sudo pacman -S openssl pkg-config

# Fedora
sudo dnf install openssl-devel
```

---

### Issue: "cargo: command not found"

**Solution:** Install Rust or add Cargo to PATH.

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source $HOME/.cargo/env

# Verify
cargo --version
```

---

## Build Performance Tips

> [!TIP]
> Speed up compilation with these tricks:

### Use `sccache` (Shared Compilation Cache)

```bash
# Install sccache
cargo install sccache

# Configure Cargo to use it
export RUSTC_WRAPPER=sccache

# Rebuild (subsequent builds will be much faster)
cargo build --release
```

### Parallel Compilation

```bash
# Use all CPU cores (default behavior)
cargo build --release -j$(nproc)
```

### Link-Time Optimization (Already enabled in release)

LTO is automatically enabled in `--release` builds for maximum performance.

---

## System Requirements

| Component | Minimum | Recommended |
|:----------|:--------|:------------|
| **OS** | Linux Kernel 3.0+ | 5.0+ |
| **RAM** | 512 MB | 2 GB |
| **Disk** | 50 MB | 100 MB |
| **Terminal** | Basic ANSI | 256-color + Unicode |
| **CPU** | 1 core | 2+ cores |

> [!NOTE]
> Munux is **extremely lightweight** and runs well even on Raspberry Pi!

---

## Next Steps

After successful installation:

1. ✅ **Launch Munux**: `munux` or `cargo run --release`
2. 📚 **Read Quick Start**: [quick-start.md](quick-start.md)
3. 🎮 **Learn Gamification**: [gamification-system.md](gamification-system.md)
4. 🔧 **Troubleshooting**: [troubleshooting.md](troubleshooting.md)

**Welcome to the Munux community!** 🐧🚀
