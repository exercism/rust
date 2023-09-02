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

- Prefer Rust tests over shell scripts for anything non-trivial.
    `dev/crates/repo-conventions` is a great place for general-purpose tests.
- Bash scripts should set the following options at the top:
    ```bash
    set -eo pipefail
    ```
