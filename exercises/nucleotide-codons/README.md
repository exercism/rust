# Nucleotide Codons

Write a function that returns the name of an amino acid a particular codon,
possibly using shorthand, encodes for.

In DNA sequences of 3 nucleotides, called codons, encode for amino acids. Often
several codons encode for the same amino acid. The International Union of Pure
and Applied Chemistry developed a shorthand system for designating groups of
codons that encode for the same amino acid.

Simply put they've expanded the four letters A, C, G and T with a bunch of
letters that stand for different possibilities. For example R means A and G.
So TAR stands for TAA and TAG (think of "TAR" as "TA[AG]" in regex notation).

Write some code that given a codon, which may use shorthand, returns the
name of the amino acid that that codon encodes for. You will be given
a list of non-shorthand-codon/name pairs to base your computation on.

See: [wikipedia](https://en.wikipedia.org/wiki/DNA_codon_table).

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

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
