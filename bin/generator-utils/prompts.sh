#!/usr/bin/env bash


# shellcheck source=./colors.sh
source ./bin/generator-utils/colors.sh

get_exercise_difficulty() {

    read -rp "Difficulty of exercise (1-10): " exercise_difficulty
    echo "$exercise_difficulty"
}


validate_difficulty_input() {

    local valid_input=false
    while ! $valid_input; do
        if [[ "$1" =~ ^[1-9]$|^10$ ]]; then
            local exercise_difficulty=$1
            local valid_input=true
        else
            read -rp "${red}Invalid input. ${reset_color}Please enter an integer between 1 and 10. " exercise_difficulty

            [[ "$exercise_difficulty" =~ ^[1-9]$|^10$ ]] && valid_input=true

        fi
    done
    echo "$exercise_difficulty"
}


get_author_handle() {
    local default_author_handle
    default_author_handle="$(git config user.name)"

    if [ -z "$default_author_handle" ]; then
        read -rp "Hey! Couldn't find your Github handle. Add it now or skip with enter and add it later in the .meta.config.json file: " author_handle
    else
        local author_handle="$default_author_handle"

    fi
    echo "$author_handle"
}
