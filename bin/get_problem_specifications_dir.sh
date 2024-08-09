#!/usr/bin/env bash
set -euo pipefail

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  prefix="${XDG_CACHE_HOME:-$HOME/.cache}"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  prefix="${XDG_CACHE_HOME:-$HOME/Library/Caches}"
else
  echo "Unsupported OS: $OSTYPE" >&2
  exit 1
fi

echo -n "$prefix/exercism/configlet/problem-specifications"
