_default:
    just --list --unsorted

test:
    cd rust-tooling && cargo test

# configlet wrapper, uses problem-specifications submodule
configlet *args="":
    @[ -f bin/configlet ] || bin/fetch-configlet
    ./bin/configlet  {{ args }}

# TODO update all if no exercise specified
update exercise:
    @just configlet sync --update --yes --docs --metadata --tests include --exercise {{ exercise }}

add-practice-exercise:
    cd rust-tooling && cargo run --quiet --bin add_practice_exercise

# TODO remove. resets result of add-practice-exercise.
clean:
    git restore config.json
    git clean -- exercises/practice
