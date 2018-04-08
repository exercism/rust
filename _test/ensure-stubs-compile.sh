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
      continue
    fi

    # In Travis CI, we may have already compiled using the example solution.
    # Touch the src/lib.rs file so that we surely recompile using the stub.
    touch $dir/src/lib.rs

    # Backup tests; this script will modify them.
    cp -r $dir/tests $dir/tests.orig

    # Deny warnings in the tests that may result from compiling the stubs.
    # This helps avoid, for example, an overflowing literal warning
    # that could be caused by a stub with a type that is too small.
    sed -i -e '1i #![deny(warnings)]' $dir/tests/*.rs

    if ! (cd $dir && cargo test --quiet --no-run); then
      echo "$exercise's stub does not compile; please make it compile or remove all non-commented lines"
      broken="$broken\n$exercise"
    fi

    # Restore tests.
    rm -r $dir/tests
    mv $dir/tests.orig $dir/tests
  fi
done

if [ -n "$broken" ]; then
  echo "Exercises that don't compile:$broken"
  exit 1
fi
