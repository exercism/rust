# Hints

The solution is case insensitive, which means `"WOrd"` is the same as `"word"` or `"woRd"`. It may help to take a peek at the [std library](https://doc.rust-lang.org/std/primitive.char.html) for functions that can convert between them.

The solution cannot contain the input word. A word is always an anagram of itself, which means it is not an interesting result. Given `"hello"` and the list `["hello", "olleh"]` the answer is `["olleh"]`.

You are going to have to adjust the function signature provided in the stub in order for the lifetimes to work out properly. This is intentional: what's there demonstrates the basics of lifetime syntax, and what's missing teaches how to interpret lifetime-related compiler errors.
