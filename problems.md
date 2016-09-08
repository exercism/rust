# Rust Track Problem Order

The actual source of truth of problem order is [config.json](config.json), but this file documents our reasoning behind the problem order in that file.

## Background

- https://github.com/exercism/xrust/issues/126
- https://github.com/exercism/xrust/issues/127
- http://designisrefactoring.com/2016/07/09/exercism-shouldnt-make-you-cry/

# The Problems, By Section

## Introduction

The first section contains the sort of stuff you expect when learning any programming languages: conditionals, booleans, looping and some higher-order functions.

problem | topics
----- | -----
hello-world |  Some/None. println!
gigasecond |  crates, math
leap |  math, booleans, conditionals
raindrops |  case (or `format`). Mutable string
bob |  chars, string functions
beer-song |  case, string concatenation, vector (optional), loop
difference-of-squares |  fold & map

## Getting Rusty

Problems begin to use more Rust-specific features. Try to only introduce one new language feature at a time. And if several problems rely on a feature, try to group them so as to reinforce its usage.

problem | topics
----- | -----
hamming |  Result
scrabble-score |  chaining higher-order functions, HashMap (optional)
pangram | filter, ascii (optional)
nucleotide-count |  filter, entry api, mutablity, match
word-count |  hashmap, str vs string, chars, entry api
etl |  btree
sieve |  vector, map, while let (optional)
rna-transcription |  match, struct, str vs string
roman-numerals |  mutable, results, loops, struct, traits
all-your-base |  Result, enumerate, fold, map
grade-school |  struct, entry api, Vec, Option
tournament |  enum, sorting, hashmap, structs
robot-simulator | Immutability, enum
bracket-push | From trait, stack or recursion
queen-attack |  struct, trait (optional), Result
sublist |  enum, generic over type
space-age | Custom Trait, From Trait, Default Trait implementation
allergies |  struct, enum, bitwise (probably), vectors, filter
variable-length-quantity | Encodings, slices, bitwise, Result
phone-number |  option, format, unwrap_or, iters, match
wordy | Result, string parsing, operators (optional)
custom-set |  generic over type, vector, equality, struct

## Rust Gets Strange

Exercises that pay the cost of Rust's [strangeness budget](http://words.steveklabnik.com/the-language-strangeness-budget). Features that are very specific to Rust.

problem | topics
----- | -----
anagram |  lifetimes, str vs string, loops, iter, vector
nucleotide-codons |  struct, hash map, lifetimes, Result
robot-name |  struct, slices, randomness, lifetimes, self mut

## Putting it all Together

These problems don’t necessarily require additional Rust knowledge, but they do require complex solutions.

problem | topics
----- | -----
ocr-numbers | Lines, Chunks, slices
minesweeper |  Board state
dominoes |  Graph theory, searching
parallel-letter-frequency | multi-threading
rectangles |  Enum, structs, traits, Lifetimes
forth |  Parser reimplementation
circular-buffer |  Buffer reimplementation, Generics
react |  Lifetimes, generics, closures
