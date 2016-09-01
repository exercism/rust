extern crate react;

use std::fmt::Debug;

#[allow(unused_mut)]

// TODO: [ignore] tests

use react::*;

fn assert_cell_value<'a, T: Copy + PartialEq + Debug>(reactor: &Reactor<'a, T>, id: CellID, expected: T) {
    let cell = reactor.get(id).unwrap();
    assert_eq!(cell.value(), expected);
}

#[test]
fn set_value_of_input() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    assert_cell_value(&reactor, input, 1);
    {
        let mut cell = reactor.get_mut(input).unwrap();
        cell.set_value(2);
    }
    assert_cell_value(&reactor, input, 2);
}

#[test]
fn compute1_depending_on_input() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor.create_compute(&vec![input], |v| v[0] + 1);
    assert_cell_value(&reactor, output, 2);
}
