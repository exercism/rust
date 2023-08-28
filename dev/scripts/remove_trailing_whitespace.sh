#!/usr/bin/env bash

# removes all trailing whitespaces from *.sh and *.py files in this folder
find . -type f \( -name "*.sh" -o -name "*.py" \) -exec sed -i 's/[[:space:]]\+$//' {} \;
