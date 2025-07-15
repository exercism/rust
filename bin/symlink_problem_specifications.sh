#!/usr/bin/env bash
set -eo pipefail

# This script creates a symlink to the problem-specifications cache used
# by configlet. When working on an exercise, it is handy to have the
# problem-specifions files in reach. Symlinking to configlet's cache ensures
# you're always looking at the up-to-date files.

cd "$(git rev-parse --show-toplevel)"

[ -e "problem-specifications" ] || ln -s "$(./bin/get_problem_specifications_dir.sh)" "problem-specifications"

# ensure populated cache
[ -f ./bin/configlet ] || ./bin/fetch-configlet
./bin/configlet info &> /dev/null

for exercise in exercises/practice/*; do
    name="$(basename "$exercise")"
    if [ -d "problem-specifications/exercises/$name" ]; then
        [ -e "$exercise/.prob-spec" ] && rm "$exercise/.prob-spec"
        ln -s "../../../problem-specifications/exercises/$name" "$exercise/.prob-spec"
    fi
done
