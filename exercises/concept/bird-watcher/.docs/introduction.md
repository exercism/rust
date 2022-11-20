# Introduction

Vectors are re-sizable arrays that can grow or shrink in size. It can only store values of the same type.

Some operations that vectors support are:

### Initialize

A vector can be initialized with values by:

```
let vector = vec![2, 5, 0, 7, 4, 1]
// or
let vector: Vec<u8> = Vec::from([2, 5, 0, 7, 4, 1]);
```

### Iterate

Immutable vectors can be iterated over in the following manner:

```
for value in vector.iter() {
	println!("{}", value)
}
```

If the vector is mutable, `iter()` will have to be replaced by `iter_mut()`

A vector can also be iterated over it's key and value simultaneously:

```
for (i, val) in vector.iter().enumerate() {
    println!("Index {} has value {}", i, val);
}
```

### Retrieve

The value at index can be retrieved by using an index or range. Use index to get a single value, and use range to get a slice.

```
// Show value at index 3
println!("{:?}", vector[3])

// Show (slice of) values from index 2 to 5 (excluding)
println!("{:?}", &vector[2..5]);
```

Vectors support functionality like:

+ Calculating the sum of elements

+ Geting a (underlying) slice

+ Performing increment with a custom step

+ Getting the last element

+ Checking if a value is present

In this exercise we will be covering all of the above.
