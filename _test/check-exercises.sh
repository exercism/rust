#!/bin/bash
set -e
tmp=${TMPDIR:-/tmp/}
check_assignment () {
    intest="$1"
    testfn=${intest##*/}
    assignment=${testfn%_test.rs}
    workdir=$(mktemp -d "${tmp}${assignment}.XXXXXXXXXX")
    modname=$(awk '/^mod / { sub(";", ""); print $2 }' "${intest}")
    cp "${intest%/*}/example.rs" "${workdir}/${modname}.rs"
    grep -v '^#\[ignore\]' "${intest}" > "${workdir}/${testfn}"
    (
        cd "${workdir}"
        rustc --test -F warnings "${testfn}" && "./${testfn%%.*}"
    )
    status=$?
    rm -rf "${workdir}"
    return $status
}

failures=()

if [ "$#" -eq 1 ]; then
    dirs=$1
elif [ "$#" -eq 0 ]; then
    dirs=*
else
    echo "Usage: check-exercises.sh [dirname]"
    exit 1
fi

for fn in $dirs/*_test.rs; do
    (check_assignment "${fn}")
    if [ $? -ne 0 ]; then
        echo "check failed"
        testfn=${fn##*/}
        assignment=${testfn%_test.rs}
        failures=(${failures[@]} "${testfn%_test.rs}")
    fi
done
if [ "${#failures[*]}" -eq "0" ]; then
    echo "SUCCESS!"
else
    output=$(printf ", %s" "${failures[@]}")
    echo "FAILURES: ${output:2}"
    exit 1
fi
