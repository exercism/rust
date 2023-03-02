#!/usr/bin/env bash

# shellcheck source=/dev/null
source ./bin/generator-utils/colors.sh

function get_exercise_difficulty() {
    local VALID_INPUT=false
    while ! $VALID_INPUT; do
        read -rp "Difficulty of exercise (1-10): " EXERCISE_DIFFICULTY
        if [[ "$EXERCISE_DIFFICULTY" =~ ^[1-9]$|^10$ ]]; then
            VALID_INPUT=true
        else
            printf "Invalid input. Please enter an integer between 1 and 10."
        fi
    done
    echo "$EXERCISE_DIFFICULTY"
}



function get_exercise_name {
    DEFAULT_EXERCISE_NAME=$(echo "$1" | sed 's/-/ /g; s/\b\(.\)/\u\1/g')
    read -rp "Enter exercise name or use default (${YELLOW}${DEFAULT_EXERCISE_NAME}${RESET}): " EXERCISE_NAME
    
    # If the user didn't input anything, set EXERCISE_NAME to a pregenerated default
    if [[ -z "$EXERCISE_NAME" ]]; then
        EXERCISE_NAME="$DEFAULT_EXERCISE_NAME"
    fi
    
    echo "$EXERCISE_NAME"
}


function get_author_name {
    DEFAULT_AUTHOR_NAME="$(git config --get-regexp user.name | sed 's/user.name //g')"
    read -rp "Hey! Are you ${YELLOW}${DEFAULT_AUTHOR_NAME}${RESET}? If not, please enter your github handle: " AUTHOR_NAME
    
    # If the user didn't input anything, set AUTHOR_NAME to a pregenerated default
    if [[ -z "$AUTHOR_NAME" ]]; then
        AUTHOR_NAME="$DEFAULT_AUTHOR_NAME"
    fi
    
    echo "$AUTHOR_NAME"
}
