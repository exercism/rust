#!/usr/bin/env bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

[ -e "problem-specifications" ] || ln -s "$(./bin/get_problem_specifications_dir.sh)" "problem-specifications"

./bin/checkout_pinned_problem_specifications_commit.sh

for exercise in exercises/practice/*; do
    name="$(basename "$exercise")"
    if [ -d "problem-specifications/exercises/$name" ]; then
        [ -e "$exercise/.prob-spec" ] && rm "$exercise/.prob-spec"
        ln -s "../../../problem-specifications/exercises/$name" "$exercise/.prob-spec"
    fi
done
