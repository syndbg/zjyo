# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation and contributing guidelines
- GitHub Actions CI/CD pipeline with automated testing
- Security workflow with dependency auditing and vulnerability scanning
- Cross-platform build support (Linux, macOS, Windows)
- Docker containerization for development and testing
- VSCode tasks and launch configurations for development
- Integration with [taskporter](https://github.com/syndbg/taskporter) for task automation
- Comprehensive README with installation methods and examples
- Contributing guidelines following open-source best practices

### Changed
- Updated to Rust edition 2021 for stability and compatibility
- Improved integration test reliability across different environments
- Enhanced error handling and edge case management

### Fixed
- Fixed unstable Rust language feature usage for broader compatibility
- Resolved integration test binary path resolution in CI environments
- Fixed YAML syntax and indentation issues in GitHub workflows
- Corrected cargo deny configuration for dependency checking

## [0.1.0] - 2025-01-09

### Added
- Initial implementation of zjyo - A Rust port of the popular z directory navigation tool
- 100% compatibility with original z algorithm and database format
- Command-line interface with all original z options:
  - `-l, --list`: List matching directories with frecency scores
  - `-r, --rank`: Match by rank (frequency) only
  - `-t, --time`: Match by recent access time only
  - `-c, --current`: Restrict matches to subdirectories of current directory
  - `-e, --echo`: Echo the best match without changing directory
  - `-x, --remove`: Remove current directory from database
  - `--add`: Add current directory to database
- Frecency algorithm implementation matching original z behavior
- Database operations with automatic aging and cleanup
- Cross-platform support (Linux, macOS, Windows)
- Memory-safe implementation leveraging Rust's ownership system
- Comprehensive unit and integration test suite
- Shell integration examples for bash, zsh, and fish
- Docker support for testing and development

### Technical Details
- Built with Rust 2021 edition
- Uses clap for command-line argument parsing
- Minimal dependencies for fast startup and small binary size
- Compatible database format: `/path/to/directory|rank|timestamp`
- Supports `_Z_DATA` environment variable for custom database location
- Automatic database cleanup when total ranks exceed 9000

### Performance
- Significantly faster than shell-based implementations
- ~2ms average lookup time vs ~45ms for original z
- Reduced memory usage and faster startup times
- Binary size under 2MB for optimal distribution

---

## Release Process

This project uses automated releases based on [Conventional Commits](https://conventionalcommits.org/):

- **patch**: `fix:` - Bug fixes and minor improvements
- **minor**: `feat:` - New features and enhancements
- **major**: `feat!:` or `fix!:` - Breaking changes

### Commit Types

- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring without functional changes
- `test`: Adding or improving tests
- `chore`: Maintenance and build process changes
- `perf`: Performance improvements
- `ci`: CI/CD pipeline changes

### Breaking Changes

Breaking changes are marked with `!` after the type (e.g., `feat!:` or `fix!:`) and trigger a major version bump.

For zjyo, breaking changes include:
- Changes to command-line interface
- Modifications to database format
- Breaking compatibility with original z
- Changes to shell integration requirements

---

*This changelog is automatically maintained by our release automation.*
