# Answer array

```rust
const ANSWERS: &'static [&'static str] = &[
    "Whatever.",
    "Sure.",
    "Whoa, chill out!",
    "Calm down, I know what I'm doing!",
];

pub fn reply(msg: &str) -> &str {
    let message = msg.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = if message.ends_with('?') { 1 } else { 0 };
    let is_yelling =
        if message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase() {
            2
        } else {
            0
        };

    ANSWERS[is_questioning + is_yelling]
}
```

In this approach you define an array that contains Bob’s answers, and each condition is given a score.
The correct answer is selected from the array by using the score as the array index.

The array is defined using the [`static` lifetime annotation][static-lifetime] for a reference to an array of [`str`][str] [references][reference].
`static` means that the value will live for the life of the program.
The `static` annotation can be removed, as described in the [Shortening section](#shortening).

The reference symbols `&` are needed because the compiler must know the size of each binding.
The compiler doesn't know the size of each `str`, but it does know the size of a reference to a `str`,
which is the size of a pointer for that platform.
For a 64-bit system, the size of a pointer is 64 bits.
So, each `str` is defined as a `str` reference (`&str`).
The array itself is defined as a reference, since the compiler does not know the size of the whole array and its contents.

- The [`str`][str] method [`trim_end`][trim-end] is used to remove whitespace from the end of the input.
- If the trimmed input [`is_empty`][is-empty], then the response for nothing said is returned.
- The [`ends_with`][ends-with] method is used to determine if the input ends with a question mark.
If so, `is_questioning` is given the value `1`, otherwise it is given the value `0`.

- The first half of the yell condition

```rust
message.chars().any(|ch| ch.is_alphabetic())
```

calls the `str` method [`chars`][chars] to create an [`Iterator`][iterator] for the characters in the trimmed input.
- The iterated characters are passed into the `Iterator` method [`any`][any].
- `any` passes each character to a [closure][closure] using the [`char`][char] method [`is_alphabetic`][is-alphabetic] to ensure there is at least one letter character in the `str`.
This is because the second half of the condition tests that the uppercased input is the same as the input.
If the input were only `"123"` it would equal itself uppercased, but without letters it would not be a yell.
The uppercasing is done by using the `str` method [`to_uppercase`][to-uppercase].
If the input is a yell, then `is_yelling` is given the value `2`, otherwise it is given the value `0`.

The final expression returns the value in the `ANSWERS` array at the index of the combined values for `is_questioning` and `is_yelling`.

```exercism/note
Note that the final line is just `ANSWERS[is_questioning + is_yelling]`
This is because the [last expression can be returned](https://doc.rust-lang.org/rust-by-example/fn.html)
from a function without using `return` and a semicolon.
```

| is_yelling | is_questioning | Index     | Answer                                |
| ---------- | -------------- | --------- | ------------------------------------- |
| `false`    | `false`        | 0 + 0 = 0 | `"Whatever."`                         |
| `false`    | `true`         | 0 + 1 = 1 | `"Sure."`                             |
| `true`     | `false`        | 2 + 0 = 2 | `"Whoa, chill out!"`                  |
| `true`     | `true`         | 2 + 1 = 3 | `"Calm down, I know what I'm doing!"` |

## Shortening

For defining the array the `static` lifetime specifiers can be dropped, like so

```rust
const ANSWERS: &[&str] = &[
    "Whatever.",
    "Sure.",
    "Whoa, chill out!",
    "Calm down, I know what I'm doing!",
];
```

A lifetime specifier doesn't change the actual [lifetime][lifetime] of a binding.
It just makes the lifetime explicit.
When the compiler can figure out the lifetime, then the annotation can be dropped, which is called [lifetime elision][lifetime-elision].

[static-lifetime]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime
[str]: https://doc.rust-lang.org/std/primitive.str.html
[reference]: https://doc.rust-lang.org/std/primitive.reference.html
[lifetime]: https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
[lifetime-elision]:https://doc.rust-lang.org/reference/lifetime-elision.html
[trim-end]: https://doc.rust-lang.org/std/primitive.str.html#method.trim_end
[is-empty]: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
[ends-with]: https://doc.rust-lang.org/std/primitive.str.html#method.ends_with
[chars]: https://doc.rust-lang.org/std/primitive.str.html#method.chars
[closure]: https://doc.rust-lang.org/rust-by-example/fn/closures.html
[iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[any]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
[char]: https://doc.rust-lang.org/std/primitive.char.html
[is-alphabetic]: https://doc.rust-lang.org/std/primitive.char.html#method.is_alphabetic
[to-uppercase]: https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase
