# Maintaining Notes

This document captures informal policies, tips, and topics useful to maintaining the Rust track.

## Internal Tooling

We have a number of scripts for CI tests.
They live in `bin/` and `_test/`.

## Internal Tooling Style Guide

This is non-exhaustive.

- Adopt a Unix philosophy for tooling
  - prefer using tools that do one thing well
  - prefer using tools that are ubiquitous: `jq` or `sed` instead of `prettier` or `sd`
  - write scripts to do one thing well
  - prefer GNU versions of `sed` and other utilities
- Prefer Bash for scripting
  - Strive for compatibility. macOS still distributes Bash v3.x by default, despite v5.x being current; this means that the scripts can't depend on certain features like map.
- Scripts should use `#!/usr/bin/env bash` as their shebang
  - This increases portability on NixOS and macOS because contributors' preferred bash may not be installed in `/bin/bash`.
- Prefer snake case for script file names

    ```sh
    hello_world.sh
    ```

   - This simplifies development when upgrading a script into a proper language. *Rusty tooling anyone?*
- Script file names should include the `.sh` extension
- Set the executable bit on scripts that should be called directly.
- Scripts should set the following options at the top

    ```bash
    set -eo pipefail
    ```

## Running CI Locally

You can run CI tools locally.
Scripts expect GNU versions of tooling, so you may see unexpected results on macOS.
[Here](https://github.com/exercism/rust/issues/1138) is one example.
Windows users can also run tooling locally using [WSL](https://docs.microsoft.com/en-us/windows/wsl/).
We recommend WSL 2 with the distribution of your choice.

## Maintainer Tips and Tricks

Exercism tracks follow a specification that has evolved over time.
Maintainers often need to make ad-hoc migrations to files in this repository.
If you find yourself scripting such ad-hoc changes, include the source of your script using markdown codeblocks in a commit message.

See [this commit](https://github.com/exercism/rust/commit/45eb8cc113a733636212394dee946ceff5949cc3) for an example.
