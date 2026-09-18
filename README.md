# zjyo [zee-jay-oh]

[![PR checks](https://github.com/syndbg/zjyo/actions/workflows/on_pr.yml/badge.svg)](https://github.com/syndbg/zjyo/actions/workflows/on_pr.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

A Rust port of [rupa/z](https://github.com/rupa/z): same algorithm, same database format, same commands. Rewritten in Rust mainly so the binary starts fast and doesn't depend on a shell interpreter for the matching logic.

The name comes from "z is the new j, yo", the description rupa's z used to carry.

## Demo

![zjyo demo](.github/assets/demo.gif)

## Why this exists

I tried `jump`, `zoxide`, and a few others before writing this. Each one changed the interface or the ranking behavior in ways I didn't want to relearn. What I actually wanted was the original `z` algorithm and database format, just not implemented in shell script.

zjyo is that: a drop-in for `z`, same frecency ranking, same `~/.z` file layout, so it works interchangeably with the original if you ever need to fall back.

| Tool | Language | Database format | Algorithm |
|------|----------|------------------|-----------|
| **zjyo** | Rust | z-compatible | original z |
| [rupa/z](https://github.com/rupa/z) | Shell | original | original |
| [zoxide](https://github.com/ajeetdsouza/zoxide) | Rust | custom | different |
| [jump](https://github.com/gsamokovarov/jump) | Go | custom | different |
| [fasd](https://github.com/clvv/fasd) | Shell | custom | different |

## Features

- Drop-in replacement for [rupa/z](https://github.com/rupa/z), same commands and database
- Frecency ranking that balances visit frequency and recency
- Cross-platform: Linux, macOS, other Unix-like systems
- No runtime dependencies beyond a shell for the wrapper function

## Installation

### Homebrew (recommended)

```bash
brew install --HEAD syndbg/tap/zjyo
```

Builds from `main` on macOS or Linux. To check for a newer commit:

```bash
brew upgrade --fetch-HEAD syndbg/tap/zjyo
```

A stable formula (`brew install syndbg/tap/zjyo`, no `--HEAD`) is also available, pinned to the latest tagged release.

### Quick install script

```bash
curl -sSL https://github.com/syndbg/zjyo/raw/main/install.sh | bash
```

### Pre-built binaries

Linux (`.deb`, `.rpm`, glibc/musl tarballs) and macOS (Intel/Apple Silicon tarballs) builds are available on the [Releases](https://github.com/syndbg/zjyo/releases/latest) page.

### Build from source

```bash
git clone https://github.com/syndbg/zjyo.git
cd zjyo
cargo build --release
sudo cp target/release/zjyo /usr/local/bin/
```

## Shell integration

The binary can't change your shell's working directory on its own, so it needs a wrapper function plus a hook that tracks directories as you move around.

Use the shell's prompt command (`precmd`) to track directories, not a `cd` override. This is what upstream `z` does. It fires on any directory change, not just explicit `cd` calls, and it doesn't get silently clobbered if another plugin also redefines `cd`. Run `zjyo --doctor` to check whether the hook is installed; see [Database](#database) below.

### Bash/Zsh (`.bashrc` or `.zshrc`)

```bash
z() {
    if [[ "$*" == *"--help"* ]] || [[ "$*" == *"-h"* ]] || [[ "$*" == *"--version"* ]] || [[ "$*" == *"-V"* ]] || [[ "$*" == *"-l"* ]] || [[ "$*" == *"-r"* ]] || [[ "$*" == *"-t"* ]] || [[ "$*" == *"-c"* ]] || [[ "$*" == *"-e"* ]] || [[ "$*" == *"-x"* ]] || [[ "$*" == *"--add"* ]] || [[ "$*" == *"--doctor"* ]]; then
        command zjyo "$@"
        return
    fi

    if [[ $# -eq 0 ]]; then
        command zjyo
    else
        local result=$(command zjyo -e "$*")
        if [[ -n $result ]]; then
            cd "$result"
        fi
    fi
}

# zsh: track directories via precmd, matches upstream z's approach
_zjyo_precmd() {
    (zjyo --add &)
}
[[ -n "$ZSH_VERSION" ]] && precmd_functions+=(_zjyo_precmd)

# bash has no precmd_functions array; use PROMPT_COMMAND instead
if [[ -n "$BASH_VERSION" ]]; then
    PROMPT_COMMAND="(zjyo --add &);${PROMPT_COMMAND}"
fi
```

### Fish shell

```fish
function z
    if contains -- "--help" $argv; or contains -- "-h" $argv; or contains -- "--version" $argv; or contains -- "-V" $argv; or contains -- "-l" $argv; or contains -- "-r" $argv; or contains -- "-t" $argv; or contains -- "-c" $argv; or contains -- "-e" $argv; or contains -- "-x" $argv; or contains -- "--add" $argv; or contains -- "--doctor" $argv
        command zjyo $argv
        return
    end

    if test (count $argv) -eq 0
        command zjyo
    else
        set result (command zjyo -e (string join " " $argv))
        if test -n "$result"
            cd "$result"
        end
    end
end

function cd
    builtin cd $argv; and zjyo --add
end
```

### Basic usage

```bash
# Track current directory
z --add

# Jump to directories (fuzzy matching)
z proj          # Navigate to project directory
z doc notes     # Navigate to documents/notes
z ~/.config     # Navigate to config directory

# List tracked directories
z -l            # Show all directories with scores
z -l proj       # Show directories matching "proj"

# Advanced navigation
z -r proj       # Match by rank (frequency) only
z -t proj       # Match by recent access only
z -c proj       # Restrict to subdirs of current directory
z -e proj       # Echo match without changing directory
z -x            # Remove current directory from database
z --doctor      # Remove missing directories from database
```

## Shell completion

### Bash
```bash
# Add to ~/.bashrc
source /path/to/zjyo/completions/zjyo.bash
```

### Zsh
```bash
# Add to ~/.zshrc or place in fpath
fpath=(~/.config/zjyo/completions $fpath)
# Then copy: cp completions/zjyo.zsh ~/.config/zjyo/completions/_zjyo
```

### Fish
```bash
cp completions/zjyo.fish ~/.config/fish/completions/
```

Completions suggest directory patterns pulled from your actual `.z` database.

## How it works

### Frecency algorithm

Same formula as the original z:

```
frecency = 10000 * rank * (3.75 / ((0.0001 * age_in_seconds + 1) + 0.25))
```

- **Rank** increments on each visit (frequency)
- **Age** is time since last visit (recency)
- Recent visits outweigh old frequent ones, but don't erase them outright

### Database

- Location: `~/.z` (or `$_Z_DATA`)
- Format: `/path/to/directory|rank|timestamp`, one entry per line, compatible with the original z
- Aging: when total rank across all entries exceeds 9000, every rank is multiplied by 0.99
- Garbage collection: entries with rank below 1.0 after aging are dropped
- `--doctor` reloads the database and drops any entry whose directory no longer exists on disk, then rewrites the file if anything was removed. It also checks whether a `precmd`/`PROMPT_COMMAND` hook that calls `zjyo --add` is present in your shell's rc file, and reports whether it found one.

## CLI reference

```
Usage: zjyo [OPTIONS] [PATTERN]

Arguments:
  [PATTERN]  Directory pattern to match (supports fuzzy matching)

Options:
  -l, --list     List matching directories with frecency scores
  -r, --rank     Match by rank (frequency) only
  -t, --time     Match by recent access time only
  -c, --current  Restrict matches to subdirectories of current directory
  -e, --echo     Echo the best match without changing directory
  -x, --remove   Remove current directory from database
      --add      Add current directory to database
      --doctor   Remove database entries for missing directories
  -h, --help     Print help information
  -V, --version  Print version information (includes the git commit hash)
```

## Usage examples

```bash
# Build up your database
cd ~/projects/awesome-rust-project && z --add
cd ~/documents/work/reports && z --add
cd ~/downloads/development-tools && z --add

# Jump around with fuzzy matching
z awesome    # → ~/projects/awesome-rust-project
z rep        # → ~/documents/work/reports
z dev tool   # → ~/downloads/development-tools

z -l
#   25000      5.0        /home/user/projects/awesome-rust-project
#   15000      3.0        /home/user/documents/work/reports
#   10000      2.0        /home/user/downloads/development-tools

# Multiple word matching - all words must be in the path
z rust proj    # Matches directories containing both "rust" and "proj"

# Rank-based navigation (frequency wins over recency)
z -r config

# Time-based navigation (recency wins over frequency)
z -t temp

# Current directory restriction
cd ~/projects
z -c rust      # Only match rust directories under ~/projects

# Preview without jumping
z -e backend

# Database management
z -x                       # Remove current directory from database
z --doctor                 # Remove entries for missing directories
rm ~/.z && touch ~/.z      # Clear entire database
```

## Configuration

- `_Z_DATA` - Database location (default: `~/.z`)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, code style, testing, and the pull request process.

## License

Apache License 2.0. See [LICENSE](LICENSE).

## Acknowledgments

- [rupa](https://github.com/rupa) for the original z tool and algorithm
- [Contributors](https://github.com/syndbg/zjyo/graphs/contributors)

---

*Made by [Anton Antonov](https://github.com/syndbg)*
