#!/usr/bin/env bash

# Prohibit internal shell script file names snake_case
set -eo pipefail

# find a list of files whose names do not match our convention
errant_files=$(gfind bin/ _test/ -iname '*\.sh' -exec basename {} \; | ggrep '[^a-z_.].*')

if [ -n "$errant_files" ]; then
    echo "These file names do not follow our snake case convention:"
    # find them again to print the whole relative path
    while IFS= read -r file_name; do
        gfind -iname "$file_name"
    done <<< "$errant_files"
    echo "Please correct them!"
    exit 1
fi
