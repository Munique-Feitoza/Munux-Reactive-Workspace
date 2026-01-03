# 🧪 Testing Guide - Munux Reactive Workspace

## Basic Linux Commands (Work via shell)

### Navigation
- `ls` - Lists files (displays in right panel)
- `pwd` - Shows current directory
- `cd <folder>` - Changes directory

### Creation
- `mkdir test` - Creates directory
- `touch file.txt` - Creates empty file
- `echo "text" > file.txt` - Creates file with content

### Viewing  
- `cat file.txt` - Shows content (with preview in right panel)
- `head file.txt` - First lines
- `tail file.txt` - Last lines

### Manipulation
- `cp file.txt copy.txt` - Copies file
- `mv file.txt new.txt` - Moves/renames
- `rm file.txt` - Removes file
- `rmdir folder` - Removes empty directory

### System
- `top` - Process monitor (right panel shows metrics)
- `free` - Available memory
- `df` - Disk space
- `ps` - Processes

## Munux Special Commands

### Gamification
- `stats` - Shows statistics panel (commands, success rate, streak)
- `quests` - Shows active missions
- `achievements` - Lists unlocked achievements
- `xp 100` - Adds XP (for testing level up)

### Easter Eggs 🥚

#### Animations
- `sl` - ASCII train (when you mistype 'ls')
- `matrix` - Matrix-style message
- `hack` - Hacking message

#### Interactive
- `cowsay Hello!` - Talking cow
- `cowsay Linux is awesome` - Cow with custom message
- `fortune` - Random quotes about Linux/programming

#### Special
- `sudo su` - Uncle Ben message
- `sudo rm -rf /` - Blocked with nuclear warning
- `hack the planet` - Hackers (1995) reference
- `konami` - Konami code (secret bonus)
- `whoami` - Philosophical message

## Complete Test Flow

### 1. First Login
```bash
# When opening, you'll see:
# - Tux (penguin) on welcome screen
# - Level 1, rank "Beginner"
# - Cyan theme (beginner)
```

### 2. First Commands (Unlocking Achievements)
```bash
ls                    # Achievement: "Listing Master" (+20 XP)
mkdir test           # Achievement: "Creator" (+30 XP)
cd test              # Achievement: "Navigator" (+20 XP)
touch hello.txt      # Achievement: "Creator" (+30 XP)
cat hello.txt        # Achievement unlocked
```

### 3. Testing Quests
```bash
quests              # View active missions
# Execute commands requested by quests
# When complete, you'll see "QUEST COMPLETE!"
```

### 4. Testing Easter Eggs
```bash
sl                  # Train passing by
cowsay Moo!         # Talking cow
fortune             # Random quote
matrix              # Matrix message
hack the planet     # Hackers easter egg
```

### 5. Testing Level Progression
```bash
xp 500              # Level 5 - Matrix Green theme
xp 1000             # Level 10 - Hacker theme
xp 5000             # Level 20 - Cyberpunk theme
xp 10000            # Level 30 - Elite theme
xp 20000            # Level 50 - LEGEND!
```

### 6. Testing Package Managers

#### Arch/Manjaro
```bash
pacman -Syu         # System update
yay -S firefox      # Install Firefox with yay
paru -S discord     # Install Discord with paru
```

#### Ubuntu/Debian
```bash
sudo apt update     # Update package list
sudo apt install firefox
snap install code   # Install VS Code with snap
```

#### Fedora
```bash
sudo dnf update     # Update system
sudo dnf install git
```

#### openSUSE
```bash
sudo zypper refresh # Refresh repos
sudo zypper install vim
```

### 7. Testing Help System
```bash
help                # List special commands
help arch           # Manjaro/Arch guide
help debian         # Ubuntu/Debian guide
help fedora         # Fedora/RHEL guide
help opensuse       # openSUSE guide
help linux          # Universal Linux commands
# Press ESC to exit help panel
```

### 8. Testing Network Tools
```bash
ping google.com     # Test connectivity
curl https://example.com
wget https://example.com/file.txt
ssh user@server     # SSH connection (if you have server)
```

### 9. Testing Danger Zone
```bash
# Try these (will show warnings):
rm -rf *            # Shows DANGER warning
sudo rm -rf /       # Shows NUCLEAR warning (blocked!)
chmod 000 file.txt  # Shows WARNING
```

### 10. Testing Streak System
```bash
# Execute several correct commands in a row:
ls
pwd
date
whoami
hostname
# Watch your streak counter increase!
# One error will reset it to 0
```

## Performance Testing

### Stress Test
```bash
# Execute many commands rapidly:
for i in {1..100}; do echo "test $i"; done
```

### Resource Monitoring
```bash
# While Munux is running, in another terminal:
htop                # Check CPU/Memory usage
# Munux should use < 5% CPU idle, < 15% active
# Memory should be < 20 MB
```

## Expected Behaviors

### ✅ Should Work
- All Linux commands execute normally
- XP increases with each command
- Achievements unlock at milestones
- Quests update in real-time
- Themes change with level progression
- Tux evolves visually
- Easter eggs trigger correctly
- Help system accessible via `help`
- ESC closes panels

### ⚠️ Known Limitations (v0.1.0)
- No Tab completion yet
- No command history persistence between sessions
- No progress save/load
- Some advanced shell features may not work (pipes, redirects might have issues)

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| **Type normally** | Add characters |
| **Enter** | Execute command |
| **Backspace** | Delete last character |
| **↑ / ↓** | Navigate history |
| **Ctrl+C** | Exit Munux |
| **Ctrl+L** | Clear screen |
| **ESC** | Clear input / Close panels |

## Troubleshooting Tests

### Test 1: Terminal Rendering
If you see ANSI codes instead of colors:
```bash
# Make sure your terminal supports 256 colors:
echo $TERM
# Should show: xterm-256color or similar
```

### Test 2: Exit Issues
If terminal gets stuck:
```bash
# In another terminal:
reset
# Or:
stty sane
```

### Test 3: Permission Issues
If commands fail:
```bash
# Check if you're in correct directory:
pwd
# Check permissions:
ls -la
```

## Automated Testing (Future)

### Unit Tests (Planned v0.2.0)
```bash
cargo test
```

### Integration Tests (Planned v0.2.0)
```bash
cargo test --test integration_tests
```

### Coverage (Planned v0.2.0)
```bash
cargo tarpaulin --out Html
```

---

## Test Results Checklist

Use this checklist when testing a new build:

- [ ] Munux starts without errors
- [ ] Split-screen displays correctly
- [ ] Commands execute and show output
- [ ] XP increases with commands
- [ ] Level up works correctly
- [ ] Theme changes at level milestones
- [ ] Tux evolves with levels
- [ ] Achievements unlock
- [ ] Quests track progress
- [ ] Easter eggs work
- [ ] Help system accessible
- [ ] ESC exits panels
- [ ] Ctrl+C exits Munux
- [ ] Package managers recognized
- [ ] Network commands work
- [ ] File operations work
- [ ] Danger zone shows warnings
- [ ] Streak counter works
- [ ] Stats panel shows correct data

---

**Testing Status:** ✅ Manual testing complete  
**Automated Tests:** 🚧 Planned for v0.2.0  
**Last Tested:** January 3, 2026

For issues or bugs, please report on [GitHub Issues](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues).
