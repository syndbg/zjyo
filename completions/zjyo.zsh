#compdef zjyo z

# Zsh completion for zjyo
# Add this to your ~/.zshrc or place in your fpath:
# fpath=(~/.config/zjyo/completions $fpath)

_zjyo() {
    local context state line
    typeset -A opt_args

    _arguments \
        '(-l --list)'{-l,--list}'[List matching directories with frecency scores]' \
        '(-r --rank)'{-r,--rank}'[Match by rank (frequency) only]' \
        '(-t --time)'{-t,--time}'[Match by recent access time only]' \
        '(-c --current)'{-c,--current}'[Restrict matches to subdirectories of current directory]' \
        '(-e --echo)'{-e,--echo}'[Echo the best match without changing directory]' \
        '(-x --remove)'{-x,--remove}'[Remove current directory from database]' \
        '--add[Add current directory to database]' \
        '(-h --help)'{-h,--help}'[Print help information]' \
        '(-V --version)'{-V,--version}'[Print version information]' \
        '*:pattern:_zjyo_patterns'
}

_zjyo_patterns() {
    local patterns
    if [[ -f "${_Z_DATA:-$HOME/.z}" ]]; then
        patterns=(${(f)"$(cut -d'|' -f1 "${_Z_DATA:-$HOME/.z}" | xargs -I {} basename {} | sort -u)"})
        _describe 'directory patterns' patterns
    fi
}

_zjyo "$@"
