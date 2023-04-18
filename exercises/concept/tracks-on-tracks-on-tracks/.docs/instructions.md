# Instructions

In this exercise you'll be writing code to keep track of a list of programming languages you want to learn on Exercism.

You have nine tasks, which will all involve dealing with vectors.

## 1. List finished

You'll want to keep track of languages you've already finished learning.

Implement the `list_finished()` function that returns a new, empty vector.

```rust
list_finished()
// => []
```

## 2. Define an existing wish list

Currently, you have a piece of paper listing the languages you wish to learn: Rust, Clojure and Elm.

Implement the `planned_wishlist()` function to return the vector.

```rust
planned_wishlist();
// => ["Rust", "Clojure", "Elm"]
```

## 3. Add a new language to the wish list

As you explore Exercism and find more interesting languages, you want to add them to your wishlist.

Implement the `add_to_wishlist()` function to add a new language to the end of your wish list.

```rust
add_to_wishlist(planned_wishlist(), "VBA");
// => ["Rust", "Clojure", "Elm", "VBA"]
```

## 4. Count the languages in the wish list

Counting the languages one-by-one is inconvenient.

Implement the `count_languages()` function to count the number of languages on
your wish list.

```rust
count_languages(planned_wishlist())
// => 3
```

## 5. Check to see if a language is in the wish list

Implement the `has_language()` function to check if a language is present.

```rust
has_language(planned_wishlist(), "Elm")
// => true
```

## 6. Reverse the list

At some point, you realize that your list is actually ordered backwards!

Implement the `reverse_wishlist()` function to reverse your wish list.

```rust
reverse_wishlist(planned_wishlist())
// => ["Elm", "Clojure", "Rust"]
```

## 7. Check if list is exciting

While you love all languages, Rust has a special place in your heart. As such, you're really excited about a list of languages if:

- The first on the list is Rust.
- The second item on the list is Rust and the list contains either two or three items 

Implement the `is_exciting()` function to check if a list of languages is exciting:

```rust
is_exciting(planned_wishlist())
// => false
```

## 8.Remove Language from wish list

Implement the `remove_from_wishlist()` function to remove a specified language
from the wish list.

```rust
remove_from_wishlist(planned_wishlist(), "Clojure")
// => [ "Rust", "Elm" ]
```

## 9. Check if all languages in the wish list are unique

Implement the `is_unique()` function to check if any of the languages is
repeated in the wish list.

The list of languages (i.e. the parameter) is guaranteed not to be empty when this function is called and it doesn't matter if the list is modified.

```rust
is_unique(planned_wishlist())
// => true
```
