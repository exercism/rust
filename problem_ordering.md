# Rust Track Problem Order

The actual source of truth of problem order and topics is [config.json](config.json), but this file documents our reasoning behind the problem order in that file.

## Background

- https://github.com/exercism/xrust/issues/126
- https://github.com/exercism/xrust/issues/127
- http://designisrefactoring.com/2016/07/09/exercism-shouldnt-make-you-cry/

# The Problems, By Section

## Introduction

The first section contains the sort of stuff you expect when learning any programming languages: conditionals, booleans, looping and some higher-order functions.

This section starts at `hello-world` and ends at `grains`.

## Getting Rusty

Problems begin to use more Rust-specific features. Try to only introduce one new language feature at a time. And if several problems rely on a feature, try to group them so as to reinforce its usage.

This section starts at `hamming` and ends at `alphametics`.

## Rust Gets Strange

Exercises that pay the cost of Rust's [strangeness budget](http://words.steveklabnik.com/the-language-strangeness-budget). Features that are very specific to Rust.

This section starts at `anagram` and ends at `robot-name`.

## Putting it all Together

These problems don’t necessarily require additional Rust knowledge, but they do require complex solutions.

This section starts at `ocr-numbers` and extends to the end of the track.
