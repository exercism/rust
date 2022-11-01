# `if` statements

```rust
pub fn reply(msg: &str) -> &str {
    let message = msg.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = message.ends_with('?');
    let is_yelling =
        message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase();

    if is_yelling {
        return if is_questioning {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        };
    }
    if is_questioning {
        return "Sure.";
    }
    "Whatever."
}
```

In this approach you have a series of `if` statements to evaluate the conditions.
As soon as the right condition is found, the correct response is returned.

- First, the [`str`][str] method [`trim_end`][trim-end] is used to remove whitespace from the end of the input.
- If the trimmed input [`is_empty`][is-empty], then the response for nothing said is returned.
- The [`ends_with`][ends-with] method is used to determine if the input ends with a question mark.
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

- If the input is a yell, then the function returns the response for if the input is a yelled question os just a yell.
- If the input is a question, then the function returns the response for that.
- Finally, if the function has not returned by the end, the response for neither a yell nor a question is returned.

```exercism/note
Note that the final line is just `"Whatever."`
This is because the [last expression can be returned](https://doc.rust-lang.org/rust-by-example/fn.html)
from a function without using `return` and a semicolon.
```

[str]: https://doc.rust-lang.org/std/primitive.str.html
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
