# Luhn: Using a Custom Trait

Before doing this exercise you should probably do the original Luhn exercise and its successor, "Luhn: Using the From Trait"

To get the original Luhn exercise, run `exercism fetch rust luhn`

To get the "Luhn: Using the From Trait" exercise, run `exercism fetch rust luhn-from`

In the original Luhn exercise you only validated strings, but the Luhn algorithm can be applied to integers as well.

In "Luhn: Using the From Trait" you implemented a From trait, which also required you to create a Luhn struct.

Instead of creating a Struct just to perform the validation, what if you you validated the primitives (i.e, String, u8, etc.) themselves?

In this exercise you'll create and implement a custom [trait](https://doc.rust-lang.org/book/traits.html) that performs the validation.

Note: It is [not idiomatic Rust to implement traits on on primitives](https://doc.rust-lang.org/book/traits.html#rules-for-implementing-traits). In this exercise we're showing something that you _can_ do, not something you _should_ do. If you find yourself implementing traits on primitives, perhaps you have a case of [Primitive Obsession](http://wiki.c2.com/?PrimitiveObsession).
