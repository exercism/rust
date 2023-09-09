_default:
    just --list --unsorted

test:
    cd rust-tooling && cargo test

# configlet wrapper, uses problem-specifications submodule
configlet *args="":
    @[ -f bin/configlet ] || bin/fetch-configlet
    ./bin/configlet  {{ args }}

add-practice-exercise:
    cd rust-tooling && cargo run --quiet --bin generate_exercise

update-practice-exercise:
    cd rust-tooling && cargo run --quiet --bin generate_exercise update

# TODO remove. resets result of add-practice-exercise.
clean:
    git restore config.json exercises/practice
    git clean -- exercises/practice
