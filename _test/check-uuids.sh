#!/bin/sh

repo=$(cd "$(dirname "$0")/.." && pwd)

# Check for invalid UUIDs.
# can be removed once `configlet lint` gains this ability.
# Check issue https://github.com/exercism/configlet/issues/99
bad_uuid=$(jq --raw-output '.exercises | map(.uuid) | .[]' $repo/config.json | grep -vE '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$')
if [ -n "$bad_uuid" ]; then
  echo "invalid UUIDs found! please correct these to be valid UUIDs:"
  echo "$bad_uuid"
  exit 1
fi
