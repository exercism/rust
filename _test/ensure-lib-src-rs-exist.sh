#!/bin/sh

repo=$(cd "$(dirname "$0")/.." && pwd)

missing=""

for dir in $repo/exercises/*/; do
  exercise=$(basename "$dir")
  if [ ! -f $dir/src/lib.rs ]; then
    # https://github.com/exercism/rust/pull/270
    echo "$exercise is missing a src/lib.rs; please create one (an empty file is acceptable)"
    missing="$missing\n$exercise"
  fi
done

if [ -n "$missing" ]; then
  echo "Exercises missing src/lib.rs:$missing"
  exit 1
fi
