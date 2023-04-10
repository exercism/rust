# Introduction

An `array` is a collection of objects of the same type `T`, stored in contiguous
memory.
Arrays are created using brackets `[]`, and their length, which is known
at compile time, is part of their type signature `[T; length]`(...)

Unlike arrays in some other languages, arrays in Rust have a fixed length.


## Arrays
### Defining an array
We write the values in an array as a comma-separated list inside square
brackets:

```rust
let my_array = [1, 2, 3, 4, 5];
```

We write an array’s type using square brackets with the type of each element, a
semicolon, and then the number of elements in the array, like so:

```rust
let my_array: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of each element. After the semicolon, the number 5
indicates the array contains five elements.

You can also initialize an array to contain the same value for each element by
specifying the initial value, followed by a semicolon, and then the length of
the array in square brackets, as shown here:

```rust
let my_array = [3; 5];
// => [3, 3, 3, 3, 3]
```

### Accessing Array Elements
You can access elements of an array using indexing, like this:

```rust
let a = [1, 2, 3, 4, 5];

let first = a[0];
let second = a[1];
```


### Invalid Array Element Access
Let’s see what happens if you try to access an element of an array that is past
the end of the array.

```rust
let a = [1, 2, 3, 4, 5];

let sixth = a[5];
```

The above program will fail to compile with an error similar to:
```txt
let value = a[5];
            ^^^^^^^^^^^^ index out of bounds: the length is 5 but the index is 5
```

Now consider when you don't know the index you want to access and it is provided
to you via a function parameter or a user input:

```rust
fn access_element(index: usize) -> i32 {
    let a = [1, 2, 3, 4, 5];
    return a[index];
}

fn main() {
    access_element(5);
}
```

In this case, the Rust compiler cannot know if the index is out of bounds.
The code will compile without errors but fail at runtime with the following error.

```txt
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5'
```


### Use as a type

An array type can be used as a variable type, function parameter or return type.

```rust
let my_var: [i32; 5];

fn my_func(input: [i32; 5]) -> [i32; 5] {}
```

### Changing array elements
Like for any other variable, arrays also have to be defined using `mut` keyword
to allow change.
An element can be changed by using assignment operator while accessing it via an index.

```rust
let mut my_array = [2, 2, 3];
my_array[0] = 1;
// => [1, 2, 3]
```

In the above example, we access the first element via it's index 0 and assign a
new value to it.

## For loops

It’s often useful to execute a block of code more than once.
For this task, Rust provides several loops, which will run through the code
inside the loop body to the end and then start immediately back at the
beginning.

### `for` and range

The `for in` construct can be used to iterate through an `Iterator`.
We will learn more about `Iterators` in later concepts.
One of the easiest ways to create an iterator is to use the range notation `a..b`.
This yields values from a (inclusive) to b (exclusive) in steps of one.

```rust
// A for loop that iterates 10 times
for n in 0..10 {
    // n will have a starting value of 0
    // n will have the last value as 9
}
```

Alternatively, `a..=b` can be used for a range that is inclusive on both ends. The above can be written as:

```rust
// A for loop that iterates 10 times
for n in 1..=10 {
    // n will have a starting value of 1
    // n will have the last value as 10
}
```

### Break and Continue a loop
A for loop can be halted by using the `break` statement.

```rust
// A for loop that iterates 10 times
for n in 1..=10 {
    println!(n);
    break;
}
// => 1
```

One can conditionally break a for loop as follows:

```rust
// A for loop that iterates 10 times
for n in 1..=10 {
    println!(n);
    if n == 5 {
        break;
    }
}
// => 1
// => 2
// => 3
// => 4
// => 5
```

A `continue` statement can be used to skip over an iteration.
```rust
// A for loop that iterates 10 times
for n in 1..=10 {
    println!(n);
    continue;
}
// => 1
```

Similar to `break`, one can also conditionally use `continue`:

```rust
// A for loop that iterates 10 times
for n in 1..=10 {
    println!(n);
    if n >= 5 {
        continue;
    }
}
// => 1
// => 2
// => 3
// => 4
// => 5
```

### Looping Through an Array with `for`
The following shows an example of looping through an array using the `for in` construct. 

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}

// => the value is: 10
// => the value is: 20
// => the value is: 30
// => the value is: 40
// => the value is: 50

```
