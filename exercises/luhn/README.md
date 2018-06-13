# Luhn

Given a number determine whether or not it is valid per the Luhn formula.

The [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) is
a simple checksum formula used to validate a variety of identification
numbers, such as credit card numbers and Canadian Social Insurance
Numbers.

The task is to check if a given string is valid.

Validating a Number
------

Strings of length 1 or less are not valid. Spaces are allowed in the input,
but they should be stripped before checking. All other non-digit characters
are disallowed.

## Example 1: valid credit card number

```text
4539 1488 0343 6467
```

The first step of the Luhn algorithm is to double every second digit,
starting from the right. We will be doubling

```text
4_3_ 1_8_ 0_4_ 6_6_
```

If doubling the number results in a number greater than 9 then subtract 9
from the product. The results of our doubling:

```text
8569 2478 0383 3437
```

Then sum all of the digits:

```text
8+5+6+9+2+4+7+8+0+3+8+3+3+4+3+7 = 80
```

If the sum is evenly divisible by 10, then the number is valid. This number is valid!

## Example 2: invalid credit card number

```text
8273 1232 7352 0569
```

Double the second digits, starting from the right

```text
7253 2262 5312 0539
```

Sum the digits

```text
7+2+5+3+2+2+6+2+5+3+1+2+0+5+3+9 = 57
```

57 is not evenly divisible by 10, so this number is not valid.

## Rust Installation

Refer to the [exercism help page][help-page] for Rust installation and learning
resources.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored. If you wish to run every possible test use:

```bash
$ cargo test -- --ignored
```

To run specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

To find test names, view the tests source file wich is located in `tests` directory.
Every function that contains `#[test]` flag above it's name is a separate test.
If the specfic test is ignored use:

```bash
$ cargo test some_test -- --ignored
```

Alternatively you can edit the tests source file and remove the `#[ignore]` flag
from the tests you wish to run and use the first `cargo test` command to run them.

Make sure to read the [Modules](https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html) chapter if you
haven't already, it will help you with organizing your files.

## Feedback, Issues, Pull Requests

The [exercism/rust](https://github.com/exercism/rust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the rust track team are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: http://exercism.io/languages/rust
[modules]: https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/second-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/second-edition/ch11-02-running-tests.html

## Source

The Luhn Algorithm on Wikipedia [http://en.wikipedia.org/wiki/Luhn_algorithm](http://en.wikipedia.org/wiki/Luhn_algorithm)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
