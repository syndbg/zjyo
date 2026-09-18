# Contributing to zjyo

## Ways to contribute

- **Bug reports**: search [existing issues](https://github.com/syndbg/zjyo/issues) first, include repro steps and your OS/shell.
- **Feature requests**: check [existing ones](https://github.com/syndbg/zjyo/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement), explain the use case, and note whether it stays compatible with original `z`.
- **Docs**: fix typos, improve clarity, update shell integration examples.
- **Tests**: unit tests, integration tests, cross-platform/cross-shell coverage.
- **Code**: bug fixes, features (discuss large ones in an issue first), refactoring.

## Development setup

Prerequisites: Rust 1.89+ (stable), Git, a shell (bash/zsh/fish) to test integrations.

```bash
git clone https://github.com/syndbg/zjyo.git
cd zjyo
cargo build --release
cargo test --all-features --workspace
```

Enable the pre-commit hook (`cargo fmt --check` + `cargo clippy`, same checks as CI):

```bash
git config core.hooksPath scripts/git-hooks
```

## Project structure

```
zjyo/
├── src/
│   ├── main.rs        # Binary entry point
│   ├── lib.rs          # Library root
│   ├── cli.rs           # Argument parsing and command dispatch
│   ├── database.rs      # Persistence, matching, aging
│   ├── entry.rs          # Directory entry model
│   └── tests.rs           # Unit tests
├── tests/integration_tests.rs
└── Cargo.toml
```

## Workflow

1. Branch: `git checkout -b feat/your-feature` or `fix/issue-description`.
2. Make the change. Add tests. Update docs if behavior changed.
3. Verify:
   ```bash
   cargo test --all-features --workspace
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt -- --check
   ```
4. Commit with [Conventional Commits](https://conventionalcommits.org/): `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`. Keep the first line under 50 characters. A `!` after the type (`feat!:`) signals a breaking change and bumps the major version.
5. Push and open a PR.

## Code style

Follow `rustfmt` defaults and `clippy` suggestions. Keep modules small, named after their domain. Doc-comment public functions when the behavior isn't obvious from the name. Preserve compatibility with original `z` before adding convenience features on top.

## Testing

Unit tests live in `src/tests.rs`, named by behavior (`test_find_matches_case_insensitive`, `test_rank_sorting`). Integration tests in `tests/integration_tests.rs` cover full CLI workflows. Cover database-format compatibility, frecency sorting, and edge cases around missing or temporary data files.

## Release process

Releases are automated: `on_main.yml` reads conventional commits since the last tag to decide the version bump (`fix:` → patch, `feat:` → minor, `!` → major), then `cargo release` bumps `Cargo.toml`, commits, and tags. The tag push triggers `on_release.yml`, which builds Linux/macOS binaries and `.deb`/`.rpm` packages and publishes a GitHub Release. No manual tagging needed.

## Code review

Keep PRs focused. Explain the change and link the issue if there is one. Make sure CI passes before requesting review. Reviewers check correctness, compatibility with original `z`, and whether docs/tests need updates alongside the code.

## Reporting security issues

Don't open a public issue for a vulnerability. Email **security@antonov.ee** with a description, repro steps, and impact. We'll respond within 48 hours.

## Code of conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Report violations to conduct@antonov.ee.
