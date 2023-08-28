#!/usr/bin/env bash

reset_color=$(echo -e '\033[0m')

red=$(echo -e '\033[0;31m')
green=$(echo -e '\033[0;32m')
yellow=$(echo -e '\033[0;33m')
blue=$(echo -e '\033[0;34m')
cyan=$(echo -e '\033[0;36m')

bold_green=$(echo -e '\033[1;32m')

export red green blue yellow bold_green reset_color cyan
