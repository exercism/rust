# Loops

Loops in Rust can be used to repeatedly execute a block of code until you tell it to stop or a certain condition is met.

The most basic version of a loop uses the `loop` keyword:

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

The above code-snippet makes an infinite loop. To exit a loop use the break keyword:

```rust
fn main() {
    let mut count = 0;
    loop {
        println!("again!");
        count += 1;
        if count > 10 { break; }
    }
}
```

## Loops using `for` and `while`

Apart from the `loop` keyword, you can also make loops with `for` and `while`.

A `for` loop is used to iterate over the elements of a collection or iterator.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for x in a {
        println!("the value is: {}", x);
    }
}
```

Ranges are iterators too, which makes iterating through them with the `for` loop very convenient:

```rust
fn main() {
    for x in 0..10 {
        println!("the value is: {}", x);
    }
}
```

A while loop keeps repeating code until a condition is met. We could re-write the earlier example using a while loop as follows:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

See [https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops](repetition with loops) in the Rust book for more information about loops.
