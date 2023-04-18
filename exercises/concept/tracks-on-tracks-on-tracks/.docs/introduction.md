# Introduction

Rust’s standard library includes a number of very useful data structures called collections.
Most other data types represent one specific value, but collections can contain multiple values.
Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.


## Vectors

Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

### Declaration
To create a new empty vector, we call the Vec::new function:

```rust
let v: Vec<i32> = Vec::new();
```

Note that we added a type annotation here.
Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.
This is an important point.
Vectors are implemented using generics; we’ll cover how to use generics with your own types in later concepts.

More often, we create a `Vec<T>` with initial values and Rust will infer the type of value we want to store.
Rust conveniently provides the `vec!` macro, which will create a new vector that holds the values you give it.
We will learn more about what is a macro in later concepts.

```rust
let v = vec![1, 2, 3];
```

Because we’ve given initial `i32` values, Rust can infer that the type of `v` is `Vec<i32>`, and the type annotation isn’t necessary.

### Updating a vector
To create a vector and then add elements to it, we can use the `push` method:

```rust
let mut v: Vec<i32> = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

As with any variable, if we want to be able to change its value, we need to make it mutable using the `mut` keyword.

Similarly, to remove an element, the built-in method `remove` can be used:

```rust
let mut v = vec![1, 2, 3];

v.remove(0); // Removes 1
v // => [2, 3]

```

Whe `remove` is used with an index that is out of bounds, the program will panic.


### Reading Elements of Vectors
There are two ways to reference a value stored in a vector: via indexing or
using the `get` method.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");
```

In the above variable `third` is a reference to an element in our vector `v`.
Similar to arrays, Vectors are zero-indexed.
Thus an index of `2` will point to the third element.

The program will panic if we try accessing an element which is out of bounds.

```rust
let v = vec![1, 2, 3, 4, 5];

let non_existent: &i32 = &v[200];
// => panic
```

There are times when we need to access an index but we don't know if it will be out of bounds. 
In these situations, it might not be desirable for the program to panic. 
The other method of accessing elements becomes very useful:

```rust
let v = vec![1, 2, 3, 4, 5];

let fifth: Option<&i32> = v.get(4);

let non_existent: Option<&i32> = v.get(200);

```

The `get` method returns an `Option`, as we've learned before they are very useful for denoting an absence of a value.


### Built-in Methods

- The `sort` method can be used to sort the elements of a vector in ascending
  order.
  ```rust
  let mut v = [-5, 4, 1, -3, 2];

  v.sort();
  v // => [-5, -3, 1, 2, 4]
  ```

- The `dedup` method can be used to remove consecutive duplicate elements.
  ```rust
  let mut v = vec![1, 2, 2, 3, 2];

  v.dedup();

  v // => [1, 2, 3, 2]
  ```

- The `len` method can be used to count the elements.
  ```rust
  let v = vec![1, 2, 3];
  v.len() // => 3
  ```


### Use as type

Similar to other types we've learned so far, `Vec` type can be used as a variable type, function parameter type as well as return type.

```rust
let my_vector: Vec<i32>;

pub fn my_function(input: Vec<i32>) -> Vec<i32> {}
```

### Looping over a vector

Similar to arrays, the `for in` construct can be used to loop over `Vector`s

```rust
let v = vec![100, 32, 57];
for i in v {
    println!("{i}");
}
```

We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
The for loop in Listing 8-8 will add 50 to each element.

The `while` construct can also be used to loop over a vector:

```rust
let v = vec![1, 2, 3];
let len = v.len();
let mut i = 0;

while i < len {
  let element = v[i];
  i += 1;
}
```
