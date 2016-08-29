extern crate react;

#[allow(unused_mut)]

// TODO: [ignore] tests

use react::*;

#[test]
fn set_value_of_input() {
    let mut reactor = Reactor::new();
    let mut input = reactor.create_input(1);
    assert_eq!(*input.value(), 1);
    input.set_value(2);
    assert_eq!(*input.value(), 2);
}

#[test]
fn compute1_depending_on_input() {
    let mut reactor = Reactor::new();
    let mut input = reactor.create_input(1);
    let output = reactor.create_compute1(&input, |v| v + 1);
    assert_eq!(*output.value(), 2);
    //input.set_value(2);
    //assert_eq!(*output.value(), 3);
}
