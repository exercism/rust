# Performance

By using [criterion crate][criterion-crate], we'll find out how to most efficiently determine the response for Bob in Rust.

The [approaches page][approaches] lists two idiomatic approaches to this exercise:

1. [Using `if` statements][approach-if].
2. [Using `match` on a `tuple`][approach-match].

For our performance investigation, we'll also include a third approach that [uses an answer array][approach-answer-array], and a fourth that uses a state machine.

The code used in the benchmark can be found [here][benchmark-application].

## Benchmarks

The strategy used to build the benchmark was to create groups to test different situations for each of the 4 approaches. Thus, for each group, each of the 4 approaches were independently tested. 

The groups were designed to cover all the possible replies from Bob. Also, for each reply (with exception for the `silence` reply), both ASCII and non-ASCII text inputs were used. Given that Bob may have 5 types of replies, we have a total of 9 groups (4 groups with ascii and non-ascii input versions and the `silence` reply).

Possible replies from Bob:
    1. "Sure" to normal question
    2. "Whoa, chill out!" to yell.
    3. "Calm down, I know what I'm doing!" to yell question.
    4. "Fine. Be that way!" to silence.
    5. "Whatever." to anything else.

Also, to make it possible for us to see the difference in performance, for each group, 5 different inputs were used, ranging in number of chars as follows: [1, 10, 100, 1000, 10.000]. For inputs with size greater than or equal to 100, an ascii and non-ascii versions of the [lorem ipsum][lorem-ipsum] text were used.

The groups are prefixed with `reply_for_`:
```rust
//...
criterion_group!(
    benches,
    reply_for_normal_question,
    reply_for_normal_question_non_ascii,
    reply_for_yell,
    reply_for_yell_non_ascii,
    reply_for_yell_question,
    reply_for_yell_question_non_ascii,
    reply_for_silence,
    reply_for_whatever,
    reply_for_whatever_non_ascii
);
```

### Results

The detailed results will be in the `report` folder generated after running the benchmark.

Generally speaking, the results were:
    - **For YELL and YELL QUESTION**: one of the state machine's worst case because it needs to loop through all the string to find the answer. Thus, it **performed worse than the other approaches for lore ipsum with only ascii chars** but, surprisingly, **performed better for lore ipsum with ascii and non-ascii chars**. The explanation is that the other approaches could not take advantage of this [ascii optimization][ascii-optimization]. 
    - **For normal question and whatever**: state machine performed better than the other approaches, but that happened because the lorem ipsum has lower case chars distributed along the string, including the end of the string, which makes it almost O(1) for finding the answer. It is possible to build a worst case situation for the state machine in which all the chars are uppercase except the first, making it necessary to loop through all the chars. In that situation, we would have something close to the YELL and YELL QUESTION.
    - **For the empty answer**: For that specific situation, all the approaches are optimized.


Excluding the state machine approach, the other three approaches are close in performance, but the `if` statements and `match` approaches may be considered to be more idiomatic.

### Reproducing the results

To reproduce the results, run the command `cargo bench` from the [benchmark application][benchmark-application] directory. Be aware that it may take a long time. After finishing the execution, the results can be found on `results/criterion/report/index.html`.

[approaches]: https://exercism.org/tracks/rust/exercises/bob/approaches
[approach-answer-array]: https://exercism.org/tracks/rust/exercises/bob/approaches/answer-array
[approach-if]: https://exercism.org/tracks/rust/exercises/bob/approaches/if-statements
[approach-match]: https://exercism.org/tracks/rust/exercises/bob/approaches/match-on-tuple
[ascii-optimization]: https://github.com/rust-lang/rust/blob/7d65abfe80f9eee93296d1ce08f845c9bf7039f8/library/alloc/src/str.rs#L471
[benchmark-application]: https://github.com/exercism/rust/blob/main/exercises/practice/bob/.articles/performance/code/
[criterion-crate]: https://docs.rs/criterion/latest/criterion/
[lorem-ipsum]: https://en.wikipedia.org/wiki/Lorem_ipsum
