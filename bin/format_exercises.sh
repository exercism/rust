#!/usr/bin/env bash
# Format existing exercises using rustfmt
set -e

repo=$(git rev-parse --show-toplevel)

# traverse either concept or practice exercise
# directory and format Rust files
format_exercises() {
    exercises_path="$repo/exercises/$1"
    source_file_name="$2"
    for exercise_dir in "$exercises_path"/*; do
        (
        cd "$exercise_dir"
        config_file="$exercise_dir/.meta/config.json"
        if jq --exit-status '.custom?."allowed-to-not-compile"?' "$config_file"; then
            exercise=$(basename "$exercise_dir")
            echo "$exercise's stub is allowed to not compile"
            # exit the subshell successfully to continue
            # to the next exercise directory
            exit 0
        fi
        cargo fmt
        file_name=".meta/$source_file_name.rs"
        if [ -f "$file_name" ]; then
            rustfmt "$file_name"
        fi
    )
    done
}
# https://github.com/exercism/docs/blob/main/anatomy/tracks/concept-exercises.md#file-exemplar-implementation
format_exercises "concept" "exemplar"
# https://github.com/exercism/docs/blob/main/anatomy/tracks/practice-exercises.md#file-example-implementation
format_exercises "practice" "example"
