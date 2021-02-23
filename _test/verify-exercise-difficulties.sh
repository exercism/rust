#!/usr/bin/env bash

set -e

repo=$(cd "$(dirname "$0")/.." && pwd)
config=$repo/config.json

es=0

# ensure every exercise has a difficulty
no_difficulty=$(
    jq --raw-output '
        .exercises |
        .concept[], .practice[] |
        select((.status != "deprecated") and (has("difficulty") | not)) |
        .slug
    ' "$config"
)
if [ -n "$no_difficulty" ]; then
    echo "Exercises without a difficulty in config.json:"
    echo "$no_difficulty"
    es=1
fi

# ensure that all difficulties are one of 1, 4, 7, 10
invalid_difficulty=$(
    jq --raw-output '
        .exercises |
        .concept[], .practice[] |
        select(
            (.status != "deprecated") and
            has("difficulty") and
            (
                .difficulty | tostring |
                in({"1":null,"4":null,"7":null,"10":null}) |
                not
            )
        ) |
        "\(.slug) (\(.difficulty))"
    ' "$config"
)
if [ -n "$invalid_difficulty" ]; then
    echo "Exercises with invalid difficulty (must be in {1, 4, 7, 10})"
    echo "$invalid_difficulty"
    es=1
fi

# ensure difficulties are sorted
#exercise_order=$(jq --raw-output '.exercises[] | select(.deprecated | not) | .slug' $config)
#sorted_order=$(jq --raw-output '.exercises | sort_by(.difficulty) | .[] | select(.deprecated | not) | .slug' $config)
#if [ "$exercise_order" != "$sorted_order" ]; then
#    echo "Exercises are not in sorted order in config.json"
#    es=1
#fi

exit $es
