# About

A [reference][reference] is a pointer: an address in memory where a value is stored.

To understand the use of references it is helpful to briefly review ownership. There is only one owner of a value at a time, and when the owner
goes out of scope the value will be dropped, as exampled below

```rust
fn log(msg: String) { // msg takes ownership of the value passed to it
    let formatted_datetime: String;
    // code  to format current datetime snipped
    println!("{} at {}", msg, formatted_datetime);
} // msg is dropped here

pub fn main() {
    let my_string = "Something happened".to_string();
    log(my_string); // passing my_string transfers ownership of its value to msg
    //println!("{:?}", my_string); // uncommenting this line will error because my_string's value has been dropped while owned by msg
}
```

A [reference][borrow] represents a _borrow_ of an owned value. Instead of having `msg` own `my_string`'s value, we can have `msg` borrow it and
then give it back to `my_string` at the end of `log`.

To have `msg` borrow `my_string` we define `msg` as a reference and give it `my_string`'s address, not `my_string` itself, like so

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

References are immutable, but they can be made mutable by appending `mut` to the ampersand. To access the value, the dereference operator (`*`)
is prepended to the reference, as exampled below

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

A value at any given time can have either one mutable reference or any number of immutable references to it, as exampled below

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
