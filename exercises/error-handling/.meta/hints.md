## Rust Specific Track Information

In Rust, it is expected (and enforced by the compiler) that all possible outcomes are handled.  There is no throw and catch error system that is present in other languages.

There is panic!(), when the program gets into an unknown and dangerous state.  In this situation, we want the program to shutdown down cleanly.  This is not a state you would catch and continue, it is more of a fatal and crashing condition.

This exercise will deal with [Result](https://doc.rust-lang.org/std/result/enum.Result.html), which is the way Rust allows of dealing with an error condition.

It will also work with [Option](https://doc.rust-lang.org/std/option/), which handles conditions where things may not exist.  By using an Option instead of a null or nil type, there is no way to try to use an object that doesn't exist in Rust.

In this exercise, you have a partial coded Ice Cream Truck implementation.  Complete the two existing methods so they work and implement the missing method.  You will be returning and handling Option and Result types.  If you have not run across this before, Enums can have function implementations.
