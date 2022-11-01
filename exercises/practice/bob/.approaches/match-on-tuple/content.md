# `match` on a `tuple`

```rust
pub fn reply(msg: &str) -> &str {
    let message = msg.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = message.ends_with('?');
    let is_yelling =
        message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase();

    match (is_yelling, is_questioning) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever.",
    }
}
```

In this approach you have a [match][match] expression to evaluate the conditions.

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

The `match` tests a [tuple][tuple] constructed from the values for being a question and being a yell.
Since those values are booleans, each arm of the `match` tests a pattern of boolean values in the tuple positions.
- If both the yell value in the tuple is `true` and the question part of the tuple is `true`,
then the response is returned for a yelled question.

```exercism/note
Note that each arm of the `match` is a single-line expression, so `return` is not needed in the responses returned by the `match` arms.
And, since the `match` is the last expression in the function, `return` is not used before `match`.
The [last expression can be returned](https://doc.rust-lang.org/rust-by-example/fn.html)
from a function without using `return` and a semicolon.
```

- If the tuple is not `(true, true)`, then the next arm of the `match` tests if the yell value in the tuple is `true`.
It uses the [wildcard pattern][wildcard] of `_` to disregard the value for the question part of the tuple.
If the pattern matches, in other words, if the yell value in the tuple is `true`, then the `match` returns the response for a yell.
- If the trimmed input is not a yell, then the next arm of the `match` tests if it is a question.
- If so, then the function returns the response for a question.
- Finally, the wildcard pattern is applied to the whole tuple.
This is similar to `default` used in `switch` statements in other languages.
It returns "Whatever." no matter what the values in the tuple are.

```exercism/note
Note that a `match` in Rust must be exhaustive.
This means it must match all conceivable patterns of the value being tested or the code will not compile.
For a boolean value, you can have one arm to match `true` and another to match `false`,
and the compiler will know that the `match` has checked all conceivable patterns for a boolean.
For a value with many possible patterns, such as a `u32`, you can have each arm match whatever patterns you care about,
such as `1` and `2`, and then have one final arm using the wildcard pattern to match on everything else.
```

[match]: https://doc.rust-lang.org/rust-by-example/flow_control/match.html
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
[tuple]: https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
[wildcard]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern
