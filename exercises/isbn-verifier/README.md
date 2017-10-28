# Isbn Verifier

Check if a given ISBN-10 is valid.

## Functionality

Given an unknown string the program should check if the provided string is a valid ISBN-10.
Putting this into place requires some thinking about preprocessing/parsing of the string prior to calculating the check digit for the ISBN.

The program should allow for ISBN-10 without the separating dashes to be verified as well.

## ISBN

Let's take a random ISBN-10 number, say `3-598-21508-8` for this.
The first digit block indicates the group where the ISBN belongs. Groups can consist of shared languages, geographic regions or countries. The leading '3' signals this ISBN is from a german speaking country.
The following number block is to identify the publisher. Since this is a three digit publisher number there is a 5 digit title number for this book.
The last digit in the ISBN is the check digit which is used to detect read errors.

The first 9 digits in the ISBN have to be between 0 and 9.
The check digit can additionally be an 'X' to allow 10 to be a valid check digit as well.

A valid ISBN-10 is calculated with this formula `(x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11 == 0`
So for our example ISBN this means:
(3 * 10 + 5 * 9 + 9 * 8 + 8 * 7 + 2 * 6 + 1 * 5 + 5 * 4 + 0 * 3 + 8 * 2 + 8 * 1) mod 11 = 0

Which proves that the ISBN is valid.

## Caveats

Converting from string to number can be tricky in certain languages.
It's getting even trickier since the check-digit of an ISBN-10 can be 'X'.

## Bonus tasks

* Generate a valid ISBN-13 from the input ISBN-10 (and maybe verify it again with a derived verifier)

* Generate valid ISBN, maybe even from a given starting ISBN
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

Converting a string into a number and some basic processing utilizing a relatable real world example. [https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-10_check_digit_calculation](https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-10_check_digit_calculation)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
