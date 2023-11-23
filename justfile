_default:
    just --list --unsorted

# configlet wrapper, uses problem-specifications submodule
configlet *args="":
    @[ -f bin/configlet ] || bin/fetch-configlet
    ./bin/configlet  {{ args }}

# simulate CI locally (WIP)
test:
    just configlet lint
    ./bin/lint_markdown.sh
    # TODO shellcheck
    ./bin/check_exercises.sh
    CLIPPY=true ./bin/check_exercises.sh
    ./bin/ensure_stubs_compile.sh
    cd rust-tooling && cargo test
    # TODO format exercises

add-exercise *args="":
    cd rust-tooling/generate; cargo run --quiet --release -- add {{ args }}

update-exercise *args="":
    cd rust-tooling/generate; cargo run --quiet --release -- update {{ args }}
