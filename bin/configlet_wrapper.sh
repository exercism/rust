#!/usr/bin/env bash
set -eo pipefail

# This wrapper makes sure the problem-specifications repository is checked out
# at the pinned commit and the --offline flag is set for the relevant commands.

cd "$(git rev-parse --show-toplevel)"

[ -f ./bin/configlet ] || ./bin/fetch-configlet

if [ "$#" == 0 ]
then
  ./bin/configlet
  exit
fi

cmd="$1" ; shift

if ! [ "$cmd" == "create" ] && ! [ "$cmd" == "sync" ] && ! [ "$cmd" == "info" ]
then
  # problem-specifications independent commands
  ./bin/configlet "$cmd" "$@"
  exit
fi

./bin/checkout_pinned_problem_specifications_commit.sh

set -x # show the added --offile flag
./bin/configlet "$cmd" --offline "$@"
