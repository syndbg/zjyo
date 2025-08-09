# Bash completion for zjyo
# Add this to your ~/.bashrc or source it:
# source /path/to/zjyo.bash

_zjyo() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="-l --list -r --rank -t --time -c --current -e --echo -x --remove --add -h --help -V --version"

    case "${prev}" in
        zjyo)
            if [[ ${cur} == -* ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            else
                # Complete with directory patterns from z database
                if [[ -f "${_Z_DATA:-$HOME/.z}" ]]; then
                    local patterns=$(cut -d'|' -f1 "${_Z_DATA:-$HOME/.z}" | xargs -I {} basename {} | sort -u)
                    COMPREPLY=( $(compgen -W "${patterns}" -- ${cur}) )
                fi
                return 0
            fi
            ;;
        *)
            ;;
    esac
}

complete -F _zjyo zjyo z
