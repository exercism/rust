#!/usr/bin/env bash

# shellcheck source=/dev/null
source ./bin/generator-utils/colors.sh

function get_exercise_difficulty() {
    local valid_input=false
    while ! $valid_input; do
        read -rp "Difficulty of exercise (1-10): " exercise_difficulty
        if [[ "$exercise_difficulty" =~ ^[1-9]$|^10$ ]]; then
            valid_input=true
        else
            printf "Invalid input. Please enter an integer between 1 and 10."
        fi
    done
    echo "$exercise_difficulty"
}

function validate_difficulty_input() {

    valid_input=false
    while ! $valid_input; do
        if [[ "$1" =~ ^[1-9]$|^10$ ]]; then
            exercise_difficulty=$1
            valid_input=true
        else
            read -rp "${RED}Invalid input. ${RESET}Please enter an integer between 1 and 10. " exercise_difficulty
            [[ "$exercise_difficulty" =~ ^[1-9]$|^10$ ]] && valid_input=true

        fi
    done
    echo "$exercise_difficulty"
}


function get_author_handle {
    DEFAULT_AUTHOR_HANDLE="$(git config user.name)"

    if [ -z "$DEFAULT_AUTHOR_HANDLE" ]; then
        read -rp "Hey! Couldn't find your Github handle. Add it now or skip with enter and add it later in the .meta.config.json file: " AUTHOR_HANDLE
    else
        AUTHOR_HANDLE="$DEFAULT_AUTHOR_HANDLE"

    fi
    echo "$AUTHOR_HANDLE"

}
