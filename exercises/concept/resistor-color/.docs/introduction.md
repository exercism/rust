# Introduction

Most programs have dependencies on some libraries. Managing those by hand can be a pain. Luckily, the Rust ecosystem comes standard with `cargo`! `cargo` can manage dependencies for a project.

You will often see external packages being referred to as "crates" in Rust. A crate is a compilation unit in Rust. A crate can be compiled into a binary or into a library.

Most of the time, adding an external dependency is as simple as adding a line to your `Cargo.toml` file.

In this exercise, [`enum-iterator`](https://docs.rs/enum-iterator/1.1.1/enum_iterator/) and [`int-enum`](https://docs.rs/int-enum/0.4.0/int_enum/) dependencies were added for you.
