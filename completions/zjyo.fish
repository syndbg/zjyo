# Fish completion for zjyo
# Place in ~/.config/fish/completions/ or run:
# cp zjyo.fish ~/.config/fish/completions/

function __zjyo_patterns
    if test -f (set -q _Z_DATA; and echo $_Z_DATA; or echo ~/.z)
        cut -d'|' -f1 (set -q _Z_DATA; and echo $_Z_DATA; or echo ~/.z) | xargs -I {} basename {} | sort -u
    end
end

# Complete zjyo command
complete -c zjyo -f
complete -c zjyo -s l -l list -d "List matching directories with frecency scores"
complete -c zjyo -s r -l rank -d "Match by rank (frequency) only"
complete -c zjyo -s t -l time -d "Match by recent access time only"
complete -c zjyo -s c -l current -d "Restrict matches to subdirectories of current directory"
complete -c zjyo -s e -l echo -d "Echo the best match without changing directory"
complete -c zjyo -s x -l remove -d "Remove current directory from database"
complete -c zjyo -l add -d "Add current directory to database"
complete -c zjyo -s h -l help -d "Print help information"
complete -c zjyo -s V -l version -d "Print version information"

# Complete patterns from z database
complete -c zjyo -a "(__zjyo_patterns)" -d "Directory pattern"

# Also complete the z wrapper function if it exists
complete -c z -f
complete -c z -s l -l list -d "List matching directories with frecency scores"
complete -c z -s r -l rank -d "Match by rank (frequency) only"
complete -c z -s t -l time -d "Match by recent access time only"
complete -c z -s c -l current -d "Restrict matches to subdirectories of current directory"
complete -c z -s e -l echo -d "Echo the best match without changing directory"
complete -c z -s x -l remove -d "Remove current directory from database"
complete -c z -l add -d "Add current directory to database"
complete -c z -s h -l help -d "Print help information"
complete -c z -a "(__zjyo_patterns)" -d "Directory pattern"
