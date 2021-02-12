# Bonus
Test your function on this string: `uüu` and see what happens. Try to write a function that properly
reverses this string. Hint: grapheme clusters

To get the bonus test to run, remove the ignore flag (`#[ignore]`) from the
last test, and execute the tests with:

```bash
$ cargo test --features grapheme
```

You will need to use external libraries (a `crate` in rust lingo) for the bonus task. A good place to look for those is [crates.io](https://crates.io/), the official repository of crates.

[Check the documentation](https://doc.rust-lang.org/cargo/guide/dependencies.html) for instructions on how to use external crates in your projects.
