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
  if [ -n "$CLIPPY" ]; then
    # We don't deny warnings in clippy mode,
    # just because there are many we don't want to deal with right now.
    # So in clippy mode it's just a touch.
    touch "$dir/src/lib.rs"
  else
    # In non-clippy mode, we do want to compile without warnings.
    sed -i -e '1i #![deny(warnings)]' "$dir/src/lib.rs"
  fi

	# Deny warnings in the tests that may result from compiling the stubs.
	# This helps avoid, for example, an overflowing literal warning
	# that could be caused by a stub with a type that is too small.
	sed -i -e '1i #![deny(warnings)]' $dir/tests/*.rs

  if [ -n "$CLIPPY" ]; then
    if ! (cd $dir && cargo clippy --lib --tests --color always 2>clippy.log); then
      cat $dir/clippy.log
      broken="$broken\n$exercise"
    elif grep -q warning $dir/clippy.log; then
      # Warnings will be outputted, but do not fail the build.
      echo "clippy $exercise WARN"
      cat $dir/clippy.log
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
