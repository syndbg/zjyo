#!/bin/bash
# zjyo installer script
# Usage: curl -sSL https://github.com/syndbg/zjyo/raw/main/install.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect OS and architecture
detect_platform() {
    local os="$(uname -s)"
    local arch="$(uname -m)"

    case "$os" in
        Darwin)
            PLATFORM="macos"
            ;;
        Linux)
            PLATFORM="linux"
            ;;
        *)
            error "Unsupported operating system: $os. zjyo supports Linux, macOS, and other Unix-like systems."
            ;;
    esac

    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            ;;
    esac

    info "Detected platform: $PLATFORM-$ARCH"
}

# Check if cargo is available
check_cargo() {
    if command -v cargo >/dev/null 2>&1; then
        info "Cargo found, installing via cargo install"
        cargo install zjyo
        success "zjyo installed successfully via cargo!"
        return 0
    else
        info "Cargo not found, will download binary"
        return 1
    fi
}

# Download and install binary
install_binary() {
    local binary_name="zjyo"
    local download_url="https://github.com/syndbg/zjyo/releases/latest/download/zjyo-${PLATFORM}-${ARCH}"

    info "Downloading zjyo from $download_url"

    # Create temporary directory
    local tmp_dir=$(mktemp -d)
    local tmp_file="$tmp_dir/zjyo"

    # Download
    if command -v curl >/dev/null 2>&1; then
        curl -sSL "$download_url" -o "$tmp_file"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$download_url" -O "$tmp_file"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi

    # Make executable
    chmod +x "$tmp_file"

    # Install to system
    local install_dir="/usr/local/bin"
    if [ ! -w "$install_dir" ]; then
        warning "No write permission to $install_dir, trying with sudo"
        sudo mv "$tmp_file" "$install_dir/zjyo"
    else
        mv "$tmp_file" "$install_dir/zjyo"
    fi

    # Cleanup
    rm -rf "$tmp_dir"

    success "zjyo installed to $install_dir/zjyo"
}

# Install shell completions
install_completions() {
    info "Installing shell completions..."

    # Create completions directory
    local comp_dir="$HOME/.config/zjyo/completions"
    mkdir -p "$comp_dir"

    # Download completion files
    local base_url="https://github.com/syndbg/zjyo/raw/main/completions"

    for shell in bash zsh fish; do
        if command -v curl >/dev/null 2>&1; then
            curl -sSL "$base_url/zjyo.$shell" -o "$comp_dir/zjyo.$shell"
        else
            wget -q "$base_url/zjyo.$shell" -O "$comp_dir/zjyo.$shell"
        fi
    done

    success "Shell completions installed to $comp_dir"

    # Provide instructions
    echo
    info "To enable shell completion, add one of the following to your shell config:"
    echo
    echo "  Bash (~/.bashrc):"
    echo "    source $comp_dir/zjyo.bash"
    echo
    echo "  Zsh (~/.zshrc):"
    echo "    fpath=($comp_dir \$fpath)"
    echo "    autoload -U compinit && compinit"
    echo
    echo "  Fish:"
    echo "    cp $comp_dir/zjyo.fish ~/.config/fish/completions/"
}

# Show usage instructions
show_usage() {
    echo
    success "zjyo installation complete! 🎉"
    echo
    info "Next steps:"
    echo "  1. Add shell integration to your shell config file:"
    echo
    echo "     For Bash/Zsh (~/.bashrc or ~/.zshrc):"
    echo '     z() {'
    echo '         if [[ "$*" == *"--help"* ]] || [[ "$*" == *"-h"* ]] || [[ "$*" == *"-l"* ]] || [[ "$*" == *"-r"* ]] || [[ "$*" == *"-t"* ]] || [[ "$*" == *"-c"* ]] || [[ "$*" == *"-e"* ]] || [[ "$*" == *"-x"* ]] || [[ "$*" == *"--add"* ]]; then'
    echo '             command zjyo "$@"'
    echo '             return'
    echo '         fi'
    echo '         if [[ $# -eq 0 ]]; then'
    echo '             command zjyo'
    echo '         else'
    echo '             local result=$(command zjyo -e "$*")'
    echo '             if [[ -n $result ]]; then'
    echo '                 cd "$result"'
    echo '             fi'
    echo '         fi'
    echo '     }'
    echo '     cd() { builtin cd "$@" && zjyo --add; }'
    echo
    echo "     For Fish (~/.config/fish/config.fish):"
    echo '     function z'
    echo '         if contains -- "--help" $argv; or contains -- "-h" $argv; or contains -- "-l" $argv; or contains -- "-r" $argv; or contains -- "-t" $argv; or contains -- "-c" $argv; or contains -- "-e" $argv; or contains -- "-x" $argv; or contains -- "--add" $argv'
    echo '             command zjyo $argv'
    echo '             return'
    echo '         end'
    echo '         if test (count $argv) -eq 0'
    echo '             command zjyo'
    echo '         else'
    echo '             set result (command zjyo -e (string join " " $argv))'
    echo '             if test -n "$result"'
    echo '                 cd "$result"'
    echo '             end'
    echo '         end'
    echo '     end'
    echo '     function cd; builtin cd $argv; and zjyo --add; end'
    echo
    echo "  2. Restart your shell or source your config file"
    echo "  3. Start using: z --add (to track), z pattern (to jump)"
    echo
    info "For more information, visit: https://github.com/syndbg/zjyo"
}

# Main installation flow
main() {
    echo
    info "Installing zjyo - the fast, compatible z directory jumper"
    echo

    detect_platform

    # Try cargo first, fallback to binary
    if ! check_cargo; then
        install_binary
    fi

    # Verify installation
    if command -v zjyo >/dev/null 2>&1; then
        local version=$(zjyo --version)
        success "zjyo installed: $version"
    else
        error "Installation verification failed"
    fi

    # Install completions
    install_completions

    # Show usage
    show_usage
}

# Run main function
main "$@"
