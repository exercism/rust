_default:
    just --list --unsorted

# configlet wrapper, uses pinned problem-specifications commit
@configlet *args="":
    ./bin/configlet_wrapper.sh {{ args }}

# update the pinned commit hash
update-problem-specs:
    ./bin/update_problem_specifications.sh

# generate a new uuid straight to your clipboard
uuid:
    just configlet uuid | tr -d '[:space:]' | wl-copy

# simulate CI locally (WIP)
test:
    just configlet lint
    ./bin/lint_markdown.sh
    # TODO shellcheck
    ./bin/check_exercises.sh
    CLIPPY=true ./bin/check_exercises.sh
    cd rust-tooling && cargo test
    ./bin/format_exercises.sh ; git diff --exit-code

add-exercise *args="":
    cd rust-tooling/generate; cargo run --quiet --release -- add {{ args }}

update-exercise *args="":
    cd rust-tooling/generate; cargo run --quiet --release -- update {{ args }}
