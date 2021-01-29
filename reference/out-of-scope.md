# What is Out Of Scope for Exercism's Rust track?

This file is intended to explain what Exercism's Rust track can and cannot
teach within the bounds of the Rust language, community, and ecosystem.

Where material is covered under "Out of scope" in _design.md_ for a
particular exercise it should not be repeated here unless it is felt that the
topic otherwise has insufficient prominence.

### The Limits of Web UI

A student using the web UI is limited to whatever is permitted by the web
interface and the test runner, so the web UI's capabilities effectively serve
as an outer limit for the Rust's track.

A student may:

- Edit a single `.rs` file
- Receive output from `stdout` (e.g. from `dbg!`)

Notably, this means they may not edit Cargo.toml so any exercises that depend
on an external crate must include all dependencies in Cargo.toml already.

### What Exercism Is Not About

Exercism is about acquiring fluency in a programming language, not teaching
more abstract skills like software design or computer science. So any subject
that is not particularly relevant to the Rust programming language is not
within the scope of the Rust track.

## Examples of Excluded Subjects

Some examples of excluded subjects include:

### Cargo

- editing Cargo.toml
- CLI commands e.g. `new`, `update`, `bench`

### Frameworks

- Amethyst
- Yew, Iced, Sauron, etc.

### Interop

- CFFI
- `asm!`

### Generally:

- File handling
- Networking
