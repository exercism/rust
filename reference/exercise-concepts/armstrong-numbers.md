# Concepts of Armstrong Numbers

> An [Armstrong number](https://en.wikipedia.org/wiki/Narcissistic_number) is a number that is the sum of its own digits each raised to the power of the number of digits.

## Example solution

```rust
pub fn is_armstrong_number(num: u32) -> bool {
    let digits = format!("{}", num);
    digits
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(digits.len() as u32))
        .sum::<u32>()
    == num
}
```

## Concepts

- functions: integral to verifying solution, defining logic
- Test Suites: needed to verify solution
- integers: u32 for argument and identity assertions
- `&str`: needed for counting the length of the number since Rust is strongly typed
- `Option<T>`: needed to access actual value of the `char::to_digit`
- type casting: required to apply arithmetic operations
- iterator features:
  - turbofish for enabling single expression solution
  - `map` and `sum` for conveniently reducing expression to what we need to assert
