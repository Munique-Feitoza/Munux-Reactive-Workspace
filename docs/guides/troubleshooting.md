# 🔧 Troubleshooting Guide

Encountering issues? Check the solutions below. If the problem persists, open an [Issue on GitHub](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues).

![Status](https://img.shields.io/badge/Status-Beta-yellow) ![Help](https://img.shields.io/badge/Community-Active-green)

---

## 🎨 Display & Font Issues

### Problem: "I see boxes `□` or question marks `?` instead of icons"

This happens when your terminal doesn't support **Nerd Fonts**.

> [!TIP]
> **Solution:** Install a Nerd Font and configure your terminal to use it.

**Step-by-step:**

1. **Download a Nerd Font:**
   - [JetBrains Mono Nerd Font](https://www.nerdfonts.com/)
   - [Fira Code Nerd Font](https://www.nerdfonts.com/)
   - [Hack Nerd Font](https://www.nerdfonts.com/)

2. **Install the font:**

```bash
# Arch/Manjaro
yay -S ttf-jetbrains-mono-nerd

# Ubuntu/Debian
mkdir -p ~/.fonts
cd ~/.fonts
wget https://github.com/ryanoasis/nerd-fonts/releases/download/v3.0.2/JetBrainsMono.zip
unzip JetBrainsMono.zip
fc-cache -fv

# Fedora
sudo dnf install -y jetbrains-mono-fonts-all
```

3. **Set as terminal font:**
   - Open your terminal settings (Konsole, GNOME Terminal, etc.)
   - Set font to: **JetBrains Mono Nerd Font** (size 11 or 12)
   - Restart Munux

4. **Test if it works:**

```bash
echo "🐧 🏆 🔥 ➜ ▶ ◆"
```

If you see colorful icons, you are ready! See [Fonts Guide](fonts.md) for more details.

---

## 🦀 Compilation Issues

### Problem: `linker 'cc' not found`

You are missing basic C build tools required by Rust dependencies.

**Solution:** Install build essentials for your distro.

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install build-essential

# Arch/Manjaro
sudo pacman -S base-devel

# Fedora/RHEL
sudo dnf groupinstall "Development Tools"

# openSUSE
sudo zypper install -t pattern devel_basis
```

Then retry:

```bash
cargo build --release
```

---

### Problem: `error: failed to run custom build command for 'some-crate'`

This usually means a system dependency is missing.

**Solution:** Install development libraries.

```bash
# Ubuntu/Debian
sudo apt install -y pkg-config libssl-dev

# Arch/Manjaro
sudo pacman -S pkgconf openssl

# Fedora
sudo dnf install pkg-config openssl-devel
```

---

### Problem: `Blocking waiting for file lock on package cache`

Another Cargo process is running.

**Solution:** Wait for other process to finish, or force unlock:

```bash
# Check for running cargo processes
ps aux | grep cargo

# If stuck, remove lock (use with caution)
rm ~/.cargo/.package-cache
```

---

## ⚡ Runtime Issues

### Problem: "The terminal feels slow / laggy"

Debug builds in Rust are slow because they include heavy runtime checks.

> [!IMPORTANT]
> **Solution:** Always run in **Release Mode** for actual usage.

```bash
# ❌ SLOW (Debug mode)
cargo run

# ✅ FAST (Release mode - 10x to 50x faster)
cargo run --release
```

For even better performance, build once and run the binary:

```bash
cargo build --release
./target/release/munux-reactive-workspace
```

---

### Problem: "Munux crashes on startup"

**Possible causes:**

1. **Terminal not supported:**

```bash
# Check if your terminal supports Crossterm
echo $TERM
```

Expected: `xterm-256color`, `screen-256color`, or similar.

**Fix:** Set proper TERM variable:

```bash
export TERM=xterm-256color
```

2. **Permissions issue:**

```bash
# Ensure you have permission to run
chmod +x target/release/munux-reactive-workspace
```

3. **Missing runtime dependencies:**

```bash
# Arch/Manjaro
sudo pacman -S glibc

# Ubuntu/Debian
sudo apt install libc6
```

---

### Problem: "Commands don't execute / No output"

**Possible causes:**

1. **Shell not found:**

Munux uses `/bin/sh`. Ensure it exists:

```bash
ls -l /bin/sh
```

If missing, create symlink:

```bash
sudo ln -s /bin/bash /bin/sh
```

2. **Permission denied:**

Some commands require sudo. Try:

```bash
sudo ls /root
```

---

## 🎮 Gamification Issues

### Problem: "XP not updating / Achievements not unlocking"

**Debug steps:**

1. **Check current state:**

```bash
# Inside Munux
stats
```

2. **Manually add XP to test:**

```bash
xp 100
```

If this works, the system is functional. The command may not be recognized.

3. **Check parser:**

Type `help` to see supported commands. If your command is not listed, it may be classified as "Unknown."

---

### Problem: "Streak keeps breaking even on successful commands"

**Possible causes:**

1. Command returns non-zero exit code (even if it looks successful).

**Test:**

```bash
# Check exit code of last command
echo $?
```

If not `0`, streak breaks.

2. Some commands always fail in certain contexts (e.g., `cd` to non-existent dir).

> [!NOTE]
> This is intentional game design to encourage correct command usage!

---

## 📁 File System Issues

### Problem: "File tree not showing / Empty panel"

**Causes:**

1. **No files in directory:**

```bash
# Create test files
touch test1.txt test2.txt
ls
```

2. **Permission denied:**

```bash
# Try with sudo
sudo ls /root
```

3. **Hidden files:**

```bash
# Show hidden files
ls -la
```

---

### Problem: "File preview shows garbage characters"

This happens with binary files or unsupported encodings.

**Solution:** Munux is designed for text files. For binary files, use:

```bash
hexdump -C binary-file
```

---

## 🌐 Network / SSH Issues

### Problem: "SSH commands don't work"

Munux executes SSH via shell, so it should work. If it doesn't:

**Debug:**

```bash
# Test SSH outside Munux first
ssh user@host

# If that works, try in Munux
ssh user@host
```

If SSH prompts for password/key, it may not display correctly in TUI. Use:

```bash
# Pre-authenticate with SSH agent
eval $(ssh-agent)
ssh-add ~/.ssh/id_rsa
```

---

## 🐞 General Debugging

### Enable Verbose Logging

> [!TIP]
> Use environment variables to debug issues.

```bash
# Run with Rust backtrace
RUST_BACKTRACE=1 cargo run --release

# Full backtrace
RUST_BACKTRACE=full cargo run --release
```

### Generate Debug Report

```bash
# System info
uname -a
cargo --version
rustc --version

# Terminal info
echo $TERM
echo $SHELL

# Munux version
cargo run --release -- --version
```

Copy this output when reporting issues on GitHub.

---

## 🆘 Getting Help

### Option 1: GitHub Issues

If you found a bug or have a feature request:

1. Go to [GitHub Issues](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues)
2. Click "New Issue"
3. Provide:
   - Your distro (`cat /etc/os-release`)
   - Error message (full output)
   - Steps to reproduce
   - Expected vs actual behavior

### Option 2: Community Discussions

For questions and general help:

1. Visit [GitHub Discussions](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/discussions)
2. Search existing topics
3. Start a new discussion if needed

### Option 3: In-App Help

```bash
# Inside Munux
help           # General commands
help arch      # Arch/Manjaro specific
help debian    # Ubuntu/Debian specific
help fedora    # Fedora/RHEL specific
```

---

## 🔍 FAQ

### Q: Does Munux work on non-Linux systems?

A: Currently, Munux is designed for Linux only. macOS support is planned. Windows requires WSL.

### Q: Can I use Munux as my default terminal?

A: Yes! Once compiled, you can set `munux-reactive-workspace` as your default terminal emulator.

### Q: Will Munux break my system with dangerous commands?

A: No. Munux **warns you** but does NOT prevent execution. It is a learning tool, not a sandbox.

### Q: Does Munux store my command history?

A: Currently, history is stored in memory only (lost on exit). Persistent history is planned for v0.2.0.

### Q: Can I customize the themes?

A: Themes are currently hardcoded based on level. User-customizable themes are planned for a future release.

### Q: Does Munux work over SSH?

A: Yes, as long as your SSH terminal supports ANSI colors and Unicode.

---

## 📚 Related Documentation

- 🏗️ [Architecture](../architecture/overview.md) - Understand how Munux works
- 🎮 [Gamification System](gamification-system.md) - XP, levels, achievements
- ⚡ [Quick Start](quick-start.md) - First steps guide
- 📦 [Package Managers](package-managers.md) - Distro-specific commands

---

## 🚀 Still Stuck?

If nothing above helped:

1. **Clear build cache and rebuild:**

```bash
cargo clean
cargo build --release
```

2. **Update Rust toolchain:**

```bash
rustup update stable
```

3. **Check for known issues:**

Visit the [GitHub Issues page](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues) and search for your problem.

4. **Ask for help:**

Open a new issue with:
- OS and version
- Rust version (`rustc --version`)
- Full error output
- Steps to reproduce

**We are here to help!** 💪🐧
