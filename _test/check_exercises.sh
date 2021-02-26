#!/usr/bin/env bash

# test for existence and executability of the test-exercise script
# this depends on that
if [ ! -f "./bin/test-exercise" ]; then
   echo "bin/test-exercise does not exist"
   exit 1
fi
if [ ! -x "./bin/test-exercise" ]; then
   echo "bin/test-exercise does not have its executable bit set"
   exit 1
fi

# In DENYWARNINGS or CLIPPY mode, do not set -e so that we run all tests.
# This allows us to see all warnings.
# If we are in neither DENYWARNINGS nor CLIPPY mode, do set -e.
if [ -z "$DENYWARNINGS" ] && [ -z "$CLIPPY" ]; then
    set -e
fi

# can't benchmark with a stable compiler; to bench, use
# $ BENCHMARK=1 rustup run nightly _test/check_exercises.sh
if [ -n "$BENCHMARK" ]; then
    target_dir=benches
else
    target_dir=tests
fi

repo=$(cd "$(dirname "$0")/.." && pwd)

if [ "$GITHUB_EVENT_NAME" = "pull_request" ]; then
   files="$(
      git diff --diff-filter=d --name-only remotes/origin/main |
      grep "exercises/" |
      cut -d '/' -f -3 |
      sort -u |
      awk -v repo="$repo" '{print repo"/"$1}'
   )"
else
    files="$repo/exercises/*/*"
fi

return_code=0
# An exercise worth testing is defined here as any top level directory with
# a 'tests' directory
for exercise in $files; do
   exercise="$exercise/$target_dir"

   # This assumes that exercises are only one directory deep
   # and that the primary module is named the same as the directory
   directory=$(dirname "${exercise}")

   release=""
   if [ -z "$BENCHMARK" ] && [ -f "$directory/.meta/test-in-release-mode" ]; then
      release="--release"
   fi

   if [ -n "$DENYWARNINGS" ]; then
      # Output a progress dot, because we may not otherwise produce output in > 10 mins.
      echo -n '.'
      # No-run mode so we see no test output.
      # Quiet mode so we see no compile output
      # (such as "Compiling"/"Downloading").
      # Compiler errors will still be shown though.
      # Both flags are necessary to keep things quiet.
      ./bin/test-exercise "$directory" --quiet --no-run
      return_code=$((return_code | $?))
   else
      # Run the test and get the status
      # We use release mode here because, while it somewhat increases
      # the compile time for all exercises, it substantially improves
      # the runtime for certain exercises such as alphametics.
      # Overall this should be an improvement.
      ./bin/test-exercise "$directory" $release
      return_code=$((return_code | $?))
   fi
done

exit $return_code
