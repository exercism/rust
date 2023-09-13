#!/usr/bin/env bash
set -eo pipefail

npx markdownlint-cli \
    docs/*.md \
    concepts/**/*.md \
    exercises/**/*.md
