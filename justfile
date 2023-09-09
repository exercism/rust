_default:
    just --list --unsorted

test:
    cd rust-tooling && cargo test

add-practice-exercise:
    cd rust-tooling && cargo run --bin add_practice_exercise
