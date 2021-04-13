# About

Functions are bodies of code containing one or more [statements or expressions][statements or expressions]. A function may optionally
[return an value][return value]. 

Rust style is to use [snake case][snake case] for a function name, which is prefaced by the `fn` keyword for a function definition. For example

```rust
fn do_nothing() {}

```

The empty parentheses are a placeholder for zero or more parameters. A parameter is the definition of a value to be passed into the function.
Even if there are no parameters the parentheses are still required in the function definition. The actual value passed to a parameter is called
an argument.

The combination of parameters and return value is known as the function's signature. A function's signature requires that each parameter must
have its type annotated.

The curly braces enclose the body of the function. Inside the braces are where the function's statements and expressions are placed. Even if the
function contains no statements or expressions the curly braces are still required for the function definition. Of course there is no point to
having an empty function.

A function could contain a simple statement. In the following example the function takes in one `i32` argument and prints it

```rust
fn print_integer(value: i32) {
    println!("{:?}", value);
}
```

Note that the parameter needs to be given a name (`value` in this case) and be annotated with its type.

A function can also return an expression. In the following example the function takes in one `i32` argument and returns it doubled

```rust
fn double_integer(value: i32) -> i32 {
    value * 2
}
```

The `-> i32` indicates that the function returns an `i32`. Unlike parameters, the returned value is not named. Note that the last expression
in a function can be returned without the `return` keyword and a semicolon. `return` _is_ needed to return early from a function, like so

```rust
fn long_function() -> i32 {
    let some_condition = false;
    // code snipped
    if (some_condition) {
        return 0;
    }
    // more code snipped
    42
}
```

A function has its own scope which borrows or takes ownership of arguments passed to it. In this example the last line in `main` causes a
compile error

```rust
fn double_integer(value: Box<i32>) -> i32 {
    *value * 2
}

pub fn main() {
    let num = Box::new(42);
    let num_doubled = double_integer(num); // value moved here
    println!("{:?}", num_doubled);
    println!("{:?}", num); // borrow of moved value: `num` value borrowed here after move
    // move occurs because `num` has type `Box<i32>`, which does not implement the `Copy` trait
}
```

`main`, which owns the boxed `num` value, moves `num` as the argument for `value` into `double_integer`, which takes ownership of it. At the
end of `double_integer`, `value` goes out of scope and is dropped, so `num` is no longer valid for the rest of the `main` function.

Note that `fn main` is prefaced by `pub`, but `fn double_integer` is not. A `pub fn` is public, meaning it can be accessed from a different
module. Functions defined without `pub` are private and can only be accessed from the current module. `pub(crate) fn` makes a function
accessible only within the current crate.

`const fn` is used to define a [constant function][constant function]. Going back to `double_integer`, by making it a `const fn` we can now use
it to set a constant, like so

```rust
const fn double_integer(value: i32) -> i32 {
    value * 2
}

pub fn main() {
    const DOUBLE_42: i32 = double_integer(42);
    println!("{:?}", DOUBLE_42);
}
```

The constraint for a constant function is that it must be able to be evaluated at compile time. If the function does not satify that constraint
a compile error will result when using `const fn`, as exampled below

```rust
const fn multiply_integer(value: i32) -> i32 {
    use std::time::SystemTime;
    if SystemTime::now().elapsed().unwrap().as_nanos() == 0 { // this line errors
        value * 2
    } else {
        value * 3
    }
// calls in constant functions are limited to constant functions, tuple structs and tuple variants
}
```

Parameters can also be defined as functions. In this example we use the [`Fn`][Fn] operator to pass in the `double_integer` function to the
`call_with_one` function. `call_with_one` takes a function with one `i32` parameter that returns an `i32`.

```rust
fn double_integer(value: i32) -> i32 {
    value * 2
}

fn call_with_one<F>(func: F) -> i32
where
    F: Fn(i32) -> i32,
{
    func(1)
}

pub fn main() {
    let num_doubled = call_with_one(double_integer);
    println!("{:?}", num_doubled);
}
```

Prints `2`

A function can also be the return value from another function. In this example we return either of two functions from `get_log_function`.

```rust
fn write_to_database(msg: &str) {
    // implemetation code snipped
}

fn write_to_file(msg: &str) {
    // implemetation code snipped
}

fn is_db_up() -> bool {
    true
}

fn get_log_function() -> Box<dyn Fn(&str)> {
    if is_db_up() {
        Box::new(write_to_database)
    } else {
        Box::new(write_to_file)
    }
}

fn log<F>(log_fn: F, msg: &str)
where
    F: Fn(&str),
{
    log_fn(&msg);
}

pub fn main() {
    let msg = "It happened";
    log(get_log_function(), msg);
}
```

There are anonymous functions which are covered in the `closures` topic.

[statements or expressions]: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions
[return value]: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values
[snake case]: https://en.wikipedia.org/wiki/Snake_case
[constant function]: https://doc.rust-lang.org/reference/const_eval.html#const-functions
[Fn]: https://doc.rust-lang.org/std/ops/trait.Fn.html
