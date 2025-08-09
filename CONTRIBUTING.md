# Contributing to zjyo

We're thrilled that you're interested in contributing to zjyo! This guide will help you get started with contributing to this project, whether you're fixing bugs, adding features, improving documentation, or helping with maintenance.

## 🤝 Ways to Contribute

### 🐛 Bug Reports
- Search [existing issues](https://github.com/syndbg/zjyo/issues) to avoid duplicates
- Use the bug report template when filing new issues
- Include clear steps to reproduce, expected vs actual behavior
- Provide system information (OS, Rust version, shell)

### ✨ Feature Requests
- Check [existing feature requests](https://github.com/syndbg/zjyo/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
- Use the feature request template
- Explain the use case and benefit to users
- Consider backward compatibility with original z

### 📚 Documentation
- Fix typos, improve clarity, add examples
- Update README.md, inline docs, or code comments
- Help with shell integration examples
- Contribute to the project wiki

### 🧪 Testing
- Add unit tests for new functionality
- Improve integration test coverage
- Test on different platforms and shells
- Performance testing and benchmarking

### 🔧 Code Contributions
- Bug fixes and performance improvements
- New features (discuss in issues first for large changes)
- Code quality improvements and refactoring
- Shell completion implementations

## 🚀 Development Setup

### Prerequisites

- **Rust**: 1.89+ (stable channel recommended)
- **Git**: For version control
- **Shell**: bash, zsh, or fish for testing integrations
- **Docker**: Optional, for containerized testing

### Getting Started

1. **Fork and Clone**
```bash
# Fork the repository on GitHub, then:
git clone https://github.com/syndbg/zjyo.git
cd zjyo
```

2. **Build and Test**
```bash
# Build the project
cargo build --release
cargo test

# Run integration tests
./test.sh
```

3. **Verify Setup**
```bash
# Check code quality
cargo clippy -- -D warnings
cargo fmt --all --check

# Build documentation
cargo doc --open
```

### **Docker Testing**

```bash
# Build and test in container
docker build --load -t zjyo .
docker run --rm zjyo /test.sh
```

### **Project Structure**

```
zjyo/
├── src/               # Source code
│   ├── main.rs        # Binary entry point
│   ├── lib.rs         # Library root
│   ├── cli.rs         # Command-line interface
│   ├── database.rs    # Database operations
│   ├── entry.rs       # Directory entry logic
│   └── tests.rs       # Unit tests
├── tests/             # Integration tests
├── Cargo.toml         # Project manifest
```

### **Taskporter Integration**

This project includes [taskporter](https://github.com/syndbg/taskporter) configuration for streamlined development:

```bash
# List available development tasks
taskporter list

# Run development tasks
taskporter run build       # Build the project
taskporter run test        # Run all tests
taskporter run clippy      # Run clippy linting
taskporter run fmt         # Format code
taskporter run doc         # Generate documentation
```

### 🔧 Development Workflow

1. **Create a Branch**
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

2. **Make Changes**
   - Write clear, well-documented code
   - Add tests for new functionality
   - Update documentation as needed
   - Follow existing code style

3. **Test Your Changes**
```bash
# Unit tests
cargo test

# Integration tests
./test.sh

# Code quality
cargo clippy -- -D warnings
cargo fmt --all

# Test in Docker (optional)
docker build --load -t zjyo .
docker run --rm zjyo /test.sh
```

4. **Commit Changes**
```bash
# Use conventional commits
git commit -m "feat: add shell completion support"
git commit -m "fix: handle edge case in frecency calculation"
git commit -m "docs: improve installation instructions"
```

5. **Push and Create PR**
```bash
git push origin your-branch-name
# Then create a Pull Request on GitHub
```

## 📋 Development Guidelines

### Code Style

**Rust Code:**
- Follow `rustfmt` defaults (run `cargo fmt`)
- Use `clippy` suggestions (run `cargo clippy`)
- Prefer explicit over implicit when it improves clarity
- Write self-documenting code with clear variable names
- Add doc comments for public functions and modules

**Commit Messages:**
- Use [Conventional Commits](https://conventionalcommits.org/)
- Format: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- Keep first line under 50 characters
- Add detailed explanation in body if needed

**Examples:**
```
feat(cli): add shell completion support
fix(database): handle corrupted entries gracefully
docs(readme): improve installation instructions
test(integration): add edge case for empty database
```

### Testing Guidelines

**Unit Tests:**
- Test individual functions and modules
- Cover edge cases and error conditions
- Use descriptive test names
- Group related tests in modules

**Integration Tests:**
- Test complete workflows and CLI usage
- Test with real file system operations
- Verify compatibility with original z behavior
- Test error handling and recovery

**Example Test Structure:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frecency_calculation() {
        // Test the frecency algorithm
        let entry = DirEntry::new("/test/path", 5.0, 1640995200);
        let frecency = entry.frecency();
        assert!(frecency > 0.0);
    }

    #[test]
    fn test_edge_case_empty_pattern() {
        // Test behavior with empty search patterns
        let db = Database::new();
        let results = db.find_matches("", None);
        assert!(results.is_empty());
    }
}
```

### Documentation Standards

**Code Documentation:**
```rust
/// Calculates frecency score for a directory entry.
///
/// Frecency combines frequency (rank) and recency to determine
/// how likely a user wants to visit this directory.
///
/// # Arguments
/// * `rank` - Number of times directory has been visited
/// * `time` - Unix timestamp of last visit
///
/// # Returns
/// Frecency score as f64, higher values indicate more relevant directories
pub fn calculate_frecency(rank: f64, time: i64) -> f64 {
    // Implementation...
}
```

**README Updates:**
- Keep installation instructions current
- Add examples for new features
- Update CLI reference for new options
- Maintain compatibility information

### Performance Considerations

- **Startup Time**: Keep binary startup fast (<10ms)
- **Memory Usage**: Minimize memory footprint
- **Database Operations**: Optimize frecency calculations
- **File I/O**: Handle large databases efficiently

**Benchmarking:**
```bash
# Time critical operations
cargo bench

# Profile with perf (Linux)
perf record --call-graph=dwarf ./target/release/zjyo -l
perf report
```

## 🏗 Project Architecture

### Core Components

**`src/main.rs`**
- Binary entry point
- Minimal CLI setup and delegation

**`src/lib.rs`**
- Library root and public API
- Module exports and documentation

**`src/cli.rs`**
- Command-line argument parsing with clap
- Main application logic and command dispatch
- Shell integration considerations

**`src/database.rs`**
- Database file operations (read/write)
- Entry management and aging
- Frecency calculations and sorting

**`src/entry.rs`**
- Directory entry representation
- Parsing and serialization
- Path matching and validation

### Design Principles

1. **Compatibility First**: Maintain 100% compatibility with original z
2. **Performance**: Fast startup and efficient operations
3. **Safety**: Leverage Rust's memory safety guarantees
4. **Simplicity**: Keep the codebase readable and maintainable
5. **Testability**: Design for comprehensive testing

### Adding New Features

When adding features, consider:

1. **Backward Compatibility**: Will this break existing z users?
2. **Command Line Interface**: Does this fit the existing CLI pattern?
3. **Database Format**: Can we maintain the same database format?
4. **Testing**: How can we thoroughly test this feature?
5. **Documentation**: What docs need updates?

## 📦 Release Process

Releases are automated via GitHub Actions when changes are merged to main:

1. **Conventional Commits** determine version bump
2. **CHANGELOG.md** is automatically updated
3. **Git tags** are created automatically
4. **Binaries** are built for multiple platforms
5. **Crates.io** publication (maintainers only)

### Manual Release (Maintainers)

```bash
# Ensure main is up to date
git checkout main
git pull origin main

# Tag the release
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0

# GitHub Actions will handle the rest
```

## 🔍 Code Review Process

### For Contributors

- Keep PRs focused and reasonably sized
- Write clear PR descriptions explaining the change
- Respond to feedback promptly and professionally
- Update documentation and tests as needed
- Ensure CI passes before requesting review

### For Reviewers

- Be constructive and helpful in feedback
- Focus on code quality, correctness, and maintainability
- Consider performance and security implications
- Verify compatibility with original z behavior
- Test the changes locally when possible

### PR Checklist

- [ ] **Tests**: New code has appropriate test coverage
- [ ] **Documentation**: README, docs, and code comments updated
- [ ] **Compatibility**: Changes don't break z compatibility
- [ ] **Performance**: No significant performance regressions
- [ ] **CI**: All automated checks pass
- [ ] **Changelog**: Breaking changes noted for next release

## 🚨 Reporting Security Issues

**Do not open public issues for security vulnerabilities.**

Instead, email security concerns to: **security@antonov.ee**

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)

We'll respond within 48 hours and work with you to resolve the issue.

## 📞 Getting Help

### Development Questions

- **GitHub Discussions**: For design discussions and questions
- **Issues**: For bug reports and feature requests
- **Discord/Slack**: [Coming soon] For real-time development chat

### Code Review

- Request reviews from maintainers on your PRs
- Join the discussion on others' PRs to learn
- Participate in design discussions in issues

### Learning Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Clap Documentation**: https://docs.rs/clap/
- **Original z**: https://github.com/rupa/z (for compatibility reference)

## 🎯 Good First Issues

Look for issues tagged with `good first issue` or `help wanted`:

- Documentation improvements
- Test coverage expansion
- Code cleanup and refactoring
- Shell completion scripts
- Platform-specific fixes

## 🌟 Recognition

Contributors are recognized in:

- **README.md**: All contributors section
- **CHANGELOG.md**: Release notes with contributor credits
- **GitHub**: Automatic contributor recognition
- **Releases**: Contributor mentions in release notes

## 📋 Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). In summary:

- **Be welcoming and inclusive**
- **Be respectful and professional**
- **Be collaborative and helpful**
- **Report unacceptable behavior**

Violations can be reported to: conduct@antonov.ee

## 🙏 Thank You

Thank you for contributing to zjyo! Your efforts help make directory navigation faster and more reliable for developers worldwide.

Every contribution matters, whether it's:
- A one-character typo fix
- A major new feature
- A bug report with detailed reproduction steps
- Helping others in discussions

**Happy coding!** 🚀

---

*This contributing guide is inspired by best practices from the Rust community and projects like [clap](https://github.com/clap-rs/clap) and [serde](https://github.com/serde-rs/serde).*
