# Installation

Methods for installing Rust change as the language evolves. To get up-to-date installation instructions head to the [Rust install page](https://www.rust-lang.org/tools/install).

## Additional utilities

### Rustfmt - Writing well-formatted code

When you are solving the exercise, you are free to choose any coding format you want.
However when you are writing a real-world application or a library, your source code will
be read by other people, not just you. To solve a problem when different people choose
different formats for a single project, the developers set a standard coding format
for the said project.

In the Rust world there is a tool, that helps developers to bring standard formatting
to their applications - [rustfmt](https://github.com/rust-lang/rustfmt).

To install `rustfmt` use the following commands:

```bash
rustup self update

rustup component add rustfmt
```

### Clippy - Writing effective code

At its core the process of programming consists of two parts: storing and managing
the resources of your computer. Rust provides a lot of means to accomplish these two
task. Unfortunately sometimes programmers do not use those means very effectively and
create programms that work correctly, but require a lot of resources like memory or time.

To catch the most common ineffective usages of the Rust language,
a tool was created - [clippy](https://github.com/rust-lang/rust-clippy).

To install `clippy` use the following commands:

```bash
rustup self update

rustup component add clippy
```
