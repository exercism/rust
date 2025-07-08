#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

PINNED_COMMIT_HASH="c4043d661357fc2d7ad07b8359adc92d283a5a00"

dir="$(./bin/get_problem_specifications_dir.sh)"

[ -d "$dir" ] || ./bin/configlet info &> /dev/null # initial population of cache

if ! git -C "$dir" checkout --quiet --detach "$PINNED_COMMIT_HASH" &> /dev/null
then
  # maybe the pinned commit hash was updated and the cache has to be refreshed
  ./bin/configlet info &> /dev/null
  git -C "$dir" checkout --quiet --detach "$PINNED_COMMIT_HASH"
fi
