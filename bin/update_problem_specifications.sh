#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

./bin/checkout_pinned_problem_specifications_commit.sh

dir="$(./bin/get_problem_specifications_dir.sh)"

git -C "$dir" checkout --quiet main
git -C "$dir" pull --quiet

new_commit_hash="$(git -C "$dir" rev-parse main)"

sed -i "s/^PINNED_COMMIT_HASH=.*$/PINNED_COMMIT_HASH=\"$new_commit_hash\"/g" ./bin/checkout_pinned_problem_specifications_commit.sh
