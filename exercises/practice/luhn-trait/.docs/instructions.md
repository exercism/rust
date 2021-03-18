# Luhn: Using a Custom Trait

Before doing this exercise you should probably do the original Luhn exercise and its successor, "Luhn: Using the From Trait"

To get the original Luhn exercise, run

```shell
exercism download --exercise=luhn --track=rust
```

To get the "Luhn: Using the From Trait" exercise, run

```shell
exercism download --exercise=luhn-from --track=rust
```

In the original Luhn exercise you only validated strings, but the Luhn algorithm can be applied to integers as well.

In "Luhn: Using the From Trait" you implemented a From trait, which also required you to create a Luhn struct.

Instead of creating a Struct just to perform the validation, what if you validated the primitives (i.e, String, u8, etc.) themselves?

In this exercise you'll create and implement a custom [trait](https://doc.rust-lang.org/book/2018-edition/ch10-02-traits.html) that performs the validation.
