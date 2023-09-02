#!/usr/bin/env bash

repo=$(git rev-parse --show-toplevel)

if grep -rnw "$repo/exercises/" --include="*.toml" -e "authors"; then
    echo "Found 'authors' field in exercises";
    exit 1;
fi
