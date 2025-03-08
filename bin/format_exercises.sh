#!/usr/bin/env bash
set -eo pipefail

# Format existing exercises using rustfmt

repo=$(git rev-parse --show-toplevel)

echo "Formatting exercise files..."

format_one_exercise() {
    exercise_dir="$1"
    source_file_name="$2"

    cd "$exercise_dir"
    config_file="$exercise_dir/.meta/config.json"
    if jq --exit-status '.custom?."allowed-to-not-compile"?' "$config_file" &> /dev/null; then
        exit 0
    fi
    cargo fmt
    file_name=".meta/$source_file_name.rs"
    if [ -f "$file_name" ]; then
        rustfmt --edition 2024 "$file_name"
    fi
}

# traverse either concept or practice exercise
# directory and format Rust files
format_exercises() {
    exercises_path="$repo/exercises/$1"
    source_file_name="$2"
    for exercise_dir in "$exercises_path"/*; do
        format_one_exercise "$exercise_dir" "$source_file_name" &
    done
    wait
}
# https://github.com/exercism/docs/blob/main/anatomy/tracks/concept-exercises.md#file-exemplar-implementation
format_exercises "concept" "exemplar"
# https://github.com/exercism/docs/blob/main/anatomy/tracks/practice-exercises.md#file-example-implementation
format_exercises "practice" "example"
