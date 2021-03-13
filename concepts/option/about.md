# About

The `Option` type is an enum provided by the standard library. It is useful for expressing a value which may not exist.
While this may sound like `null` or `nil`, it is quite different and special.

`Option<T>` holds other types and has two variants: `Some(T)` and `None`.

For example, `Option<u32>` holds the `u32` type.

## Usage

### Using the `T` within `Option<T>`

There are several common idioms for using the value which might be present in an `Option`:

- `if let Some(t) = optional { ... }`.

    The `if let` idiom extracts the value from an `Option<T>` and binds it to the variable `t`. This lets you use it directly in that block scope.

- `match optional { Some(t) => {...}, None => {...}}`

    The `match` operator also lets you unpack and handle `None` cases in one statement or expression. It is an alternative to a chain of if and else statements. Using `match` can increase the readability of some sections of code.
- `optional.map(|t| ...)`
   `Option::map` updates an Option in place, potentially with a new type, from the provided function.

   **Note**: operates on the contained value if present. The binding becomes an `Option<U>`, where the given closure is `FnOnce(T) -> U`.
- `optional.unwrap()`.
   Unwrapping an `Option<T>` extracts `T` but will panic if `T` is not there. It is expedient in some cases.

   **Important**: Unwrapping can ease development in the short term or in test code, but it introduces the possibility of runtime panics. It should be avoided when possible.

- `optional.expect("Unexpected None found! Aborting program.)`.
   The `expect` method on `Option<T>` is quite similar to `unwrap()` but it will panic using the provided message if it is `None`.

## Examples

When a value exists in an `Option<u32>`:

```rust
let has_something: Option<u32> = Some(100);
assert_eq!(has_something.unwrap(), 100);
```

When a value *does not* exist in an `Option<u32>`:

```rust
let has_nothing: Option<u32> = None;
assert_eq!(has_nothing, None);
```
