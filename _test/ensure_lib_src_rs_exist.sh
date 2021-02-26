#!/usr/bin/env bash

repo=$(cd "$(dirname "$0")/.." && pwd)

missing=""

empty_stub=""

check_status=0

IGNORED_EXERCISES=(
    "two-fer" #deprecated
    "nucleotide-codons" #deprecated
    "hexadecimal" #deprecated
)

for dir in "$repo"/exercises/*/*/; do
    exercise=$(basename "$dir")

    if [ ! -f "$dir/src/lib.rs" ]; then
        echo "$exercise is missing a src/lib.rs stub file. Please create the missing file with the template, that is necessary for the exercise, present in it."
        missing="$missing\n$exercise"
    else
        #Check if the stub file is empty
        if [ ! -s "$dir/src/lib.rs" ] || [ "$(cat "$dir/src/lib.rs")" == "" ] && [[ " ${IGNORED_EXERCISES[*]} " != *"$exercise"* ]]; then
            echo "$exercise has src/lib.rs stub file, but it is empty."
            empty_stub="$empty_stub\n$exercise"
        fi
    fi
done

if [ -n "$missing" ]; then
    # extra echo to generate a new line
    echo
    echo "Exercises missing src/lib.rs:$missing"

    check_status=1
fi

if [ -n "$empty_stub" ]; then
    echo
    echo "Exercises with empty src/lib.rs stub file:$empty_stub"

    check_status=1
fi

if [ "$check_status" -ne 0 ]; then
    exit 1
else
    exit 0
fi
