_default:
    just --list --unsorted

# configlet (downloads if necessary)
@configlet *args="":
    [ -f ./bin/configlet ] || ./bin/fetch-configlet
    ./bin/configlet {{ args }}

# generate a new uuid straight to your clipboard
uuid:
    just configlet uuid | tr -d '[:space:]' | wl-copy

# simulate CI locally (WIP)
test:
    just configlet lint
    ./bin/lint_markdown.sh
    shellcheck bin/*.sh
    ./bin/check_exercises.sh
    CLIPPY=true ./bin/check_exercises.sh
    cd rust-tooling && cargo test
    ./bin/format_exercises.sh ; git diff --exit-code

symlink-problem-specifications:
    @ [ -L problem-specifications ] || ./bin/symlink_problem_specifications.sh

add-exercise *args="": symlink-problem-specifications
    cd rust-tooling/generate; cargo run --quiet --release -- add {{ args }}

update-exercise *args="": symlink-problem-specifications
    cd rust-tooling/generate; cargo run --quiet --release -- update {{ args }}
