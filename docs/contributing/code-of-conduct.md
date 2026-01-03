# Contributing to Munux

Thank you for your interest in contributing to Munux! This guide will help you get started with contributing to the project.

## Code of Conduct

By participating in this project, you agree to maintain a respectful, inclusive, and harassment-free environment for everyone. We are committed to providing a welcoming experience for all contributors.

### Our Standards

**Positive behavior includes:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

**Unacceptable behavior includes:**
- Trolling, insulting/derogatory comments, and personal attacks
- Public or private harassment
- Publishing others' private information without permission
- Other conduct which could reasonably be considered inappropriate

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

**Good bug reports include:**
- Clear, descriptive title
- Steps to reproduce the issue
- Expected vs actual behavior
- Screenshots (if applicable)
- System information (OS, Rust version, etc.)
- Error messages or logs

**Example Bug Report:**
```markdown
**Title**: Danger zone panel doesn't show for `rm -rf` command

**Description**: 
When typing `rm -rf test/`, the danger zone panel should appear 
but the welcome screen is shown instead.

**Steps to Reproduce**:
1. Start Munux
2. Type `rm -rf test/`
3. Observe right panel

**Expected**: Danger zone warning panel
**Actual**: Welcome screen remains

**Environment**:
- OS: Manjaro Linux
- Munux version: 0.1.0
- Rust version: 1.75.0
```

### Suggesting Features

Feature suggestions are welcome! Please include:
- Clear description of the feature
- Use cases and benefits
- Possible implementation approach
- Any relevant examples or mockups

### Pull Requests

1. **Fork the repository**
   ```bash
   git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
   cd Munux-Reactive-Workspace
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b feature/my-awesome-feature
   ```

3. **Make your changes**
   - Follow the coding standards (see below)
   - Add tests if applicable
   - Update documentation

4. **Test your changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

5. **Commit with clear messages**
   ```bash
   git commit -m "feat: add new achievement for systemctl usage"
   ```

6. **Push to your fork**
   ```bash
   git push origin feature/my-awesome-feature
   ```

7. **Open a Pull Request**
   - Provide a clear description
   - Reference related issues
   - Include screenshots/videos if UI changes

## Development Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace

# Build project
cargo build

# Run tests
cargo test

# Run in development mode
cargo run
```

### Project Structure

```
src/
├── main.rs              # Entry point
├── app.rs               # Application state
├── tui.rs               # Terminal setup
├── event.rs             # Event handling
├── core/                # Business logic
│   ├── parser.rs        # Command parsing
│   ├── shell.rs         # Shell execution
│   ├── filesystem.rs    # File operations
│   └── monitor.rs       # System monitoring
├── ui/                  # User interface
│   ├── mod.rs           # UI orchestration
│   ├── layout.rs        # Layout management
│   ├── terminal.rs      # Terminal panel
│   ├── reactive.rs      # Reactive panel
│   ├── hud.rs           # Status bar
│   ├── stats.rs         # Stats panels
│   ├── popup.rs         # Popups
│   └── theme.rs         # Theming system
└── game/                # Gamification
    ├── state.rs         # Game state
    ├── logic.rs         # Game logic
    ├── achievements.rs  # Achievements
    ├── quests.rs        # Quest system
    ├── easter_eggs.rs   # Easter eggs
    └── distro_guide.rs  # Help guides
```

## Coding Standards

### Rust Style Guide

Follow the [Rust Style Guide](https://rust-lang.github.io/api-guidelines/):

```rust
// Good: descriptive names, proper error handling
pub fn execute_command(&mut self, command: &str) -> Result<String> {
    let output = shell::execute(command)?;
    self.record_success();
    Ok(output)
}

// Bad: unclear names, unwrap
pub fn exec(&mut self, c: &str) -> String {
    shell::execute(c).unwrap()
}
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

### Documentation

- Add doc comments for public APIs
- Include examples in documentation
- Update README.md and docs/ as needed

```rust
/// Executes a shell command and updates game state.
///
/// # Arguments
/// * `command` - The command string to execute
///
/// # Returns
/// * `Ok(String)` - Command output on success
/// * `Err(anyhow::Error)` - Error if execution fails
///
/// # Example
/// ```
/// let output = app.execute_command("ls -la")?;
/// println!("Output: {}", output);
/// ```
pub fn execute_command(&mut self, command: &str) -> Result<String> {
    // Implementation
}
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new achievement for Docker commands
fix: correct XP calculation for package managers
docs: update installation guide for Fedora
refactor: simplify command parser logic
test: add tests for quest system
chore: update dependencies
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

## Testing

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_classification() {
        assert_eq!(
            CommandParser::classify_command("pacman -S firefox"),
            CommandType::PackageManager
        );
    }

    #[test]
    fn test_xp_calculation() {
        let xp = calculate_xp_reward("ls", true);
        assert_eq!(xp, 5);
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_command_classification

# Run with output
cargo test -- --nocapture

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin
```

## Areas for Contribution

### High Priority

- 🐛 **Bug Fixes**: Fix reported issues
- 📖 **Documentation**: Improve or translate docs
- ✅ **Tests**: Add unit and integration tests
- ♿ **Accessibility**: Improve keyboard navigation

### Medium Priority

- 🎨 **Themes**: Create new color themes and Tux variations
- 🏆 **Achievements**: Design new achievements
- 📋 **Quests**: Create educational quest chains
- 🥚 **Easter Eggs**: Add fun hidden commands

### Future Features

- 💾 **Persistence**: Implement save/load system
- 🌍 **i18n**: Add internationalization support
- 🔌 **Plugins**: Design plugin architecture
- 🎮 **Multiplayer**: Add leaderboard/comparison features

## Getting Help

- **GitHub Issues**: [Open an issue](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues)
- **GitHub Discussions**: [Join discussions](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/discussions)
- **Documentation**: Check [docs/](../README.md)

## Recognition

Contributors will be:
- Listed in AUTHORS.md
- Mentioned in release notes
- Credited in achievement unlocks (for new achievements)

## License

By contributing, you agree that your contributions will be licensed under the GNU General Public License v3.0.

---

**Thank you for making Munux better! 🚀**
