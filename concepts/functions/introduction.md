# Introduction

Sometimes a certain piece of code needs to be used more than once. If that's the case, it might be convenient to put the code into a function. A function generally only performs one specific action.

```rust
fn say_my_name() {
    println!("Hello, ....!");
}
```

In Rust, the ```fn``` keyword is used to define functions. The code belonging to the function is always between brackets.

```rust
fn main() {
    println!("Hello, world!");
    say_my_name();
}
```

The function above is special in the sense that it is used as an entry point of programs. From that function you can call on other functions. Notice the name of ```say_my_name()```, this style of naming is called _snake_ case and is the conventional style of naming functions within Rust.

## Function parameters

The previous ```say_my_name()``` function has some dots in. This is meant to be a placeholder for a name. Let's expand on it.

```rust
fn say_my_name(name: &str) {
    println!("Hello, {}!", name);
}
```
Now you can supply the function with a parameter, which in this case is a string, a piece of text. Beside strings there are other types of parameters like integers.

```rust
fn main() {
    println!("Hello, world!");
    say_my_name("Heisenberg);
}
```

