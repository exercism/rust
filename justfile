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

add-practice-exercise:
    cd rust-tooling && cargo run --quiet --bin generate_exercise

update-practice-exercise:
    cd rust-tooling && cargo run --quiet --bin generate_exercise update

# TODO remove. resets result of add-practice-exercise.
clean:
    git restore config.json exercises/practice
    git clean -- exercises/practice
