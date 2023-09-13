#!/usr/bin/env bash
set -eo pipefail

# can't benchmark with a stable compiler; to bench, use
# $ BENCHMARK=1 rustup run nightly bin/check_exercises.sh
if [ -n "$BENCHMARK" ]; then
    target_dir=benches
else
    target_dir=tests
fi

repo=$(git rev-parse --show-toplevel)

if [ "$GITHUB_EVENT_NAME" = "pull_request" ]; then
    git fetch --depth=1 origin main
    files="$(
        git diff --diff-filter=d --name-only remotes/origin/main |
        grep "exercises/" || true |
        cut -d '/' -f -3 |
        sort -u |
        awk -v repo="$repo" '{print repo"/"$1}'
    )"
else
    files="$repo/exercises/*/*"
fi

return_code=0
# An exercise worth testing is defined here as any top level directory with
# a 'tests' directory and a .meta/config.json.
for exercise in $files; do
    exercise="$exercise/$target_dir"

    # This assumes that exercises are only one directory deep
    # and that the primary module is named the same as the directory
    directory=$(dirname "${exercise}")

    # An exercise must have a .meta/config.json
    metaconf="$directory/.meta/config.json"
    if [ ! -f "$metaconf" ]; then
        continue
    fi

    release=""
    if [ -z "$BENCHMARK" ] && jq --exit-status '.custom?."test-in-release-mode"?' "$metaconf"; then
        # release mode is enabled if not benchmarking and the appropriate key is neither `false` nor `null`.
        release="--release"
    fi

    if [ -n "$DENYWARNINGS" ]; then
        # Output a progress dot, because we may not otherwise produce output in > 10 mins.
        echo -n '.'
        # No-run mode so we see no test output.
        # Quiet mode so we see no compile output
        # (such as "Compiling"/"Downloading").
        # Compiler errors will still be shown though.
        # Both flags are necessary to keep things quiet.
        ./bin/test_exercise.sh "$directory" --quiet --no-run
        return_code=$((return_code | $?))
    else
        # Run the test and get the status
        ./bin/test_exercise.sh "$directory" $release
        return_code=$((return_code | $?))
    fi
done

exit $return_code
