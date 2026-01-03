# ⚠️ Troubleshooting Guide - Munux Reactive Workspace

## Common Issues and Solutions

### 1. Terminal Display Issues

#### Problem: Seeing ANSI codes instead of colors

**Symptoms:**
- Text like `[38;5;51m` or `[1;32m` appears
- No colors displayed
- Garbled output

**Solution:**
```bash
# Check your TERM variable
echo $TERM

# Should show: xterm-256color or similar
# If not, set it:
export TERM=xterm-256color

# For permanent fix, add to ~/.bashrc:
echo 'export TERM=xterm-256color' >> ~/.bashrc
```

#### Problem: Unicode/Emoji not displaying correctly

**Symptoms:**
- Tux penguin shows as boxes
- Emojis display as �
- Border characters broken

**Solution:**
1. Install a Nerd Font:
```bash
# Ubuntu/Debian
sudo apt install fonts-nerd-font

# Arch/Manjaro
yay -S nerd-fonts-complete

# Or download manually from https://www.nerdfonts.com/
```

2. Configure your terminal to use the Nerd Font

3. Ensure UTF-8 locale:
```bash
locale
# Should show UTF-8

# If not:
export LANG=en_US.UTF-8
```

### 2. Compilation Issues

#### Problem: "cargo: command not found"

**Solution:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload environment
source $HOME/.cargo/env

# Verify installation
cargo --version
```

#### Problem: "linker 'cc' not found"

**Solution:**

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential
```

**Fedora/RHEL:**
```bash
sudo dnf install gcc
```

**Arch/Manjaro:**
```bash
sudo pacman -S base-devel
```

**openSUSE:**
```bash
sudo zypper install -t pattern devel_basis
```

#### Problem: Compilation fails with dependency errors

**Solution:**
```bash
# Update Rust
rustup update

# Clean build cache
cargo clean

# Update dependencies
cargo update

# Try building again
cargo build --release
```

### 3. Runtime Issues

#### Problem: Terminal gets "stuck" or frozen

**Symptoms:**
- Can't type
- Screen not updating
- Ctrl+C doesn't work

**Solution:**

**Option 1:** Force close terminal
- Close the terminal window
- Open new terminal
- Run: `reset` to restore terminal state

**Option 2:** From another terminal
```bash
# Find Munux process
ps aux | grep munux

# Kill it (replace PID with actual process ID)
kill -9 <PID>

# Restore terminal
reset
```

**Option 3:** If terminal is responsive
```bash
# Try Ctrl+C multiple times
# Or try: Ctrl+Z then:
kill %1
reset
```

#### Problem: Exit leaves terminal in weird state

**Symptoms:**
- Text invisible
- No prompt showing
- Keyboard input not visible

**Solution:**
```bash
# Restore terminal to normal state
reset

# Or:
stty sane
```

### 4. Functionality Issues

#### Problem: Commands not executing

**Symptoms:**
- Pressing Enter does nothing
- Commands disappear but no output

**Solution:**
1. Check you're typing in the correct area (left panel, bottom)
2. Ensure no popup is open (press ESC to close)
3. Try simple command: `echo test`
4. Check terminal size (must be at least 80x24)

#### Problem: History navigation not working

**Symptoms:**
- Arrow keys don't show previous commands
- Up/Down does nothing

**Solution:**
- Make sure you've executed at least one command first
- History is session-based (v0.1.0), not persistent
- Try typing a command, executing it, then press ↑

#### Problem: Achievements not unlocking

**Symptoms:**
- Commands execute but no achievement popup
- Stats show 0 achievements

**Solution:**
1. Check you're executing the correct command for the achievement
2. Run `achievements` command to see what's unlocked
3. XP must be earned for achievement to trigger
4. Try `xp 100` to test system

#### Problem: Quests not updating

**Symptoms:**
- Quest progress stuck at 0/X
- Completing objective doesn't update quest

**Solution:**
1. Run `quests` command to refresh
2. Make sure you're completing the exact objective
3. For "Execute X commands" quests, only unique commands count
4. Try `xp 500` to generate new quests at different level

### 5. Performance Issues

#### Problem: Munux running slow

**Symptoms:**
- Lag when typing
- Slow screen updates
- High CPU usage

**Solution:**
1. Compile in release mode:
```bash
cargo build --release
./target/release/munux-reactive-workspace
```

2. Check system resources:
```bash
htop
# Munux should use < 5% CPU idle
```

3. Close other resource-heavy applications

4. Ensure terminal size is reasonable (not too large)

### 6. Package Manager Issues

#### Problem: Package manager commands not recognized

**Symptoms:**
- `pacman -Syu` executes but no XP
- No achievement for using package manager

**Solution:**
1. Make sure command is exactly correct:
   - Arch: `pacman -Syu` (not `sudo pacman -Syu` for recognition)
   - Ubuntu: `apt update` (not `sudo apt update` for recognition)

2. Check you're on the correct distro:
```bash
cat /etc/os-release
```

3. Try the base command first: `pacman`, `apt`, `dnf`

### 7. Help System Issues

#### Problem: Can't exit help screen

**Symptoms:**
- Help screen stuck
- ESC doesn't close
- Ctrl+C closes entire Munux

**Solution:**
1. Press ESC key
2. If not working, try Ctrl+L to clear screen
3. As last resort, Ctrl+C to exit Munux and restart

#### Problem: Help command shows nothing

**Symptoms:**
- `help` command executes but panel empty
- `help arch` shows no content

**Solution:**
1. Try different help topic: `help`, `help linux`, `help arch`
2. Ensure terminal is large enough (at least 80x24)
3. Try fullscreen mode

### 8. File Operations Issues

#### Problem: File preview not showing

**Symptoms:**
- `cat file.txt` executes but right panel empty
- No syntax highlighting

**Solution:**
1. Check file exists: `ls -la`
2. Check file size (very large files might take time)
3. Try with small file: `echo "test" > test.txt && cat test.txt`
4. Ensure file has read permissions

#### Problem: Directory navigation not working

**Symptoms:**
- `cd` command fails
- Wrong directory shown

**Solution:**
1. Use absolute paths: `cd /home/user/folder`
2. Check directory exists: `ls -la`
3. Check permissions: `ls -ld /path/to/directory`
4. Use `pwd` to confirm current location

### 9. Theme Issues

#### Problem: Theme not changing with level

**Symptoms:**
- Reached level 10 but still cyan theme
- Colors don't match level tier

**Solution:**
1. Check actual level: `xp` command
2. Themes change at: 1, 10, 20, 30, 40, 50
3. Try forcing level up: `xp 1000`
4. Restart Munux to refresh

### 10. Easter Egg Issues

#### Problem: Easter eggs not working

**Symptoms:**
- `sl` shows nothing
- `cowsay` doesn't display cow

**Solution:**
1. Type exact command: `sl` (lowercase)
2. For cowsay: `cowsay Your message here`
3. Try `fortune`, `matrix`, `hack`
4. Check `achievements` for hints

## Getting Help

If your issue isn't listed here:

1. **Check Documentation:**
   - [Quick Start Guide](guides/quick-start.md)
   - [Installation Guide](guides/installation.md)
   - [Testing Guide](TESTING.md)

2. **GitHub Issues:**
   - Search existing issues: https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues
   - Open new issue with:
     - OS and version
     - Rust version (`rustc --version`)
     - Terminal emulator
     - Steps to reproduce
     - Error messages

3. **Community:**
   - Check README for contact info
   - Join discussions on GitHub

## Debug Mode

For developers debugging issues:

```bash
# Run with debug output
RUST_LOG=debug cargo run

# Check build
cargo check

# Run tests (when available)
cargo test
```

## System Information

When reporting issues, include:

```bash
# OS information
cat /etc/os-release

# Rust version
rustc --version
cargo --version

# Terminal
echo $TERM

# Locale
locale

# Terminal size
tput cols
tput lines
```

---

**Last Updated:** January 3, 2026  
**Version:** v0.1.0 BETA

For more help, visit [GitHub Issues](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues).
