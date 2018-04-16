#!/bin/bash

# This checks every stub which exists, and emits all of their warnings.
# To get a list of exercises for which there are warnings on compilation,
# run it like this:
#
# $ _test/check-stubs.sh 2>/dev/null | rev | cut -d/ -f1 | rev

# run 'cargo check' on every exercise which possesses a stub
repo=$(cd "$(dirname "$0")/.." && pwd)

for stub in $repo/exercises/*/src/lib.rs; do
    exercise=$(dirname $(dirname $stub))
    (
        cd "$exercise"
        # copy the original stub to a backup
        backup=$(dirname $stub)/lib.rs.bak
        cp $stub $backup
        # deny warnings in the stub
        sed -i -e '1i #![deny(warnings)]' $stub
        # quiet so that we only get output when there's an error
        cargo check --quiet
        if [ $? != 0 ]; then
            echo "^- $exercise"
        fi
        # reset
        mv -f $backup $stub
    )
done