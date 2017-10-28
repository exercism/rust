#!/bin/bash

# In DENYWARNINGS mode, do not set -e so that we run all tests.
# This allows us to see all warnings.
if [ -z "$DENYWARNINGS" ]; then
    set -e
fi

if [ -n "$BENCHMARK" ]; then
    files=exercises/*/benches
else
    files=exercises/*/tests
fi

tmp=${TMPDIR:-/tmp/}
mkdir "${tmp}exercises"

exitcode=0

# An exercise worth testing is defined here as any top level directory with
# a 'tests' directory
for exercise in $files; do
    # This assumes that exercises are only one directory deep
    # and that the primary module is named the same as the directory
    directory=$(dirname "${exercise}");

    workdir=$(mktemp -d "${tmp}${directory}.XXXXXXXXXX")

    cp -R -T $directory $workdir

    # Run in subshell to change workdir without affecting current workdir.
    (
        cd $workdir

        cp example.rs src/lib.rs

        # Overwrite empty Cargo.toml if an example specific file exists
        if [ -f Cargo-example.toml ]; then
            cp Cargo-example.toml Cargo.toml
        fi

        # Forcibly strip all "ignore" statements from the testing files
        for test in tests/*.rs; do
            sed -i '/\[ignore\]/d' $test
        done

        # Run benchmarks instead of tests when enabled.
        if [ -n "$BENCHMARK" ]; then
            cargo bench
        elif [ -n "$DENYWARNINGS" ]; then
            sed -i -e '1i #![deny(warnings)]' src/lib.rs

            # No-run mode so we see no test output.
            # Quiet mode so we see no compile output
            # (such as "Compiling"/"Downloading").
            # Compiler errors will still be shown though.
            # Both flags are necessary to keep things quiet.
            cargo test --quiet --no-run
        else
            # Run the test and get the status
            cargo test
        fi
    )

    status=$?

    if [ $status -ne 0 ]
    then
        exitcode=1
    fi
done

exit $exitcode
