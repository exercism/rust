# Concepts of Leap

I prefer the first example solution because its terseness follows what I've seen of idiomatic Rust. It also avoids the closure concept addition in the canonical solution.

## Extracted concepts

### General

- Boolean logic
  - boolean
- Expressions
  - Rust functions implicitly return expressions
- Arithmetic
  - modulus

### Types

- unsigned integers - `u64`
- booleans - `bool`

## Example exercise 1

["desi-belokonska"](https://exercism.io/tracks/rust/exercises/leap/solutions/01d024321b6a491690321844aedf8b09)'s solution

```rust
pub fn is_leap_year(year: u64) -> bool {
  year % 400 == 0 || year % 4 == 0 && year % 100 != 0
}
```

## Example exercise 2

[Canonical](https://github.com/exercism/rust/blob/master/exercises/leap/example.rs)

```rust
pub fn is_leap_year(year: u64) -> bool {
    let has_factor = |n| year % n == 0;
    has_factor(4) && (!has_factor(100) || has_factor(400))
}
```
