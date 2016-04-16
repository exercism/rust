#!/bin/bash

# In DENYWARNINGS mode, do not set -e so that we run all tests.
# This allows us to see all warnings.
if [ -z "$DENYWARNINGS" ]; then
    set -e
fi

tmp=${TMPDIR:-/tmp/}
mkdir "${tmp}exercises"

exitcode=0

# An exercise worth testing is defined here as any top level directory with
# a 'tests' directory
for exercise in exercises/*/tests; do
    # This assumes that exercises are only one directory deep
    # and that the primary module is named the same as the directory
    directory=$(dirname "${exercise}");

    workdir=$(mktemp -d "${tmp}${directory}.XXXXXXXXXX")

    cp -R -T $directory $workdir

    # Run in subshell to change workdir without affecting current workdir.
    (
        cd $workdir
        [ -d src ] || mkdir src
        cp example.rs src/lib.rs

        # Forcibly strip all "ignore" statements from the testing files
        for test in tests/*.rs; do
            sed -i '/\[ignore\]/d' $test
        done

        if [ -n "$DENYWARNINGS" ]; then
            sed -i -e '1i #![deny(warnings)]' src/lib.rs
        fi

        # Run the test and get the status
        cargo test
    )

    status=$?

    if [ $status -ne 0 ]
    then
        exitcode=1
    fi
done

exit $exitcode
