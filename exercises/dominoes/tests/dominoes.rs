extern crate dominoes;

use CheckResult::*;

type Domino = (usize, usize);

#[derive(Debug)]
enum CheckResult {
    GotInvalid,             // chain returned None
    Correct,
    ChainingFailure(Vec<Domino>), // failure to match the dots at the right side of one domino with
    // the one on the left side of the next
    LengthMismatch(Vec<Domino>),
    DominoMismatch(Vec<Domino>), // different dominoes are used in input and output
}

fn normalize(d: &Domino) -> Domino {
    match d {
        &(m, n) if m > n => (n, m),
        &(m, n) => (m, n)
    }
}

fn check(input: &Vec<Domino>) -> CheckResult {
    let output = match dominoes::chain(input) {
        None => return GotInvalid,
        Some(o) => o
    };
    if input.len() != output.len() {
        return LengthMismatch(output);
    }
    else if input.len() == 0 { // and thus output.len() == 0
        return Correct;
    }

    let mut output_sorted = output.iter().map(|d| normalize(d)).collect::<Vec<Domino>>();
    output_sorted.sort();
    let mut input_sorted = input.iter().map(|d| normalize(d)).collect::<Vec<Domino>>();
    input_sorted.sort();
    if input_sorted != output_sorted {
        return DominoMismatch(output);
    }

    // both input and output have at least 1 element
    // This essentially puts the first element after the last one, thereby making it
    // easy to check whether the domino chains "wraps around".
    let mut fail = false;
    {
        let mut n = output[0].1;
        let iter = output.iter().skip(1).chain(output.iter().take(1));
        for &(first, second) in iter {
            if n != first {
                fail = true;
                break
            }
            n = second
        }
    }
    if fail {
        ChainingFailure(output)
    }
    else {
        Correct
    }
}

fn assert_correct(input: &Vec<Domino>) {
    match check(&input) {
        Correct => (),
        GotInvalid => panic!("Unexpectedly got invalid on input {:?}", input),
        ChainingFailure(output) => panic!("Chaining failure for input {:?}, output {:?}", input, output),
        LengthMismatch(output) => panic!("Length mismatch for input {:?}, output {:?}", input, output),
        DominoMismatch(output) => panic!("Domino mismatch for input {:?}, output {:?}", input, output),
    }
}

#[test]
fn empty_input_empty_output() {
    let input = vec!();
    assert_eq!(dominoes::chain(&input), Some(vec!()));
}

#[test]
#[ignore]
fn singleton_input_singleton_output() {
    let input = vec!((1, 1));
    assert_correct(&input);
}

#[test]
#[ignore]
fn singleton_that_cant_be_chained() {
    let input = vec![(1, 2)];
    assert_eq!(dominoes::chain(&input), None);
}

#[test]
#[ignore]
fn no_repeat_numbers() {
    let input = vec!((1, 2), (3, 1), (2, 3));
    assert_correct(&input);
}

#[test]
#[ignore]
fn can_reverse_dominoes() {
    let input = vec![(1, 2), (1, 3), (2, 3)];
    assert_correct(&input);
}

#[test]
#[ignore]
fn no_chains() {
    let input = vec!((1, 2), (4, 1), (2, 3));
    assert_eq!(dominoes::chain(&input), None);
}

#[test]
#[ignore]
fn disconnected_simple() {
    let input = vec![(1, 1), (2, 2)];
    assert_eq!(dominoes::chain(&input), None);
}

#[test]
#[ignore]
fn disconnected_double_loop() {
    let input = vec![(1, 2), (2, 1), (3, 4), (4, 3)];
    assert_eq!(dominoes::chain(&input), None);
}

#[test]
#[ignore]
fn disconnected_single_isolated() {
    let input = vec![(1, 2), (2, 3), (3, 1), (4, 4)];
    assert_eq!(dominoes::chain(&input), None);
}

#[test]
#[ignore]
fn need_backtrack() {
    let input = vec![(1, 2), (2, 3), (3, 1), (2, 4), (2, 4)];
    assert_correct(&input);
}

#[test]
#[ignore]
fn separate_loops() {
    let input = vec![(1, 2), (2, 3), (3, 1), (1, 1), (2, 2), (3, 3)];
    assert_correct(&input);
}

#[test]
#[ignore]
fn ten_elements() {
    let input = vec!((1, 2), (5, 3), (3, 1), (1, 2), (2, 4), (1, 6), (2, 3), (3, 4), (5, 6));
    assert_correct(&input);
}
