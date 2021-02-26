#!/usr/bin/env bash

# Require that internal shell script file names use snake_case
set -eo pipefail

# find a list of files whose names do not match our convention
errant_files=$(find bin/ _test/ -iname '*\.sh' -exec basename {} \; | grep '[^a-z_.]' || test 1)

if [ -n "$errant_files" ]; then
    echo "These file names do not follow our snake case convention:"
    # find them again to print the whole relative path
    while IFS= read -r file_name; do
        find . -name "$file_name"
    done <<< "$errant_files"
    echo "Please correct them!"
    exit 1
fi
