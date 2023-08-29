# Contributing to the Rust Exercism Track

Issues and pull requests are currently being auto-closed.
Please make a post on [the Exercism forum] to propose changes.
Contributions are very welcome if they are agreed upon on the forum.

[the Exercism forum]: https://forum.exercism.org/

As one of the many tracks in Exercism,
contributions should observe Exercism standards like the [Code of Conduct].
This document introduces the ways you can help
and what maintainers expect of contributors.

[Code of Conduct]: https://exercism.org/code-of-conduct

## General policies

- Pull requests should adhere to [Exercism's PR guidelines].

[Exercism's PR guidelines]: https://exercism.org/docs/community/being-a-good-community-member/pull-requests

## Internal Tooling Style Guide

- prefer using tools that do one thing well
- prefer using tools that are ubiquitous:
  `jq` or `sed` instead of `prettier` or `sd`
- write scripts to do one thing well
- prefer GNU versions of `sed` and other utilities
- Prefer Bash for scripting
- Strive for compatibility.
  macOS still distributes Bash v3.x by default, despite v5.x being current;
  this means that the scripts can't depend on certain features like map.
- Scripts should use `#!/usr/bin/env bash` as their shebang
  This increases portability on NixOS and macOS,
  because contributors' preferred bash may not be installed in `/bin/bash`.
- Prefer snake case for script file names
- Script file names should include the `.sh` extension
- Set the executable bit on scripts that should be called directly.
- Scripts should set the following options at the top
    ```bash
    set -eo pipefail
    ```
