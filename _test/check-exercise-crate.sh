#!/usr/bin/env sh

# A script to ensure that the util/exercise crate builds after it was modified.

EXERCISE_CRATE_PATH="util/exercise"

default_branch=$(curl --silent https://api.github.com/repos/exercism/rust  | jq --raw-output '.default_branch')
if [ "$GITHUB_EVENT_NAME" = "pull_request" ]; then
    # Check the changes on the current branch against the default branch
    if ! git diff --name-only remotes/origin/"$default_branch" | grep -q "$EXERCISE_CRATE_PATH"; then
        echo "exercise crate was not modified. The script is aborted."
        exit 0
    fi
fi
# If it's not a pull request, just always run it.
# Two scenarios:
# 1. It's being run locally,
#    in which case we assume the person running it really does want to run it.
# 2. It's being run on CI for the default branch,
#    in which case we should check regardless of changes to exercise crate,
#    in case there's a new toolchain release, etc.


TRACK_ROOT="$(git rev-parse --show-toplevel)"

if !(cd "$TRACK_ROOT/$EXERCISE_CRATE_PATH" && cargo check); then
	echo "\nAn error has occurred while building the exercise crate.\nPlease make it compile."

	exit 1
else
	echo "\nexercise crate has been successfully built."

	exit 0
fi
