extern crate react;

use react::*;

#[test]
fn input_cells_have_a_value() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(10);
    assert_eq!(reactor.value(input), Some(10));
}

#[test]
#[ignore]
fn an_input_cells_value_can_be_set() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(4);
    assert!(reactor.set_value(input, 20).is_ok());
    assert_eq!(reactor.value(input), Some(20));
}

#[test]
#[ignore]
fn error_setting_a_nonexistent_input_cell() {
    let mut dummy_reactor = Reactor::new();
    let input = dummy_reactor.create_input(1);
    assert!(Reactor::new().set_value(input, 0).is_err());
}

#[test]
#[ignore]
fn compute_cells_calculate_initial_value() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor.create_compute(&vec![input], |v| v[0] + 1).unwrap();
    assert_eq!(reactor.value(output), Some(2));
}

#[test]
#[ignore]
fn compute_cells_take_inputs_in_the_right_order() {
    let mut reactor = Reactor::new();
    let one = reactor.create_input(1);
    let two = reactor.create_input(2);
    let output = reactor.create_compute(&vec![one, two], |v| v[0] + v[1] * 10).unwrap();
    assert_eq!(reactor.value(output), Some(21));
}

#[test]
#[ignore]
fn error_creating_compute_cell_if_input_doesnt_exist() {
    let mut dummy_reactor = Reactor::new();
    let input = dummy_reactor.create_input(1);
    assert!(Reactor::new().create_compute(&vec![input], |_| 0).is_err());
}

#[test]
#[ignore]
fn do_not_break_cell_if_creating_compute_cell_with_valid_and_invalid_input() {
    let mut dummy_reactor = Reactor::new();
    let _ = dummy_reactor.create_input(1);
    let dummy_cell = dummy_reactor.create_input(2);
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    assert!(reactor.create_compute(&vec![input, dummy_cell], |_| 0).is_err());
    assert!(reactor.set_value(input, 5).is_ok());
    assert_eq!(reactor.value(input), Some(5));
}

#[test]
#[ignore]
fn compute_cells_update_value_when_dependencies_are_changed() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor.create_compute(&vec![input], |v| v[0] + 1).unwrap();
    assert_eq!(reactor.value(output), Some(2));
    assert!(reactor.set_value(input, 3).is_ok());
    assert_eq!(reactor.value(output), Some(4));
}

#[test]
#[ignore]
fn compute_cells_can_depend_on_other_compute_cells() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let times_two = reactor.create_compute(&vec![input], |v| v[0] * 2).unwrap();
    let times_thirty = reactor.create_compute(&vec![input], |v| v[0] * 30).unwrap();
    let output = reactor.create_compute(&vec![times_two, times_thirty], |v| v[0] + v[1]).unwrap();
    assert_eq!(reactor.value(output), Some(32));
    assert!(reactor.set_value(input, 3).is_ok());
    assert_eq!(reactor.value(output), Some(96));
}

#[test]
#[ignore]
fn error_setting_a_compute_cell() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor.create_compute(&vec![input], |_| 0).unwrap();
    assert!(reactor.set_value(output, 3).is_err());
}

#[test]
#[ignore]
fn compute_cells_fire_callbacks() {
    // This is a bit awkward, but the closure mutably borrows `values`.
    // So we have to end its borrow by taking reactor out of scope.
    let mut values = Vec::new();
    {
        let mut reactor = Reactor::new();
        let input = reactor.create_input(1);
        let output = reactor.create_compute(&vec![input], |v| v[0] + 1).unwrap();
        assert!(reactor.add_callback(output, |v| values.push(v)).is_ok());
        assert!(reactor.set_value(input, 3).is_ok());
    }
    assert_eq!(values, vec![4]);
}

#[test]
#[ignore]
fn error_adding_callback_to_nonexistent_cell() {
    let mut dummy_reactor = Reactor::new();
    let input = dummy_reactor.create_input(1);
    let output = dummy_reactor.create_compute(&vec![input], |_| 0).unwrap();
    assert!(Reactor::new().add_callback(output, |_: usize| println!("hi")).is_err());
}

#[test]
#[ignore]
fn callbacks_only_fire_on_change() {
    let mut values = Vec::new();
    {
        let mut reactor = Reactor::new();
        let input = reactor.create_input(1);
        let output = reactor.create_compute(&vec![input], |v| if v[0] < 3 { 111 } else { 222 }).unwrap();
        assert!(reactor.add_callback(output, |v| values.push(v)).is_ok());
        assert!(reactor.set_value(input, 2).is_ok());
        assert!(reactor.set_value(input, 4).is_ok());
    }
    assert_eq!(values, vec![222]);
}

#[test]
#[ignore]
fn callbacks_can_be_added_and_removed() {
    let mut values1 = Vec::new();
    let mut values2 = Vec::new();
    let mut values3 = Vec::new();
    {
        let mut reactor = Reactor::new();
        let input = reactor.create_input(11);
        let output = reactor.create_compute(&vec![input], |v| v[0] + 1).unwrap();
        let callback = reactor.add_callback(output, |v| values1.push(v)).unwrap();
        assert!(reactor.add_callback(output, |v| values2.push(v)).is_ok());
        assert!(reactor.set_value(input, 31).is_ok());
        assert!(reactor.remove_callback(output, callback).is_ok());
        assert!(reactor.add_callback(output, |v| values3.push(v)).is_ok());
        assert!(reactor.set_value(input, 41).is_ok());
    }
    assert_eq!(values1, vec![32]);
    assert_eq!(values2, vec![32, 42]);
    assert_eq!(values3, vec![42]);
}

#[test]
#[ignore]
fn removing_a_callback_multiple_times_doesnt_interfere_with_other_callbacks() {
    let mut values1 = Vec::new();
    let mut values2 = Vec::new();
    {
        let mut reactor = Reactor::new();
        let input = reactor.create_input(1);
        let output = reactor.create_compute(&vec![input], |v| v[0] + 1).unwrap();
        let callback = reactor.add_callback(output, |v| values1.push(v)).unwrap();
        assert!(reactor.add_callback(output, |v| values2.push(v)).is_ok());
        // We want the first remove to be Ok, but we don't care about the others.
        assert!(reactor.remove_callback(output, callback).is_ok());
        for _ in 1..5 {
            assert!(reactor.remove_callback(output, callback).is_err());
        }
        assert!(reactor.set_value(input, 2).is_ok());
    }
    assert_eq!(values1, Vec::new());
    assert_eq!(values2, vec![3]);
}

#[test]
#[ignore]
fn callbacks_should_only_be_called_once_even_if_multiple_dependencies_change() {
    let mut values = Vec::new();
    {
        let mut reactor = Reactor::new();
        let input = reactor.create_input(1);
        let plus_one = reactor.create_compute(&vec![input], |v| v[0] + 1).unwrap();
        let minus_one1 = reactor.create_compute(&vec![input], |v| v[0] - 1).unwrap();
        let minus_one2 = reactor.create_compute(&vec![minus_one1], |v| v[0] - 1).unwrap();
        let output = reactor.create_compute(&vec![plus_one, minus_one2], |v| v[0] * v[1]).unwrap();
        assert!(reactor.add_callback(output, |v| values.push(v)).is_ok());
        assert!(reactor.set_value(input, 4).is_ok());
    }
    assert_eq!(values, vec![10]);
}

#[test]
#[ignore]
fn callbacks_should_not_be_called_if_dependencies_change_but_output_value_doesnt_change() {
    let mut values = Vec::new();
    {
        let mut reactor = Reactor::new();
        let input = reactor.create_input(1);
        let plus_one = reactor.create_compute(&vec![input], |v| v[0] + 1).unwrap();
        let minus_one = reactor.create_compute(&vec![input], |v| v[0] - 1).unwrap();
        let always_two = reactor.create_compute(&vec![plus_one, minus_one], |v| v[0] - v[1]).unwrap();
        assert!(reactor.add_callback(always_two, |v| values.push(v)).is_ok());
        for i in 2..5 {
            assert!(reactor.set_value(input, i).is_ok());
        }
    }
    assert_eq!(values, Vec::new());
}

#[test]
#[ignore]
fn test_adder_with_boolean_values() {
    // This is a digital logic circuit called an adder:
    // https://en.wikipedia.org/wiki/Adder_(electronics)
    let mut reactor = Reactor::new();
    let a = reactor.create_input(false);
    let b = reactor.create_input(false);
    let carry_in = reactor.create_input(false);

    let a_xor_b = reactor.create_compute(&vec![a, b], |v| v[0] ^ v[1]).unwrap();
    let sum = reactor.create_compute(&vec![a_xor_b, carry_in], |v| v[0] ^ v[1]).unwrap();

    let a_xor_b_and_cin = reactor.create_compute(&vec![a_xor_b, carry_in], |v| v[0] && v[1]).unwrap();
    let a_and_b = reactor.create_compute(&vec![a, b], |v| v[0] && v[1]).unwrap();
    let carry_out = reactor.create_compute(&vec![a_xor_b_and_cin, a_and_b], |v| v[0] || v[1]).unwrap();

    let tests = vec![
        (false, false, false, false, false),
        (false, false, true, false, true),
        (false, true, false, false, true),
        (false, true, true, true, false),
        (true, false, false, false, true),
        (true, false, true, true, false),
        (true, true, false, true, false),
        (true, true, true, true, true),
    ];

    for (aval, bval, cinval, expected_cout, expected_sum) in tests {
        assert!(reactor.set_value(a, aval).is_ok());
        assert!(reactor.set_value(b, bval).is_ok());
        assert!(reactor.set_value(carry_in, cinval).is_ok());

        assert_eq!(reactor.value(sum), Some(expected_sum));
        assert_eq!(reactor.value(carry_out), Some(expected_cout));
    }
}
