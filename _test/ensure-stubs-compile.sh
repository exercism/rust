#!/bin/sh

repo=$(cd "$(dirname "$0")/.." && pwd)

current_branch="$(git rev-parse --abbrev-ref HEAD)"

if [ "$current_branch" != "master" ]; then
	changed_exercises="$(git diff --name-only master | grep "exercises/" | cut -d '/' -f -2)"
else
	changed_exercises=exercises/*
fi

if [ -z "$changed_exercises" ]; then
	echo "No exercise was modified. The script is aborted."

	exit 0
fi

broken=""

for dir in $changed_exercises; do
  dir="$repo/$dir"

  exercise=$(basename "$dir")

  # If src/lib.rs contains any non-comment line that contains any non-spaces,
  # it probably contains function signatures, and these should compile.
  if grep -v '^//' $dir/src/lib.rs | grep '\S' > /dev/null; then
    allowed_file=$dir/.meta/ALLOWED_TO_NOT_COMPILE

    if [ -f $allowed_file ]; then
      echo "$exercise's stub is allowed to not compile"
      continue
    fi

    # Backup tests and stub; this script will modify them.
    cp -r $dir/tests $dir/tests.orig
    cp $dir/src/lib.rs $dir/lib.rs.orig

    # This sed serves two purposes:
    # First, in Travis CI, we may have already compiled using the example solution.
    # Edit the src/lib.rs file so that we surely recompile using the stub.
    # Second, ensures that the stub compiles without warnings.
    sed -i -e '1i #![deny(warnings)]' "$dir/src/lib.rs"

    # Deny warnings in the tests that may result from compiling the stubs.
    # This helps avoid, for example, an overflowing literal warning
    # that could be caused by a stub with a type that is too small.
    sed -i -e '1i #![deny(warnings)]' $dir/tests/*.rs

    if ! (cd $dir && cargo test --quiet --no-run); then
      echo "$exercise's stub does not compile; please make it compile or remove all non-commented lines"
      broken="$broken\n$exercise"
    fi

    # Restore tests and stub.
    mv $dir/lib.rs.orig $dir/src/lib.rs
    rm -r $dir/tests
    mv $dir/tests.orig $dir/tests
  fi
done

if [ -n "$broken" ]; then
  echo "Exercises that don't compile:$broken"
  exit 1
fi
