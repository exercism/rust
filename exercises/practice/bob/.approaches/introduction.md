# Introduction

There are various idiomatic approaches to solve Bob.
A basic approach can use a series of `if` statements to test the conditions.
Or a [match][match] on a [tuple][tuple] of the conditions can be used.
An array can contain answers from which the right response is selected by an index calculated from scores given to the conditions.

## General guidance

Regardless of the approach used, some things you could look out for include

- If the input is trimmed, [trim_end][trim-end] only once.

- Use the [ends_with][ends-with] `str` method instead of checking the last character by index for `?`.

- Don't copy/paste the logic for determining a shout and for determing a question into determing a shouted question.
Combine the two determinations instead of copying them.
Not duplicating the code will keep the code [DRY][dry].

- Perhaps consider making `IsQuestion` and `IsShout` values set once instead of functions that are possibly called twice.

- If an `if` statement can return, then an `else if` or `else` is not needed.
Execution will either return or will continue to the next statement anyway.

## Approach: `if` statements

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

For more information, check the [`if` statements approach][approach-if].

## Approach: `match` on a `tuple`

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

For more information, check the [`match` on a `tuple`][approach-match].

## Other approaches

Besides the aforementioned, idiomatic approaches, you could also approach the exercise as follows:

### Other approach: answer array

An array can be defined that contains Bob’s answers, and each condition is given a score.
The correct answer is selected from the array by using the score as the array index.
For more information, check the [Answer array approach][approach-answer-array].

## Which approach to use?

Which to use is pretty much a matter of personal preference,
but the `if` statements and `match` approaches may be considered to be more idiomatic.

- To compare the performance of the approaches, check out the [Performance article][article-performance].

[trim-end]: https://doc.rust-lang.org/std/string/struct.String.html#method.trim_end
[ends-with]: https://doc.rust-lang.org/std/primitive.str.html#method.ends_with
[match]: https://doc.rust-lang.org/rust-by-example/flow_control/match.html
[tuple]: https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
[dry]: https://en.wikipedia.org/wiki/Don%27t_repeat_yourself
[approach-if]: https://exercism.org/tracks/rust/exercises/bob/approaches/if-statements
[approach-match]: https://exercism.org/tracks/rust/exercises/bob/approaches/match-on-tuple
[approach-answer-array]: https://exercism.org/tracks/rust/exercises/bob/approaches/answer-array
[article-performance]: https://exercism.org/tracks/rust/exercises/bob/articles/performance
