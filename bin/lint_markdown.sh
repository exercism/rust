#!/usr/bin/env bash
set -eo pipefail

npx markdownlint-cli \
    ./*.md \
    docs/*.md \
    concepts/**/*.md \
    exercises/**/*.md
