#!/usr/bin/env bash

# shellcheck source=/dev/null
source ./bin/generator-utils/colors.sh

function message() {
    local flag=$1
    local message=$2

    case "$flag" in
    "success")
        printf "${GREEN}%s${RESET}\n" "[success]: $message"
        ;;
    "info")
        printf "${BLUE}%s${RESET}\n" "[info]: $message"
        ;;
    "done")
        echo
        cols=$(tput cols)
        printf "%*s\n" "$cols" "" | tr " " "-"
        echo
        printf "${BOLD_GREEN}%s${RESET}\n" "[done]: $message"
        ;;
    *)
        echo "Invalid flag: $flag"
        ;;
    esac
}

function dash_to_underscore() {
    # shellcheck disable=SC2001
    echo "$1" | sed 's/-/_/g'
}
