# `HashMap`

A `HashMap` is a container that can be used to store key-value pairs. In other programming languages, this data structure is sometimes called an associative array or hash table. The map contains a set of keys, each of which is mapped to a particular value. The user can retrieve a stored value by passing in the appropriate key. Users can also insert and delete keys and the associated values.

`HashMap` is a generic collection (like most of the collections in the standard library) and thus supports a broad variety of types as keys, including user-defined structs and tuples. Its values can be of any type.

## Creating a `HashMap`

A `HashMap` is created using the `HashMap::new()` function. The below snippet of code creates a `HashMap` with team names mapped to scores.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

## Accessing values in `HashMap`

If a value is known to exist within a `HashMap`, then it's appropriate to use the indexing operator (`[]`). To retrieve the score of the blue team, for example, we can use `scores["Blue"]`. However, if the appropriate key-value pair does not exist within the hashmap, this operation will panic.

Other than the indexing operator, there are two other ways of accessing values stored in a `HashMap`. The first is using the `get` member function:

```rust
if let Some(blue_score) = scores.get("Blue") {
    println!("Blue scored: {} \n", blue_score);
}
```

`get` accesses the value stored against the requested key. It returns `None` if the supplied key doesn't exist in the `HashMap`, and `Some(value)` if the key exists. For more about Options in Rust, check the Options concept.

The other way to access values stored in a `HashMap` is to use the `entry` method. The `entry` method (or the entry API) returns a reference to the entry of the key value pair in the `HashMap`. The entry represents the state of this key in the Hash. If the key does not exist, then there is no value in the entry (and it allows one to be inserted).

```rust
let mut vote_counter: HashMap<_, usize> = HashMap::new();
let votes = ["Blue", "Red", "Red", "Blue", "Red", "Blue", "Blue"];
for vote in votes {
    let count = vote_counter.entry(vote).or_default();
    *count += 1;
}

println!("{:#?}", vote_counter);
```

This API makes certain common access patterns very convenient; it has an entire concept (Entry API) dedicated to it.

## Efficiency

`HashMap`s are relatively fast, and have amortized constant-time complexity (`O(1)`) for all operations involving a single key.

## Trait Bounds

A `HashMap` is a generic data structure, meaning it supports arbitrary types to be used as keys and values, with only one constraint: for a type to be used as the key type, it must have an implementation for two traits: `Eq` and `Hash`. There are no trait bounds on the value type.
