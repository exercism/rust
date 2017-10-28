#!/bin/sh

repo=$(cd "$(dirname "$0")/.." && pwd)

broken=""

for dir in $repo/exercises/*/; do
  exercise=$(basename "$dir")

  # If src/lib.rs contains any non-comment line that contains any non-spaces,
  # it probably contains function signatures, and these should compile.
  if grep -v '^//' $dir/src/lib.rs | grep '\S' > /dev/null; then
    allowed_file=$dir/.meta/ALLOWED_TO_NOT_COMPILE
    if [ -f $allowed_file ]; then
      echo "$exercise's stub is allowed to not compile"
    elif ! (cd $dir && cargo test --quiet --no-run); then
      echo "$exercise's stub does not compile; please make it compile or remove all non-commented lines"
      broken="$broken\n$exercise"
    fi
  fi
done

if [ -n "$broken" ]; then
  echo "Exercises that don't compile:$broken"
  exit 1
fi
