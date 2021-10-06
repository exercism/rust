# Introduction

Sometimes a certain piece of code needs to be used more than once. If that's the case, it might be convenient to put the code into a function. A function generally only performs one specific action.

```rust
fn say_my_name(name: &str) {
    println!("Hello, {}!", name);
}
```

In Rust, the ```fn``` keyword is used to define functions. The code belonging to the function is always between brackets.

```rust
fn main() {
    println!("Hello, world!");
    say_my_name("Heisenberg");
}
```

The function above is special in the sense that it is used as an entry point of programs. From that function you can call on other functions. You can see functions can also take parameters, like ```name: &str``` in ```say_my_name()``` which allows for input into the function.


