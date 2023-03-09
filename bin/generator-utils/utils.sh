#!/usr/bin/env bash

# shellcheck source=./colors.sh
source ./bin/generator-utils/colors.sh

function message() {
    local flag=$1
    local message=$2

    case "$flag" in
    "success") printf "${green}%s${reset_color}\n" "[success]: $message" ;;
    "task") printf "${cyan}%s${reset_color}\n" "[task]: $message" ;;
    "info") printf "${blue}%s${reset_color}\n" "[info]: $message" ;;
    "warning") printf "${yellow}%s${reset_color}\n" "[warning]: $message" ;;
    "error") printf "${red}%s${reset_color}\n" "[error]: $message" ;;
    "done")
        echo
        # Create a dashed-line as wide as the screen
        cols=$(tput cols)
        local cols
        printf "%*s\n" "$cols" "" | tr " " "-"
        echo
        printf "${bold_green}%s${reset_color}\n" "[done]: $message"
        ;;
    *)
        echo "Invalid flag: $flag"
        ;;
    esac
}

dash_to_underscore() {
    echo "$1" | sed 's/-/_/g'
}

# exercise_name -> Exercise Name
format_exercise_name() {
    echo "$1" | sed 's/-/ /g; s/\b\(.\)/\u\1/g'
}

check_exercise_existence() {
    message "info" "Looking for exercise.."
    local slug="$1"
    # Check if exercise is already in config.json
    if jq '.exercises.practice | map(.slug)' config.json | grep -q "$slug"; then
        echo "${1} has already been implemented."
        exit 1
    fi

    # fetch configlet and crop out exercise list
    local unimplemented_exercises
    unimplemented_exercises=$(bin/configlet info | sed -n '/With canonical data:/,/Track summary:/p' | sed -e '/\(With\(out\)\? canonical data:\|Track summary:\)/d' -e '/^$/d')
    if echo "$unimplemented_exercises" | grep -q "^$slug$"; then
        message "success" "Exercise has been found!"
    else
        message "error" "Exercise doesn't exist!"
        message "info" "These are the unimplemented practice exercises:
${unimplemented_exercises}"

        # Find closest match to typed-in not-found slug
        # see util/ngram for source
        # First it builds a binary for the system of the contributor
        if [ -e bin/generator-utils/ngram ]; then
            echo "${yellow}$(bin/generator-utils/ngram "${unimplemented_exercises}" "$slug")${reset_color}"
        else
            message "info" "Building typo-checker binary for $(uname -m) system."

            cd util/ngram && ./build && cd ../.. && echo "${yellow}$(bin/generator-utils/ngram "${unimplemented_exercises}" "$slug")${reset_color}"
        fi

        exit 1
    fi
}
