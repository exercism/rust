#!/bin/bash

set -e
tmp=${TMPDIR:-/tmp/}

# An exercise worth testing is defined here as any top level directory with
# a 'tests' directory
for exercise in */tests; do
    
    # This assumes that exercises are only one directory deep
    # and that the primary module is named the same as the directory
    directory=$(dirname "${exercise}");
    
    workdir=$(mktemp -d "${tmp}${directory}.XXXXXXXXXX")

    cp -R -T $directory $workdir
    
    current=$(pwd)

    cd $workdir

    cp example.rs "${directory}.rs"

    # Forcibly strip all "ignore" statements from the testing files
    for test in tests/*.rs; do
        sed -i '/\[ignore\]/d' $test
    done

    # Run the test and get the status    
    cargo test
    status=$?

    cd $current;
    
    rm -rf $workdir
    
    if [ $status -ne 0 ] 
    then
        echo "Failed";
        return 1;
    fi
done
