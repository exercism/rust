#!/usr/bin/env bash

repo=$(cd "$(dirname "$0")/.." && pwd)
exitcode=0

for e in "$repo"/exercises/*/*; do
   # An exercise must have a .meta/config.json
   metaconf="$e/.meta/config.json"
   if [ ! -f "$metaconf" ]; then
      continue
   fi

   if jq --exit-status '.custom?."ignore-count-ignores"?' "$metaconf"; then
      continue
   fi
   if [ -d "$e/tests" ]; then
      total_tests=0
      total_ignores=0
      for t in "$e"/tests/*.rs; do
        tests=$(grep -c "\#\[test\]" "$t" | tr -d '[:space:]')
        ignores=$(grep -c "\#\[ignore\]" "$t" | tr -d '[:space:]')

    total_tests=$((total_tests + tests))
    total_ignores=$((total_ignores + ignores))
      done
      want_ignores=$((total_tests - 1))
      if [ "$total_ignores" != "$want_ignores" ]; then
        # ShellCheck wants us to use printf,
        # but there are no other uses of printf in this repo,
        # so printf hasn't been tested to work yet.
        # (We would not be opposed to using printf and removing this disable;
        # we just haven't tested it to confirm it works yet).
        # shellcheck disable=SC2028
        echo "\033[1;31m$e: Has $total_tests tests and $total_ignores ignores (should be $want_ignores)\033[0m"
        exitcode=1
      fi
   fi
done

exit $exitcode
