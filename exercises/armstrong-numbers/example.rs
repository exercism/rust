//! Example implementation for armstrong-numbers
//!
//! - Implement the solution to your exercise here.
//! - Put the stubs for any tested functions in `src/lib.rs`,
//!   whose variable names are `_` and
//!   whose contents are `unimplemented!()`.
//! - If your example implementation has dependencies, copy
//!   `Cargo.toml` into `Cargo-example.toml` and then make
//!   any modifications necessary to the latter so your example will run.
//! - Test your example by running `../../bin/test-exercise`

pub fn is_armstrong_number(num: u32) -> bool {
    let s = format!("{}", num);
    let l = s.len();
    s.chars()
        .map(|c| c.to_digit(10).unwrap().pow(l as u32))
        .sum::<u32>() == num
}
