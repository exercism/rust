# Instructions

In this exercise you'll be using a `HashMap`, along with entry API methods, to solve a simple algorithm problem.

Given `&[&str]` representing the words of a magazine article, and `&[&str]` representing the words of a note you would like to send, can you compose your note by cutting words out of the magazine and pasting them into a letter?

Notes:

- This implies physical cutting and pasting; the magazine needs to contain at least as many copies of each word as the note requires.
- Capitalization matters; just because you're pasting together a note composed from words of a magazine doesn't mean you're willing to be ungrammatical.

You'll start with the following stubbed function signature:

```rust
pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    unimplemented!()
}
```

Given the following input

```rust
let magazine = "two times three is not four".split_whitespace().collect::<Vec<&str>>();
let note = "two times two is four".split_whitespace().collect::<Vec<&str>>();
assert!(!can_construct_note(&magazine, &note));
```

The function returns `false` since the magazine only contains one instance of `"two"` when the note requires two of them.

The following input will succeed: 

```rust
let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'".split_whitespace().collect::<Vec<&str>>();
let note = "Amy Mainzer chatting with Leonardo DiCaprio."
    .split_whitespace()
    .collect::<Vec<&str>>();
assert!(can_construct_note(&magazine, &note));
```

The function returns `true` since the magazine contains all the words that the note requires.
