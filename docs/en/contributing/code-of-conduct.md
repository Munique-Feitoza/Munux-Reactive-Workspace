# 🤝 Contributing to Munux

Welcome! We're excited that you want to contribute to Munux Reactive Workspace.

![Contributors](https://img.shields.io/badge/Contributors-Welcome-brightgreen) ![PRs](https://img.shields.io/badge/PRs-Welcome-blue) ![License](https://img.shields.io/badge/License-GPLv3-red)

---

## 🌟 Ways to Contribute

There are many ways to contribute to Munux:

- 🐛 **Report Bugs** - Found an issue? Let us know!
- ✨ **Suggest Features** - Have an idea? We'd love to hear it!
- 📝 **Improve Documentation** - Help make our docs better!
- 🔧 **Submit Code** - Fix bugs or implement features!
- 🎨 **Design Assets** - Create themes, icons, ASCII art!
- 🌍 **Translate** - Help localize Munux!
- 💬 **Help Others** - Answer questions in Discussions!

---

## 🐛 Reporting Bugs

### Before Submitting

1. **Check existing issues** - Your bug may already be reported
2. **Try the latest version** - `git pull origin main`
3. **Read troubleshooting** - See [Troubleshooting Guide](guides/troubleshooting.md)

### Bug Report Template

```markdown
**Description:**
A clear description of the bug.

**Steps to Reproduce:**
1. Open Munux
2. Type command '...'
3. See error

**Expected Behavior:**
What should have happened.

**Actual Behavior:**
What actually happened.

**Environment:**
- OS: [e.g., Arch Linux, Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Terminal: [e.g., Konsole, GNOME Terminal]
- Nerd Font: [Yes/No]

**Screenshots:**
If applicable, add screenshots.

**Additional Context:**
Any other relevant information.
```

[**Submit Bug Report →**](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues/new?template=bug_report.md)

---

## ✨ Feature Requests

### Before Suggesting

1. **Check existing issues** - Your idea may already exist
2. **Read the roadmap** - See [CHANGELOG.md](CHANGELOG.md#roadmap)
3. **Consider scope** - Does it fit Munux's vision?

### Feature Request Template

```markdown
**Feature Description:**
A clear description of the feature.

**Problem It Solves:**
What user pain point does this address?

**Proposed Solution:**
How would this feature work?

**Alternatives Considered:**
Other ways to solve the same problem.

**Implementation Ideas:**
Technical approach (if you have one).

**Additional Context:**
Screenshots, mockups, examples.
```

[**Submit Feature Request →**](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues/new?template=feature_request.md)

---

## 🔧 Contributing Code

### Development Setup

1. **Fork the repository**

```bash
# Click "Fork" on GitHub
```

2. **Clone your fork**

```bash
git clone https://github.com/YOUR_USERNAME/Munux-Reactive-Workspace.git
cd Munux-Reactive-Workspace
```

3. **Add upstream remote**

```bash
git remote add upstream https://github.com/Munique-Feitoza/Munux-Reactive-Workspace.git
```

4. **Create a branch**

```bash
git checkout -b feature/my-awesome-feature
```

5. **Make your changes**

6. **Test your changes**

```bash
cargo test
cargo clippy
cargo fmt
```

7. **Commit with conventional commits**

```bash
git commit -m "feat: add awesome feature"
```

8. **Push to your fork**

```bash
git push origin feature/my-awesome-feature
```

9. **Create Pull Request**

Go to GitHub and click "New Pull Request"

---

## 📋 Commit Message Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

| Type | Description | Example |
|:-----|:------------|:--------|
| `feat` | New feature | `feat: add SSH command support` |
| `fix` | Bug fix | `fix: correct XP calculation` |
| `docs` | Documentation | `docs: update installation guide` |
| `style` | Code style | `style: format with rustfmt` |
| `refactor` | Code refactoring | `refactor: simplify parser logic` |
| `test` | Tests | `test: add unit tests for parser` |
| `chore` | Maintenance | `chore: update dependencies` |
| `perf` | Performance | `perf: optimize rendering loop` |

### Examples

```bash
# Good commits
git commit -m "feat(game): add new achievement for SSH usage"
git commit -m "fix(ui): resolve theme rendering bug"
git commit -m "docs: add mermaid diagrams to architecture"

# Bad commits
git commit -m "fixed stuff"
git commit -m "WIP"
git commit -m "changes"
```

---

## 🎨 Code Style

### Rust Style Guidelines

1. **Use `rustfmt`**

```bash
cargo fmt
```

2. **Follow Clippy suggestions**

```bash
cargo clippy -- -D warnings
```

3. **Write documentation**

```rust
/// Calculate XP reward for a command type.
///
/// # Arguments
/// * `cmd_type` - The type of command executed
///
/// # Returns
/// The base XP amount (before multipliers)
///
/// # Examples
/// ```
/// let xp = Parser::calculate_xp(&CommandType::PackageManager);
/// assert_eq!(xp, 50);
/// ```
pub fn calculate_xp(cmd_type: &CommandType) -> u32 {
    match cmd_type {
        CommandType::Navigation => 5,
        CommandType::PackageManager => 50,
        // ...
    }
}
```

4. **Write tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xp_calculation() {
        assert_eq!(Parser::calculate_xp(&CommandType::Navigation), 5);
        assert_eq!(Parser::calculate_xp(&CommandType::PackageManager), 50);
    }
}
```

---

## ✅ Pull Request Checklist

Before submitting a PR:

- [ ] Code compiles without errors: `cargo build --release`
- [ ] All tests pass: `cargo test`
- [ ] No Clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt`
- [ ] Documentation updated (if needed)
- [ ] Tests added for new features
- [ ] Commit messages follow convention
- [ ] PR description explains changes
- [ ] No merge conflicts with `main`

---

## 🔍 Code Review Process

1. **Automated checks** run (CI/CD)
2. **Maintainer reviews** code
3. **Feedback provided** (if needed)
4. **Revisions made** (if needed)
5. **PR approved** and merged!

**Typical review time:** 1-3 days

---

## 🏗️ Project Structure

Understanding the codebase:

```
munux-reactive-workspace/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Application state
│   ├── event.rs             # Event handling
│   ├── tui.rs               # Terminal management
│   ├── core/                # Core business logic
│   │   ├── parser.rs        # Command classification
│   │   ├── shell.rs         # Shell execution
│   │   ├── filesystem.rs    # File operations
│   │   └── monitor.rs       # System monitoring
│   ├── game/                # Gamification system
│   │   ├── state.rs         # Game state
│   │   ├── logic.rs         # Game calculations
│   │   ├── achievements.rs  # Achievements
│   │   └── quests.rs        # Quest system
│   └── ui/                  # User interface
│       ├── terminal.rs      # Terminal panel
│       ├── reactive.rs      # Reactive panel
│       ├── theme.rs         # Theme system
│       └── hud.rs           # Status bar
├── docs/                    # Documentation
│   ├── guides/              # User guides
│   ├── architecture/        # Technical docs
│   └── api/                 # API reference
├── tests/                   # Integration tests
└── Cargo.toml               # Dependencies
```

See [Architecture Overview](architecture/overview.md) for detailed design.

---

## 🧪 Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Integration Tests

Create `tests/integration_test.rs`:

```rust
use munux_reactive_workspace::*;

#[test]
fn test_end_to_end_flow() {
    let mut app = App::new();
    app.execute_command("ls").unwrap();
    assert!(app.game_state.xp > 0);
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_xp_calculation

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

---

## 📚 Documentation Guidelines

### Code Documentation

- ✅ **Document all public APIs**
- ✅ **Include examples** in doc comments
- ✅ **Explain complex logic** with inline comments
- ✅ **Keep docs up-to-date** with code changes

### Markdown Documentation

- ✅ **Use GitHub Flavored Markdown**
- ✅ **Add callouts** (`> [!NOTE]`, `> [!TIP]`, `> [!WARNING]`)
- ✅ **Include Mermaid diagrams** for complex flows
- ✅ **Add code examples** with proper syntax highlighting
- ✅ **Use tables** for structured data

---

## 🌍 Internationalization

Want to translate Munux? We'd love your help!

**Currently supported:**
- 🇺🇸 English (primary)
- 🇧🇷 Portuguese (backup)

**Planned:**
- 🇪🇸 Spanish
- 🇫🇷 French
- 🇩🇪 German
- 🇯🇵 Japanese

Contact us if you'd like to contribute a translation!

---

## 💬 Community Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive environment.

#### Our Standards

✅ **DO:**
- Be respectful and kind
- Welcome newcomers
- Give constructive feedback
- Accept criticism gracefully
- Focus on what's best for the community

❌ **DON'T:**
- Use offensive language
- Harass or troll others
- Share private information
- Engage in political arguments
- Spam or advertise

#### Enforcement

Violations may result in:
1. **Warning** - First offense
2. **Temporary ban** - Repeated offenses
3. **Permanent ban** - Severe or continued violations

Report violations to: [GitHub Issues](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues)

---

## 🏆 Recognition

Contributors are recognized in:

- 📝 **CHANGELOG.md** - Feature/fix credits
- 🌟 **GitHub Contributors** page
- 🎉 **Release notes** - Special mentions
- 💬 **Community highlights** - Outstanding contributions

---

## 📞 Getting Help

Need help contributing?

1. **Read the docs**: [Architecture](architecture/overview.md), [API](api/core-modules.md)
2. **Check discussions**: [GitHub Discussions](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/discussions)
3. **Ask questions**: Open a discussion or issue
4. **Join the community**: Participate in existing discussions

---

## 🎯 Good First Issues

New to Munux? Look for issues labeled:

- `good first issue` - Perfect for newcomers
- `help wanted` - We need assistance
- `documentation` - Improve our docs
- `enhancement` - Add new features

[**Browse Good First Issues →**](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

---

## 🚀 Release Process

Maintainers follow this process:

1. **Merge PRs** to `main`
2. **Update CHANGELOG.md**
3. **Bump version** in `Cargo.toml`
4. **Create git tag** (`v0.2.0`)
5. **Push tag** to trigger release
6. **Publish release** on GitHub

---

## 📄 License

By contributing, you agree that your contributions will be licensed under the **GNU General Public License v3.0**.

See [LICENSE](../LICENSE) for details.

---

## 🙏 Thank You!

Every contribution, no matter how small, makes Munux better. Thank you for being part of our community!

**Happy coding!** 🐧💻✨

---

## Quick Links

- 🌐 [Repository](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace)
- 📖 [Documentation](README.md)
- 🐛 [Issues](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/issues)
- 💬 [Discussions](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/discussions)
- 📋 [Project Board](https://github.com/Munique-Feitoza/Munux-Reactive-Workspace/projects)
