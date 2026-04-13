#!/usr/bin/env bash

# Synopsis:
# Verify that each exercise's example/exemplar solution passes the tests
# using the track's test runner Docker image.
# You can either verify all exercises or a single exercise.

# Example: verify all exercises in Docker
# bin/verify-exercises-in-docker

# Example: verify single exercise in Docker
# bin/verify-exercises-in-docker two-fer

set -eo pipefail

die() { echo "$*" >&2; exit 1; }

required_tool() {
    command -v "${1}" >/dev/null 2>&1 ||
        die "${1} is required but not installed. Please install it and make sure it's in your PATH."
}

required_tool docker

copy_example_or_examplar_to_solution() {
    jq -c '[.files.solution, .files.exemplar // .files.example] | transpose | map(select(.[0] and .[1]) | {src: .[1], dst: .[0]}) | .[]' .meta/config.json \
    | while read -r src_and_dst; do
        cp "$(jq -r '.src' <<< "${src_and_dst}")" "$(jq -r '.dst' <<< "${src_and_dst}")"
    done
    if test -f Cargo-example.toml ; then
        mv Cargo-example.toml Cargo.toml
    fi
}

pull_docker_image() {
    docker pull "${image}" ||
        die $'Could not find the `'"${image}"$'` Docker image.\nCheck the test runner docs at https://exercism.org/docs/building/tooling/test-runners for more information.'
}

run_tests() {
    local slug
    slug="${1}"

    docker run \
        -u "$(id -u):$(id -g)" \
        --rm \
        --network none \
        --mount type=bind,src="${PWD}",dst=/solution \
        --mount type=bind,src="${PWD}",dst=/output \
        --mount type=tmpfs,dst=/tmp \
        "${image}" "${slug}" /solution /output
    jq -e '.status == "pass"' "${PWD}/results.json" >/dev/null 2>&1
}

verify_exercise() {
    local dir
    local slug
    local tmp_dir
    dir=$(realpath "${1}")
    slug=$(basename "${dir}")
    tmp_dir=$(mktemp -d -t "exercism-verify-${slug}-XXXXX")

    echo "Verifying ${slug} exercise..."

    (
        trap 'rm -rf "$tmp_dir"' EXIT    # remove tempdir when subshell ends
        cp -r "${dir}/." "${tmp_dir}"
        cd "${tmp_dir}"
        # avoid version mismatch between lockfile and test runner
        rm Cargo.lock 2> /dev/null || true

        copy_example_or_examplar_to_solution
        run_tests "${slug}" || { cat "${PWD}/results.json"; exit 1; }
    )
}

verify_exercises() {
    local exercise_slug
    exercise_slug="${1}"

    shopt -s nullglob
    count=0
    for exercise_dir in ./exercises/{concept,practice}/${exercise_slug}/; do
        if [[ -d "${exercise_dir}" ]]; then
            verify_exercise "${exercise_dir}"
            ((++count))
        fi
    done
    ((count > 0)) || die 'no matching exercises found!'
}

image=''
while getopts :i: opt; do
    case $opt in
        i) image=$OPTARG ;;
        ?) echo >&2 "Unknown option: -$OPTARG"; exit 1 ;;
    esac
done
shift "$((OPTIND - 1))"

if [[ -z "${image}" ]]; then
    image="exercism/rust-test-runner"
    pull_docker_image
fi

exercise_slug="${1:-*}"
verify_exercises "${exercise_slug}"
