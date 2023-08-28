#!/usr/bin/env bash

# Report any .toml files that have trailing white space
# so user can fix them
set -eo pipefail

files=$(find . \( -iname '*.toml' -o -iname '*.sh' -o -iname '*.rs' \) -exec grep -l '[[:space:]]$' {} \;)

if [ -n "$files" ]; then
    echo "These files have trailing whitespace:"
    echo "$files"
    echo "Our conventions disallow this so please remove the trailing whitespace."
    exit 1
fi
