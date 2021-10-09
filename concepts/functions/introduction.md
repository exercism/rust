# Introduction

Sometimes a certain piece of code needs to be used more than once. If that's the case, it might be convenient to put the code into a function. A function generally only performs one specific action. In Rust, the ```fn``` keyword is used to define functions. The code belonging to the function is always between brackets. The function named `main` is special because it is the entry point for programs. From that function you can call other functions. 

```rust
fn say_my_name(name: &str) -> &str {
    name
}
```
Functions can also take parameters, like ```name: &str``` in ```say_my_name()``` which allows for input into the function. Functions can also return values if the return type is specified, with like -> &str above. The function `say_my_name` takes the parameter value and returns it. While not very useful, it shows the basic purpose.  Note the lack of a semicolon, if there would be one it would not return anything but would just be a regular statement.

