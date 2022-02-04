#!/bin/sh

status=0
repo="$(cd "$(dirname "$0")/.." && pwd)"

for ex in "$repo"/exercises/*/*/; do
  name=$(grep '^name =' "$ex/Cargo.toml" | cut -d\" -f2)
  if [ -z "$name" ]; then
    echo "don't know name of $ex"
    status=1
    continue
  fi
  cargo clean --manifest-path "$ex/Cargo.toml" --package "$name"
done

exit $status
