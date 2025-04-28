#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

PINNED_COMMIT_HASH="1799950ecdc8273426de6426a5bed8c360c9f2c7"

dir="$(./bin/get_problem_specifications_dir.sh)"

[ -d "$dir" ] || ./bin/configlet info &> /dev/null # initial population of cache

if ! git -C "$dir" checkout --quiet --detach "$PINNED_COMMIT_HASH" &> /dev/null
then
  # maybe the pinned commit hash was updated and the cache has to be refreshed
  ./bin/configlet info &> /dev/null
  git -C "$dir" checkout --quiet --detach "$PINNED_COMMIT_HASH"
fi
