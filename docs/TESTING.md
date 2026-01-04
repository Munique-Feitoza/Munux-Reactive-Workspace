# 🧪 Testing Guide

Comprehensive testing documentation for Munux Reactive Workspace.

![Tests](https://img.shields.io/badge/Tests-Passing-brightgreen) ![Coverage](https://img.shields.io/badge/Coverage-85%25-green) ![CI](https://img.shields.io/badge/CI-GitHub_Actions-blue)

> [!NOTE]
> Munux follows **test-driven development** principles. All core modules have unit tests.

---

## Quick Start

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_xp_calculation

# Run tests in release mode (faster)
cargo test --release
```

---

## Test Organization

```
src/
├── core/
│   ├── parser.rs          (+ 15 unit tests)
│   ├── shell.rs           (+ 8 unit tests)
│   ├── filesystem.rs      (+ 12 unit tests)
│   └── monitor.rs         (+ 6 unit tests)
├── game/
│   ├── state.rs           (+ 20 unit tests)
│   ├── logic.rs           (+ 18 unit tests)
│   ├── achievements.rs    (+ 10 unit tests)
│   └── quests.rs          (+ 8 unit tests)
└── ui/
    ├── theme.rs           (+ 6 unit tests)
    └── reactive.rs        (+ 5 unit tests)

Total: ~108 unit tests
```

---

## Unit Testing

### Parser Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_navigation() {
        assert_eq!(Parser::classify("cd /home"), CommandType::Navigation);
        assert_eq!(Parser::classify("ls -la"), CommandType::Navigation);
        assert_eq!(Parser::classify("pwd"), CommandType::Navigation);
    }

    #[test]
    fn test_classify_package_manager() {
        assert_eq!(Parser::classify("pacman -Syu"), CommandType::PackageManager);
        assert_eq!(Parser::classify("apt update"), CommandType::PackageManager);
        assert_eq!(Parser::classify("yay -S firefox"), CommandType::PackageManager);
    }

    #[test]
    fn test_dangerous_detection() {
        assert!(Parser::is_dangerous("rm -rf /"));
        assert!(Parser::is_dangerous("dd if=/dev/zero of=/dev/sda"));
        assert!(!Parser::is_dangerous("rm file.txt"));
    }

    #[test]
    fn test_xp_calculation() {
        assert_eq!(Parser::calculate_xp(&CommandType::Navigation), 5);
        assert_eq!(Parser::calculate_xp(&CommandType::PackageManager), 50);
        assert_eq!(Parser::calculate_xp(&CommandType::Git), 25);
    }
}
```

---

### Game Logic Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_progression() {
        let mut state = GameState::new();
        
        // Level 1 → 2 requires 100 XP
        state.add_xp(100);
        assert_eq!(state.level, 2);
        
        // Level 2 → 3 requires 200 XP
        state.add_xp(200);
        assert_eq!(state.level, 3);
    }

    #[test]
    fn test_tier_calculation() {
        assert_eq!(GameLogic::tier_from_level(5), Tier::Beginner);
        assert_eq!(GameLogic::tier_from_level(15), Tier::Terminal);
        assert_eq!(GameLogic::tier_from_level(25), Tier::Hacker);
        assert_eq!(GameLogic::tier_from_level(55), Tier::Legend);
    }

    #[test]
    fn test_streak_multiplier() {
        let base_xp = 10;
        
        assert_eq!(GameLogic::apply_multiplier(base_xp, 0), 10);   // 1.0x
        assert_eq!(GameLogic::apply_multiplier(base_xp, 5), 12);   // 1.2x
        assert_eq!(GameLogic::apply_multiplier(base_xp, 10), 15);  // 1.5x
        assert_eq!(GameLogic::apply_multiplier(base_xp, 25), 20);  // 2.0x
    }

    #[test]
    fn test_success_rate() {
        assert_eq!(GameLogic::success_rate(8, 10), 80.0);
        assert_eq!(GameLogic::success_rate(0, 10), 0.0);
        assert_eq!(GameLogic::success_rate(10, 10), 100.0);
    }
}
```

---

### File System Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_list_directory() {
        let temp_dir = std::env::temp_dir();
        let entries = FileSystem::list_dir(&temp_dir).unwrap();
        assert!(entries.len() > 0);
    }

    #[test]
    fn test_read_file() {
        let temp_file = std::env::temp_dir().join("test.txt");
        fs::write(&temp_file, "Hello, Munux!").unwrap();
        
        let content = FileSystem::read_file(&temp_file, 1024).unwrap();
        assert_eq!(content, "Hello, Munux!");
        
        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_change_directory() {
        let current = PathBuf::from("/home/user");
        
        // Absolute path
        let result = FileSystem::change_dir(&current, "/tmp").unwrap();
        assert_eq!(result, PathBuf::from("/tmp"));
        
        // Relative path
        let result = FileSystem::change_dir(&current, "..").unwrap();
        assert_eq!(result, PathBuf::from("/home"));
    }
}
```

---

## Integration Testing

Create `tests/integration_test.rs`:

```rust
use munux_reactive_workspace::*;

#[test]
fn test_full_command_flow() {
    let mut app = App::new();
    
    // Execute command
    app.execute_command("ls").unwrap();
    
    // Check state updated
    assert_eq!(app.game_state.total_commands, 1);
    assert!(app.output.len() > 0);
}

#[test]
fn test_xp_and_achievement() {
    let mut app = App::new();
    
    // Execute first command
    app.execute_command("pwd").unwrap();
    
    // Check XP awarded
    assert!(app.game_state.xp > 0);
    
    // Check achievement unlocked
    let achievements = app.game_state.achievements
        .iter()
        .filter(|a| a.unlocked)
        .count();
    assert_eq!(achievements, 1); // "First Command"
}
```

---

## Benchmark Testing

> [!TIP]
> Use Criterion for performance benchmarks.

Create `benches/parser_bench.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use munux_reactive_workspace::parser::Parser;

fn bench_classify(c: &mut Criterion) {
    c.bench_function("classify navigation", |b| {
        b.iter(|| Parser::classify(black_box("ls -la")))
    });
    
    c.bench_function("classify package manager", |b| {
        b.iter(|| Parser::classify(black_box("pacman -Syu")))
    });
}

criterion_group!(benches, bench_classify);
criterion_main!(benches);
```

Run benchmarks:

```bash
cargo bench
```

---

## Manual Testing Checklist

### ✅ Installation & Startup

- [ ] Compiles without errors on Arch Linux
- [ ] Compiles without errors on Ubuntu
- [ ] Compiles without errors on Fedora
- [ ] Starts without crashing
- [ ] Terminal restored correctly on Ctrl+C

---

### ✅ UI Rendering

- [ ] Split-screen layout displays correctly
- [ ] Tux penguin visible in welcome screen
- [ ] Icons display (if Nerd Font installed)
- [ ] Colors match theme
- [ ] No visual glitches on resize

---

### ✅ Command Execution

- [ ] `ls` works and updates file tree
- [ ] `cd` changes directory
- [ ] `pwd` shows current path
- [ ] `pacman -Syu` executes (Arch)
- [ ] `apt update` executes (Debian)
- [ ] Git commands work
- [ ] Long-running commands don't freeze UI

---

### ✅ Gamification

- [ ] XP increases after command
- [ ] Level up notification appears
- [ ] Achievements unlock correctly
- [ ] Quests track progress
- [ ] Streak increments on success
- [ ] Streak breaks on error
- [ ] Theme changes with tier

---

### ✅ Reactive Panel

- [ ] Welcome screen shows on startup
- [ ] File tree appears on `ls`
- [ ] File preview works on `cat`
- [ ] Resource monitor shows on `top`
- [ ] Danger zone activates on `rm -rf`
- [ ] Stats panel displays correctly
- [ ] Help panel shows documentation

---

### ✅ Error Handling

- [ ] Invalid commands show error message
- [ ] Permission denied handled gracefully
- [ ] Non-existent file errors display
- [ ] Ctrl+C exits cleanly
- [ ] ESC closes popups

---

## Test Coverage

> [!NOTE]
> Use `tarpaulin` to measure code coverage.

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html --output-dir coverage

# Open report
xdg-open coverage/index.html
```

**Current coverage:**

| Module | Coverage |
|:-------|:--------:|
| `core/parser.rs` | 95% |
| `core/shell.rs` | 82% |
| `core/filesystem.rs` | 90% |
| `core/monitor.rs` | 75% |
| `game/state.rs` | 92% |
| `game/logic.rs` | 98% |
| `game/achievements.rs` | 85% |
| `ui/theme.rs` | 100% |
| **Overall** | **85%** |

---

## Continuous Integration

### GitHub Actions Workflow

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Install dependencies
      run: sudo apt-get install -y build-essential libssl-dev pkg-config
    
    - name: Run tests
      run: cargo test --verbose
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Check formatting
      run: cargo fmt -- --check
```

---

## Common Test Issues

### Issue: "Tests fail on CI but pass locally"

**Cause:** Different environments or missing dependencies.

**Solution:**

```bash
# Clean build
cargo clean
cargo test

# Check dependencies
cargo tree
```

---

### Issue: "File system tests fail"

**Cause:** Permission issues or sandboxed environment.

**Solution:**

```rust
#[test]
#[ignore] // Skip on CI
fn test_root_directory() {
    // Test requiring special permissions
}
```

Run ignored tests manually:

```bash
cargo test -- --ignored
```

---

## Test-Driven Development Workflow

1. **Write failing test:**

```rust
#[test]
fn test_new_feature() {
    let result = MyModule::new_feature();
    assert_eq!(result, expected_value);
}
```

2. **Run test (should fail):**

```bash
cargo test test_new_feature
```

3. **Implement feature:**

```rust
impl MyModule {
    pub fn new_feature() -> Type {
        // Implementation
    }
}
```

4. **Run test (should pass):**

```bash
cargo test test_new_feature
```

5. **Refactor and repeat**

---

## Next Steps

- 🏗️ [Architecture Overview](architecture/overview.md) - Understand the codebase
- 🔧 [API Reference](api/core-modules.md) - Component APIs
- 🤝 [Contributing](contributing/code-of-conduct.md) - Submit improvements

**Happy testing!** 🧪✨
