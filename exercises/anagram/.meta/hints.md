# Hints

The solution is case insensitive, which means `"WOrd"` is the same as `"word"` or `"woRd"`. It may help to take a peek at the [std library](https://doc.rust-lang.org/std/index.html) for functions that can convert between them.

The solution cannot contain the word itself. Basically a word is always an anagram of it's self but for this exercise we don't care. This mean, given `"hello"` and the list `["hello", "olleh"]` the answear is `["olleh"]`
