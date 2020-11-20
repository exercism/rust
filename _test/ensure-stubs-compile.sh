#!/bin/sh

repo=$(cd "$(dirname "$0")/.." && pwd)

if [ "$GITHUB_EVENT_NAME" = "pull_request" ]; then
	changed_exercises="$(git diff --diff-filter=d --name-only remotes/origin/master | grep "exercises/" | cut -d '/' -f -2 | sort -u | awk -v repo=$repo '{print repo"/"$1}')"
else
	changed_exercises=$repo/exercises/*
fi

if [ -z "$changed_exercises" ]; then
	echo "No exercise was modified. The script is aborted."

	exit 0
fi

broken=""

for dir in $changed_exercises; do
	exercise=$(basename "$dir")

	allowed_file=$dir/.meta/ALLOWED_TO_NOT_COMPILE

	if [ -f $allowed_file ]; then
	  echo "$exercise's stub is allowed to not compile"
	  continue
	fi

	# Backup tests and stub; this script will modify them.
	cp -r $dir/tests $dir/tests.orig
	cp $dir/src/lib.rs $dir/lib.rs.orig

  # In CI, we may have already compiled using the example solution.
  # So we want to touch the src/lib.rs in some way,
  # so that we surely recompile using the stub.
  # Since we also want to deny warnings, that will also serve as the touch.
  sed -i -e '1i #![deny(warnings)]' "$dir/src/lib.rs"

	# Deny warnings in the tests that may result from compiling the stubs.
	# This helps avoid, for example, an overflowing literal warning
	# that could be caused by a stub with a type that is too small.
	sed -i -e '1i #![deny(warnings)]' $dir/tests/*.rs

  if [ -n "$CLIPPY" ]; then
    # We don't find it useful in general to have students implement Default,
    # since we're generally not going to test it.
    if ! (cd $dir && cargo clippy --lib --tests --color always -- --allow clippy::new_without_default 2>clippy.log); then
      cat $dir/clippy.log
      broken="$broken\n$exercise"
    else
      # Just to show progress
      echo "clippy $exercise OK"
    fi
  elif ! (cd $dir && cargo test --quiet --no-run); then
    echo "$exercise's stub does not compile; please make it compile"
    broken="$broken\n$exercise"
  fi

	# Restore tests and stub.
	mv $dir/lib.rs.orig $dir/src/lib.rs
	rm -r $dir/tests
	mv $dir/tests.orig $dir/tests
done

if [ -n "$broken" ]; then
  echo "Exercises that don't compile:$broken"
  exit 1
fi
