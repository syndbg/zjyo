# zjyo [zee-jay-oh]

[![CI/CD](https://github.com/syndbg/zjyo/actions/workflows/ci-cd.yml/badge.svg)](https://github.com/syndbg/zjyo/actions/workflows/ci-cd.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/zjyo.svg)](https://crates.io/crates/zjyo)
[![Downloads](https://img.shields.io/crates/d/zjyo.svg)](https://crates.io/crates/zjyo)

A Rust implementation of the popular "z" directory navigation tool. This is a 1:1 port of [rupa/z](https://github.com/rupa/z) that maintains complete compatibility with the original while being implemented in Rust for better performance and reliability.

The name comes from "z is the new j, yo" that was the description
 of rupa's z once upon a time and hence the name `zjyo`.

## 🎬 Demo

![zjyo demo](.github/assets/demo.gif)

## Why zjyo?

After trying various directory jumping tools like `jump`, `zoxide`, and others, I found they all added unnecessary complexity or changed the behavior I was used to. All I wanted was a simple tool that uses the exact same algorithm as the original `z`, providing seamless integration without learning new commands or behaviors.

zjyo delivers exactly that:
- **100% compatible** with original z algorithm and database format
- **Same commands** - no need to relearn anything
- **Faster execution** thanks to Rust's performance
- **Cross-platform** support (Linux, macOS, Unix-like systems)
- **Minimal dependencies** - just works

```bash
# After using directories...
cd ~/projects/rust-project
cd ~/documents/important-notes
cd ~/downloads/tools

# Jump instantly with fuzzy matching
z rust     # → ~/projects/rust-project
z notes    # → ~/documents/important-notes
z tool     # → ~/downloads/tools
```

## Comparison with Other Tools

| Tool | Language | Database Compatibility | Algorithm | Learning Curve |
|------|----------|------------------------|-----------|----------------|
| **zjyo** | Rust | ✅ z compatible | ✅ Original z | None - same as z |
| [rupa/z](https://github.com/rupa/z) | Shell | ✅ Original | ✅ Original | None |
| [zoxide](https://github.com/ajeetdsouza/zoxide) | Rust | ❌ Custom | ❌ Different | New commands (`zi`, etc.) |
| [jump](https://github.com/gsamokovarov/jump) | Go | ❌ Custom | ❌ Different | New interface |
| [fasd](https://github.com/clvv/fasd) | Shell | ❌ Custom | ❌ Different | Complex options |

### Why Choose zjyo?

**If you're already using z**: Drop-in replacement with better performance and reliability.

**If you're new to directory jumping**: Learn one tool that's been proven for over a decade.

**If you tried other tools**: Get back to the simplicity and predictability of the original z algorithm.

**If you value compatibility**: Use the same database format and commands across different machines and implementations.

## ✨ Features

- 🔥 **100% Compatible** - Drop-in replacement for [rupa/z](https://github.com/rupa/z)
- ⚡ **Blazingly Fast** - Built with Rust for optimal performance
- 🗄️ **Same Database** - Uses identical format and algorithm as original z
- 🔀 **Cross-Platform** - Works on Linux, macOS, and other Unix-like systems
- 📊 **Smart Frecency** - Balances frequency and recency for intelligent navigation
- 🎯 **Zero Learning Curve** - Identical commands and behavior to original z
- 🛡️ **Memory Safe** - Rust's safety guarantees prevent crashes and data corruption
- 📦 **Minimal Dependencies** - Lightweight with fast startup time

## 🚀 Quick Start

### 📦 Installation

#### **Quick Install Script (Recommended)**

```bash
curl -sSL https://github.com/syndbg/zjyo/raw/main/install.sh | bash
```

#### **Cargo**

```bash
cargo install zjyo
```

#### **Pre-built Binaries**

Download from [Releases](https://github.com/syndbg/zjyo/releases/latest):

**Linux:**
```bash
# Download and install .deb package
curl -L -O https://github.com/syndbg/zjyo/releases/latest/download/zjyo_amd64.deb
sudo dpkg -i zjyo_amd64.deb

# Or .rpm for Red Hat/CentOS/Fedora
curl -L -O https://github.com/syndbg/zjyo/releases/latest/download/zjyo_x86_64.rpm
sudo rpm -i zjyo_x86_64.rpm
```

**macOS:**
```bash
# Download binary
curl -L -O https://github.com/syndbg/zjyo/releases/latest/download/zjyo-macos
chmod +x zjyo-macos
sudo mv zjyo-macos /usr/local/bin/zjyo
```

#### **Build from Source**

```bash
git clone https://github.com/syndbg/zjyo.git
cd zjyo
cargo build --release
sudo cp target/release/zjyo /usr/local/bin/
```

### 🔧 Shell Integration

Since binaries can't change your shell's directory, add these wrapper functions:

#### **Bash/Zsh** (`.bashrc` or `.zshrc`)

```bash
# zjyo wrapper function
z() {
    # Handle all flags that don't require directory change (pass through to zjyo directly)
    if [[ "$*" == *"--help"* ]] || [[ "$*" == *"-h"* ]] || [[ "$*" == *"-l"* ]] || [[ "$*" == *"-r"* ]] || [[ "$*" == *"-t"* ]] || [[ "$*" == *"-c"* ]] || [[ "$*" == *"-e"* ]] || [[ "$*" == *"-x"* ]] || [[ "$*" == *"--add"* ]]; then
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

# Auto-track directories when using cd
cd() {
    builtin cd "$@" && zjyo --add
}
```

#### **Fish Shell**

```fish
function z
    # Handle all flags that don't require directory change (pass through to zjyo directly)
    if contains -- "--help" $argv; or contains -- "-h" $argv; or contains -- "-l" $argv; or contains -- "-r" $argv; or contains -- "-t" $argv; or contains -- "-c" $argv; or contains -- "-e" $argv; or contains -- "-x" $argv; or contains -- "--add" $argv
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

# Auto-track directories when using cd
function cd
    builtin cd $argv; and zjyo --add
end
```

### 🎯 Basic Usage

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
```

### 🔧 Shell Completion

zjyo includes shell completion for commands and directory patterns:

#### **Bash**
```bash
# Add to ~/.bashrc
source /path/to/zjyo/completions/zjyo.bash
```

#### **Zsh**
```bash
# Add to ~/.zshrc or place in fpath
fpath=(~/.config/zjyo/completions $fpath)
# Then copy: cp completions/zjyo.zsh ~/.config/zjyo/completions/_zjyo
```

#### **Fish**
```bash
# Copy to fish completions directory
cp completions/zjyo.fish ~/.config/fish/completions/
```

**Smart Completions**: The completions intelligently suggest directory patterns from your actual z database!

## 📊 How It Works

### **Frecency Algorithm**

zjyo uses the same proven algorithm as the original z:

```
frecency = 10000 * rank * (3.75 / ((0.0001 * age_in_seconds + 1) + 0.25))
```

- **Rank** - Increments each visit (frequency)
- **Age** - Time since last visit (recency)
- **Balance** - Recent visits outweigh old frequent ones

### **Smart Database Management**

- 📁 **Location**: `~/.z` (or `$_Z_DATA` environment variable)
- 📝 **Format**: `/path/to/directory|rank|timestamp` (z-compatible)
- 🧹 **Auto-cleanup**: Aging when total ranks exceed 9000
- 🗑️ **Garbage collection**: Removes directories with rank < 1.0

## 📖 Complete CLI Reference

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
  -h, --help     Print help information
  -V, --version  Print version information
```

## 💡 Usage Examples

### **Basic Navigation**

```bash
# Build up your database
cd ~/projects/awesome-rust-project && z --add
cd ~/documents/work/reports && z --add
cd ~/downloads/development-tools && z --add

# Jump around with fuzzy matching
z awesome    # → ~/projects/awesome-rust-project
z rep        # → ~/documents/work/reports
z dev tool   # → ~/downloads/development-tools

# Check your frecency scores
z -l
# Output:
#   25000      5.0        /home/user/projects/awesome-rust-project
#   15000      3.0        /home/user/documents/work/reports
#   10000      2.0        /home/user/downloads/development-tools
```

### **Advanced Patterns**

```bash
# Multiple word matching - ALL words must be in the path
z rust proj    # Matches directories containing both "rust" and "proj"
z work doc     # Matches directories with "work" and "doc"
z projects api # Matches directories with both "projects" and "api"

# Rank-based navigation (frequency wins)
z -r config    # Jump to most frequently accessed config directory

# Time-based navigation (recency wins)
z -t temp      # Jump to most recently accessed temp directory

# Current directory restriction
cd ~/projects
z -c rust      # Only match rust directories under ~/projects

# Preview without jumping
z -e backend   # Print the match without cd'ing
```

### **Database Management**

```bash
# Manual tracking
z --add                    # Add current directory

# Remove directories
z -x                       # Remove current directory from database
rm ~/.z && touch ~/.z      # Nuclear option: clear entire database

# Integration with other tools
z -l | grep "old-project" | cut -d' ' -f3- | xargs rm -rf  # Cleanup old projects
```

## 🔧 Configuration

### **Environment Variables**

- `_Z_DATA` - Database location (default: `~/.z`)

## 🤝 Contributing

We welcome contributions! Whether you're fixing bugs, adding features, improving documentation, or optimizing performance - every contribution helps make zjyo better.

**👉 See our comprehensive [Contributing Guidelines](CONTRIBUTING.md) for:**
- Development setup and prerequisites
- Code style and testing guidelines
- Pull request process and review criteria
- Architecture principles and design goals

**Quick start:** Fork → Branch → Code → Test → PR 🚀

## 📄 License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.

The Apache 2.0 license ensures:
- ✅ Commercial use allowed
- ✅ Modification allowed
- ✅ Distribution allowed
- ✅ Patent use allowed
- ⚠️ Must include license and copyright
- ⚠️ Changes must be documented

## 🙏 Acknowledgments

- **[rupa](https://github.com/rupa)** - Creator of the original z tool
- **[Rust Community](https://www.rust-lang.org/community)** - For the amazing ecosystem
- **[All Contributors](https://github.com/syndbg/zjyo/graphs/contributors)** - Making zjyo better
---

**⚡ Jump faster. Navigate smarter. Stay compatible.**

*Made with ❤️ and ⚡ by [Anton Antonov](https://github.com/syndbg)*
