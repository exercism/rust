# About

To understand references it is necessary to understand ownership. Every value has a single owner. When its scope is exited, the value is dropped.
For example:

```rust
fn log(msg: String) { // msg takes ownership of the value passed to it
    let formatted_datetime = String::new();
    // code  to format current datetime snipped
    println!("{} at {}", msg, formatted_datetime);
} // msg is dropped here

pub fn main() {
    let my_string = "Something happened".to_string();
    log(my_string); // passing my_string transfers ownership of its value to msg
    //println!("{:?}", my_string); // uncommenting this line will error because my_string's value has been dropped while owned by msg
}
```

A [reference][borrow] represents a _borrow_ of an owned value. Instead of transferring ownership of `my_string`, we can have `log` borrow it and
then give it back to `my_string` in `main` at the end of `log`.

To do this, we define `msg` as a reference by prefixing an ampersand. The type `&String` is read as "reference to a `String`".

As `log`'s argument is now a reference, we need to pass a reference. To get a reference to `my_string`, prefix it with an ampersand, which is
the [borrow operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#borrow-operators). This lends `my_string` to `log`
for the duration of that function.

```rust
fn log(msg: &String) { //msg is defined as a reference with an ampersand
    let formatted_datetime: String;
    // code  to format current datetime snipped
    println!("{} at {}", msg, formatted_datetime);
}

pub fn main() {
    let my_string = "Something happened".to_string();
    log(&my_string); // my_string is passed as a reference with an ampersand
    println!("{:?}", my_string);
}
```

## References are immutable by default

A value at any given time can have multiple references to it as long as they are all immutable, as shown below

```rust
fn main() {
    let fibonacci = vec![1, 1, 2, 3, 5, 8, 13];
    // fibonacci is simultaneously passed by reference three times, which is okay because they are all immutable references
    println!(
        "{:#?}",
        check_shapes(&fibonacci[..=1], &fibonacci[1..=3], &fibonacci[2..],)
    );
}

fn check_shapes(constant: &[u8], linear: &[u8], superlinear: &[u8]) -> (bool, bool, bool) {
    (
        is_constant(constant),
        is_linear(linear),
        is_superlinear(superlinear),
    )
}

// understanding the implementations of the following functions is not necessary for this example
// but are provided should you be interested

fn is_constant(slice: &[u8]) -> bool { 
    slice
        .first()
        .map(|first| slice.iter().all(|v| v == first))
        .unwrap_or_default()
}

fn is_linear(slice: &[u8]) -> bool {
    let diffs: Vec<_> = slice
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
    is_constant(&diffs)
}

fn is_superlinear(slice: &[u8]) -> bool {
    let diffs: Vec<_> = slice
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
    diffs.windows(2).all(|window| window[1] > window[0])
}
```

## References can be dereferenced

Sometimes a reference needs to be dereferenced to access its value.

```rust
fn is_meaning_of_life(value: &u8) -> bool {
  value == 42
}
// oops! compiler error: `==` operator is not defined for `&u8`, `u8`
```

To access the value, the dereference operator (`*`) is prepended to the reference, as shown below

```rust
fn is_meaning_of_life(value: &u8) -> bool {
  *value == 42 // okay, the reference is dereferenced from `&u8` to `u8` which `==` can compare with another `u8`
}
```

## References can be mutable

References can be made mutable by appending `mut` to the ampersand. as shown below

```rust
fn add_five(counter: &mut i32) { //counter is defined as a mutable reference
    *counter += 5; // counter is dereferenced to access its value
}

pub fn main() {
    let mut my_count = 0; // my_count must be defined as mutable
    println!("{:?}", my_count);
    add_five(&mut my_count); // my_count is passed as a mutable reference
    println!("{:?}", my_count);
}
```

Prints
`0`
`5`

A value at any given time can have either one mutable reference or any number of immutable references to it, as shown below

```rust
fn add_five(counter: &mut i32) {
    *counter += 5;
}

pub fn main() {
    let mut my_count = 0;
    println!("{:?}", my_count);
    let mut my_count2 = &mut my_count; // first mutable reference to my_count
    add_five(&mut my_count); // this errors because my_count's mutable borrow by my_count2 will be used on the next line
    add_five(my_count2);
    println!("{:?}", my_count);
}
```

This  works, because the compiler knows that the mutable borrows do not overlap

```rust
fn add_five(counter: &mut i32) {
    *counter += 5;
}

pub fn main() {
    let mut my_count = 0;
    println!("{:?}", my_count);
    let mut my_count2 = &mut my_count; // first mutable reference to my_count
    add_five(my_count2); // my_count2's borrow of my_count is not used afer this point
    add_five(&mut my_count); // compiler allows second mutable borrow of my_count
    println!("{:?}", my_count); // because it does not conflict with first mutable borrow
}
```

Prints
`0`
`10`

[reference]: https://doc.rust-lang.org/std/primitive.reference.html
[borrow]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
