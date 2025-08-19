# Introduction

## Tuples

A _tuple_ is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
We’ve added optional type annotations in this example:

```rust
let my_tuple: (i32, f64, u8) = (500, 6.4, 1);
```

The variable `my_tuple` binds to the entire tuple because a tuple is considered
a single compound element.
To get the individual values out of a tuple, we can use pattern matching to
destructure a tuple value, like this:

```rust
let (x, y, z) = my_tuple;

println!("{}", y); 
// => 6.4

```

This program first creates a tuple and binds it to the variable `my_tuple`.
It then uses a pattern with let to take `my_tuple` and turn it into three separate variables, `x`, `y`, and `z`.
This is called _destructuring_ because it breaks the single tuple into three
parts.
Finally, the program prints the value of `y`, which is `6.4`.

Sometimes, when _destructuring_ a tuple, some values might not be important or
needed, these can be discarded by labeling them with "`_`" (underscore).

```rust
let (_, y, _) = my_tuple;

println!("{}", y);
// => 6.4

```

We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
For example:

```rust
let my_tuple: (i32, f64, u8) = (500, 6.4, 1);

let five_hundred = my_tuple.0;

let six_point_four = my_tuple.1;

let one = my_tuple.2;
```

This program creates the tuple `my_tuple` and then accesses each of its elements using their respective indices.
As with most programming languages, the first index in a tuple is 0.

A tuple can contain 0, or upto 12 elements. A tuple with zero elements has a
special name, _unit_.

```rust
let my_zero_tuple = ();
let my_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
```

### Calling methods
One can call any methods on a value held by a tuple by either first destructuring that value out of the tuple or accessing it using it's index.

```rust
let my_tuple = (12, "hello");

let (my_num, my_str) = my_tuple;

my_str.to_uppercase();

// OR

my_tuple.1.to_uppercase();
```

### Functions can accept a tuple as a parameter.
Accepting a tuple as a parameter requires to explicitly define its type. The
following example illustrates that.

```rust
fn my_function(my_tuple: (i32, &str)) {
    // Do something with my_tuple
}
```

### Functions can return tuple as a result.
Returning a tuple as a result requires to explicitly define its type. The
following example illustrates that.

```rust
fn make_tuple(an_int: i32, a_string: &str) -> (i32, &str) {
    return (an_int, a_string);
}
```

### Methods can return tuple as a result.
Methods on various types sometimes return a tuple as a result. Consider the
following example of a `&str` variable's `.split_at()` method.

```rust
let str = "Per Martin-Löf";

let (first, last) = str.split_at(3);

first // => "Per"
last // => " Martin-Löf"
```
