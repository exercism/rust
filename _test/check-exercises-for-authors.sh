#!/usr/bin/env bash

repo=$(cd "$(dirname "$0")/.." && pwd)

grep -rnw "$repo/exercises/" --include="*.toml" -e "authors"
# shellcheck disable=SC2181
if [ $? -eq 0 ]; then
	echo "Found 'authors' field in exercises";
	exit 1;
fi
