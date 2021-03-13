# Rust Track Problem Order

The actual source of truth of problem order and topics is [config.json](config.json), but this file documents our reasoning behind the problem order in that file.

## Background

- https://github.com/exercism/rust/issues/126
- https://github.com/exercism/rust/issues/127
- http://designisrefactoring.com/2016/07/09/exercism-shouldnt-make-you-cry/

## Our Approach

We order our problems for students who are learning Rust. With that audience in mind, we follow two rules when adding new problems.

- An exercise should introduce one new concept
- Exercises should progress from Rust basics to advanced Rust concepts

### One New Concept

If a problem solution requires students to use both `Result` and `HashMap`, then at least one of these concepts should be introduced by an earlier problem.

If the track has several problems that rely on a concept, group them together to reinforce learning.

### Progress Through Rust

#### Early Problems

Early problems should focus on Rust syntax and concepts that are present in many programming languages: conditionals, looping, higher-order-functions.

#### Middle Problems

Problems in the middle of the of the track should introduce concepts that are less common. `Result` and `Option` and Traits, for example. Or syntax such as `while let`.

Through the middle section of the track students should gain familiarity with a wide range of Rust concepts.

#### Later Problems

Problems late in the track should do one of two things:

1. Introduce advanced concepts -- Lifetimes, Concurrency, Unsafety
2. Require a complex solution that synthesizes all the knowledge students have gained to date
