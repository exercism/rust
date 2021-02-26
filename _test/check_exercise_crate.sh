#!/usr/bin/env bash
set -eo pipefail

# A script to ensure that the util/exercise crate builds after it was modified.

EXERCISE_CRATE_PATH="util/exercise"

if [ "$GITHUB_EVENT_NAME" = "pull_request" ]; then
    # Check the changes on the current branch against main branch
    if ! git diff --name-only remotes/origin/main | grep -q "$EXERCISE_CRATE_PATH"; then
        echo "exercise crate was not modified. The script is aborted."
        exit 0
    fi
fi
# If it's not a pull request, just always run it.
# Two scenarios:
# 1. It's being run locally,
#    in which case we assume the person running it really does want to run it.
# 2. It's being run on CI for main,
#    in which case we should check regardless of changes to exercise crate,
#    in case there's a new toolchain release, etc.


TRACK_ROOT="$(git rev-parse --show-toplevel)"

if ! (cd "$TRACK_ROOT/$EXERCISE_CRATE_PATH" && cargo check); then
    echo
    echo "An error has occurred while building the exercise crate."
    echo "Please make it compile."

    exit 1
else
    echo
    echo "exercise crate has been successfully built."

    exit 0
fi
