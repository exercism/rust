#!/usr/bin/env bash

repo=$(cd "$(dirname "$0")/.." && pwd)
exitcode=0

for e in $repo/exercises/*/*; do
   if [ -f "$e/.meta/ignore-count-ignores" ]; then
      continue
   fi
   if [ -d "$e/tests" ]; then
      total_tests=0
      total_ignores=0
      for t in $e/tests/*.rs; do
        tests=$(grep  "\#\[test\]" $t | wc -l | tr -d '[:space:]')
        ignores=$(grep "\#\[ignore\]" $t | wc -l | tr -d '[:space:]')

	total_tests=$(expr $total_tests + $tests)
	total_ignores=$(expr $total_ignores + $ignores)
      done
      want_ignores=$(expr $total_tests - 1)
      if [ "$total_ignores" != "$want_ignores" ]; then
        echo "\033[1;31m$e: Has $total_tests tests and $total_ignores ignores (should be $want_ignores)\033[0m"
        exitcode=1
      fi
   fi
done

exit $exitcode
