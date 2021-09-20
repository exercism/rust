# Hashmap

Hashmap is a container that can be used to store "key-value pairs". This container is also known by the name associative array or Hashtable in other programming langauges. In short, there is a value stored in the hashmap and it is mapped to a key. The user can retrieve the stored value by passing in the key or store a value mapped to a specific key.

Hashmap is a generic collection (like most of the collections in the standard library) and thus supports a broad variety of types as keys (including user-defined structs and tuples).

# Creating a Hashmap

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

Other than using the `[]` operator, there are 2 other ways of accessing values stored in a Hashmap. The first is using the `get` member function:

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

This is a very useful API and makes it convenient to address common ways in which Hashmaps are used that it is very popular and has an entire concept dedicated to it.
