#!/usr/bin/env bash
set -eo pipefail

repo=$(git rev-parse --show-toplevel)

if [ "$GITHUB_EVENT_NAME" = "pull_request" ]; then
    git fetch --depth=1 origin main
    files="$(
        git diff --diff-filter=d --name-only remotes/origin/main |
        grep "exercises/" |
        cut --delimiter '/' --fields -3 |
        sort --unique || true
    )"
else
    files="$repo/exercises/*/*"
fi

# An exercise worth testing is defined here as any top level directory with
# a 'tests' directory and a .meta/config.json.
for exercise in $files; do
    slug=$(basename "$exercise")

    # An exercise must have a .meta/config.json
    metaconf="$exercise/.meta/config.json"
    if [ ! -f "$metaconf" ]; then
        continue
    fi

    # suppress compilation output
    cargo_args="--quiet"

    if [ -z "$BENCHMARK" ] && jq --exit-status '.custom?."test-in-release-mode"?' "$metaconf" > /dev/null; then
        # release mode is enabled if not benchmarking and the appropriate key is neither `false` nor `null`.
        cargo_args="$cargo_args --release"
    fi

    if [ -n "$DENYWARNINGS" ] && [ -z "$CLIPPY" ]; then
        # No-run mode so we see no test output.
        # clippy does not understand this flag.
        cargo_args="$cargo_args --no-run"
    fi

    echo "Checking $slug..."
    ./bin/test_exercise.sh "$slug" "$cargo_args"
done
