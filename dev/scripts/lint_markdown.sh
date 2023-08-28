#!/usr/bin/env bash
set -e
npx markdownlint-cli concepts/**/*.md exercises/**/*.md docs/maintaining.md docs/CONTRIBUTING.md
