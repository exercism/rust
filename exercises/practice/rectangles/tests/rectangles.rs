use rectangles::*;

#[test]
fn test_no_rows() {
    let input = &[];
    let output = count(input);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_no_columns() {
    let input = &[""];
    let output = count(input);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_no_rectangles() {
    let input = &[" "];
    let output = count(input);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_one_rectangle() {
    #[rustfmt::skip]
    let input = &[
        "+-+",
        "| |",
        "+-+",
    ];
    let output = count(input);
    let expected = 1;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_two_rectangles_without_shared_parts() {
    #[rustfmt::skip]
    let input = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| |  ",
        "+-+  ",
    ];
    let output = count(input);
    let expected = 2;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_five_rectangles_with_shared_parts() {
    #[rustfmt::skip]
    let input = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| | |",
        "+-+-+",
    ];
    let output = count(input);
    let expected = 5;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_rectangle_of_height_1_is_counted() {
    #[rustfmt::skip]
    let input = &[
        "+--+",
        "+--+",
    ];
    let output = count(input);
    let expected = 1;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_rectangle_of_width_1_is_counted() {
    #[rustfmt::skip]
    let input = &[
        "++",
        "||",
        "++",
    ];
    let output = count(input);
    let expected = 1;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_1x1_square_is_counted() {
    #[rustfmt::skip]
    let input = &[
        "++",
        "++",
    ];
    let output = count(input);
    let expected = 1;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_only_complete_rectangles_are_counted() {
    #[rustfmt::skip]
    let input = &[
        "  +-+",
        "    |",
        "+-+-+",
        "| | -",
        "+-+-+",
    ];
    let output = count(input);
    let expected = 1;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_rectangles_can_be_of_different_sizes() {
    #[rustfmt::skip]
    let input = &[
        "+------+----+",
        "|      |    |",
        "+---+--+    |",
        "|   |       |",
        "+---+-------+",
    ];
    let output = count(input);
    let expected = 3;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_corner_is_required_for_a_rectangle_to_be_complete() {
    #[rustfmt::skip]
    let input = &[
        "+------+----+",
        "|      |    |",
        "+------+    |",
        "|   |       |",
        "+---+-------+",
    ];
    let output = count(input);
    let expected = 2;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_large_input_with_many_rectangles() {
    #[rustfmt::skip]
    let input = &[
        "+---+--+----+",
        "|   +--+----+",
        "+---+--+    |",
        "|   +--+----+",
        "+---+--+--+-+",
        "+---+--+--+-+",
        "+------+  | |",
        "          +-+",
    ];
    let output = count(input);
    let expected = 60;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_rectangles_must_have_four_sides() {
    #[rustfmt::skip]
    let input = &[
        "+-+ +-+",
        "| | | |",
        "+-+-+-+",
        "  | |  ",
        "+-+-+-+",
        "| | | |",
        "+-+ +-+",
    ];
    let output = count(input);
    let expected = 5;
    assert_eq!(output, expected);
}
