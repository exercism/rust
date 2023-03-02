#!/usr/bin/env bash

# shellcheck disable=SC2034
# Reset
RESET=$(echo -e '\033[0m')

# Regular Colors
BLACK=$(echo -e '\033[0;30m')
RED=$(echo -e '\033[0;31m')
GREEN=$(echo -e '\033[0;32m')
YELLOW=$(echo -e '\033[0;33m')
BLUE=$(echo -e '\033[0;34m')
PURPLE=$(echo -e '\033[0;35m')
CYAN=$(echo -e '\033[0;36m')
WHITE=$(echo -e '\033[0;37m')

# Bold
BOLD_BLACK=$(echo -e '\033[1;30m')
BOLD_RED=$(echo -e '\033[1;31m')
BOLD_GREEN=$(echo -e '\033[1;32m')
BOLD_YELLOW=$(echo -e '\033[1;33m')
BOLD_BLUE=$(echo -e '\033[1;34m')
BOLD_PURPLE=$(echo -e '\033[1;35m')
BOLD_CYAN=$(echo -e '\033[1;36m')
BOLD_WHITE=$(echo -e '\033[1;37m')

# Underline
UNDERLINE=$(echo -e ='\033[4m')

# Background
BG_BLACK=$(echo -e ='\033[40m')
BG_RED=$(echo -e ='\033[41m')
BG_GREEN=$(echo -e ='\033[42m')
BG_YELLOW=$(echo -e ='\033[43m')
BG_BLUE=$(echo -e ='\033[44m')
BG_PURPLE=$(echo -e ='\033[45m')
BG_CYAN=$(echo -e ='\033[46m')
BG_WHITE=$(echo -e ='\033[47m')
