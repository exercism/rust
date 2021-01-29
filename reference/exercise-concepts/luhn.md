# Concepts of Luhn

Luhn is a validation algorithm that combines textual and numeric assertions. Rust solutions favor functional approaches.

## General

### primitives

- `&str` and `char`
  - students must grasp the distinction of individual `char` from `&str` in order to correctly exclude or include parts of the provided input
- integer
  - understanding integers are important to do the required numeric validation
- booleans
  - integral for identity assertions and final comparisons

### iterator features

- chaining
  - it is important to understand iterator chains here to solve the problem in an ordered fashion.
- `map`, `filter`, turbofish possibly `fold`
  - combining map and filter provides a foundational design pattern for validation problems such as this one. `fold` solutions also boast further brevity.
  - the turbofish is needed in some solutions to tell Rust what type you are creating from a chained expression

## Example Solution 1

york's [solution](https://exercism.io/tracks/rust/exercises/luhn/solutions/7038c269a4af4f5b95ca35a99c5c3b4d)

```rust
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .rev()
        .try_fold((0, 0), |(num_count, sum), c| {
            c.to_digit(10).map(|x| {
                let double = x * (num_count % 2 + 1);
                let double = if double > 9 { double - 9 } else { double };
                (num_count + 1, sum + double)
            })
        })
        .map_or(false, |(num_count, sum)| num_count >= 2 && sum % 10 == 0)
}
```

## Example Solution 2

Canonical

```rust
pub fn is_valid(candidate: &str) -> bool {
    if candidate.chars().filter(|c| c.is_digit(10)).take(2).count() <= 1
        || candidate.chars().any(|c| !c.is_digit(10) && c != ' ')
    {
        return false;
    }

    candidate
        .chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(index, digit)| if index % 2 == 0 { digit } else { digit * 2 })
        .map(|digit| if digit > 9 { digit - 9 } else { digit })
        .sum::<u32>()
        % 10
        == 0
}
```
