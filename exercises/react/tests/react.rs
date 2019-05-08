use react::*;

#[test]
fn input_cells_have_a_value() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(10);
    assert_eq!(Some(10), reactor.value(CellID::Input(input)));
}

#[test]
#[ignore]
fn an_input_cells_value_can_be_set() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(4);
    assert!(reactor.set_value(input, 20));
    assert_eq!(Some(20), reactor.value(CellID::Input(input)));
}

#[test]
#[ignore]
fn error_setting_a_nonexistent_input_cell() {
    let mut dummy_reactor = Reactor::new();
    let input = dummy_reactor.create_input(1);
    assert!(!Reactor::new().set_value(input, 0));
}

#[test]
#[ignore]
fn compute_cells_calculate_initial_value() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] + 1)
        .unwrap();
    assert_eq!(Some(2), reactor.value(CellID::Compute(output)));
}

#[test]
#[ignore]
fn compute_cells_take_inputs_in_the_right_order() {
    let mut reactor = Reactor::new();
    let one = reactor.create_input(1);
    let two = reactor.create_input(2);
    let output = reactor
        .create_compute(&[CellID::Input(one), CellID::Input(two)], |v| {
            v[0] + v[1] * 10
        })
        .unwrap();
    assert_eq!(Some(21), reactor.value(CellID::Compute(output)));
}

#[test]
#[ignore]
fn error_creating_compute_cell_if_input_doesnt_exist() {
    let mut dummy_reactor = Reactor::new();
    let input = dummy_reactor.create_input(1);
    assert_eq!(
        Err(CellID::Input(input)),
        Reactor::new().create_compute(&[CellID::Input(input)], |_| 0)
    );
}

#[test]
#[ignore]
fn do_not_break_cell_if_creating_compute_cell_with_valid_and_invalid_input() {
    let mut dummy_reactor = Reactor::new();
    let _ = dummy_reactor.create_input(1);
    let dummy_cell = dummy_reactor.create_input(2);
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    assert_eq!(
        Err(CellID::Input(dummy_cell)),
        reactor.create_compute(&[CellID::Input(input), CellID::Input(dummy_cell)], |_| 0)
    );
    assert!(reactor.set_value(input, 5));
    assert_eq!(Some(5), reactor.value(CellID::Input(input)));
}

#[test]
#[ignore]
fn compute_cells_update_value_when_dependencies_are_changed() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] + 1)
        .unwrap();
    assert_eq!(Some(2), reactor.value(CellID::Compute(output)));
    assert!(reactor.set_value(input, 3));
    assert_eq!(Some(4), reactor.value(CellID::Compute(output)));
}

#[test]
#[ignore]
fn compute_cells_can_depend_on_other_compute_cells() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let times_two = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] * 2)
        .unwrap();
    let times_thirty = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] * 30)
        .unwrap();
    let output = reactor
        .create_compute(
            &[CellID::Compute(times_two), CellID::Compute(times_thirty)],
            |v| v[0] + v[1],
        )
        .unwrap();
    assert_eq!(Some(32), reactor.value(CellID::Compute(output)));
    assert!(reactor.set_value(input, 3));
    assert_eq!(Some(96), reactor.value(CellID::Compute(output)));
}

/// A CallbackRecorder helps tests whether callbacks get called correctly.
/// You'll see it used in tests that deal with callbacks.
/// The names should be descriptive enough so that the tests make sense,
/// so it's not necessary to fully understand the implementation,
/// though you are welcome to.
struct CallbackRecorder {
    // Note that this `Cell` is https://doc.rust-lang.org/std/cell/
    // a mechanism to allow internal mutability,
    // distinct from the cells (input cells, compute cells) in the reactor
    value: std::cell::Cell<Option<i32>>,
}

impl CallbackRecorder {
    fn new() -> Self {
        CallbackRecorder {
            value: std::cell::Cell::new(None),
        }
    }

    fn expect_to_have_been_called_with(&self, v: i32) {
        assert_ne!(
            self.value.get(),
            None,
            "Callback was not called, but should have been"
        );
        assert_eq!(
            Some(v),
            self.value.replace(None),
            "Callback was called with incorrect value"
        );
    }

    fn expect_not_to_have_been_called(&self) {
        assert_eq!(
            None,
            self.value.get(),
            "Callback was called, but should not have been"
        );
    }

    fn callback_called(&self, v: i32) {
        assert_eq!(
            None,
            self.value.replace(Some(v)),
            "Callback was called too many times; can't be called with {}",
            v
        );
    }
}

#[test]
#[ignore]
fn compute_cells_fire_callbacks() {
    let cb = CallbackRecorder::new();
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] + 1)
        .unwrap();
    assert!(reactor
        .add_callback(output, |v| cb.callback_called(v))
        .is_some());
    assert!(reactor.set_value(input, 3));
    cb.expect_to_have_been_called_with(4);
}

#[test]
#[ignore]
fn error_adding_callback_to_nonexistent_cell() {
    let mut dummy_reactor = Reactor::new();
    let input = dummy_reactor.create_input(1);
    let output = dummy_reactor
        .create_compute(&[CellID::Input(input)], |_| 0)
        .unwrap();
    assert_eq!(
        None,
        Reactor::new().add_callback(output, |_: u32| println!("hi"))
    );
}

#[test]
#[ignore]
fn callbacks_only_fire_on_change() {
    let cb = CallbackRecorder::new();
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor
        .create_compute(
            &[CellID::Input(input)],
            |v| if v[0] < 3 { 111 } else { 222 },
        )
        .unwrap();
    assert!(reactor
        .add_callback(output, |v| cb.callback_called(v))
        .is_some());

    assert!(reactor.set_value(input, 2));
    cb.expect_not_to_have_been_called();
    assert!(reactor.set_value(input, 4));
    cb.expect_to_have_been_called_with(222);
}

#[test]
#[ignore]
fn callbacks_can_be_called_multiple_times() {
    let cb = CallbackRecorder::new();
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] + 1)
        .unwrap();
    assert!(reactor
        .add_callback(output, |v| cb.callback_called(v))
        .is_some());

    assert!(reactor.set_value(input, 2));
    cb.expect_to_have_been_called_with(3);
    assert!(reactor.set_value(input, 3));
    cb.expect_to_have_been_called_with(4);
}

#[test]
#[ignore]
fn callbacks_can_be_called_from_multiple_cells() {
    let cb1 = CallbackRecorder::new();
    let cb2 = CallbackRecorder::new();
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let plus_one = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] + 1)
        .unwrap();
    let minus_one = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] - 1)
        .unwrap();
    assert!(reactor
        .add_callback(plus_one, |v| cb1.callback_called(v))
        .is_some());
    assert!(reactor
        .add_callback(minus_one, |v| cb2.callback_called(v))
        .is_some());

    assert!(reactor.set_value(input, 10));
    cb1.expect_to_have_been_called_with(11);
    cb2.expect_to_have_been_called_with(9);
}

#[test]
#[ignore]
fn callbacks_can_be_added_and_removed() {
    let cb1 = CallbackRecorder::new();
    let cb2 = CallbackRecorder::new();
    let cb3 = CallbackRecorder::new();

    let mut reactor = Reactor::new();
    let input = reactor.create_input(11);
    let output = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] + 1)
        .unwrap();

    let callback = reactor
        .add_callback(output, |v| cb1.callback_called(v))
        .unwrap();
    assert!(reactor
        .add_callback(output, |v| cb2.callback_called(v))
        .is_some());

    assert!(reactor.set_value(input, 31));
    cb1.expect_to_have_been_called_with(32);
    cb2.expect_to_have_been_called_with(32);

    assert!(reactor.remove_callback(output, callback).is_ok());
    assert!(reactor
        .add_callback(output, |v| cb3.callback_called(v))
        .is_some());

    assert!(reactor.set_value(input, 41));
    cb1.expect_not_to_have_been_called();
    cb2.expect_to_have_been_called_with(42);
    cb3.expect_to_have_been_called_with(42);
}

#[test]
#[ignore]
fn removing_a_callback_multiple_times_doesnt_interfere_with_other_callbacks() {
    let cb1 = CallbackRecorder::new();
    let cb2 = CallbackRecorder::new();

    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let output = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] + 1)
        .unwrap();
    let callback = reactor
        .add_callback(output, |v| cb1.callback_called(v))
        .unwrap();
    assert!(reactor
        .add_callback(output, |v| cb2.callback_called(v))
        .is_some());
    // We want the first remove to be Ok, but the others should be errors.
    assert!(reactor.remove_callback(output, callback).is_ok());
    for _ in 1..5 {
        assert_eq!(
            Err(RemoveCallbackError::NonexistentCallback),
            reactor.remove_callback(output, callback)
        );
    }

    assert!(reactor.set_value(input, 2));
    cb1.expect_not_to_have_been_called();
    cb2.expect_to_have_been_called_with(3);
}

#[test]
#[ignore]
fn callbacks_should_only_be_called_once_even_if_multiple_dependencies_change() {
    let cb = CallbackRecorder::new();
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let plus_one = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] + 1)
        .unwrap();
    let minus_one1 = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] - 1)
        .unwrap();
    let minus_one2 = reactor
        .create_compute(&[CellID::Compute(minus_one1)], |v| v[0] - 1)
        .unwrap();
    let output = reactor
        .create_compute(
            &[CellID::Compute(plus_one), CellID::Compute(minus_one2)],
            |v| v[0] * v[1],
        )
        .unwrap();
    assert!(reactor
        .add_callback(output, |v| cb.callback_called(v))
        .is_some());
    assert!(reactor.set_value(input, 4));
    cb.expect_to_have_been_called_with(10);
}

#[test]
#[ignore]
fn callbacks_should_not_be_called_if_dependencies_change_but_output_value_doesnt_change() {
    let cb = CallbackRecorder::new();
    let mut reactor = Reactor::new();
    let input = reactor.create_input(1);
    let plus_one = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] + 1)
        .unwrap();
    let minus_one = reactor
        .create_compute(&[CellID::Input(input)], |v| v[0] - 1)
        .unwrap();
    let always_two = reactor
        .create_compute(
            &[CellID::Compute(plus_one), CellID::Compute(minus_one)],
            |v| v[0] - v[1],
        )
        .unwrap();
    assert!(reactor
        .add_callback(always_two, |v| cb.callback_called(v))
        .is_some());
    for i in 2..5 {
        assert!(reactor.set_value(input, i));
        cb.expect_not_to_have_been_called();
    }
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

    let a_xor_b = reactor
        .create_compute(&[CellID::Input(a), CellID::Input(b)], |v| v[0] ^ v[1])
        .unwrap();
    let sum = reactor
        .create_compute(&[CellID::Compute(a_xor_b), CellID::Input(carry_in)], |v| {
            v[0] ^ v[1]
        })
        .unwrap();

    let a_xor_b_and_cin = reactor
        .create_compute(&[CellID::Compute(a_xor_b), CellID::Input(carry_in)], |v| {
            v[0] && v[1]
        })
        .unwrap();
    let a_and_b = reactor
        .create_compute(&[CellID::Input(a), CellID::Input(b)], |v| v[0] && v[1])
        .unwrap();
    let carry_out = reactor
        .create_compute(
            &[CellID::Compute(a_xor_b_and_cin), CellID::Compute(a_and_b)],
            |v| v[0] || v[1],
        )
        .unwrap();

    let tests = &[
        (false, false, false, false, false),
        (false, false, true, false, true),
        (false, true, false, false, true),
        (false, true, true, true, false),
        (true, false, false, false, true),
        (true, false, true, true, false),
        (true, true, false, true, false),
        (true, true, true, true, true),
    ];

    for &(aval, bval, cinval, expected_cout, expected_sum) in tests {
        assert!(reactor.set_value(a, aval));
        assert!(reactor.set_value(b, bval));
        assert!(reactor.set_value(carry_in, cinval));

        assert_eq!(Some(expected_sum), reactor.value(CellID::Compute(sum)));
        assert_eq!(
            Some(expected_cout),
            reactor.value(CellID::Compute(carry_out))
        );
    }
}
