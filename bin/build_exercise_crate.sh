#!/bin/bash
# Compile the 'exercise' crate and put it in the 'bin/' folder

TRACK_ROOT="$(git rev-parse --show-toplevel)"

EXERCISE_CRATE_PATH="$TRACK_ROOT/exercise"

BIN_DIR_PATH="$TRACK_ROOT/bin"

echo $TRACK_ROOT

echo $EXERCISE_CRATE_PATH

echo $BIN_DIR_PATH

(
	cd "$EXERCISE_CRATE_PATH"

	echo "Building exercise crate"

	cargo build --release

	echo "Copying exercise crate from $EXERCISE_CRATE_PATH/target/release/exercise into $BIN_DIR_PATH"

	cp "$EXERCISE_CRATE_PATH/target/release/exercise" "$BIN_DIR_PATH"
)
