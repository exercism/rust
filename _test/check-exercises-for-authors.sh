#!/usr/bin/env bash

repo=$(cd "$(dirname "$0")/.." && pwd)

if grep -rnw "$repo/exercises/" --include="*.toml" -e "authors"; then
    echo "Found 'authors' field in exercises";
    exit 1;
fi
