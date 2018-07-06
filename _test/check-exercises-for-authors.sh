#!/bin/sh

repo=$(cd "$(dirname "$0")/.." && pwd)

grep -rnw "$repo/exercises/" --include="*.toml" -e "authors"

if [ $? -eq 0 ]; then
	echo "Found 'authors' field in exercises";
	exit 1;
fi
