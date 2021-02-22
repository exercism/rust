# Maintaining Notes

This document captures informal policies, tips, and topics useful to maintaining the Rust track.

## Internal Tooling

We have a number of scripts for CI tests.
They live in `bin/` and `_test/`.

## Internal Tooling Style Guide

This is non-exhaustive.

- Prefer snakecase for file names
    ```sh
    hello_world.sh
    ```
- Prefer BASH for scripting
- Adopt a Unix philosophy for tooling
  - prefer using tools that do one thing well
  - prefer using tools that are ubiquitous: `jq` or `sed` instead of `prettier` or `sd`
  - write scripts to do one thing well
  - prefer GNU versions of `sed` and other utilities
- Scripts should use `#!/usr/bin/env bash` as their shebang
  - This increases portability on NixOS and macOS because contributors' preferred bash may not be installed in `/bin/bash`.


## Running CI Locally
You can run CI tools locally. 
Scripts expect GNU versions of tooling, so you may see unexpected results on macOS.
[Here](https://github.com/exercism/rust/issues/1138) is one example.
