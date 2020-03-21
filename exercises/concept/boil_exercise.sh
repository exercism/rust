#!/bin/sh
# This script boilerplates a concept exercise for the Rust track.

if [ -z "$1" ]; then
    echo >&2 "Usage: sh boil_exercise.sh <slug-kebab>"
    exit 1
fi

slug=$1
# Automatically convert to snake_case
snake_case=$(echo "$slug" | sed s/-/_/g)
echo "Boilerplating concept exercise ${slug}"

cargo init --lib "${slug}" --vcs none
cd "${slug}" || exit 1
# The student is the author, so we clear the Cargo.toml
echo "[package]
name = \"${slug}\"
version = \"0.1.0\"
edition = \"2018\"
" > Cargo.toml

mkdir tests
cp ./src/lib.rs "tests/${snake_case}.rs"
mkdir .meta
cp ./src/lib.rs ./.meta/example.rs
touch ./.meta/config.json
touch ./.meta/design.md
mkdir .docs
touch ./.docs/instructions.md
touch ./.docs/introduction.md
touch ./.docs/hints.md
touch ./.docs/after.md