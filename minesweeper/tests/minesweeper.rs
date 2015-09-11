extern crate minesweeper;

use minesweeper::annotate;

fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars().map(|ch| match ch {
        '*' => '*',
        _ => ' '
    }).collect()
}

fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn empty_board_has_no_annotations() {
    run_test(&[
        "   ",
        "   ",
        "   "
    ]);
}

#[test]
#[ignore]
fn board_full_of_mines_has_no_annotations() {
    run_test(&[
        "***",
        "***",
        "***"
    ]);
}

#[test]
#[ignore]
fn one_horizontal_row_with_one_mine() {
    run_test(&[
        "   1*1 "
    ]);
}

#[test]
#[ignore]
fn one_horizontal_row_with_two_mines() {
    run_test(&[
        " 1*2*1 "
    ]);
}

#[test]
#[ignore]
fn one_horizontal_row_with_one_mine_at_the_left_end() {
    run_test(&[
        "*1  "
    ]);
}

#[test]
#[ignore]
fn one_horizontal_row_with_one_mine_at_the_right_end() {
    run_test(&[
        "  1*"
    ]);
}

#[test]
#[ignore]
fn one_vertical_row_with_one_mine() {
    run_test(&[
        " ",
        "1",
        "*",
        "1",
        " ",
    ]);
}

#[test]
#[ignore]
fn one_vertical_row_with_two_mines() {
    run_test(&[
        " ",
        "1",
        "*",
        "2",
        "*",
        "1",
        " ",
    ]);
}

#[test]
#[ignore]
fn one_vertical_row_with_one_mine_at_the_top() {
    run_test(&[
        "*",
        "1",
        " ",
        " ",
    ]);
}

#[test]
#[ignore]
fn one_vertical_row_with_one_mine_at_the_bottom() {
    run_test(&[
        " ",
        " ",
        "1",
        "*",
    ]);
}

#[test]
#[ignore]
fn one_mine_in_the_middle() {
    run_test(&[
        "***",
        "*8*",
        "***"
    ]);
}

#[test]
#[ignore]
fn complex_case() {
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}