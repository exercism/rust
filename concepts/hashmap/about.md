# Hashmap

Hashmap is a container that can be used to store "key-value pairs". This container is also known by the name associative array or Hashtable in other programming langauges. In short, the map contains a set of keys, each of which is mapped to a particular value. The user can retrieve a stored value by passing in the appropriate key. Users can also insert and delete keys and the associated values.

Hashmap is a generic collection (like most of the collections in the standard library) and thus supports a broad variety of types as keys (including user-defined structs and tuples).

## Creating a Hashmap

A hashmap is created using the `HashMap::new()` function. The below snippet of code creates a Hashmap with scores mapped to team names.

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

To retrieve the score of the blue team, for example, we can use `scores["Blue"]`.

# Accessing values in Hashmap

Other than using the indexing (`[]`) operator, there are two other ways of accessing values stored in a Hashmap. The first is using the `get` member function:

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    println!("Blue scored: {} \n", scores.get(&team_name).unwrap());
}
```

The last line accesses the value stored against the key "Blue". The `get` member function returns an optional value which is `None` if the supplied key doesn't exist in the Hashmap and is `Some(value)` if the key exists. For more about Options in Rust, check the Options concept.

The other way to access values stored in a Hashmap is to use the `entry` method. The `entry` method (or the entry API) returns a reference to the entry of the key value pair in the Hashmap. The entry represents the state of this key in the Hash. If the key does not exist, then there is no value in the entry (and it allows one to be inserted).

```rust
fn main() {
    use std::collections::HashMap;

    let mut vote_counter = HashMap::new();
    let votes = ["Blue", "Red", "Red", "Blue", "Red", "Blue", "Blue"];
    for vote in votes {
        let count = vote_counter.entry(vote).or_insert(0);
        *count += 1;
    }

    println!("{:?}", vote_counter);
}
```

This API makes certain common access patterns very convenient; it has an entire concept (Entry API) dedicated to it.

## Efficiency

One of the reasons a Hashmap is popular is because it is fast.

Below is a list of operations support by Hashmap and it's time-complexity:

| Operation        | Time-Complexity                |
| ---------------- | ------------------------------ |
| Insertion        | O(1) (amortized constant-time) |
| Removal          | O(1) (amortized constant-time) |
| Lookup           | O(1) (amortized constant-time) |
| Retrieve value   | O(1) (amortized constant-time) |
| Set/Update value | O(1) (amortized constant-time) |

In summary, all standard operations in hashmap have an amortized constant time-complexity, i.e., O(1) operations

## Trait Bounds

A Hashmap is a generic data structure, meaning it supports arbitrary types to be used as keys and values, with only one constraint - for a type to be used as the key type, it must have an implementation for two traits: `Eq` and `Hash`. These two traits allow the Hashmap to check for equality/ duplicate keys as well as to group similar keys for fast lookup using a hashing function (hence the name Hashmap). There are no trait bounds on the value type.
