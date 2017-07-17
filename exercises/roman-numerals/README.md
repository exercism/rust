# Roman Numerals

Write a function to convert from normal numbers to Roman Numerals.

The Romans were a clever bunch. They conquered most of Europe and ruled
it for hundreds of years. They invented concrete and straight roads and
even bikinis. One thing they never discovered though was the number
zero. This made writing and dating extensive histories of their exploits
slightly more challenging, but the system of numbers they came up with
is still in use today. For example the BBC uses Roman numerals to date
their programmes.

The Romans wrote numbers using letters - I, V, X, L, C, D, M. (notice
these letters have lots of straight lines and are hence easy to hack
into stone tablets).

```
 1  => I
10  => X
 7  => VII
```

There is no need to be able to convert numbers larger than about 3000.
(The Romans themselves didn't tend to go any higher)

Wikipedia says: Modern Roman numerals ... are written by expressing each
digit separately starting with the left most digit and skipping any
digit with a value of zero.

To see this in practice, consider the example of 1990.

In Roman numerals 1990 is MCMXC:

1000=M
900=CM
90=XC

2008 is written as MMVIII:

2000=MM
8=VIII

See also: http://www.novaroma.org/via_romana/numbers.html

## Rust Installation

Refer to the [exercism help page][help-page] for Rust installation and learning
resources.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored.  After you get the first test to
pass, remove the ignore flag (`#[ignore]`) from the next test and get the tests
to pass again.  The test file is located in the `tests` directory.   You can
also remove the ignore flag from all the tests to get them to run all at once
if you wish.

Make sure to read the [Crates and Modules](https://doc.rust-lang.org/stable/book/crates-and-modules.html) chapter if you
haven't already, it will help you with organizing your files.

## Feedback, Issues, Pull Requests

The [exercism/rust](https://github.com/exercism/rust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the [rust track team](https://github.com/orgs/exercism/teams/rust) are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: http://exercism.io/languages/rust
[crates-and-modules]: http://doc.rust-lang.org/stable/book/crates-and-modules.html

## Source

The Roman Numeral Kata [http://codingdojo.org/cgi-bin/index.pl?KataRomanNumerals](http://codingdojo.org/cgi-bin/index.pl?KataRomanNumerals)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
