#!/bin/sh

repo=$(cd "$(dirname "$0")/.." && pwd)

if [ "$TRAVIS_PULL_REQUEST" != "false" ]; then
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
done

if [ -n "$broken" ]; then
  echo "Exercises that don't compile:$broken"
  exit 1
fi
