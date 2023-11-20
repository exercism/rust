# Rust Traits for `.to_string()`

You will also need to implement `.to_string()` for the `Clock` struct.
We will be using this to display the Clock's state.
You can either do it via implementing it directly or using the [Display trait][display-trait].

If so, try implementing the [Display trait][display-trait] for `Clock` instead.

Traits allow for a common way to implement functionality for various types.

For additional learning, consider how you might implement `String::from` for the `Clock` type.
You don't have to actually implement this—it's redundant with `Display`, which is generally the
better choice when the destination type is `String`—but it's useful to have a few type-conversion
traits in your toolkit.

[display-trait]: https://doc.rust-lang.org/std/fmt/trait.Display.html
