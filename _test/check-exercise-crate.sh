#!/usr/bin/env sh

# A script to ensure that the util/exercise crate builds after it was modified.

EXERCISE_CRATE_PATH="util/exercise"

if [ "$TRAVIS_PULL_REQUEST" != "false" ]; then
    # Check the changes on the current branch against master branch
    git diff --name-only remotes/origin/master | grep "$EXERCISE_CRATE_PATH"
else
    # Check the commits on the master branch made during the week
    # This is because Travis cron is set to test the master branch weekly.
    git diff --name-only "@{7 days ago}" | grep "$EXERCISE_CRATE_PATH"
fi

if [ $? != 0 ]; then
    echo "exercise crate was not modified. The script is aborted."

    exit 0
fi

TRACK_ROOT="$(git rev-parse --show-toplevel)"

if !(cd "$TRACK_ROOT/$EXERCISE_CRATE_PATH" && cargo check); then
	echo "\nAn error has occurred while building the exercise crate.\nPlease make it compile."

	exit 1
else
	echo "\nexercise crate has been successfully built."

	exit 0
fi
