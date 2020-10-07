#!/bin/bash

# This ensures that config.json and config/maintainers.json are compatible
# with the output of configlet fmt.

# Improve error propagation during CI
set -e -o pipefail

# Check if config.json or maintainers.json were modified
check_pattern="config.json\|config/maintainers.json"

if [ "$TRAVIS_PULL_REQUEST" != "false" ]; then
    # Check the changes on the current branch against master branch
    git diff --name-only master | grep "$check_pattern"
else
    # Check the commits on the master branch made during the week
    # This is because Travis cron is set to test the master branch weekly.
    git diff --name-only "@{7 days ago}" | grep "$check_pattern"
fi

if [ $? != 0 ]; then
    echo "config.json or maintainers.json were not changed - configlet fmt is aborted."

    exit 0
fi

repo=$(cd "$(dirname "$0")/.." && pwd)
configlet="${repo}/bin/configlet"

c="${repo}/config.json"
m="${repo}/config/maintainers.json"

function md5 {
    md5sum $1 | cut -d' ' -f1
}

# before S sum
bcs=$(md5 $c)
bms=$(md5 $m)

cfg=$($configlet fmt .)
if [ $? != 0 ]; then
    echo "configlet fmt returned non-0 exit code with output:"
    echo $cfg
    exit 1
fi

# after S sum
acs=$(md5 $c)
ams=$(md5 $m)

if [ $bcs != $acs -o $bms != $ams ]; then
    echo "configlet fmt:"
    echo $cfg
    echo "Please update the PR to incorporate those changes."
    exit 1
fi
exit 0
