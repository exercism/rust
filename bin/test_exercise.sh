#!/usr/bin/env bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

if [ $# -eq 0 ]; then
    echo "Usage: <exercise-slug> [cargo args]"
    exit 1
fi

slug="$1"
cargo_args="$2"

# determine the exercise path from the slug
for p in exercises/{practice,concept}/* ; do
    current_slug="$(basename "$p")"
    p="$(dirname "$p")"
    if [[ "$current_slug" = "$slug" ]]; then
        exercise_path="$p/$slug"
        break
    fi
done
if [ -z "$exercise_path" ]; then
    echo "Could not find exercise path for $slug"
    exit 1
fi

# what cargo command will we use?
if [ -n "$BENCHMARK" ]; then
    command="+nightly bench"
    if [ ! -e "$exercise_path/benches" ]; then
        # no benches, nothing to do
        exit
    fi
else
    command="test"
fi

# setup temporary directory for the exercise
tmp_dir=$(mktemp -d)
trap 'rm -rf $tmp_dir' EXIT INT TERM

# copy the exercise to the temporary directory
contents_to_copy=(
    "src"
    "tests"
    "benches"
    "Cargo.toml"
)
for c in "${contents_to_copy[@]}"; do
    if [ ! -e "$exercise_path/$c" ]; then
        continue
    fi
    cp -r "$exercise_path/$c" "$tmp_dir"
done

# Move example files to where Cargo expects them
if [ -f "$exercise_path/.meta/example.rs" ]; then
    cp -f "$exercise_path/.meta/example.rs" "$tmp_dir/src/lib.rs"
elif [ -f "$exercise_path/.meta/exemplar.rs" ]; then
    cp -f "$exercise_path/.meta/exemplar.rs" "$tmp_dir/src/lib.rs"
else
    echo "Could not locate example implementation for $exercise_path"
    exit 1
fi
if [ -f "$exercise_path/.meta/Cargo-example.toml" ]; then
    cp -f "$exercise_path/.meta/Cargo-example.toml" "$tmp_dir/Cargo.toml"
fi

if [ -n "$DENYWARNINGS" ]; then
    export RUSTFLAGS="$RUSTFLAGS -D warnings"
fi

# run tests from within temporary directory
cd "$tmp_dir"
if [ -n "$CLIPPY" ]; then
    export RUSTFLAGS="$RUSTFLAGS -D warnings"
    # shellcheck disable=SC2086
    cargo clippy --tests $cargo_args
else
    # shellcheck disable=SC2086
    cargo $command $cargo_args -- --include-ignored
fi
